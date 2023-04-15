use std::io::{BufWriter, Write};

fn file_modified_than_file (file: &str, base: &str) -> bool {
    let file = std::fs::metadata(file).unwrap();
    let base = std::fs::metadata(base);
    if base.is_err() {
        return true;
    }
    let base = base.unwrap().modified();
    if base.is_err() {
        return true;
    }
    let base = base.unwrap();
    match file.modified() {
        Ok(t) => {
            t >= base
        }
        _ => true
    }
}

fn make_types_rs () {
    let out = std::env::var("OUT_DIR").unwrap();
    let path = out + "/types.inc";
    if ! file_modified_than_file("build.rs", &path) {
        return
    }
    #[cfg(target_pointer_width = "64")]
    let bits = 64;
    #[cfg(target_pointer_width = "32")]
    let bits= 32;
    let mut w = BufWriter::new(std::fs::File::create(path).unwrap());
write!(w,r#"
pub type SChar = i8;
pub type UChar = u8;
pub type SShrt = i16;
pub type UShrt = u16;
pub type SInt = i32;
pub type UInt = u32;
pub type SLong = i{};
pub type ULong = u{};
pub type SLLong = i{};
pub type ULLong = u{};

pub type Pid = SInt;
pub type Ptr = usize;

pub const BITS:u8 = {};
"#, bits, bits, bits, bits, bits).unwrap();
}

fn main () {
    make_types_rs();
}
