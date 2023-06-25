
pub fn write_advice(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &arch::types::madvise::MADVISE_ADVICE)
}

