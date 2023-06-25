use crate::FORMATS;

const AT_FLAG: [(u32, &'static str); 5] = [ (0x100,"AT_SYMLINK_NOFOLLOW"), (0x200,"AT_REMOVEDIR"), (0x400,"AT_SYMLINK_FOLLOW"), (0x800,"AT_NO_AUTOMOUNT"), (0x1000,"AT_EMPTY_PATH"),];

pub fn write_dir_fd(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as arch::types::a64::SInt;
    if value == -100 {
        printer.write(b"AT_FDCWD")
    } else {
        printer.write_number(value, &FORMATS::DEC)
    }
}

pub fn write_at_flags(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"0")
    } else {
        printer.write_mask_enum(value, &AT_FLAG)
    }
}

const OPEN_MODE: [(u32, &'static str); 4] = [ (0o00000003, "O_ACCMODE"), (0o00000000, "O_RDONLY"), (0o00000001, "O_WRONLY"), (0o00000002, "O_RDWR"), ];
const O_FLAG: [(u32, &'static str); 14] = [
(0o00000100, "O_CREAT"), (0o00000200, "O_EXCL"), (0o00000400, "O_NOCTTY"), (0o00001000, "O_TRUNC"), (0o00002000, "O_APPEND"),
(0o00004000, "O_NONBLOCK"), (0o00010000, "O_DSYNC"), (0o00020000, "FASYNC"), (0o00040000, "O_DIRECT"), (0o00100000, "O_LARGEFILE"),
(0o00200000, "O_DIRECTORY"), (0o00400000, "O_NOFOLLOW"), (0o01000000, "O_NOATIME"), (0o02000000, "O_CLOEXEC"),
];

pub fn write_open_flags(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    let mode = value & 0x3;
    printer.write_enum(mode, &OPEN_MODE)?;
    let mode = value & !0x3;
    if mode != 0 {
        printer.write(b" | ")?;
        printer.write_mask_enum(mode, &O_FLAG)?;
    }
    Ok(())
}

pub fn write_fd_flags(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    let mode = value & !0x3;
    if mode == 0 {
        printer.write_number(value, &FORMATS::DEC)
    } else {
        printer.write_mask_enum(mode, &O_FLAG)
    }
}

const LSEEK_WHENCE: [(u32, &'static str); 5] = [ (0, "SEEK_SET"), (1, "SEEK_CUR"), (2, "SEEK_END"), (3, "SEEK_DATA"), (4, "SEEK_HOLE"), ];

pub fn write_lseek_whence(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &LSEEK_WHENCE)
}

const ACCESS_AT_FLAG: [(u32, &'static str); 2] = [ (0x100,"AT_SYMLINK_NOFOLLOW"), (0x200,"AT_EACCESS"), ];
pub fn write_accessat_flags(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"0")
    } else {
        printer.write_mask_enum(value, &ACCESS_AT_FLAG)
    }
}

const RENAME_FLAG: [(u32, &'static str); 3] = [ (1 << 0, "RENAME_NOREPLACE"), (1 << 1, "RENAME_EXCHANGE"), (1 << 2, "RENAME_WHITEOUT"),];
pub fn write_rename_flag(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &RENAME_FLAG)
}
