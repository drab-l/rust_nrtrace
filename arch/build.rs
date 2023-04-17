use std::io::{BufReader, BufRead, BufWriter, Write};
use std::collections::{BTreeSet, BTreeMap};

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

fn get_arch_name() -> (&'static str, &'static str) {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "x86_64" || target_arch == "x86" {
        ("x86_64", "x86")
    } else if target_arch == "aarch64" || target_arch == "arm" {
        ("aarch64", "arm")
    } else {
        panic!();
    }
}

fn get_arch_nr_file_path(arch: &str) -> String {
    "../cfg/".to_owned() + arch + "/NR"
}

fn get_arch_types_file_path(arch: &str) -> String {
    "../cfg/".to_owned() + arch + "/types"
}

fn get_default_types_file_path() -> String {
    "../cfg/".to_owned() + "types"
}

fn get_sys_uni_list_path() -> String {
    "../cfg/".to_owned() + "syscall_list"
}

fn get_default_header_dir_path() -> String {
    "../cfg/".to_owned() + "header"
}

fn get_arch_header_dir_path(arch: &str) -> String {
    "../cfg/".to_owned() + arch + "/header"
}

fn get_default_uni_header_dir_path() -> String {
    "../cfg/".to_owned() + "uni_header"
}

fn get_arch_uni_header_dir_path(arch: &str) -> String {
    "../cfg/".to_owned() + arch + "/uni_header"
}

fn get_nr_file_path() -> (String, String) {
    let (a64, a32) = get_arch_name();
    (get_arch_nr_file_path(a64), get_arch_nr_file_path(a32))
}

fn get_types_file_path() -> (String, String, String) {
    let (a64, a32) = get_arch_name();
    (get_arch_types_file_path(a64), get_arch_types_file_path(a32), get_default_types_file_path())
}

fn get_header_dir_path() -> (String, String, String) {
    let (a64, a32) = get_arch_name();
    (get_arch_header_dir_path(a64), get_arch_header_dir_path(a32), get_default_header_dir_path())
}

fn get_uni_header_dir_path() -> (String, String) {
    let (a64, _) = get_arch_name();
    (get_arch_uni_header_dir_path(a64), get_default_uni_header_dir_path())
}

fn get_types_rs_file_path(dir: &str) -> (String, String) {
    ((dir.to_owned() + "/types_64.inc"), (dir.to_owned() + "/types_32.inc"))
}

fn get_nr_rs_file_path(dir: &str) -> (String, String, String) {
    ((dir.to_owned() + "/sys_64.inc"), (dir.to_owned() + "/sys_32.inc"), (dir.to_owned() + "/sys_uni.rs"))
}

fn file_modified_than_file(file: &str, base: &str) -> bool {
    let file = std::fs::metadata(file);
    if file.is_err() {
        return false;
    }
    let base = std::fs::metadata(base);
    if base.is_err() {
        return true;
    }
    let base = base.unwrap().modified();
    if base.is_err() {
        return true;
    }
    let base = base.unwrap();
    match file.unwrap().modified() {
        Ok(t) => {
            t >= base
        }
        _ => true
    }
}

fn load_sys_uni_list(src: &str, set: &mut BTreeSet<String>) {
    for line in BufReader::new(std::fs::File::open(src).unwrap()).lines() {
        let l = line.as_ref().unwrap();
        set.insert(l.to_owned());
    }
}

fn make_nr_inc(src: &str, dst: &str) {
    let mut w = BufWriter::new(std::fs::File::create(dst).unwrap());
    for line in BufReader::new(std::fs::File::open(src).unwrap()).lines() {
        let mut l = line.as_ref().unwrap().split(':');
        w.write(b"const sys_").unwrap();
        let name = l.next().unwrap();
        w.write(name.as_bytes()).unwrap();
        w.write(b": u64 = ").unwrap();
        w.write(l.next().unwrap().as_bytes()).unwrap();
        w.write(b";\n").unwrap();
    }
    w.write(b"\n").unwrap();
    w.write(b"/// Get common syscall enum from raw syscall numer\n").unwrap();
    w.write(b"pub fn to_uni(sys: u64) -> super::NR {\nmatch sys {\n").unwrap();
    for line in BufReader::new(std::fs::File::open(src).unwrap()).lines() {
        let mut l = line.as_ref().unwrap().split(':');
        let name = l.next().unwrap();
        w.write(b"sys_").unwrap();
        w.write(name.as_bytes()).unwrap();
        w.write(b" => super::NR::sys_").unwrap();
        w.write(name.as_bytes()).unwrap();
        w.write(b",\n").unwrap();
    }
    w.write(b"_ => super::NR::sys_unknown,\n").unwrap();
    w.write(b"}\n}\n").unwrap();
}

