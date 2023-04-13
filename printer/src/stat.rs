use crate::FORMATS;

#[repr(C)]#[allow(non_camel_case_types)]
pub struct stat {
    data: arch::types::stat::a64::stat,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_stat {
    data: arch::types::stat::a64::compat_stat,
}

macro_rules! print_stat {
    ($self:expr, $printer: ident) => {
        $printer.write(b".st_dev = ")?; $printer.write_number($self.st_dev, &FORMATS::HEX)?;
        $printer.write(b", .st_ino = ")?; $printer.write_number($self.st_ino, &FORMATS::DEC)?;
        $printer.write(b", .st_mode = ")?; $printer.write_number($self.st_mode, &FORMATS::OCT)?;
        $printer.write(b", .st_nlink = ")?; $printer.write_number($self.st_nlink, &FORMATS::DEC)?;
        $printer.write(b", .st_uid = ")?; $printer.write_number($self.st_uid, &FORMATS::DEC)?;
        $printer.write(b", .st_gid = ")?; $printer.write_number($self.st_gid, &FORMATS::DEC)?;
        $printer.write(b", .st_rdev = ")?; $printer.write_number($self.st_rdev, &FORMATS::HEX)?;
        $printer.write(b", .st_size = ")?; $printer.write_number($self.st_size, &FORMATS::DEC)?;
        $printer.write(b", .st_blksize = ")?; $printer.write_number($self.st_blksize, &FORMATS::DEC)?;
        $printer.write(b", .st_blocks = ")?; $printer.write_number($self.st_blocks, &FORMATS::DEC)?;
        $printer.write(b", .st_atime = ")?; $printer.write_number($self.st_atime, &FORMATS::DEC)?;
        $printer.write(b", .st_atime_nsec = ")?; $printer.write_number($self.st_atime_nsec, &FORMATS::DEC)?;
        $printer.write(b", .st_mtime = ")?; $printer.write_number($self.st_mtime, &FORMATS::DEC)?;
        $printer.write(b", .st_mtime_nsec = ")?; $printer.write_number($self.st_mtime_nsec, &FORMATS::DEC)?;
        $printer.write(b", .st_ctime = ")?; $printer.write_number($self.st_ctime, &FORMATS::DEC)?;
        $printer.write(b", .st_ctime_nsec = ")?; $printer.write_number($self.st_ctime_nsec, &FORMATS::DEC)?;
    };
}

impl crate::Print for stat {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        print_stat!(self.data, printer);
        Ok(())
    }
}

