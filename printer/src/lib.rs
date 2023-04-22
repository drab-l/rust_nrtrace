//! Print SyscallSummery
#![allow(non_snake_case)]
use arch::types::{a64, a32};

mod logger;
mod number;
mod config;
mod madvise;
mod mmap;
mod stat;
mod statfs;
mod rlimit;
mod epoll;
mod open;
mod dirent;
mod socket;
mod sockaddr;
mod iovec;
mod uname;
mod errno;
mod time;
mod sys;

use number::ToString;
use config::{TYPES, FORMATS};

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

#[allow(unused_macros)]
macro_rules! print_type_as_hex {
    ($self:ident, $value:expr) => {
        $self.write($value.htoa().as_bytes()).unwrap();
    };
}

macro_rules! print_bit_number {
    ($self:ident, $value:expr, $type:tt, $fmt:ident, $summery:expr) => {
        if $summery.is_64() {
            $self.write_number($value as a64::$type, $fmt)
        } else {
            $self.write_number($value as a32::$type, $fmt)
        }
    };
}

macro_rules! print_bit_pointer {
    ($self:ident, $value:expr, $summery:expr) => {
        if $value == 0 {
            $self.write(b"NULL")
        } else  {
            if $summery.is_64() {
                $self.write_number_as_pointer($value as a64::Ptr)
            } else {
                $self.write_number_as_pointer($value as a32::Ptr)
            }
        }
    };
}

macro_rules! peek_print_number {
    ($self:ident, $addr:expr, $type:ty, $fmt:ident, $pid:ident, $summery:expr) => {
        if $addr == 0 {
            $self.write(b"NULL")
        } else  {
            if let Ok(value) = peek::peek_data::<$type>($pid, $addr) {
                $self.write(b"{")?;
                $self.write_number(value, $fmt)?;
                $self.write(b"}")
            } else {
                print_bit_pointer!($self, $addr, $summery)
            }
        }
    };
}

macro_rules! peek_print_bit_number {
    ($self:ident, $addr:expr, $type:tt, $fmt:ident, $pid:ident, $summery:expr) => {
        if $summery.is_64() {
            peek_print_number!($self, $addr, a64::$type, $fmt, $pid, $summery)
        } else {
            peek_print_number!($self, $addr, a32::$type, $fmt, $pid, $summery)
        }
    };
}

macro_rules! peek_write_struct {
    ($self:ident, $addr:ident, $type:ty, $pid:ident, $summery:ident) => {
        $self.peek_write_struct::<$type>($addr as types::Ptr, $pid, $summery)
    };
}

macro_rules! peek_write_bit_struct {
    ($self:ident, $addr:ident, $type64:ty, $type32:ty, $pid:ident, $summery:ident) => {
        $self.peek_write_bit_struct::<$type64, $type32>($addr as types::Ptr, $pid, $summery)
    };
}

macro_rules! peek_write_struct_array {
    ($self:ident, $addr:ident, $type:ty, $len:expr, $pid:ident, $summery:ident) => {
        $self.peek_write_struct_array::<$type>($addr as types::Ptr, $len as usize, $pid, $summery)
    };
}

macro_rules! peek_write_bit_struct_array {
    ($self:ident, $addr:ident, $type64:ty, $type32:ty, $len:expr, $pid:ident, $summery:ident) => {
        if $summery.is_64() {
            $self.peek_write_struct_array::<$type64>($addr as types::Ptr, $len as usize, $pid, $summery)
        } else {
            $self.peek_write_struct_array::<$type32>($addr as types::Ptr, $len as usize, $pid, $summery)
        }
    };
}

const fn sizeof<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

