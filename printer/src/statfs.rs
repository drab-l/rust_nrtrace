use crate::FORMATS;

#[repr(C)]#[allow(non_camel_case_types)]
pub struct statfs {
    f_type: u64,
    f_bsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [crate::number::A64SIntDec; 2],
    f_namelen: u64,
    f_frsize: u64,
    f_flags: u64,
    f_spare: [u64; 4],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_statfs{
    f_type: u32,
    f_bsize: u32,
    f_blocks: u32,
    f_bfree: u32,
    f_bavail: u32,
    f_files: u32,
    f_ffree: u32,
    f_fsid: [crate::number::A32SIntDec; 2],
    f_namelen: u32,
    f_frsize: u32,
    f_flags: u32,
    f_spare: [u32; 4],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct statfs64 {
    f_type: u64,
    f_bsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [crate::number::A32SIntDec; 2],
    f_namelen: u64,
    f_frsize: u64,
    f_flags: u64,
    f_spare: [u64; 4],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_statfs64 {
    f_type: u32,
    f_bsize: u32,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [crate::number::A64SIntDec; 2],
    f_namelen: u32,
    f_frsize: u32,
    f_flags: u32,
    f_spare: [u32; 4],
}

macro_rules! statfs_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &mut crate::Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b".f_type = ")?; printer.write_number(self.f_type, &FORMATS::HEX)?;
                printer.write(b", .f_bsize = ")?; printer.write_number(self.f_bsize, &FORMATS::DEC)?;
                printer.write(b", .f_blocks = ")?; printer.write_number(self.f_blocks, &FORMATS::DEC)?;
                printer.write(b", .f_bfree = ")?; printer.write_number(self.f_bfree, &FORMATS::DEC)?;
                printer.write(b", .f_bavail = ")?; printer.write_number(self.f_bavail, &FORMATS::DEC)?;
                printer.write(b", .f_files = ")?; printer.write_number(self.f_files, &FORMATS::DEC)?;
                printer.write(b", .f_ffree = ")?; printer.write_number(self.f_ffree, &FORMATS::DEC)?;
                printer.write(b", .f_sid = ")?; printer.write_struct_array(&self.f_fsid, pid, e)?;
                printer.write(b", .f_namelen = ")?; printer.write_number(self.f_namelen, &FORMATS::DEC)?;
                printer.write(b", .f_frsize = ")?; printer.write_number(self.f_frsize, &FORMATS::DEC)?;
                printer.write(b", .f_flags = ")?; printer.write_number(self.f_flags, &FORMATS::HEX)?;
                Ok(())
            }
        }
    };
}

statfs_impl_print!(statfs);
statfs_impl_print!(compat_statfs);
statfs_impl_print!(statfs64);
statfs_impl_print!(compat_statfs64);