impl crate::Print for compat_stat {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        print_stat!(self.data, printer);
        Ok(())
    }
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct statx_timestamp {
    tv_sec: i64,
    tv_nsec: u32,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct statx {
    stx_mask: u32,
    stx_blksize: u32,
    stx_attributes: u64,
    stx_nlink: u32,
    stx_uid: u32,
    stx_gid: u32,
    stx_mode: u16,
    stx_ino: u64,
    stx_size: u64,
    stx_blocks: u64,
    stx_attributes_mask: u64,
    stx_atime: statx_timestamp,
    stx_btime: statx_timestamp,
    stx_ctime: statx_timestamp,
    stx_mtime: statx_timestamp,
    stx_rdev_major: u32,
    stx_rdev_minor: u32,
    stx_dev_major: u32,
    stx_dev_minor: u32,
}

const STATX_MASK: [(u32, &'static str); 17] = [
    (0x00000fff,"STATX_ALL"),
    (0x00000001,"STATX_TYPE"), (0x00000002,"STATX_MODE"), (0x00000004,"STATX_NLINK"), (0x00000008,"STATX_UID"), (0x00000010,"STATX_GID"),
    (0x00000020,"STATX_ATIME"), (0x00000040,"STATX_MTIME"), (0x00000080,"STATX_CTIME"), (0x00000100,"STATX_INO"), (0x00000200,"STATX_SIZE"),
    (0x00000400,"STATX_BLOCKS"), (0x000007ff,"STATX_BASIC_STATS"), (0x00000800,"STATX_BTIME"), (0x00001000,"STATX_MNT_ID"), (0x00002000,"STATX_DIOALIGN"),
    (0x80000000,"STATX__RESERVED"),
];

const STATX_ATTR: [(u64, &'static str); 7] = [
    (0x00000004,"STATX_ATTR_COMPRESSED"), (0x00000010,"STATX_ATTR_IMMUTABLE"), (0x00000020,"STATX_ATTR_APPEND"), (0x00000040,"STATX_ATTR_NODUMP"),
    (0x00000800,"STATX_ATTR_ENCRYPTED"), (0x00001000,"STATX_ATTR_AUTOMOUNT"), (0x00100000,"STATX_ATTR_VERITY"),
];

impl crate::Print for statx_timestamp {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".tv_sec = ")?; printer.write_number(self.tv_sec, &FORMATS::DEC)?;
        printer.write(b", .tv_nsec = ")?; printer.write_number(self.tv_nsec, &FORMATS::DEC)
    }
}

impl crate::Print for statx {
    fn print(&self, printer: &mut crate::Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        if self.stx_mask == 0 { printer.write(b".stx_mask = 0")?; }
        else { printer.write(b".stx_mask = ")?; printer.write_mask_enum(self.stx_mask, &STATX_MASK)?; }
        printer.write(b", .stx_blksize = ")?; printer.write_number(self.stx_blksize, &FORMATS::DEC)?;
        if self.stx_attributes == 0 { printer.write(b", .stx_attributes = 0")?; }
        else { printer.write(b", .stx_attributes = 0")?; printer.write_mask_enum(self.stx_attributes, &STATX_ATTR)?; }
        printer.write(b", .stx_nlink = ")?; printer.write_number(self.stx_nlink, &FORMATS::DEC)?;
        printer.write(b", .stx_uid = ")?; printer.write_number(self.stx_uid, &FORMATS::DEC)?;
        printer.write(b", .stx_gid = ")?; printer.write_number(self.stx_gid, &FORMATS::DEC)?;
        printer.write(b", .stx_mode = ")?; printer.write_number(self.stx_mode, &FORMATS::OCT)?;
        printer.write(b", .stx_ino = ")?; printer.write_number(self.stx_ino, &FORMATS::DEC)?;
        printer.write(b", .stx_size = ")?; printer.write_number(self.stx_size, &FORMATS::DEC)?;
        printer.write(b", .stx_blocks = ")?; printer.write_number(self.stx_blocks, &FORMATS::DEC)?;
        if self.stx_attributes_mask == 0 { printer.write(b", .stx_attributes_mask = 0")?; }
        else { printer.write(b", .stx_attributes_mask = 0")?; printer.write_mask_enum(self.stx_attributes_mask, &STATX_ATTR)?; }
        printer.write(b", .stx_atime = {")?; self.stx_atime.print(printer, pid, e)?;
        printer.write(b"}, .stx_btime = {")?; self.stx_btime.print(printer, pid, e)?;
        printer.write(b"}, .stx_ctime = {")?; self.stx_ctime.print(printer, pid, e)?;
        printer.write(b"}, .stx_mtime = {")?; self.stx_mtime.print(printer, pid, e)?;
        printer.write(b"}, .stx_rdev_major = ")?; printer.write_number(self.stx_rdev_major, &FORMATS::HEX)?;
        printer.write(b", .stx_rdev_minor = ")?; printer.write_number(self.stx_rdev_minor, &FORMATS::HEX)?;
        printer.write(b", .stx_dev_major = ")?; printer.write_number(self.stx_dev_major, &FORMATS::HEX)?;
        printer.write(b", .stx_dev_minor = ")?; printer.write_number(self.stx_dev_minor, &FORMATS::HEX)
    }
}

pub fn write_newfstatat_flags(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"0")
    } else {
        printer.write_mask_enum(value, &arch::types::stat::NEWFSTATAT_FLAG)
    }
}

