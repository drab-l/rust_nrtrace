use crate::config::FORMATS;

macro_rules! offset_of {
    ($type:ty, $member:ident) => { unsafe { (&(*(4096 as *const $type)).$member).as_ptr() as usize - 4096} };
}

type SockFamily = types::UShrt;

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_un_path {
    sun_family: SockFamily,
    sun_path: [u8; 0],
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_un_unname {
    sun_family: SockFamily,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_un_abstract {
    sun_family: SockFamily,
    _padding: u8,
    sun_path: [u8; 0],
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_in {
    sin_family: SockFamily,
    sin_port: u16,
    sin_addr: [u8; 4],
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_in6 {
    sin6_family: SockFamily,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: [u8; 16],
    sin6_scope_id: u32,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_nl {
    nl_family: SockFamily,
    nl_pad: u16,
    nl_pid: u32,
    nl_groups: u32,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct sockaddr_vm {
    svm_family: SockFamily,
    svm_reserved1: u16,
    svm_port: u16,
    svm_cid: u16,
}

unsafe fn strlen(base: *const u8) -> usize {
    let mut addr = base;
    unsafe {
        while *addr != 0 {
            addr = addr.add(1);
        }
    }
    addr as usize - base as usize
}

impl crate::Print for sockaddr_un_path {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".sun_family = AF_UNIX")
    }
    fn print_flex_tail(&self, printer: &crate::Printer, buf: &[u8], _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write_maybe_ascii(buf)
    }
    fn flex_tail_size(&self) -> usize {
        let base = self.sun_path.as_ptr();
        unsafe { strlen(base) }
    }
    fn flex_tail_offset(&self) -> usize {
        offset_of!(sockaddr_un_path, sun_path)
    }
    fn total_size(&self) -> usize where Self: Sized {
        self.flex_tail_size() + self.flex_tail_offset()
    }
}

impl crate::Print for sockaddr_un_abstract {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".sun_family = AF_UNIX")
    }
    fn print_flex_tail(&self, printer: &crate::Printer, buf: &[u8], _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b", sun_path = \"\\x00")?;
        printer.write_maybe_ascii(buf)?;
        printer.write(b"\"")
    }
    fn flex_tail_size(&self) -> usize {
        let base = self.sun_path.as_ptr();
        unsafe { strlen(base) }
    }
    fn flex_tail_offset(&self) -> usize {
        offset_of!(sockaddr_un_abstract, sun_path)
    }
    fn total_size(&self) -> usize where Self: Sized {
        self.flex_tail_size() + self.flex_tail_offset()
    }
}

impl crate::Print for sockaddr_un_unname {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".sun_family = AF_UNIX")?;
        Ok(())
    }
}

impl crate::Print for sockaddr_in {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".sin_family = AF_INET")?;
        printer.write(b", .sin_port = hton(")?;
        printer.write_number(self.sin_port.swap_bytes(), &FORMATS::DEC)?;
        printer.write(b"), .sin_addr = \"")?;
        printer.write_number(self.sin_addr[0], &FORMATS::DEC)?;
        printer.write(b".")?;
        printer.write_number(self.sin_addr[1], &FORMATS::DEC)?;
        printer.write(b".")?;
        printer.write_number(self.sin_addr[2], &FORMATS::DEC)?;
        printer.write(b".")?;
        printer.write_number(self.sin_addr[3], &FORMATS::DEC)?;
        printer.write(b"\"")?;
        Ok(())
    }
}

impl crate::Print for sockaddr_in6 {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".sin6_family = AF_INET6")?;
        printer.write(b", .sin6_flowinfo = hton(")?;
        printer.write_number(self.sin6_flowinfo.swap_bytes(), &FORMATS::DEC)?;
        printer.write(b", .sin6_port = hton(")?;
        printer.write_number(self.sin6_port.swap_bytes(), &FORMATS::DEC)?;
        printer.write(b"), .sin6_addr = \"")?;
        printer.write_hex(self.sin6_addr[0])?;
        for addr in self.sin6_addr[1..].iter() {
            printer.write(b":")?;
            printer.write_hex(*addr)?;
        }
        printer.write(b", .sin6_scope_id = ")?;
        printer.write_number(self.sin6_scope_id, &FORMATS::DEC)?;
        printer.write(b"\"")?;
        Ok(())
    }
}

impl crate::Print for sockaddr_nl {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".nl_family = AF_NETLINK")?;
        printer.write(b", .nl_pid = ")?;
        printer.write_number(self.nl_pid, &FORMATS::DEC)?;
        printer.write(b", .nl_groups = ")?;
        printer.write_number(self.nl_groups, &FORMATS::DEC)?;
        Ok(())
    }
}

impl crate::Print for sockaddr_vm {
    fn print(&self, printer: &crate::Printer, _pid: types::Pid, _e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".svm_family = AF_VSOCK")?;
        printer.write(b", .svm_port = ")?;
        printer.write_number(self.svm_port, &FORMATS::DEC)?;
        printer.write(b", .svm_cid = ")?;
        printer.write_number(self.svm_cid, &FORMATS::DEC)?;
        Ok(())
    }
}

