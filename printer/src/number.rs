use crate::FORMATS;

pub trait ToString {
    fn htoa(self) -> String;
    fn dtoa(self) -> String;
    fn otoa(self) -> String;
    fn htov(self, r: &mut Vec<u8>);
    fn dtov(self, r: &mut Vec<u8>);
    fn otov(self, r: &mut Vec<u8>);
}

pub trait ToPtrString {
    fn ptoa(self) -> String;
    fn ptov(self, r: &mut Vec<u8>);
}

const TO_HEX_TABLE: [u8;16] = [0x30,0x31,0x32,0x33,0x34,0x35,0x36,0x37,0x38,0x39,0x61,0x62,0x63,0x64,0x65,0x66];

macro_rules! impl_uto_proc_in {
    ($uvalue:ident, $dst:ident, $floor:ident, $base:literal) => {
        while $floor > 0 {
            $dst.push((TO_HEX_TABLE[(($uvalue / $floor) % $base) as usize]) as u8);
            $uvalue %= $floor;
            $floor /= $base;
        }
    };
}
macro_rules! impl_uto_proc {
    ($uvalue:ident, $dst:ident, $floor:ident, $base:literal) => {
        if $uvalue < $base {
            $dst.push((TO_HEX_TABLE[$uvalue as usize]) as u8);
        } else {
            while $floor > 0 && $floor > $uvalue {
                $floor /= $base;
            }
            impl_uto_proc_in!($uvalue, $dst, $floor, $base);
        }
    };
}

macro_rules! impl_prefix_proc {
    ($dst:ident, $base:literal) => {
        if $base == 0x10 || $base == 0o10 || $base == 0b10 {
            $dst.push('0' as u8);
            $dst.push((if $base == 0x10 { 'x' } else if $base == 0o10 { 'o' } else { 'b' }) as u8);
        }
    };
}

macro_rules! impl_utov_proc {
    ($uvalue:ident, $dst:ident, $utype:ty, $digit:literal, $base:literal) => {
        let mut n = $uvalue;
        let mut floor = ($base as $utype).pow($digit - 1);
        impl_prefix_proc!($dst, $base);
        impl_uto_proc!(n, $dst, floor, $base);
    };
}

macro_rules! impl_utov {
    ($name:tt, $utype:ty, $itype:ty, $digit:literal, $base:literal) => {
        fn $name(self, r: &mut Vec<u8>) {
            impl_utov_proc!(self, r, $utype, $digit, $base);
        }
    };
}

macro_rules! impl_utoa {
    ($name:tt, $utype:ty, $itype:ty, $digit:literal, $base:literal) => {
        fn $name(self) -> String {
            let mut r = Vec::with_capacity(2 + $digit);
            impl_utov_proc!(self, r, $utype, $digit, $base);
            std::string::String::from_utf8(r).unwrap()
        }
    };
}

macro_rules! impl_sign_proc {
    ($value:ident, $dst:ident) => {
        if $value < 0 {
            $dst.push('-' as u8);
        }
    };
}

macro_rules! impl_itov {
    ($name:tt, $utype:ty, $itype:ty, $digit:literal, $base:literal) => {
        fn $name(self, r: &mut Vec<u8>) {
            let n = self;
            impl_sign_proc!(n, r);
            let n = if n < 0 { (n.wrapping_neg() as $utype) } else { n as $utype };
            impl_utov_proc!(n, r, $utype, $digit, $base);
        }
    };
}

macro_rules! impl_itoa {
    ($name:tt, $utype:ty, $itype:ty, $digit:literal, $base:literal) => {
        fn $name(self) -> String {
            let n = self;
            let mut r = Vec::with_capacity(3 + $digit);
            impl_sign_proc!(n, r);
            let n = if n < 0 { (n.wrapping_neg() as $utype) } else { n as $utype };
            impl_utov_proc!(n, r, $utype, $digit, $base);
            std::string::String::from_utf8(r).unwrap()
        }
    };
}

