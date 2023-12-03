use crate::FORMATS;

#[repr(C)]
pub struct pollfd {
    fd: types::SInt,
    events: types::SShrt,
    revents: types::SShrt,
}

const EVENTS: [(types::SShrt, &'static str); 13] = [
(0x0001, "POLLIN"), (0x0002, "POLLPRI"), (0x0004, "POLLOUT"), (0x0008, "POLLERR"),
(0x0010, "POLLHUP"), (0x0020, "POLLNVAL"), (0x0040, "POLLRDNORM"), (0x0080, "POLLRDBAND"),
(0x0100, "POLLWRNORM"), (0x0200, "POLLWRBAND"), (0x0400, "POLLMSG"), (0x1000, "POLLREMOVE"),
(0x2000, "POLLRDHUP"),
];

impl crate::Print for pollfd {
    fn print(&self, printer: &crate::Printer, _: types::Pid, _: &peek::SyscallSummery) -> std::result::Result<(), std::io::Error> {
        printer.write(b".fd = ")?; printer.write_number(self.fd, &FORMATS::DEC)?;
        printer.write(b", .events = ")?; printer.write_mask_enum(self.events, &EVENTS)?;
        printer.write(b", .revents = ")?; printer.write_mask_enum(self.revents, &EVENTS)?;
        Ok(())
    }
}

