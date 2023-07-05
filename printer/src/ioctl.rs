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
    (dir << IOC_DIRSHIFT) | (type_ << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

const fn IOC_TYPECHECK<T>() -> u32 { std::mem::size_of::<T>() as u32 }

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

pub struct WriteIoctl {
    write_ioctl_request: fn(printer: &Printer, value: u64) -> std::result::Result<bool, std::io::Error>,
    write_ioctl_arg: fn(printer: &Printer, value: u64, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<bool, std::io::Error>,
    write_ioctl_arg_nopeek: fn(printer: &Printer, value: u64) -> std::result::Result<bool, std::io::Error>,
}

const WRITER: [WriteIoctl; 1] = [termios::TERMIOS];

pub fn write_ioctl_request(printer: &Printer, value: u64) -> std::result::Result<(), std::io::Error> {
    for e in WRITER {
        if (e.write_ioctl_request)(printer, value)? {
            return Ok(())
        }
    }
    printer.write_number(value, &FORMATS::HEX)?;
    Ok(())
}

pub fn write_ioctl_arg(printer: &Printer, value: u64, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    for e in WRITER {
        if (e.write_ioctl_arg)(printer, value, _pid, _e)? {
            return Ok(())
        }
    }
    printer.write_number(value, &FORMATS::HEX)?;
    Ok(())
}

pub fn write_ioctl_arg_nopeek(printer: &Printer, value: u64) -> std::result::Result<(), std::io::Error> {
    for e in WRITER {
        if (e.write_ioctl_arg_nopeek)(printer, value)? {
            return Ok(())
        }
    }
    printer.write_number(value, &FORMATS::HEX)?;
    Ok(())
}
