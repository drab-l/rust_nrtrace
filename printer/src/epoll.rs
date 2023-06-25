use crate::FORMATS;

#[cfg(target_arch = "x86_64")]
#[repr(C,packed)]
pub struct epoll_event {
    events: types::UInt,
    data: u64,
}

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
pub struct epoll_event {
    events: types::UInt,
    data: u64,
}

const EVENTS: [(types::UInt, &'static str); 12] = [
(0x00000001, "EPOLLIN"), (0x00000002, "EPOLLPRI"), (0x00000004, "EPOLLOUT"), (0x00000008, "EPOLLERR"), (0x00000010, "EPOLLHUP"),
(0x00000020, "EPOLLNVAL"), (0x00000040, "EPOLLRDNORM"), (0x00000080, "EPOLLRDBAND"), (0x00000100, "EPOLLWRNORM"), (0x00000200, "EPOLLWRBAND"),
(0x00000400, "EPOLLMSG"), (0x00002000, "EPOLLRDHUP"),
];

impl crate::Print for epoll_event {
    fn print(&self, printer: &crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".events = ")?; printer.write_mask_enum(self.events, &EVENTS)?;
        printer.write(b", .data = ")?; printer.write_number(self.data, &FORMATS::HEX)
    }
}

const OP: [(types::SInt, &'static str); 3] = [ (1, "EPOLL_CTL_ADD"), (2, "EPOLL_CTL_DEL"), (3, "EPOLL_CTL_MOD"), ];

pub fn write_op(printer: &crate::Printer, value: u64, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
    let value = value as u32;
    if value == 0 {
        printer.write(b"0")
    } else {
        printer.write_enum(value as types::SInt, &OP)
    }
}