trait Print {
    fn print(&self, printer: &mut Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error>;
    fn print_flex_tail(&self, _printer: &mut Printer, _buf: &[u8], _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> { Ok(()) }
    fn print_array_prefix(_printer: &mut Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> { Ok(()) }
    fn print_array_delim(printer: &mut Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> { printer.write(b", ") }
    fn print_array_suffix(_printer: &mut Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> { Ok(()) }
    fn flex_tail_size(&self) -> usize { 0 }
    fn flex_tail_offset(&self) -> usize { 0 }
    fn total_size(&self) -> usize where Self: Sized { std::mem::size_of::<Self>() }
}

/// Module interface struct
pub struct Printer {
    writer: logger::Logger,
    conf: config::Config,
    prv_data: config::PrivData,
}

impl Printer {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        self.writer.write(buf)?;
        Ok(())
    }

    fn flush_line(&mut self) -> std::result::Result<(), std::io::Error> {
        self.writer.flush_line()
    }

    fn peek_vec(&mut self, addr: types::Ptr, size: usize, pid: types::Pid) -> std::result::Result<Vec<u8>, std::io::Error> {
        let size = std::cmp::min(4096 as usize, size);
        let mut buf = Vec::<u8>::with_capacity(size);
        peek::peek_vec(pid, addr, &mut buf, size)?;
        Ok(buf)
    }

    fn peek_vec_align<T>(&mut self, addr: types::Ptr, size: usize, pid: types::Pid) -> std::result::Result<Vec<u8>, std::io::Error> {
        let elem = (size + std::mem::size_of::<T>() - 1) / std::mem::size_of::<T>();
        let buf = Vec::<T>::with_capacity(elem);
        let mut buf = std::mem::ManuallyDrop::new(buf);
        let mut buf = unsafe { Vec::<u8>::from_raw_parts(buf.as_mut_ptr().cast::<u8>(), 0, std::mem::size_of::<T>() * buf.capacity()) };
        peek::peek_vec(pid, addr, &mut buf, size)?;
        Ok(buf)
    }

    fn write_width(&mut self, buf: &[u8], n: usize) -> std::result::Result<(), std::io::Error> {
        const SPACE: [u8; 20] = [' ' as u8; 20];
        let len = buf.len();
        let pad_len = std::cmp::min(SPACE.len(), if len < n { n - len } else { 0 });
        self.write(&SPACE[(SPACE.len() - pad_len)..])?;
        self.write(buf)
    }

    fn write_hex_char(&mut self, hex: u8) -> std::result::Result<(), std::io::Error> {
        const TABLE: [&'static str; 256] = [
            r#"\x00"#, r#"\x01"#, r#"\x02"#, r#"\x03"#, r#"\x04"#, r#"\x05"#, r#"\x06"#, r#"\x07"#, r#"\x08"#, r#"\x09"#, r#"\x0a"#, r#"\x0b"#, r#"\x0c"#, r#"\x0d"#, r#"\x0e"#, r#"\x0f"#,
            r#"\x10"#, r#"\x11"#, r#"\x12"#, r#"\x13"#, r#"\x14"#, r#"\x15"#, r#"\x16"#, r#"\x17"#, r#"\x18"#, r#"\x19"#, r#"\x1a"#, r#"\x1b"#, r#"\x1c"#, r#"\x1d"#, r#"\x1e"#, r#"\x1f"#,
            r#"\x20"#, r#"\x21"#, r#"\x22"#, r#"\x23"#, r#"\x24"#, r#"\x25"#, r#"\x26"#, r#"\x27"#, r#"\x28"#, r#"\x29"#, r#"\x2a"#, r#"\x2b"#, r#"\x2c"#, r#"\x2d"#, r#"\x2e"#, r#"\x2f"#,
            r#"\x30"#, r#"\x31"#, r#"\x32"#, r#"\x33"#, r#"\x34"#, r#"\x35"#, r#"\x36"#, r#"\x37"#, r#"\x38"#, r#"\x39"#, r#"\x3a"#, r#"\x3b"#, r#"\x3c"#, r#"\x3d"#, r#"\x3e"#, r#"\x3f"#,
            r#"\x40"#, r#"\x41"#, r#"\x42"#, r#"\x43"#, r#"\x44"#, r#"\x45"#, r#"\x46"#, r#"\x47"#, r#"\x48"#, r#"\x49"#, r#"\x4a"#, r#"\x4b"#, r#"\x4c"#, r#"\x4d"#, r#"\x4e"#, r#"\x4f"#,
            r#"\x50"#, r#"\x51"#, r#"\x52"#, r#"\x53"#, r#"\x54"#, r#"\x55"#, r#"\x56"#, r#"\x57"#, r#"\x58"#, r#"\x59"#, r#"\x5a"#, r#"\x5b"#, r#"\x5c"#, r#"\x5d"#, r#"\x5e"#, r#"\x5f"#,
            r#"\x60"#, r#"\x61"#, r#"\x62"#, r#"\x63"#, r#"\x64"#, r#"\x65"#, r#"\x66"#, r#"\x67"#, r#"\x68"#, r#"\x69"#, r#"\x6a"#, r#"\x6b"#, r#"\x6c"#, r#"\x6d"#, r#"\x6e"#, r#"\x6f"#,
            r#"\x70"#, r#"\x71"#, r#"\x72"#, r#"\x73"#, r#"\x74"#, r#"\x75"#, r#"\x76"#, r#"\x77"#, r#"\x78"#, r#"\x79"#, r#"\x7a"#, r#"\x7b"#, r#"\x7c"#, r#"\x7d"#, r#"\x7e"#, r#"\x7f"#,
            r#"\x80"#, r#"\x81"#, r#"\x82"#, r#"\x83"#, r#"\x84"#, r#"\x85"#, r#"\x86"#, r#"\x87"#, r#"\x88"#, r#"\x89"#, r#"\x8a"#, r#"\x8b"#, r#"\x8c"#, r#"\x8d"#, r#"\x8e"#, r#"\x8f"#,
            r#"\x90"#, r#"\x91"#, r#"\x92"#, r#"\x93"#, r#"\x94"#, r#"\x95"#, r#"\x96"#, r#"\x97"#, r#"\x98"#, r#"\x99"#, r#"\x9a"#, r#"\x9b"#, r#"\x9c"#, r#"\x9d"#, r#"\x9e"#, r#"\x9f"#,
            r#"\xa0"#, r#"\xa1"#, r#"\xa2"#, r#"\xa3"#, r#"\xa4"#, r#"\xa5"#, r#"\xa6"#, r#"\xa7"#, r#"\xa8"#, r#"\xa9"#, r#"\xaa"#, r#"\xab"#, r#"\xac"#, r#"\xad"#, r#"\xae"#, r#"\xaf"#,
            r#"\xb0"#, r#"\xb1"#, r#"\xb2"#, r#"\xb3"#, r#"\xb4"#, r#"\xb5"#, r#"\xb6"#, r#"\xb7"#, r#"\xb8"#, r#"\xb9"#, r#"\xba"#, r#"\xbb"#, r#"\xbc"#, r#"\xbd"#, r#"\xbe"#, r#"\xbf"#,
            r#"\xc0"#, r#"\xc1"#, r#"\xc2"#, r#"\xc3"#, r#"\xc4"#, r#"\xc5"#, r#"\xc6"#, r#"\xc7"#, r#"\xc8"#, r#"\xc9"#, r#"\xca"#, r#"\xcb"#, r#"\xcc"#, r#"\xcd"#, r#"\xce"#, r#"\xcf"#,
            r#"\xd0"#, r#"\xd1"#, r#"\xd2"#, r#"\xd3"#, r#"\xd4"#, r#"\xd5"#, r#"\xd6"#, r#"\xd7"#, r#"\xd8"#, r#"\xd9"#, r#"\xda"#, r#"\xdb"#, r#"\xdc"#, r#"\xdd"#, r#"\xde"#, r#"\xdf"#,
            r#"\xe0"#, r#"\xe1"#, r#"\xe2"#, r#"\xe3"#, r#"\xe4"#, r#"\xe5"#, r#"\xe6"#, r#"\xe7"#, r#"\xe8"#, r#"\xe9"#, r#"\xea"#, r#"\xeb"#, r#"\xec"#, r#"\xed"#, r#"\xee"#, r#"\xef"#,
            r#"\xf0"#, r#"\xf1"#, r#"\xf2"#, r#"\xf3"#, r#"\xf4"#, r#"\xf5"#, r#"\xf6"#, r#"\xf7"#, r#"\xf8"#, r#"\xf9"#, r#"\xfa"#, r#"\xfb"#, r#"\xfc"#, r#"\xfd"#, r#"\xfe"#, r#"\xff"#,
        ];
        self.write(TABLE[hex as usize].as_bytes())
    }

    fn write_hex(&mut self, hex: u8) -> std::result::Result<(), std::io::Error> {
        const TABLE: [&'static str; 256] = [
            r#"00"#, r#"01"#, r#"02"#, r#"03"#, r#"04"#, r#"05"#, r#"06"#, r#"07"#, r#"08"#, r#"09"#, r#"0a"#, r#"0b"#, r#"0c"#, r#"0d"#, r#"0e"#, r#"0f"#,
            r#"10"#, r#"11"#, r#"12"#, r#"13"#, r#"14"#, r#"15"#, r#"16"#, r#"17"#, r#"18"#, r#"19"#, r#"1a"#, r#"1b"#, r#"1c"#, r#"1d"#, r#"1e"#, r#"1f"#,
            r#"20"#, r#"21"#, r#"22"#, r#"23"#, r#"24"#, r#"25"#, r#"26"#, r#"27"#, r#"28"#, r#"29"#, r#"2a"#, r#"2b"#, r#"2c"#, r#"2d"#, r#"2e"#, r#"2f"#,
            r#"30"#, r#"31"#, r#"32"#, r#"33"#, r#"34"#, r#"35"#, r#"36"#, r#"37"#, r#"38"#, r#"39"#, r#"3a"#, r#"3b"#, r#"3c"#, r#"3d"#, r#"3e"#, r#"3f"#,
            r#"40"#, r#"41"#, r#"42"#, r#"43"#, r#"44"#, r#"45"#, r#"46"#, r#"47"#, r#"48"#, r#"49"#, r#"4a"#, r#"4b"#, r#"4c"#, r#"4d"#, r#"4e"#, r#"4f"#,
            r#"50"#, r#"51"#, r#"52"#, r#"53"#, r#"54"#, r#"55"#, r#"56"#, r#"57"#, r#"58"#, r#"59"#, r#"5a"#, r#"5b"#, r#"5c"#, r#"5d"#, r#"5e"#, r#"5f"#,
            r#"60"#, r#"61"#, r#"62"#, r#"63"#, r#"64"#, r#"65"#, r#"66"#, r#"67"#, r#"68"#, r#"69"#, r#"6a"#, r#"6b"#, r#"6c"#, r#"6d"#, r#"6e"#, r#"6f"#,
            r#"70"#, r#"71"#, r#"72"#, r#"73"#, r#"74"#, r#"75"#, r#"76"#, r#"77"#, r#"78"#, r#"79"#, r#"7a"#, r#"7b"#, r#"7c"#, r#"7d"#, r#"7e"#, r#"7f"#,
            r#"80"#, r#"81"#, r#"82"#, r#"83"#, r#"84"#, r#"85"#, r#"86"#, r#"87"#, r#"88"#, r#"89"#, r#"8a"#, r#"8b"#, r#"8c"#, r#"8d"#, r#"8e"#, r#"8f"#,
            r#"90"#, r#"91"#, r#"92"#, r#"93"#, r#"94"#, r#"95"#, r#"96"#, r#"97"#, r#"98"#, r#"99"#, r#"9a"#, r#"9b"#, r#"9c"#, r#"9d"#, r#"9e"#, r#"9f"#,
            r#"a0"#, r#"a1"#, r#"a2"#, r#"a3"#, r#"a4"#, r#"a5"#, r#"a6"#, r#"a7"#, r#"a8"#, r#"a9"#, r#"aa"#, r#"ab"#, r#"ac"#, r#"ad"#, r#"ae"#, r#"af"#,
            r#"b0"#, r#"b1"#, r#"b2"#, r#"b3"#, r#"b4"#, r#"b5"#, r#"b6"#, r#"b7"#, r#"b8"#, r#"b9"#, r#"ba"#, r#"bb"#, r#"bc"#, r#"bd"#, r#"be"#, r#"bf"#,
            r#"c0"#, r#"c1"#, r#"c2"#, r#"c3"#, r#"c4"#, r#"c5"#, r#"c6"#, r#"c7"#, r#"c8"#, r#"c9"#, r#"ca"#, r#"cb"#, r#"cc"#, r#"cd"#, r#"ce"#, r#"cf"#,
            r#"d0"#, r#"d1"#, r#"d2"#, r#"d3"#, r#"d4"#, r#"d5"#, r#"d6"#, r#"d7"#, r#"d8"#, r#"d9"#, r#"da"#, r#"db"#, r#"dc"#, r#"dd"#, r#"de"#, r#"df"#,
            r#"e0"#, r#"e1"#, r#"e2"#, r#"e3"#, r#"e4"#, r#"e5"#, r#"e6"#, r#"e7"#, r#"e8"#, r#"e9"#, r#"ea"#, r#"eb"#, r#"ec"#, r#"ed"#, r#"ee"#, r#"ef"#,
            r#"f0"#, r#"f1"#, r#"f2"#, r#"f3"#, r#"f4"#, r#"f5"#, r#"f6"#, r#"f7"#, r#"f8"#, r#"f9"#, r#"fa"#, r#"fb"#, r#"fc"#, r#"fd"#, r#"fe"#, r#"ff"#,
        ];
        self.write(TABLE[hex as usize].as_bytes())
    }

    fn write_hex_str_dump(&mut self, hex: &[u8]) -> std::result::Result<(), std::io::Error> {
        for h in hex.iter() { self.write_hex_char(*h)?; }
        Ok(())
    }

    fn write_as_hex(&mut self, hex: &[u8]) -> std::result::Result<(), std::io::Error> {
        for h in hex.iter() { self.write_hex(*h)?; }
        Ok(())
    }

    fn write_head_graph_ascii<'a>(&mut self, buf: &'a [u8]) -> std::result::Result<&'a [u8], std::io::Error> {
        for i in 0..buf.len()  {
            if buf[i] < 0x20 || buf[i] > 0x7e {
                self.write(&buf[0..i])?;
                return Ok(&buf[i..]);
            }
        }
        self.write(buf)?;
        Ok(&buf[buf.len()..])
    }

    fn write_head_non_graph_ascii_as_hex<'a>(&mut self, buf: &'a [u8]) -> std::result::Result<&'a [u8], std::io::Error> {
        for i in 0..buf.len() {
            if buf[i] < 0x20 || buf[i] > 0x7e {
                self.write_hex_char(buf[i])?;
            } else {
                return Ok(&buf[i..]);
            }
        }
        Ok(&buf[buf.len()..])
    }

