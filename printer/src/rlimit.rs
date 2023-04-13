use crate::FORMATS;

const RESOURCE: [(u32, &'static str);17] = [
    (0,"RLIMIT_CPU"), (1,"RLIMIT_FSIZE"), (2,"RLIMIT_DATA"), (3,"RLIMIT_STACK"), (4,"RLIMIT_CORE"),
    (5,"RLIMIT_RSS"), (6,"RLIMIT_NPROC"), (7,"RLIMIT_NOFILE"), (8,"RLIMIT_MEMLOCK"), (9,"RLIMIT_AS"),
    (10,"RLIMIT_LOCKS"), (11,"RLIMIT_SIGPENDING"), (12,"RLIMIT_MSGQUEUE"), (13,"RLIMIT_NICE"), (14,"RLIMIT_RTPRIO"),
    (15,"RLIMIT_RTTIME"), (16,"RLIM_NLIMITS"),
];

pub fn write_resource(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &RESOURCE)
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct rlimit64 {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct rlimit {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_rlimit {
    pub rlim_cur: u32,
    pub rlim_max: u32,
}

impl crate::Print for rlimit64 {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".rlim_cur = ")?;
        printer.write_number(self.rlim_cur, &FORMATS::DEC)?;
        printer.write(b", .rlim_max = ")?;
        printer.write_number(self.rlim_max, &FORMATS::DEC)?;
        Ok(())
    }
}

impl crate::Print for rlimit {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".rlim_cur = ")?;
        printer.write_number(self.rlim_cur, &FORMATS::DEC)?;
        printer.write(b", .rlim_max = ")?;
        printer.write_number(self.rlim_max, &FORMATS::DEC)?;
        Ok(())
    }
}

impl crate::Print for compat_rlimit {
    fn print(&self, printer: &mut crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".rlim_cur = ")?;
        printer.write_number(self.rlim_cur, &FORMATS::DEC)?;
        printer.write(b", .rlim_max = ")?;
        printer.write_number(self.rlim_max, &FORMATS::DEC)?;
        Ok(())
    }
}

