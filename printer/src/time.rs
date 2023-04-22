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

#[repr(C)]#[allow(non_camel_case_types)]
pub struct timex {
    modes: types::SInt,
    _pad1: u32,
    offset: types::SLLong,
    freq: types::SLLong,
    maxerror: types::SLLong,
    esterror: types::SLLong,
    status: types::SInt,
    _pad2: u32,
    constant: types::SLLong,
    precision: types::SLLong,
    tolerance: types::SLLong,
    time: timeval,
    tick: types::SLLong,
    ppsfreq: types::SLLong,
    jitter: types::SLLong,
    shift: types::SInt,
    _pad3: u32,
    stabil: types::SLLong,
    jitcnt: types::SLLong,
    calcnt: types::SLLong,
    errcnt: types::SLLong,
    stbcnt: types::SLLong,
    tai: types::SInt,
    // _pad4[u32; 11],
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

impl crate::Print for timex {
    fn print(&self, printer: &mut crate::Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".modes = ")?; printer.write_number(self.modes, &FORMATS::DEC)?;
        printer.write(b", .offset = ")?; printer.write_number(self.offset, &FORMATS::DEC)?;
        printer.write(b", .freq = ")?; printer.write_number(self.freq, &FORMATS::DEC)?;
        printer.write(b", .maxerror = ")?; printer.write_number(self.maxerror, &FORMATS::DEC)?;
        printer.write(b", .esterror = ")?; printer.write_number(self.esterror, &FORMATS::DEC)?;
        printer.write(b", .status = ")?; printer.write_number(self.status, &FORMATS::DEC)?;
        printer.write(b", .constant = ")?; printer.write_number(self.constant, &FORMATS::DEC)?;
        printer.write(b", .precision = ")?; printer.write_number(self.precision, &FORMATS::DEC)?;
        printer.write(b", .tolerance = ")?; printer.write_number(self.tolerance, &FORMATS::DEC)?;
        printer.write(b", .time = {")?; self.time.print(printer, pid, e)?;
        printer.write(b"}, .tick = ")?; printer.write_number(self.tick, &FORMATS::DEC)?;
        printer.write(b", .ppsfreq = ")?; printer.write_number(self.ppsfreq, &FORMATS::DEC)?;
        printer.write(b", .jitter = ")?; printer.write_number(self.jitter, &FORMATS::DEC)?;
        printer.write(b", .shift = ")?; printer.write_number(self.shift, &FORMATS::DEC)?;
        printer.write(b", .stabil = ")?; printer.write_number(self.stabil, &FORMATS::DEC)?;
        printer.write(b", .jitcnt = ")?; printer.write_number(self.jitcnt, &FORMATS::DEC)?;
        printer.write(b", .calcnt = ")?; printer.write_number(self.calcnt, &FORMATS::DEC)?;
        printer.write(b", .errcnt = ")?; printer.write_number(self.errcnt, &FORMATS::DEC)?;
        printer.write(b", .stbcnt = ")?; printer.write_number(self.stbcnt, &FORMATS::DEC)?;
        printer.write(b", .tai = ")?; printer.write_number(self.tai, &FORMATS::DEC)?;
        Ok(())
    }
}

const CLOCKID: [(u32, &'static str); 10] = [
(0, "CLOCK_REALTIME"), (1, "CLOCK_MONOTONIC"), (2, "CLOCK_PROCESS_CPUTIME_ID"), (3, "CLOCK_THREAD_CPUTIME_ID"), (4, "CLOCK_MONOTONIC_RAW"),
(5, "CLOCK_REALTIME_COARSE"), (6, "CLOCK_MONOTONIC_COARSE"), (7, "CLOCK_BOOTTIME"), (8, "CLOCK_REALTIME_ALARM"), (9, "CLOCK_BOOTTIME_ALARM"),
];

pub fn write_clockid(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &CLOCKID)
}
