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
/// signed char
pub type SChar = i8;
/// unsigned char
pub type UChar = u8;
/// signed short
pub type SShrt = i16;
/// unsigned short
pub type UShrt = u16;
/// signed int
pub type SInt = i32;
/// unsigned int
pub type UInt = u32;
/// signed long
pub type SLong = i{};
/// unsigned long
pub type ULong = u{};
/// signed long long
pub type SLLong = i{};
/// unsigned long long
pub type ULLong = u{};

/// pid_t
pub type Pid = SInt;
/// void*
pub type Ptr = usize;

/// size_t
pub type SSizeT = isize;
/// unsigned long
pub type USizeT = usize;

/// 64 or 32
pub const BITS:u8 = {};
"#, bits, bits, bits, bits, bits).unwrap();
}

fn main () {
    make_types_rs();
}