pub fn write_sockaddr(printer: &crate::Printer, buf: &[u8], pid: types::Pid, e: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    if buf.len() < std::mem::size_of::<SockFamily>() {
        printer.write(b"{")?;
        printer.write_as_hex(buf)?;
        printer.write(b"}")?;
        Ok(())
    } else {
        match SockFamily::from_ne_bytes([buf[0], buf[1]]) {
            AF_UNIX => {
                if buf.len() <= (offset_of!(sockaddr_un_path, sun_path) + 1) {
                    printer.write_struct_from_buf::<sockaddr_un_unname>(buf, pid, e)?;
                } else if buf[offset_of!(sockaddr_un_path, sun_path)] == 0 {
                    printer.write_struct_from_buf::<sockaddr_un_abstract>(buf, pid, e)?;
                } else {
                    printer.write_struct_from_buf::<sockaddr_un_path>(buf, pid, e)?;
                }
            },
            AF_INET => { printer.write_struct_from_buf::<sockaddr_in>(buf, pid, e)?; },
            AF_INET6 => { printer.write_struct_from_buf::<sockaddr_in6>(buf, pid, e)?; },
            AF_NETLINK => { printer.write_struct_from_buf::<sockaddr_nl>(buf, pid, e)?; },
            AF_VSOCK => { printer.write_struct_from_buf::<sockaddr_vm>(buf, pid, e)?; },
            _ => {
                printer.write(b"{")?;
                printer.write_as_hex(buf)?;
                printer.write(b"}")?;
            },
        }
        Ok(())
    }
}

#[allow(dead_code)]const AF_UNSPEC: SockFamily = 0;
#[allow(dead_code)]const AF_UNIX: SockFamily = 1;
#[allow(dead_code)]const AF_LOCAL: SockFamily = 1;
#[allow(dead_code)]const AF_INET: SockFamily = 2;
#[allow(dead_code)]const AF_AX25: SockFamily = 3;
#[allow(dead_code)]const AF_IPX: SockFamily = 4;
#[allow(dead_code)]const AF_APPLETALK: SockFamily = 5;
#[allow(dead_code)]const AF_NETROM: SockFamily = 6;
#[allow(dead_code)]const AF_BRIDGE: SockFamily = 7;
#[allow(dead_code)]const AF_ATMPVC: SockFamily = 8;
#[allow(dead_code)]const AF_X25: SockFamily = 9;
#[allow(dead_code)]const AF_INET6: SockFamily = 10;
#[allow(dead_code)]const AF_ROSE: SockFamily = 11;
#[allow(dead_code)]const AF_DECNET: SockFamily = 12;
#[allow(dead_code)]const AF_NETBEUI: SockFamily = 13;
#[allow(dead_code)]const AF_SECURITY: SockFamily = 14;
#[allow(dead_code)]const AF_KEY: SockFamily = 15;
#[allow(dead_code)]const AF_NETLINK: SockFamily = 16;
#[allow(dead_code)]const AF_PACKET: SockFamily = 17;
#[allow(dead_code)]const AF_ASH: SockFamily = 18;
#[allow(dead_code)]const AF_ECONET: SockFamily = 19;
#[allow(dead_code)]const AF_ATMSVC: SockFamily = 20;
#[allow(dead_code)]const AF_RDS: SockFamily = 21;
#[allow(dead_code)]const AF_SNA: SockFamily = 22;
#[allow(dead_code)]const AF_IRDA: SockFamily = 23;
#[allow(dead_code)]const AF_PPPOX: SockFamily = 24;
#[allow(dead_code)]const AF_WANPIPE: SockFamily = 25;
#[allow(dead_code)]const AF_LLC: SockFamily = 26;
#[allow(dead_code)]const AF_IB: SockFamily = 27;
#[allow(dead_code)]const AF_MPLS: SockFamily = 28;
#[allow(dead_code)]const AF_CAN: SockFamily = 29;
#[allow(dead_code)]const AF_TIPC: SockFamily = 30;
#[allow(dead_code)]const AF_BLUETOOTH: SockFamily = 31;
#[allow(dead_code)]const AF_IUCV: SockFamily = 32;
#[allow(dead_code)]const AF_RXRPC: SockFamily = 33;
#[allow(dead_code)]const AF_ISDN: SockFamily = 34;
#[allow(dead_code)]const AF_PHONET: SockFamily = 35;
#[allow(dead_code)]const AF_IEEE802154: SockFamily = 36;
#[allow(dead_code)]const AF_CAIF: SockFamily = 37;
#[allow(dead_code)]const AF_ALG: SockFamily = 38;
#[allow(dead_code)]const AF_NFC: SockFamily = 39;
#[allow(dead_code)]const AF_VSOCK: SockFamily = 40;
#[allow(dead_code)]const AF_KCM: SockFamily = 41;
#[allow(dead_code)]const AF_QIPCRTR: SockFamily = 42;
#[allow(dead_code)]const AF_SMC: SockFamily = 43;
#[allow(dead_code)]const AF_XDP: SockFamily = 44;

