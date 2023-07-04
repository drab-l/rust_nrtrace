#![allow(dead_code)]
use crate::FORMATS;
use crate::Printer;

mod termios;

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;

const IOC_SIZEBITS: u32 = 14;
const IOC_DIRBITS: u32 = 2;

const IOC_NRMASK: u32 = (1 << IOC_NRBITS) - 1;
const IOC_TYPEMASK: u32 = (1 << IOC_TYPEBITS) - 1;
const IOC_SIZEMASK: u32 = (1 << IOC_SIZEBITS) - 1;
const IOC_DIRMASK: u32 = (1 << IOC_DIRBITS) - 1;

const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;

const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn IOC(dir: u32, type_: u32, nr: u32, size: u32) -> u32 {
    (dir   << IOC_DIRSHIFT) |
    (type_ << IOC_TYPESHIFT) |
    (nr    << IOC_NRSHIFT) |
    (size  << IOC_SIZESHIFT)
}

const fn IOC_TYPECHECK<T>() -> u32 {
    std::mem::size_of::<T>() as u32
}

const fn IO(type_: u32, nr: u32) -> u32 { IOC(IOC_NONE, type_, nr, 0) }
const fn IOR(type_: u32, nr: u32, size: u32) -> u32 { IOC(IOC_READ, type_ ,nr , size) }
const fn IOW(type_: u32, nr: u32, size: u32) -> u32 { IOC(IOC_WRITE, type_, nr, size) }
const fn IOWR(type_: u32, nr: u32, size: u32) -> u32 { IOC(IOC_READ | IOC_WRITE, type_, nr, size) }

macro_rules! IO { ($ty: expr, $nr: expr) => { IO($ty, $nr) }; }
macro_rules! IOR { ($ty: expr, $nr: expr, $size: ty) => { IOR($ty, $nr, IOC_TYPECHECK::<$size>()) }; }
macro_rules! IOW { ($ty: expr, $nr: expr, $size: ty) => { IOW($ty, $nr, IOC_TYPECHECK::<$size>()) }; }
macro_rules! IOWR { ($ty: expr, $nr: expr, $size: ty) => { IOWR($ty, $nr, IOC_TYPECHECK::<$size>()) }; }

const fn _IO<const TYPE: u32, const NR: u32>() -> u32 { IO!(TYPE, NR) }
const fn _IOR<const TYPE: u32, const NR: u32, SIZE>() -> u32 { IOR!(TYPE, NR, SIZE) }
const fn _IOW<const TYPE: u32, const NR: u32, SIZE>() -> u32 { IOW!(TYPE, NR, SIZE) }
const fn _IOWR<const TYPE: u32, const NR: u32, SIZE>() -> u32 { IOWR!(TYPE, NR, SIZE) }
//const fn IOR_BAD(type_: u32, nr: u32,size)	_IOC(_IOC_READ,(type_),(nr),sizeof(size))
//const fn IOW_BAD(type_: u32, nr: u32, size)	_IOC(_IOC_WRITE,(type_),(nr),sizeof(size))
//const fn IOWR_BAD(type_: u32, nr: u32, size)	_IOC(_IOC_READ|_IOC_WRITE,(type_),(nr),sizeof(size))

/* used to decode ioctl numbers.. */
//#define _IOC_DIR(nr)		(((nr) >> _IOC_DIRSHIFT) & _IOC_DIRMASK)
//#define _IOC_TYPE(nr)		(((nr) >> _IOC_TYPESHIFT) & _IOC_TYPEMASK)
//#define _IOC_NR(nr)		(((nr) >> _IOC_NRSHIFT) & _IOC_NRMASK)
//#define _IOC_SIZE(nr)		(((nr) >> _IOC_SIZESHIFT) & _IOC_SIZEMASK)

/* ...and for the drivers/sound files... */

//#define IOC_IN		(_IOC_WRITE << _IOC_DIRSHIFT)
//#define IOC_OUT		(_IOC_READ << _IOC_DIRSHIFT)
//#define IOC_INOUT	((_IOC_WRITE|_IOC_READ) << _IOC_DIRSHIFT)
//#define IOCSIZE_MASK	(_IOC_SIZEMASK << _IOC_SIZESHIFT)
//#define IOCSIZE_SHIFT	(_IOC_SIZESHIFT)

pub fn write_ioctl_request(printer: &Printer, value: u64) -> std::result::Result<(), std::io::Error> {
    if termios::write_ioctl_request(printer, value)? {
    } else {
        printer.write_number(value, &FORMATS::HEX)?;
    }
    Ok(())
}

pub fn write_ioctl_arg(printer: &Printer, value: u64, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_number(value, &FORMATS::HEX)?;
    Ok(())
}