macro_rules! impl_htoa_dtoa_otoa {
    ($utype:ty, $itype:ty, $hdigit:literal, $ddigit:literal, $odigit:literal) => {
        impl ToString for $utype {
            impl_utoa!(htoa, $utype, $itype, $hdigit, 16);
            impl_utoa!(dtoa, $utype, $itype, $ddigit, 10);
            impl_utoa!(otoa, $utype, $itype, $odigit, 8);
            impl_utov!(htov, $utype, $itype, $hdigit, 16);
            impl_utov!(dtov, $utype, $itype, $ddigit, 10);
            impl_utov!(otov, $utype, $itype, $odigit, 8);
        }
        impl ToString for $itype {
            impl_itoa!(htoa, $utype, $itype, $hdigit, 16);
            impl_itoa!(dtoa, $utype, $itype, $ddigit, 10);
            impl_itoa!(otoa, $utype, $itype, $odigit, 8);
            impl_itov!(htov, $utype, $itype, $hdigit, 16);
            impl_itov!(dtov, $utype, $itype, $ddigit, 10);
            impl_itov!(otov, $utype, $itype, $odigit, 8);
        }
    };
}

impl_htoa_dtoa_otoa!(u8, i8, 2, 3, 3);
impl_htoa_dtoa_otoa!(u16, i16, 4, 5, 6);
impl_htoa_dtoa_otoa!(u32, i32, 8, 10, 11);
impl_htoa_dtoa_otoa!(u64, i64, 16, 20, 22);
impl_htoa_dtoa_otoa!(u128, i128, 32, 39, 43);
#[cfg(target_pointer_width = "64")]
impl_htoa_dtoa_otoa!(usize, isize, 16, 20, 22);
#[cfg(target_pointer_width = "32")]
impl_htoa_dtoa_otoa!(usize, isize, 8, 10, 11);

macro_rules! impl_ptov_proc {
    ($uvalue:ident, $dst:ident, $utype:ty, $digit:literal, $base:literal) => {
        let mut n = $uvalue;
        let mut floor = ($base as $utype).pow($digit - 1);
        impl_prefix_proc!($dst, $base);
        impl_uto_proc_in!(n, $dst, floor, $base);
    };
}

macro_rules! impl_ptoa {
    ($type:ty, $digit:literal) => {
        impl ToPtrString for $type {
            fn ptov(self, r: &mut Vec<u8>) {
                impl_ptov_proc!(self, r, $type, $digit, 16);
            }
            fn ptoa(self) -> String {
                let mut r = Vec::with_capacity(2 + $digit);
                impl_ptov_proc!(self, r, $type, $digit, 16);
                std::string::String::from_utf8(r).unwrap()
            }
        }
    };
}

impl_ptoa!(u8, 2);
impl_ptoa!(u16, 4);
impl_ptoa!(u32, 8);
impl_ptoa!(u64, 16);

macro_rules! impl_print_trait {
    ($name:tt, $type:ty, $fmt:expr) => {
        #[repr(C)]#[allow(dead_code)]
        pub struct $name {
            data: $type,
        }
        impl crate::Print for $name {
            fn print(&self, printer: &crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write_number(self.data, &$fmt)
            }
        }
    };
}

macro_rules! impl_print_trait_fmts {
    ($hname:tt, $dname:tt, $oname:tt, $type:ty) => {
        impl_print_trait!($hname, $type, FORMATS::HEX);
        impl_print_trait!($dname, $type, FORMATS::DEC);
        impl_print_trait!($oname, $type, FORMATS::OCT);
    };
}

impl_print_trait_fmts!(U8Hex, U8Dec, U8Oct, u8);
impl_print_trait_fmts!(I8Hex, I8Dec, I8Oct, i8);
impl_print_trait_fmts!(U16Hex, U16Dec, U16Oct, u16);
impl_print_trait_fmts!(I16Hex, I16Dec, I16Oct, i16);
impl_print_trait_fmts!(U32Hex, U32Dec, U32Oct, u32);
impl_print_trait_fmts!(I32Hex, I32Dec, I32Oct, i32);
impl_print_trait_fmts!(U64Hex, U64Dec, U64Oct, u64);
impl_print_trait_fmts!(I64Hex, I64Dec, I64Oct, i64);

impl_print_trait_fmts!(SIntHex, SIntDec, SIntOct, types::SInt);

impl_print_trait_fmts!(A64SIntHex, A64SIntDec, A64SIntOct, arch::types::a64::SInt);
impl_print_trait_fmts!(A32SIntHex, A32SIntDec, A32SIntOct, arch::types::a32::SInt);
