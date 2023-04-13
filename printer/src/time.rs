use crate::FORMATS;
use arch::types::{a64, a32};

#[repr(C)]#[allow(non_camel_case_types)]
pub struct kernel_timespec {
    tv_sec: types::SLLong,
    tv_nsec: types::SLLong,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct timeval {
    tv_sec: a64::SLong,
    tv_nsec: a64::SLong,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_timeval {
    tv_sec: a32::SLong,
    tv_nsec: a32::SLong,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct timezone {
    tz_minuteswest: types::SInt,
    tz_dsttime: types::SInt,
}

macro_rules! timeval_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b".tv_sec = ")?; printer.write_number(self.tv_sec, &FORMATS::DEC)?;
                printer.write(b", .tv_nsec = ")?; printer.write_number(self.tv_nsec, &FORMATS::DEC)
            }
        }
    };
}

timeval_impl_print!(kernel_timespec);

timeval_impl_print!(timeval);
timeval_impl_print!(compat_timeval);

impl crate::Print for timezone {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".tz_minuteswest = ")?; printer.write_number(self.tz_minuteswest, &FORMATS::DEC)?;
        printer.write(b", .tz_dsttime = ")?; printer.write_number(self.tz_dsttime, &FORMATS::DEC)
    }
}

const CLOCKID: [(u32, &'static str); 10] = [
(0, "CLOCK_REALTIME"), (1, "CLOCK_MONOTONIC"), (2, "CLOCK_PROCESS_CPUTIME_ID"), (3, "CLOCK_THREAD_CPUTIME_ID"), (4, "CLOCK_MONOTONIC_RAW"),
(5, "CLOCK_REALTIME_COARSE"), (6, "CLOCK_MONOTONIC_COARSE"), (7, "CLOCK_BOOTTIME"), (8, "CLOCK_REALTIME_ALARM"), (9, "CLOCK_BOOTTIME_ALARM"),
];

pub fn write_clockid(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &CLOCKID)
}
