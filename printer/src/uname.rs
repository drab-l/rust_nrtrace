
#[repr(C)]#[allow(non_camel_case_types)]
pub struct oldold_utsname {
    sysname: [u8; 9],
    nodename: [u8; 9],
    release: [u8; 9],
    version: [u8; 9],
    machine: [u8; 9],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct old_utsname {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct new_utsname {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

macro_rules! impl_print_null_sentinel_str {
    ($self:ident, $printer:ident, $member:ident, $prefix:expr) => {
        $printer.write($prefix)?;
        if let Some(pos) = $self.$member.iter().position(|x| *x == 0) {
            $printer.write(&$self.$member[0..pos])?;
        } else {
            $printer.write(b"\"\"")?;
        }
    };
}

impl crate::Print for new_utsname {
    fn print(&self, printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        impl_print_null_sentinel_str!(self, printer, sysname, b".sysname = ");
        impl_print_null_sentinel_str!(self, printer, nodename, b", .nodename = ");
        impl_print_null_sentinel_str!(self, printer, release, b", .release = ");
        impl_print_null_sentinel_str!(self, printer, version, b", .version = ");
        impl_print_null_sentinel_str!(self, printer, machine, b", .machine = ");
        impl_print_null_sentinel_str!(self, printer, domainname, b", .domainname = ");
        Ok(())
    }
}

macro_rules! old_utsname_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                impl_print_null_sentinel_str!(self, printer, sysname, b".sysname = ");
                impl_print_null_sentinel_str!(self, printer, nodename, b", .nodename = ");
                impl_print_null_sentinel_str!(self, printer, release, b", .release = ");
                impl_print_null_sentinel_str!(self, printer, version, b", .version = ");
                impl_print_null_sentinel_str!(self, printer, machine, b", .machine = ");
                Ok(())
            }
        }
    };
}

old_utsname_impl_print!(oldold_utsname);
old_utsname_impl_print!(old_utsname);