fn make_uni_enum_inc(dst: &str, set: &BTreeSet<String>) {
    let mut w = BufWriter::new(std::fs::File::create(dst).unwrap());
    w.write(b"/// Common syscall enum for architecture-independent\n").unwrap();
    w.write(b"#[derive(PartialEq,Copy,Clone)]pub enum NR {\n").unwrap();
    set.iter().for_each(|x|{
        w.write(b"sys_").unwrap();
        w.write(x.as_bytes()).unwrap();
        w.write(b",\n").unwrap();
    });
    w.write(b"}\n").unwrap();
}

fn make_uni_to_str_inc(dst: &str, set: &BTreeSet<String>) {
    let mut w = BufWriter::new(std::fs::File::create(dst).unwrap());
    w.write(b"/// Get syscall name\n").unwrap();
    w.write(b"pub fn to_str(sys: NR) -> &'static str {\nmatch sys {\n").unwrap();
    set.iter().for_each(|x|{
        write!(w, "NR::sys_{} => \"{}\",\n", x, x).unwrap();
    });
    w.write(b"}\n}\n").unwrap();
}

fn make_uni_map_inc(dst: &str, set: &BTreeSet<String>) {
    let mut w = BufWriter::new(std::fs::File::create(dst).unwrap());
    w.write(b"/// Map of common syscall enum and name, for lookup syscall by name\n").unwrap();
    write!(w, "pub const map: [(&'static str, NR);{}] = [\n", set.len()).unwrap();
    set.iter().for_each(|x|{
        write!(w, "(\"{}\", NR::sys_{}),\n", x, x).unwrap();
    });
    w.write(b"];\n").unwrap();
}

fn make_uni_nr_rs(dst: &str, set: &BTreeSet<String>) {
    let enum_inc = dst.to_owned() + ".enum.inc";
    make_uni_enum_inc(&enum_inc, set);
    let to_str_inc = dst.to_owned() + ".to_str.inc";
    make_uni_to_str_inc(&to_str_inc, set);
    let to_str_inc = dst.to_owned() + ".map.inc";
    make_uni_map_inc(&to_str_inc, set);
}

fn make_sys_rs (dir: &str) {
    let (a64, a32) = get_nr_file_path();
    let aun = get_sys_uni_list_path();
    let (r64, r32, run) = get_nr_rs_file_path(dir);

    println!("cargo:rerun-if-changed={}", a64);
    println!("cargo:rerun-if-changed={}", a32);
    println!("cargo:rerun-if-changed={}", aun);

    let w64 = ["build.rs", &a64].iter().any(|x|file_modified_than_file(x, &r64));
    let w32 = ["build.rs", &a32].iter().any(|x|file_modified_than_file(x, &r32));
    let wun = w64 || w32;
    if w64 {
        make_nr_inc(&a64, &r64);
    }
    if w32 {
        make_nr_inc(&a32, &r32);
    }
    if wun {
        let mut bs = BTreeSet::new();
        load_sys_uni_list(&aun, &mut bs);
        make_uni_nr_rs(&run, &bs);
    }
}

fn make_types_inc(dst: &str, btm: &BTreeMap<String, String>) {
    let mut w = BufWriter::new(std::fs::File::create(dst).unwrap());
    for (k,v) in btm.iter() {
        write!(w, "pub type {} = {};\n", k, v).unwrap();
    };
}

