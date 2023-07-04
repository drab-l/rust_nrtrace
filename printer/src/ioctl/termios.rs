use super::{_IO, _IOR, _IOW};

#[allow(non_camel_case_types)]
type tcflag_t = types::UInt;
#[allow(non_camel_case_types)]
type cc_t = types::UChar;
#[allow(non_camel_case_types)]
type speed_t = types::UInt;

const NCCS: usize = 19;

#[repr(C)]#[allow(non_camel_case_types)]
struct termios2 {
    c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    c_line: cc_t,
    c_cc: [cc_t; NCCS],
    c_ispeed: speed_t,
    c_ospeed: speed_t,
}

const TERMIOS_TYPES: u32 = 'T' as u32;

const TCGETS: u32 = 0x5401;
const TCSETS: u32 = 0x5402;
const TCSETSW: u32 = 0x5403;
const TCSETSF: u32 = 0x5404;
const TCGETA: u32 = 0x5405;
const TCSETA: u32 = 0x5406;
const TCSETAW: u32 = 0x5407;
const TCSETAF: u32 = 0x5408;
const TCSBRK: u32 = 0x5409;
const TCXONC: u32 = 0x540A;
const TCFLSH: u32 = 0x540B;
const TIOCEXCL: u32 = 0x540C;
const TIOCNXCL: u32 = 0x540D;
const TIOCSCTTY: u32 = 0x540E;
const TIOCGPGRP: u32 = 0x540F;
const TIOCSPGRP: u32 = 0x5410;
const TIOCOUTQ: u32 = 0x5411;
const TIOCSTI: u32 = 0x5412;
const TIOCGWINSZ: u32 = 0x5413;
const TIOCSWINSZ: u32 = 0x5414;
const TIOCMGET: u32 = 0x5415;
const TIOCMBIS: u32 = 0x5416;
const TIOCMBIC: u32 = 0x5417;
const TIOCMSET: u32 = 0x5418;
const TIOCGSOFTCAR: u32 = 0x5419;
const TIOCSSOFTCAR: u32 = 0x541A;
const FIONREAD: u32 = 0x541B;
const TIOCINQ: u32 = FIONREAD;
const TIOCLINUX: u32 = 0x541C;
const TIOCCONS: u32 = 0x541D;
const TIOCGSERIAL: u32 = 0x541E;
const TIOCSSERIAL: u32 = 0x541F;
const TIOCPKT: u32 = 0x5420;
const FIONBIO: u32 = 0x5421;
const TIOCNOTTY: u32 = 0x5422;
const TIOCSETD: u32 = 0x5423;
const TIOCGETD: u32 = 0x5424;
const TCSBRKP: u32 = 0x5425;
const TIOCSBRK: u32 = 0x5427;
const TIOCCBRK: u32 = 0x5428;
const TIOCGSID: u32 = 0x5429;
const TCGETS2: u32 = _IOR::<TERMIOS_TYPES, 0x2A, termios2>();
const TCSETS2: u32 = _IOW::<TERMIOS_TYPES, 0x2B, termios2>();
const TCSETSW2: u32 = _IOW::<TERMIOS_TYPES, 0x2C, termios2>();
const TCSETSF2: u32 = _IOW::<TERMIOS_TYPES, 0x2D, termios2>();
const TIOCGPTN: u32 = _IOR::<TERMIOS_TYPES, 0x30, types::UInt>();
const TIOCSPTLCK: u32 = _IOW::<TERMIOS_TYPES, 0x31, types::SInt>();
const TIOCGDEV: u32 = _IOR::<TERMIOS_TYPES, 0x32, types::SInt>();
const TIOCSIG: u32 = _IOW::<TERMIOS_TYPES, 0x36, types::SInt>();
const TIOCGPKT: u32 = _IOR::<TERMIOS_TYPES, 0x38, types::SInt>();
const TIOCGPTLCK: u32 = _IOR::<TERMIOS_TYPES, 0x39, types::SInt>();
const TIOCGEXCL: u32 = _IOR::<TERMIOS_TYPES, 0x40, types::SInt>();
const TIOCGPTPEER: u32 = _IO::<TERMIOS_TYPES, 0x41>();
//const TIOCGISO7816: u32 = _IOR::<TERMIOS_TYPES, 0x42, serial_iso7816>();
//const TIOCSISO7816: u32 = _IOWR::<TERMIOS_TYPES, 0x43, serial_iso7816>();

const TERMIOS_REQ: [(u32, &'static str); 53] = [
(TCGETS, "TCGETS"), (TCSETS, "TCSETS"), (TCSETSW, "TCSETSW"), (TCSETSF, "TCSETSF"), (TCGETA, "TCGETA"),
(TCSETA, "TCSETA"), (TCSETAW, "TCSETAW"), (TCSETAF, "TCSETAF"), (TCSBRK, "TCSBRK"), (TCXONC, "TCXONC"),
(TCFLSH, "TCFLSH"), (TIOCEXCL, "TIOCEXCL"), (TIOCNXCL, "TIOCNXCL"), (TIOCSCTTY, "TIOCSCTTY"), (TIOCGPGRP, "TIOCGPGRP"),
(TIOCSPGRP, "TIOCSPGRP"), (TIOCOUTQ, "TIOCOUTQ"), (TIOCSTI, "TIOCSTI"), (TIOCGWINSZ, "TIOCGWINSZ"), (TIOCSWINSZ, "TIOCSWINSZ"),
(TIOCMGET, "TIOCMGET"), (TIOCMBIS, "TIOCMBIS"), (TIOCMBIC, "TIOCMBIC"), (TIOCMSET, "TIOCMSET"), (TIOCGSOFTCAR, "TIOCGSOFTCAR"),
(TIOCSSOFTCAR, "TIOCSSOFTCAR"), (FIONREAD, "FIONREAD"), (TIOCINQ, "TIOCINQ"), (TIOCLINUX, "TIOCLINUX"), (TIOCCONS, "TIOCCONS"),
(TIOCGSERIAL, "TIOCGSERIAL"), (TIOCSSERIAL, "TIOCSSERIAL"), (TIOCPKT, "TIOCPKT"), (FIONBIO, "FIONBIO"), (TIOCNOTTY, "TIOCNOTTY"),
(TIOCSETD, "TIOCSETD"), (TIOCGETD, "TIOCGETD"), (TCSBRKP, "TCSBRKP"), (TIOCSBRK, "TIOCSBRK"), (TIOCCBRK, "TIOCCBRK"),
(TIOCGSID, "TIOCGSID"),
(TCGETS2, "TCGETS2"), (TCSETS2, "TCSETS2"), (TCSETSW2, "TCSETSW2"), (TCSETSF2, "TCSETSF2"),
(TIOCGPTN, "TIOCGPTN"), (TIOCSPTLCK, "TIOCSPTLCK"), (TIOCGDEV, "TIOCGDEV"), (TIOCSIG, "TIOCSIG"),
(TIOCGPKT, "TIOCGPKT"), (TIOCGPTLCK, "TIOCGPTLCK"), (TIOCGEXCL, "TIOCGEXCL"), (TIOCGPTPEER, "TIOCGPTPEER"),
];

pub fn write_ioctl_request(printer: &crate::Printer, value: u64) -> std::result::Result<bool, std::io::Error> {
    printer.try_write_enum(value as u32, &TERMIOS_REQ)
}
