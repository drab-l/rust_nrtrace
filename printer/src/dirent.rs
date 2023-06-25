use crate::FORMATS;

#[repr(C)]#[allow(non_camel_case_types)]
pub struct linux_dirent64 {
    d_ino: u64,
    d_off: i64,
    d_reclen: types::UShrt,
    d_type: types::UChar,
    d_name: [u8; 0],
}

macro_rules! offset_of {
    ($type:ty, $member:ident) => { unsafe { (&(*(4096 as *const $type)).$member).as_ptr() as usize - 4096} };
}

impl crate::Print for linux_dirent64 {
    fn print(&self, printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".d_ino = ")?;
        printer.write_number(self.d_ino, &FORMATS::HEX)?;
        printer.write(b", .d_off = ")?;
        printer.write_number(self.d_off, &FORMATS::HEX)?;
        printer.write(b", .d_reclen = ")?;
        printer.write_number(self.d_reclen, &FORMATS::DEC)?;
        printer.write(b", .d_type = ")?;
        printer.write_number_zero_fill(self.d_type)?;
        Ok(())
    }

    fn print_flex_tail(&self, printer: &mut crate::Printer, buf: &[u8], _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b", .d_name = \"")?;
        let n = buf.iter().position(|x| *x == 0).unwrap_or(buf.len());
        printer.write(&buf[..n])?;
        printer.write(b"\"")?;
        Ok(())
    }

    fn flex_tail_size(&self) -> usize {
        self.d_reclen as usize - offset_of!(linux_dirent64, d_name)
    }

    fn flex_tail_offset(&self) -> usize {
        offset_of!(linux_dirent64, d_name)
    }

    fn total_size(&self) -> usize {
        self.d_reclen as usize
    }

    fn print_array_delim(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b",\n\t")
    }

    fn print_array_prefix(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b"\n\t")
    }
    fn print_array_suffix(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b"\n\t")
    }
}

