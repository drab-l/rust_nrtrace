use arch::types::{a64, a32};
use crate::FORMATS;

#[repr(C)]#[allow(non_camel_case_types)]
pub struct iovec {
    iov_base: a64::Ptr,
    iov_len: a64::USizeT,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_iovec {
    iov_base: a32::Ptr,
    iov_len: a32::USizeT,
}

macro_rules! iovec_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &crate::Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b".io_base = ")?;
                if let crate::config::PrivData::IOVEC(s) = printer.prv_data.get() {
                    let min = std::cmp::min(s, self.iov_len as usize);
                    printer.peek_write_maybe_ascii_str(self.iov_base as types::Ptr, min, pid, e)?;
                    printer.prv_data.set(crate::config::PrivData::IOVEC(s - min));
                } else {
                    printer.peek_write_maybe_ascii_str(self.iov_base as types::Ptr, self.iov_len as usize, pid, e)?;
                }
                printer.write(b", .io_len = ")?;
                printer.write_number(self.iov_len, &FORMATS::DEC)?;
                Ok(())
            }
            fn print_array_delim(printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b",\n\t")
            }
            fn print_array_prefix(printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b"\n\t")
            }
            fn print_array_suffix(printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b"\n\t")
            }
        }
    };
}

iovec_impl_print!(iovec);
iovec_impl_print!(compat_iovec);

