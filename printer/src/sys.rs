use crate::FORMATS;
use arch::types::{a64, a32};

#[repr(C)]#[allow(non_camel_case_types)]
pub struct sysinfo {
    uptime: a64::SLong,
    loads: [a64::ULong; 3],
    totalram: a64::ULong,
    freeram: a64::ULong,
    sharedram: a64::ULong,
    bufferram: a64::ULong,
    totalswap: a64::ULong,
    freeswap: a64::ULong,
    procs: a64::UShrt,
    pad: a64::UShrt,
    totalhigh: a64::ULong,
    freehigh: a64::ULong,
    mem_unit: a64::UInt,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_sysinfo {
    uptime: a32::SLong,
    loads: [a32::ULong; 3],
    totalram: a32::ULong,
    freeram: a32::ULong,
    sharedram: a32::ULong,
    bufferram: a32::ULong,
    totalswap: a32::ULong,
    freeswap: a32::ULong,
    procs: a32::UShrt,
    pad: a32::UShrt,
    totalhigh: a32::ULong,
    freehigh: a32::ULong,
    mem_unit: a32::UInt,
}

macro_rules! sysinfo_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b".uptime = ")?; printer.write_number(self.uptime, &FORMATS::DEC)?;
                printer.write(b", .loads = ")?; printer.write_number_array(&self.loads, &FORMATS::DEC)?;
                printer.write(b", .totalram = ")?; printer.write_number(self.totalram, &FORMATS::DEC)?;
                printer.write(b", .freeram = ")?; printer.write_number(self.freeram, &FORMATS::DEC)?;
                printer.write(b", .sharedram = ")?; printer.write_number(self.sharedram, &FORMATS::DEC)?;
                printer.write(b", .bufferram = ")?; printer.write_number(self.bufferram, &FORMATS::DEC)?;
                printer.write(b", .totalswap = ")?; printer.write_number(self.totalswap, &FORMATS::DEC)?;
                printer.write(b", .freeswap = ")?; printer.write_number(self.freeswap, &FORMATS::DEC)?;
                printer.write(b", .procs = ")?; printer.write_number(self.procs, &FORMATS::DEC)?;
                printer.write(b", .totalhigh = ")?; printer.write_number(self.totalhigh, &FORMATS::DEC)?;
                printer.write(b", .freehigh = ")?; printer.write_number(self.freehigh, &FORMATS::DEC)?;
                printer.write(b", .mem_unit = ")?; printer.write_number(self.mem_unit, &FORMATS::DEC)?;
                Ok(())
            }
        }
    };
}

sysinfo_impl_print!(sysinfo);
sysinfo_impl_print!(compat_sysinfo);