fn load_types_uni_list(src: &str, b64: &mut BTreeMap<String, String>, b32: &mut BTreeMap<String, String>) {
    for line in BufReader::new(std::fs::File::open(src).unwrap()).lines() {
        let mut l = line.as_ref().unwrap().split(':');
        let name = l.next().unwrap();
        let n64 = l.next().unwrap();
        let n32 = l.next().unwrap();
        b64.insert(name.to_owned(), n64.to_owned());
        b32.insert(name.to_owned(), n32.to_owned());
    }
}

fn load_types_list(src: &str, btm: &mut BTreeMap<String, String>) -> std::io::Result<()> {
    for line in BufReader::new(std::fs::File::open(src)?).lines() {
        let mut l = line.as_ref().unwrap().split(':');
        let name = l.next().unwrap();
        let n = l.next().unwrap();
        btm.insert(name.to_owned(), n.to_owned());
    }
    Ok(())
}

fn is_file_exist(dir: &str, e: &std::fs::DirEntry) -> bool {
    let mut path  = std::path::PathBuf::new();
    path.push(dir);
    path.push(e.file_name());
    path.is_file()
}

fn select_copy(dst: &str, src1: &str, src2: &str, e: &std::fs::DirEntry) {
    let mut dpath  = std::path::PathBuf::new();
    let mut spath  = std::path::PathBuf::new();
    dpath.push(dst);
    dpath.push(e.file_name());
    if is_file_exist(src1, e) {
        spath.push(src1);
    } else {
        spath.push(src2);
    }
    spath.push(e.file_name());
    if file_modified_than_file(spath.to_str().unwrap(), dpath.to_str().unwrap()) {
        println!("cargo:rerun-if-changed={}", spath.to_str().unwrap());
        std::fs::create_dir_all(dst).unwrap();
        std::fs::copy(spath, dpath).unwrap();
    }
}

fn make_header_inc(dst: &str) {
    let (a64, a32, aun) = get_header_dir_path();
    let d64 = dst.to_owned() + "/header/a64";
    let d32 = dst.to_owned() + "/header/a32";
    println!("cargo:rerun-if-changed={}", a64);
    println!("cargo:rerun-if-changed={}", a32);
    println!("cargo:rerun-if-changed={}", aun);
    for h in std::fs::read_dir(&aun).unwrap() {
        let h = h.unwrap();
        if !h.file_type().unwrap().is_file() {
            continue;
        }
        select_copy(&d64, &a64, &aun, &h);
        select_copy(&d32, &a32, &aun, &h);
    }
}

fn make_uni_header_inc(dst: &str) {
    let (a64, aun) = get_uni_header_dir_path();
    let dun = dst.to_owned() + "/uni_header";
    println!("cargo:rerun-if-changed={}", a64);
    println!("cargo:rerun-if-changed={}", aun);
    for h in std::fs::read_dir(&aun).unwrap() {
        let h = h.unwrap();
        if !h.file_type().unwrap().is_file() {
            continue;
        }
        select_copy(&dun, &a64, &aun, &h);
    }
}

fn make_types_rs (dir: &str) {
    let (a64, a32, aun) = get_types_file_path();
    let (r64, r32) = get_types_rs_file_path(dir);

    println!("cargo:rerun-if-changed={}", a64);
    println!("cargo:rerun-if-changed={}", a32);
    println!("cargo:rerun-if-changed={}", aun);

    let w64 = ["build.rs", &a64, &aun].iter().any(|x|file_modified_than_file(x, &r64));
    let w32 = ["build.rs", &a32, &aun].iter().any(|x|file_modified_than_file(x, &r32));
    let mut b64 = BTreeMap::new();
    let mut b32 = BTreeMap::new();
    load_types_uni_list(&aun, &mut b64, &mut b32);
    load_types_list(&a64, &mut b64).unwrap_or(());
    load_types_list(&a32, &mut b32).unwrap_or(());
    if w64 {
        make_types_inc(&r64, &b64);
    }
    if w32 {
        make_types_inc(&r32, &b32);
    }
}

fn main () {
    let out = std::env::var("OUT_DIR").unwrap();
    make_sys_rs(&out);
    make_types_rs(&out);
    make_header_inc(&out);
    make_uni_header_inc(&out);
}