    fn write_graph_ascii_or_hex(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        let mut buf = buf;
        loop {
            if buf.len() == 0 { break; }
            buf = self.write_head_graph_ascii(buf)?;
            if buf.len() == 0 { break; }
            buf = self.write_head_non_graph_ascii_as_hex(buf)?;
        }
        Ok(())
    }

    fn write_maybe_ascii(&mut self, buf: &[u8]) -> std::result::Result<(), std::io::Error> {
        match buf.iter().find(|x| (**x == 0x00 || **x > 0x7e)) {
            Some(_) => self.write_hex_str_dump(buf),
            None => self.write_graph_ascii_or_hex(buf),
        }
    }

    #[allow(dead_code)]
    fn peek_write_graph_ascii_or_hex(&mut self, addr: types::Ptr, size: usize, pid: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let buf = self.peek_vec(addr, size, pid)?;
        self.write_graph_ascii_or_hex(buf.as_slice())
    }

    fn peek_write_maybe_ascii(&mut self, addr: types::Ptr, size: usize, pid: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let buf = self.peek_vec(addr, size, pid)?;
        self.write_maybe_ascii(buf.as_slice())
    }

    fn peek_write_maybe_ascii_str(&mut self, addr: types::Ptr, size: usize, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"\"")?;
        self.peek_write_maybe_ascii(addr, size, pid, e)?;
        self.write(b"\"")?;
        Ok(())
    }

    fn peek_write_as_hex(&mut self, addr: types::Ptr, size: usize, pid: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        if addr == 0 {
            self.write(b"NULL")?;
        } else {
            self.write(b"{")?;
            let buf = self.peek_vec(addr, size, pid)?;
            self.write_as_hex(buf.as_slice())?;
            self.write(b"}")?;
        }
        Ok(())
    }

    fn peek_write(&mut self, addr: types::Ptr, size: usize, pid: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let buf = self.peek_vec(addr, size, pid)?;
        self.write(buf.as_slice())
    }

    fn write_number_as_pointer<T: number::ToPtrString>(&mut self, value: T) -> std::result::Result<(), std::io::Error> {
        self.write(value.ptoa().as_bytes())
    }

    fn write_number_zero_fill<T: number::ToPtrString>(&mut self, value: T) -> std::result::Result<(), std::io::Error> {
        self.write(value.ptoa().as_bytes())
    }

    fn write_number<T: number::ToString>(&mut self, value: T, fmt: &FORMATS) -> std::result::Result<(), std::io::Error> {
        match fmt {
            FORMATS::HEX => self.write(value.htoa().as_bytes()),
            FORMATS::DEC => self.write(value.dtoa().as_bytes()),
            FORMATS::OCT => self.write(value.otoa().as_bytes()),
        }
    }

    fn write_number_array<T: number::ToString + Copy>(&mut self, value: &[T], fmt: &FORMATS) -> std::result::Result<(), std::io::Error> {
        self.write(b"{")?;
        for (i,v) in value.iter().enumerate() {
            if i != 0 {
                self.write(b", ")?;
            }
            self.write_number(*v, fmt)?;
        }
        self.write(b"}")?;
        Ok(())
    }

    fn write_enum<T: PartialEq + number::ToString>(&mut self, value: T, tbl: &[(T, &'static str)]) -> std::result::Result<(), std::io::Error> {
        for (v,n) in tbl.iter() {
            if value == *v {
                return self.write(n.as_bytes());
            }
        }
        self.write_number(value, &FORMATS::HEX)
    }

    fn write_mask_enum<T>(&mut self, value: T, tbl: &[(T, &'static str)]) -> std::result::Result<(), std::io::Error>
    where
        T: number::ToString + From<u8> + Copy + PartialEq + std::ops::BitAnd<Output = T> + std::ops::BitXor<Output = T>,
    {
        let mut value = value;
        let mut tail = false;
        for (v,n) in tbl.iter() {
            if (value & *v) == *v  {
                if tail {
                    self.write(b" | ")?;
                }
                self.write(n.as_bytes())?;
                tail = true;
                value = value ^ *v;
            }
        }
        if value != T::from(0 as u8) {
            if tail {
                self.write(b" | ")?;
            }
            self.write_number(value, &FORMATS::HEX)?;
        }
        Ok(())
    }

    fn peek_write_str_null_sentinel(&mut self, addr: types::Ptr, pid: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"\"")?;
        self.write_graph_ascii_or_hex(&peek::peek_until_null(pid, addr)?)?;
        self.write(b"\"")?;
        Ok(())
    }

    fn peek_write_execve_str_args(&mut self, addr: types::Ptr, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"{")?;
        let mut addr = addr;
        loop {
            if e.is_64() {
                let s = peek::peek_data::<a64::Ptr>(pid, addr)?;
                if s == 0 { break; }
                self.peek_write_str_null_sentinel(s as types::Ptr, pid, e)?;
                addr += std::mem::size_of::<a64::Ptr>();
            } else {
                let s = peek::peek_data::<a32::Ptr>(pid, addr)?;
                if s == 0 { break; }
                self.peek_write_str_null_sentinel(s as types::Ptr, pid, e)?;
                addr += std::mem::size_of::<a32::Ptr>();
            }
            self.write(b", ")?;
        }
        self.write(b"NULL")?;
        self.write(b"}")
    }

    fn write_struct_with_tail<T: Print>(&mut self, data: &T, tail: &[u8], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"{")?;
        data.print(self, pid, e)?;
        data.print_flex_tail(self, &tail, pid, e)?;
        self.write(b"}")
    }

    fn write_struct_none_tail<T: Print>(&mut self, data: &T, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"{")?;
        data.print(self, pid, e)?;
        self.write(b"}")
    }

    fn write_struct_from_buf<T: Print>(&mut self, buf: &[u8], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<usize, std::io::Error> {
        if buf.len() < std::mem::size_of::<T>() {
            self.write(b"{}")?;
            Ok(buf.len())
        } else {
            let data  = unsafe { buf.as_ptr().cast::<T>().as_ref().unwrap() };
            let offset = data.flex_tail_offset();
            let len = data.flex_tail_size();
            if offset != 0 && len != 0 {
                let len = std::cmp::min(offset + len, buf.len());
                let tail = &buf[offset..len];
                self.write_struct_with_tail(data, tail, pid, e)?;
            } else {
                self.write_struct_none_tail(data, pid, e)?; 
            }
            Ok(data.total_size())
        }
    }

    fn peek_write_struct_impl<T: Print>(&mut self, addr: types::Ptr, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<usize, std::io::Error> {
        if addr == 0 {
            self.write(b"NULL")?;
            Ok(0)
        } else {
            let data = peek::peek_data::<T>(pid, addr)?;
            let offset = data.flex_tail_offset();
            let len = data.flex_tail_size();
            let tail = if offset != 0 && len != 0 {
                self.peek_vec(addr + offset, len, pid)?
            } else {
                vec![]
            };
            self.write_struct_with_tail(&data, &tail, pid, e)?;
            Ok(data.total_size())
        }
    }

    fn peek_write_struct<T: Print>(&mut self, addr: types::Ptr, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.peek_write_struct_impl::<T>(addr, pid, e)?;
        Ok(())
    }

    fn peek_write_bit_struct_impl<T: Print, U: Print>(&mut self, addr: types::Ptr, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<usize, std::io::Error> {
        if e.is_64() {
            self.peek_write_struct_impl::<T>(addr, pid, e)
        } else {
            self.peek_write_struct_impl::<U>(addr, pid, e)
        }
    }

    fn peek_write_bit_struct<T: Print, U: Print>(&mut self, addr: types::Ptr, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.peek_write_bit_struct_impl::<T, U>(addr, pid, e)?;
        Ok(())
    }

    fn peek_write_struct_array<T: Print>(&mut self, addr: types::Ptr, elem: usize, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        if addr == 0 {
            self.write(b"NULL")
        } else {
            self.peek_write_flex_tail_struct_array::<T>(addr, elem * std::mem::size_of::<T>() , pid, e)
        }
    }

    fn write_flex_tail_struct_array_from_buf<T: Print>(&mut self, buf: &[u8], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let mut len = 0;
        let bytes = buf.len();
        self.write(b"{")?;
        while len < bytes {
            if len != 0 {
                T::print_array_delim(self, pid, e)?;
            } else {
                T::print_array_prefix(self, pid, e)?;
            }
            len += self.write_struct_from_buf::<T>(&buf[len..], pid, e)?;
        }
        T::print_array_suffix(self, pid, e)?;
        self.write(b"}")
    }

    fn write_struct_array<T: Print>(&mut self, array: &[T], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"{")?;
        for (index, elem) in array.iter().enumerate() {
            if index != 0 {
                T::print_array_delim(self, pid, e)?;
            } else {
                T::print_array_prefix(self, pid, e)?;
            }
            elem.print(self, pid, e)?;
        }
        T::print_array_suffix(self, pid, e)?;
        self.write(b"}")
    }

    fn peek_write_flex_tail_struct_array<T: Print>(&mut self, addr: types::Ptr, bytes: usize, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        if addr == 0 {
            self.write(b"NULL")
        } else {
            let buf = self.peek_vec_align::<T>(addr, bytes, pid)?;
            self.write_flex_tail_struct_array_from_buf::<T>(&buf, pid, e)
        }
    }

    fn peek_write_callback<T, U>(&mut self, addr: types::Ptr, size: usize, cb: U, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error>
    where
        U: Fn(&mut Printer, &[u8], types::Pid, &peek::SyscallSummery) -> std::result::Result<(), std::io::Error>
    {
        if addr == 0 {
            self.write(b"NULL")
        } else {
            let buf = self.peek_vec_align::<T>(addr, size, pid)?;
            cb(self, buf.as_slice(), pid, e)
        }
    }

    fn write_any_type(&mut self, value: u64, print: &TYPES, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        match print {
            TYPES::U8(fmt) => { self.write_number(value as u8, fmt) },
            TYPES::U16(fmt) => { self.write_number(value as u16, fmt) },
            TYPES::U32(fmt) => { self.write_number(value as u32, fmt) },
            TYPES::U64(fmt) => { self.write_number(value as u64, fmt) },
            TYPES::I8(fmt) => { self.write_number(value as i8, fmt) },
            TYPES::I16(fmt) => { self.write_number(value as i16, fmt) },
            TYPES::I32(fmt) => { self.write_number(value as i32, fmt) },
            TYPES::I64(fmt) => { self.write_number(value as i64, fmt) },
            TYPES::UINT(fmt) => { print_bit_number!(self, value, UInt, fmt, e) },
            TYPES::INT(fmt) => { print_bit_number!(self, value, SInt, fmt, e) },
            TYPES::ULONG(fmt) => { print_bit_number!(self, value, ULong, fmt, e) },
            TYPES::LONG(fmt) => { print_bit_number!(self, value, SLong, fmt, e) },
            TYPES::SSIZE(fmt) => { print_bit_number!(self, value, SSizeT, fmt, e) },
            TYPES::USIZE(fmt) => { print_bit_number!(self, value, USizeT, fmt, e) },

            TYPES::PID => { self.write_number(value as types::Pid, &FORMATS::DEC) },
            TYPES::PTR => { print_bit_pointer!(self, value, e) },

            TYPES::IntPtr(fmt) => { peek_print_bit_number!(self, value as types::Ptr, SInt, fmt, pid, e) },
            TYPES::StrPtr => { self.peek_write_str_null_sentinel(value as types::Ptr, pid, e) },
            TYPES::StrPtrLenArgR => { self.peek_write(value as types::Ptr, e.return_value()? as usize, pid, e) },
            TYPES::ArgsPtr => { self.peek_write_execve_str_args(value as types::Ptr, pid, e) },

            TYPES::FdsetPtrArg1 => { self.peek_write_as_hex(value as types::Ptr, std::cmp::max(1, ((e.argn(peek::Arg::ONE) as usize) + 7) / 8), pid, e) },
            TYPES::IntArrayPtrLen2 => { peek_write_bit_struct_array!(self, value, number::A64SIntDec, number::A32SIntDec, 2, pid, e) },
            TYPES::Linuxdirent64PtrLenArgR => { self.peek_write_flex_tail_struct_array::<dirent::linux_dirent64>(value as types::Ptr, e.return_value()? as usize, pid, e) },

            TYPES::AccessatFlag => { open::write_accessat_flags(self, value, e) },
            TYPES::AtFlag => { open::write_at_flags(self, value, e) },
            TYPES::AsciiOrHexLenArg3 => { self.peek_write_maybe_ascii_str(value as types::Ptr, e.argn(peek::Arg::THR) as usize, pid, e) },
            TYPES::AsciiOrHexLenArgR => {
                match e.return_value() {
                    Ok(r) => self.peek_write_maybe_ascii_str(value as types::Ptr, r as usize, pid, e),
                    _ => Ok(())
                }
            },
            TYPES::Clockid => { time::write_clockid(self, value, e) },
            TYPES::DirFd => { open::write_dir_fd(self, value, e) },
            TYPES::EpollctlOp => { epoll::write_op(self, value, e) },
            TYPES::EpolleventPtr => { peek_write_struct!(self, value, epoll::epoll_event, pid, e) },
            TYPES::EpolleventArrayPtrLenArgR => { peek_write_struct_array!(self, value, epoll::epoll_event, e.return_value()?, pid, e) },
            TYPES::FdFlag => { open::write_fd_flags(self, value, e) },
            TYPES::IoctlReqest => { self.write_number(value, &FORMATS::HEX) },
            TYPES::IovecPtrLenArg3 => { peek_write_bit_struct_array!(self, value, iovec::iovec, iovec::compat_iovec, e.argn(peek::Arg::THR), pid, e) },
            TYPES::IovecPtrLenArg3BufLenArgR => {
                self.prv_data = config::PrivData::IOVEC(e.return_value()? as usize);
                let r = peek_write_bit_struct_array!(self, value, iovec::iovec, iovec::compat_iovec, e.argn(peek::Arg::THR), pid, e);
                self.prv_data = config::PrivData::NONE;
                r
            },
            TYPES::LseekWhence => { open::write_lseek_whence(self, value, e) },
            TYPES::MadviseAdvice => { madvise::write_advice(self, value, e) },
            TYPES::MmapFlag => { mmap::write_flag(self, value, e) },
            TYPES::MmapProt => { mmap::write_prot(self, value, e) },
            TYPES::MsghdrPtr => { peek_write_bit_struct!(self, value, socket::msghdr, socket::compat_msghdr, pid, e) },
            TYPES::MsghdrPtrBufLenArgR => {
                self.prv_data = config::PrivData::IOVEC(e.return_value()? as usize);
                let r = peek_write_bit_struct!(self, value, socket::msghdr, socket::compat_msghdr, pid, e);
                self.prv_data = config::PrivData::NONE;
                r
            },
            TYPES::NewfstatatFlag => { stat::write_newfstatat_flags(self, value, e) },
            TYPES::OldoldutsnamePtr => { peek_write_struct!(self, value, uname::oldold_utsname, pid, e) },
            TYPES::OldutsnamePtr => { peek_write_struct!(self, value, uname::old_utsname, pid, e) },
            TYPES::OpenFlag => { open::write_open_flags(self, value, e) },
            TYPES::RenameFlag => { open::write_rename_flag(self, value, e) },
            TYPES::RlimitResource => { rlimit::write_resource(self, value, e) },
            TYPES::Rlimit64Ptr => { peek_write_struct!(self, value, rlimit::rlimit64, pid, e) },
            TYPES::RlimitPtr => { peek_write_bit_struct!(self, value, rlimit::rlimit, rlimit::compat_rlimit, pid, e) },
            TYPES::SendFlag => { socket::write_send_flag(self, value, e) },
            TYPES::SockaddrPtrLenArg3 => { self.peek_write_callback::<u64, _>(value as types::Ptr, e.argn(peek::Arg::THR) as usize, sockaddr::write_sockaddr, pid, e) },
            TYPES::SockaddrPtrLenArg3Ptr => { self.peek_write_callback::<u64, _>(value as types::Ptr, peek::peek_data::<types::SInt>(pid, e.argn(peek::Arg::THR) as types::Ptr)? as usize, sockaddr::write_sockaddr, pid, e) },
            TYPES::SocketDomain => { socket::write_domain(self, value, e) },
            TYPES::SocketFlag => { socket::write_flag(self, value, e) },
            TYPES::SocketType => { socket::write_type(self, value, e) },
            TYPES::SocketcallCall => { socket::write_socketcall_call(self, value, e) },
            TYPES::SocketcallArg => { socket::write_socketcall_arg(self, value, pid, e) },
            TYPES::StatfsPtr => { peek_write_bit_struct!(self, value, statfs::statfs, statfs::compat_statfs, pid, e) },
            TYPES::Statfs64Ptr => { peek_write_bit_struct!(self, value, statfs::statfs64, statfs::compat_statfs64, pid, e) },
            TYPES::StatPtr => { peek_write_bit_struct!(self, value, stat::stat, stat::compat_stat, pid, e) },
            TYPES::StatxPtr => { peek_write_struct!(self, value, stat::statx, pid, e) },
            TYPES::SysinfoPtr => { peek_write_bit_struct!(self, value, sys::sysinfo, sys::compat_sysinfo, pid, e) },
            TYPES::TimespecPtr => { peek_write_struct!(self, value, time::kernel_timespec, pid, e) },
            TYPES::TimexPtr => { peek_write_struct!(self, value, time::timex, pid, e) },
            TYPES::TimevalPtr => { peek_write_bit_struct!(self, value, time::timeval, time::compat_timeval, pid, e) },
            TYPES::TimezonePtr => { peek_write_struct!(self, value, time::timezone, pid, e) },
            TYPES::UtsnamePtr => { peek_write_struct!(self, value, uname::new_utsname, pid, e) },

            TYPES::UNDEF => { self.write_number(value, &FORMATS::HEX) },
            TYPES::SKIP | TYPES::NONE => { Ok(()) },
        }
    }

    fn write_header_suf(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write_width(pid.dtoa().as_bytes(), 6)?;
        self.write(b":")?;
        self.write_width(e.sysnum().dtoa().as_bytes(), 6)?;
        self.write(b"]")?;
        self.write_width(e.sysname().as_bytes(), 20)
    }

    fn write_entry_header(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"in  [")?;
        self.write_header_suf(pid, e)
    }

    fn write_exit_header(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write(b"out [")?;
        self.write_header_suf(pid, e)?;
        self.write(b"(...) = ")
    }

    fn write_args_impl(&mut self, print: &config::SyscallPrintInfoSet, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let a = e.args();
        for i in 0..a.len() {
            if print.args[i] == TYPES::NONE { break }
            if i != 0 {
                self.write(b", ")?;
            }
            self.write_any_type(a[i], &print.args[i], pid, e)?;
        }
        Ok(())
    }

    fn dump_args(&mut self, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let a = e.args();
        for i in 0..a.len() {
            if i != 0 {
                self.write(b", ")?;
            }
            self.write_width(a[i].htoa().as_bytes(), 20)?;
        }
        Ok(())
    }

    fn write_ret_args_impl(&mut self, print: &config::SyscallPrintInfoSet, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        let a = e.args();
        for i in 0..a.len() {
            if print.args[i] == TYPES::NONE { continue }
            self.write(b", ")?;
            self.write_number(i, &FORMATS::DEC)?;
            self.write(b": ")?;
            self.write_any_type(a[i], &print.args[i], pid, e)?;
        }
        Ok(())
    }

    fn write_ret_args(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        match self.conf.get_print_info_for_ret_args(e.uni_sysnum()) {
            p if p.is_skip() => Ok(()),
            p => self.write_ret_args_impl(p, pid, e),
        }
    }

    fn write_errno(&mut self, err: std::io::Error, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        match err.raw_os_error() {
            Some(r) if r >= 0 => errno::write_errno(self, r as u64, e),
            Some(r) if r < 0 => errno::write_errno(self, r.wrapping_abs() as u64, e),
            _ => self.write(b"?"),
        }
    }
    fn write_ret(&mut self, print: &config::SyscallPrintInfoSet, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        self.write_exit_header(pid, e)?;
        match e.return_value() {
            Ok(r) => {
                self.write_any_type(r, &print.ret, pid, e)?;
                if !print.is_undef() {
                    self.write_ret_args(pid, e)?;
                }
            },
            Err(r) => self.write_errno(r, e)?,
        }
        self.flush_line()
    }
    fn write_syscall_exit(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        match self.conf.get_print_info(e.uni_sysnum()) {
            p if p.is_skip() => Ok(()),
            p => self.write_ret(p, pid, e),
        }
    }

    fn write_syscall_entry(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        match self.conf.get_print_info(e.uni_sysnum()) {
            p if p.is_skip() => Ok(()),
            p if p.is_undef() => {
                self.write_entry_header(pid, e)?;
                self.dump_args(e)?;
                self.flush_line()
            },
            p  => {
                self.write_entry_header(pid, e)?;
                self.write(b"(")?;
                self.write_args_impl(p, pid, e)?;
                self.write(b")")?;
                self.flush_line()
            },
        }
    }

    /// Create Printer strut as default value 
    pub fn new() -> Self {
        let writer = logger::Logger::default();
        let conf = config::Config::new();
        let prv_data = config::PrivData::NONE;
        Printer{writer, conf, prv_data}
    }

    /// Output SyscallSummery to log destination
    /// # Arguments
    /// * `pid` - A process ID of log output target
    /// * `e` - Syscall summery of log output target
    pub fn output(&mut self, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        if e.is_entry() {
            self.write_syscall_entry(pid, e)
        } else  {
            self.write_syscall_exit(pid, e)
        }
    }

    /// Set default value as skip output
    pub fn set_skip_for_default(&mut self) {
        self.conf.set_skip_for_default()
    }

    /// Set to skip output for specified name's syscall
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_skip_by_name(&mut self, name: &str) {
        self.conf.set_skip_by_name(name)
    }

    /// Set to not skip output for specified name's syscall
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_not_skip_by_name(&mut self, name: &str) {
        self.conf.set_not_skip_by_name(name)
    }

    /// Set to simple format output for specified name's syscall
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_simple_by_name(&mut self, name: &str) {
        self.conf.set_simple_by_name(name)
    }

    /// Set to skip output for syscall that name contain speccified
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_skip_by_include_name(&mut self, name: &str) {
        self.conf.set_skip_by_include_name(name)
    }

    /// Set to not skip output for syscall that name contain speccified
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_not_skip_by_include_name(&mut self, name: &str) {
        self.conf.set_not_skip_by_include_name(name)
    }

    /// Set to simple format output for syscall that name contain speccified
    /// # Arguments
    /// * `name` - Target syscall's name
    pub fn set_simple_by_include_name(&mut self, name: &str) {
        self.conf.set_simple_by_include_name(name)
    }

    /// Set log destinaion
    /// # Arguments
    /// * `path` - file path for log destinaion
    pub fn file(&mut self, path: String) {
        self.writer = logger::Logger::file(path);
    }

}
