use crate::types::a64::*;
#[repr(C)]
pub struct stat {
    pub st_dev: ULong,
    pub st_ino: ULong,
    pub st_nlink: ULong,
    pub st_mode: UInt,
    pub st_uid: UInt,
    pub st_gid: UInt,
    pub st_rdev: ULong,
    pub st_size: SLong,
    pub st_blksize: SLong,
    pub st_blocks: SLong,
    pub st_atime: ULong,
    pub st_atime_nsec: ULong,
    pub st_mtime: ULong,
    pub st_mtime_nsec: ULong,
    pub st_ctime: ULong,
    pub st_ctime_nsec: ULong,
    pub __unused4: ULong,
    pub __unused5: ULong,
    pub __unused6: ULong,
}

#[repr(C)]
pub struct compat_stat {
    pub st_dev: u16,
    pub __pad1: u16,
    pub st_ino: u32,
    pub st_mode: u16,
    pub st_nlink: u16,
    pub st_uid: u16,
    pub st_gid: u16,
    pub st_rdev: u16,
    pub __pad2: u16,
    pub st_size: u32,
    pub st_blksize: u32,
    pub st_blocks: u32,
    pub st_atime: u32,
    pub st_atime_nsec: u32,
    pub st_mtime: u32,
    pub st_mtime_nsec: u32,
    pub st_ctime: u32,
    pub st_ctime_nsec: u32,
    pub __unused4: u32,
    pub __unused5: u32,
}

