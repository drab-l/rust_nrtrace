
pub fn write_prot(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"PROT_NONE")
    } else {
        printer.write_mask_enum(value, &arch::types::mmap::MMAP_PROT)
    }
}

pub fn write_flag(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"0")
    } else {
        printer.write_mask_enum(value, &arch::types::mmap::MMAP_FLAG)
    }
}

