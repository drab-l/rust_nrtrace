use arch::types::{a64, a32};
use arch::sys_uni::NR;
use crate::FORMATS;
use crate::iovec::{iovec, compat_iovec};

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

#[repr(C)]#[allow(non_camel_case_types)]
pub struct msghdr {
    msg_name: a64::Ptr,
    msg_namelen: a64::SInt,
    msg_iov: a64::Ptr,
    msg_iovlen: a64::USizeT,
    msg_control: a64::Ptr,
    msg_controllen: a64::USizeT,
    msg_flags: a64::UInt,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct cmsghdr {
    cmsg_len: a64::USizeT,
    cmsg_level: a64::SInt,
    cmsg_type: a64::SInt,
    cmsg_data: [u8; 0],
}

#[repr(C)]#[allow(non_camel_case_types)]
pub struct compat_msghdr {
    msg_name: a32::Ptr,
    msg_namelen: a32::SInt,
    msg_iov: a32::Ptr,
    msg_iovlen: a32::USizeT,
    msg_control: a32::Ptr,
    msg_controllen: a32::USizeT,
    msg_flags: a32::UInt,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct compat_cmsghdr {
    cmsg_len: a32::USizeT,
    cmsg_level: a32::SInt,
    cmsg_type: a32::SInt,
    cmsg_data: [u8; 0],
}

macro_rules! offset_of {
    ($type:ty, $member:ident) => { unsafe { (&(*(4096 as *const $type)).$member).as_ptr() as usize - 4096} };
}

const SEND_FLAG: [(u32, &'static str); 7] = [ (0x800, "MSG_CONFIRM"), (4, "MSG_DONTROUTE"), (0x40, "MSG_DONTWAIT"), (0x80, "MSG_EOR"), (0x8000, "MSG_MORE"), (0x4000, "MSG_NOSIGNAL"), (1, "MSG_OOB"),];

pub fn write_send_flag(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    if value == 0 {
        printer.write(b"0")?;
    } else {
        printer.write_mask_enum(value as u32, &SEND_FLAG)?;
    }
    Ok(())
}

macro_rules! cmsg_align {
    ($cmsghdr:ident) => {
        (($cmsghdr.cmsg_len as usize + super::sizeof(&$cmsghdr.cmsg_len) - 1) & !(super::sizeof(&$cmsghdr.cmsg_len) - 1)) as usize
    };
}

macro_rules! cmsg_print {
    ($self:ident, $printer:ident) => {
        $printer.write(b".cmsg_len = ")?;
        $printer.write_number($self.cmsg_len, &FORMATS::DEC)?;
        $printer.write(b", .cmsg_level = ")?;
        $printer.write_number($self.cmsg_level, &FORMATS::DEC)?;
        $printer.write(b", .cmsg_type = ")?;
        $printer.write_number($self.cmsg_type, &FORMATS::DEC)?;
    };
}

macro_rules! cmsg_print_tail {
    ($buf:ident, $printer:ident) => {
        $printer.write(b", .cmsg_data = ")?;
        $printer.write_as_hex($buf)?;
    };
}

macro_rules! cmsg_impl_print {
    ($type:ty) => {
        impl crate::Print for $type {
            fn print(&self, printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                cmsg_print!(self, printer);
                Ok(())
            }
            fn print_flex_tail(&self, printer: &mut crate::Printer, buf: &[u8], _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                cmsg_print_tail!(buf, printer);
                Ok(())
            }
            fn flex_tail_size(&self) -> usize {
                (self.cmsg_len as usize) -  offset_of!($type, cmsg_data)
            }
            fn flex_tail_offset(&self) -> usize {
                offset_of!($type, cmsg_data)
            }
            fn total_size(&self) -> usize {
                cmsg_align!(self)
            }
            fn print_array_delim(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b",\n\t")
            }
            fn print_array_prefix(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b"\n\t")
            }
            fn print_array_suffix(printer: &mut crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b"\n\t")
            }
        }
    };
}

fn write_cmsghdr_array<T: crate::Print>(printer: &mut crate::Printer, buf: &[u8], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    if buf.len() < std::mem::size_of::<T>() {
        return printer.write(b"{}");
    }
    printer.write_flex_tail_struct_array_from_buf::<T>(buf, pid, e)?;
    Ok(())
}

macro_rules!  msghdr_impl_print {
    ($msghdr:ty, $iovec:ty, $cmsghdr:ty) => {
        cmsg_impl_print!($cmsghdr);
        impl crate::Print for $msghdr {
            fn print(&self, printer: &mut crate::Printer, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
                printer.write(b".msg_name = ")?;
                printer.peek_write_maybe_ascii(self.msg_name as types::Ptr, self.msg_namelen as usize, pid, e)?;
                printer.write(b", .msg_namelen = ")?;
                printer.write_number(self.msg_namelen, &FORMATS::DEC)?;
                printer.write(b", .msg_iov = ")?;
                if self.msg_iov == 0 || self.msg_iovlen == 0 {
                    printer.write_number(self.msg_iov, &FORMATS::HEX)?;
                } else {
                    printer.peek_write_struct_array::<$iovec>(self.msg_iov as types::Ptr, self.msg_iovlen as usize, pid, e)?;
                }
                printer.write(b", .msg_iovlen = ")?;
                printer.write_number(self.msg_iovlen, &FORMATS::DEC)?;
                printer.write(b", .msg_control = ")?;
                if self.msg_control == 0 || (self.msg_controllen as usize) < std::mem::size_of::<$cmsghdr>() {
                    printer.write_number(self.msg_control, &FORMATS::HEX)?;
                } else {
                    printer.peek_write_callback::<$cmsghdr, _>(self.msg_control as types::Ptr, self.msg_controllen as usize, write_cmsghdr_array::<$cmsghdr>, pid, e)?;
                }
                printer.write(b", .msg_controllen = ")?;
                printer.write_number(self.msg_controllen, &FORMATS::DEC)?;
                Ok(())
            }
        }
    };
}

msghdr_impl_print!(msghdr, iovec, cmsghdr);
msghdr_impl_print!(compat_msghdr, compat_iovec, compat_cmsghdr);

const DOMAIN: [(u32, &'static str); 45] = [
(0, "AF_UNSPEC"), (1, "AF_UNIX"), (2, "AF_INET"), (3, "AF_AX25"), (4, "AF_IPX"),
(5, "AF_APPLETALK"), (6, "AF_NETROM"), (7, "AF_BRIDGE"), (8, "AF_ATMPVC"), (9, "AF_X25"),
(10, "AF_INET6"), (11, "AF_ROSE"), (12, "AF_DECnet"), (13, "AF_NETBEUI"), (14, "AF_SECURITY"),
(15, "AF_KEY"), (16, "AF_NETLINK"), (17, "AF_PACKET"), (18, "AF_ASH"), (19, "AF_ECONET"),
(20, "AF_ATMSVC"), (21, "AF_RDS"), (22, "AF_SNA"), (23, "AF_IRDA"), (24, "AF_PPPOX"),
(25, "AF_WANPIPE"), (26, "AF_LLC"), (27, "AF_IB"), (28, "AF_MPLS"), (29, "AF_CAN"),
(30, "AF_TIPC"), (31, "AF_BLUETOOTH"), (32, "AF_IUCV"), (33, "AF_RXRPC"), (34, "AF_ISDN"),
(35, "AF_PHONET"), (36, "AF_IEEE802154"), (37, "AF_CAIF"), (38, "AF_ALG"), (39, "AF_NFC"),
(40, "AF_VSOCK"), (41, "AF_KCM"), (42, "AF_QIPCRTR"), (43, "AF_SMC"), (44, "AF_XDP"),
];

pub fn write_domain(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    printer.write_enum(value, &DOMAIN)
}

const SOCK_TYPE: [(u32, &'static str); 7] = [ (1, "STREAM"), (2, "DGRAM"), (3, "RAW"), (4, "RDM"), (5, "SEQPACKET"), (6, "DCCP"), (10, "PACKET"), ];
const SOCK_FLAG: [(u32, &'static str); 3] = [ (0o02004000, "SOCK_NONBLOCK | SOCK_CLOEXEC"), (0o00004000, "SOCK_NONBLOCK"), (0o02000000, "SOCK_CLOEXEC"), ];

pub fn write_flag(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &SOCK_FLAG)
}

pub fn write_type(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    for (v,n) in SOCK_FLAG.iter() {
        if value == *v {
            printer.write(n.as_bytes())?;
            printer.write(b" | ")?;
        }
    }
    printer.write_enum(value & 0xF, &SOCK_TYPE)
}

const CALL: [(u32, &'static str); 21] = [
(0, "unknwon"), (1, "socket"), (2, "bind"), (3, "connect"), (4, "listen"), (5, "accept"),
(6, "getsockname"), (7, "getpeername"), (8, "socketpair"), (9, "send"), (10, "recv"),
(11, "sendto"), (12, "recvfrom"), (13, "shutdown"), (14, "setsockopt"), (15, "getsockopt"),
(16, "sendmsg"), (17, "recvmsg"), (18, "accept4"), (19, "recvmmsg"), (20, "sendmmsg"),
];

pub fn write_socketcall_call(printer: &mut crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    printer.write_enum(value as u32, &CALL)
}

pub fn write_socketcall_arg(printer: &mut crate::Printer, value: u64, pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    const UNI: [NR; 21] = [
        NR::sys_unknown, NR::sys_socket, NR::sys_bind, NR::sys_connect, NR::sys_listen, NR::sys_accept,
        NR::sys_getsockname, NR::sys_getpeername, NR::sys_socketpair, NR::sys_send, NR::sys_recv,
        NR::sys_sendto, NR::sys_recvfrom, NR::sys_shutdown, NR::sys_setsockopt, NR::sys_getsockopt,
        NR::sys_sendmsg, NR::sys_recvmsg, NR::sys_accept4, NR::sys_recvmmsg, NR::sys_sendmmsg,
    ];
    const ARG_N: [u32; 21] = [0, 3, 3, 3, 2, 3, 3, 3, 4, 4, 4, 6, 6, 2, 5, 5, 3, 3, 4, 5, 4];
    let call = e.argn(peek::Arg::ONE) as usize;
    if call >= ARG_N.len() {
        printer.write_number_as_pointer(value)?;
        return Ok(());
    }
    let is64 = e.is_64();
    let mut addr = value as types::Ptr;
    let mut args: [u64; 6] = [0; 6];
    if is64 {
        for i in 0..ARG_N[call] as usize {
            args[i] = peek::peek_data::<a64::ULong>(pid, addr)? as u64;
            addr += std::mem::size_of::<a64::ULong>() as types::Ptr;
        }
    } else {
        for i in 0..ARG_N[call] as usize {
            args[i] = peek::peek_data::<a32::ULong>(pid, addr)? as u64;
            addr += std::mem::size_of::<a32::ULong>() as types::Ptr;
        }
    }
    printer.write(b"{")?;
    let dummy = peek::SyscallSummery::new_dummy_entry(is64, UNI[call], e.sysnum(), args, 0);
    if e.is_entry() {
        match printer.conf.get_print_info(dummy.uni_sysnum()) {
            p if p.is_skip() => (),
            p if p.is_undef() => (),
            p  => {
                printer.write(CALL[call].1.as_bytes())?;
                printer.write(b", ")?;
                printer.write_args_impl(p, pid, &dummy)?;
            }
        }
    } else {
        match printer.conf.get_print_info_for_ret_args(dummy.uni_sysnum()) {
            p if p.is_skip() => (),
            p if p.is_undef() => (),
            p => {
                printer.write(CALL[call].1.as_bytes())?;
                printer.write_ret_args_impl(p, pid, &dummy)?;
            },
        }
    }
    printer.write(b"}")?;
    Ok(())
}
