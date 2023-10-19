
use arch::sys_uni::NR;
use TYPES::*;

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum TYPES {
    SKIP, NONE, UNDEF,
    U8(FORMATS), I8(FORMATS), U16(FORMATS), I16(FORMATS), U32(FORMATS), I32(FORMATS), U64(FORMATS), I64(FORMATS),
    INT(FORMATS), UINT(FORMATS), ULONG(FORMATS), LONG(FORMATS), USIZE(FORMATS), SSIZE(FORMATS), PID,
    U64LOW, U64HIGH(FORMATS),
    AccessatFlag, AtFlag, Clockid, DirFd, EpollctlOp, FdFlag, IoctlArgNoPeek, IoctlReqest, LseekWhence, MadviseAdvice, MmapFlag, MmapProt, NewfstatatFlag, OpenFlag, RenameFlag, RlimitResource, SendFlag, SocketDomain, SocketFlag, SocketType, SocketcallCall,
    IoctlArg, /* maybe peek data */
    PTR,
    IntPtr(FORMATS), I64Ptr(FORMATS), StrPtr, StrPtrLenArgR, ArgsPtr, IntArrayPtrLen2,
    EpolleventPtr, EpolleventArrayPtrLenArgR, FdsetPtrArg1, IovecPtrLenArg3, IovecPtrLenArg3BufLenArgR, Linuxdirent64PtrLenArgR, MsghdrPtr, MsghdrPtrBufLenArgR,
    OldoldutsnamePtr, OldutsnamePtr, Rlimit64Ptr, RlimitPtr, SockaddrPtrLenArg3, SockaddrPtrLenArg3Ptr, SocketcallArgPtr, Statfs64Ptr, StatfsPtr, StatPtr, StatxPtr, SysinfoPtr, TimespecPtr, TimevalPtr, TimexPtr, TimezonePtr, UtsnamePtr,
    AsciiOrHexPtrLenArg3, AsciiOrHexPtrLenArgR,
}

impl TYPES {
    pub fn nopeek_type(&self) -> &Self {
        const PTR_: TYPES = PTR;
        const IOC_: TYPES = IoctlArgNoPeek;
        match self {
            r if *r >= PTR => &PTR_,
            r if *r == IoctlArg => &IOC_,
            r => r,
        }
    }
}
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum FORMATS { HEX, DEC, OCT }

pub struct SyscallPrintInfoSet {
    pub ret: TYPES,
    pub args: [TYPES; 6],
}

#[derive(Clone)]
pub struct SyscallPrintConf {
    conf: CONF,
    print: &'static [SyscallPrintInfoSet],
}

trait SyscallPrinter {
    fn get_print_info(&self) -> &'static [SyscallPrintInfoSet];
    fn get_print_info_for_ret_args(&self) -> &'static [SyscallPrintInfoSet];
}

#[derive(Clone, PartialEq)]
pub enum CONF { PRINT, SKIP, SIMPLE, NOPEEK }
struct PrintConf {
    nr: NR,
    conf: CONF,
}

#[derive(Copy, Clone)]
pub enum PrivData {
    NONE,
    IOVEC(usize),
    U64LOW(u64),
}

pub struct Config {
    default: Option<SyscallPrintConf>,
    conf: Vec<PrintConf>,
}

impl SyscallPrintConf {
    fn new(conf: CONF, print: &'static [SyscallPrintInfoSet]) -> Self { SyscallPrintConf{conf, print} }
    pub fn is_simple(&self) -> bool { self.conf == CONF::SIMPLE }
    pub fn is_skip(&self) -> bool { self.conf == CONF::SKIP }
    pub fn is_nopeek(&self) -> bool { self.conf == CONF::NOPEEK }
    pub fn is_undef(&self) -> bool { self.print[0].ret == UNDEF }
    pub fn get_print_info(&self, is64: bool) -> &'static SyscallPrintInfoSet {
        if self.print.len() == 1 || is64 { &self.print[0] } else { &self.print[1] }
    }
}

impl Config {
    pub fn new() -> Self {
        let conf: Vec<PrintConf> = vec![];
        Self{default: None, conf}
    }

    pub fn get_print_info(&self, nr: NR) -> SyscallPrintConf {
        match self.conf.iter().find(|x|{x.nr == nr}) {
            Some(PrintConf{nr:_, conf:CONF::SIMPLE}) => SyscallPrintConf::new(CONF::SIMPLE, nr.get_print_info()),
            Some(PrintConf{nr:_, conf:CONF::SKIP}) => SyscallPrintConf::new(CONF::SKIP, &SKIPPRINT),
            Some(PrintConf{nr:_, conf:CONF::PRINT}) => SyscallPrintConf::new(CONF::PRINT, nr.get_print_info()),
            Some(PrintConf{nr:_, conf:CONF::NOPEEK}) => SyscallPrintConf::new(CONF::NOPEEK, nr.get_print_info()),
            _ => if self.default.is_some() { self.default.as_ref().unwrap().clone() } else { SyscallPrintConf::new(CONF::PRINT, nr.get_print_info())},
        }
    }

    pub fn get_print_info_for_ret_args(&self, nr: NR) -> SyscallPrintConf {
        match self.conf.iter().find(|x|{x.nr == nr}) {
            Some(PrintConf{nr:_, conf:CONF::SIMPLE}) => SyscallPrintConf::new(CONF::SIMPLE, nr.get_print_info_for_ret_args()),
            Some(PrintConf{nr:_, conf:CONF::SKIP}) => SyscallPrintConf::new(CONF::SKIP, &SKIPPRINT),
            Some(PrintConf{nr:_, conf:CONF::PRINT}) => SyscallPrintConf::new(CONF::PRINT, nr.get_print_info_for_ret_args()),
            Some(PrintConf{nr:_, conf:CONF::NOPEEK}) => SyscallPrintConf::new(CONF::NOPEEK, nr.get_print_info_for_ret_args()),
            _ => if self.default.is_some() { self.default.as_ref().unwrap().clone() } else { SyscallPrintConf::new(CONF::PRINT, nr.get_print_info_for_ret_args())},
        }
    }

    pub fn set_skip_for_default(&mut self) {
        self.default = Some(SyscallPrintConf::new(CONF::SKIP, &SKIPPRINT))
    }

    pub fn set_skip_by_name(&mut self, name: &str) {
        if let Some((_, nr)) = arch::sys_uni::map.iter().find(|(sys, _)|{ &name == sys }) {
            self.set_conf(*nr, CONF::SKIP);
        }
    }

    pub fn set_not_skip_by_name(&mut self, name: &str) {
        if let Some((_, nr)) = arch::sys_uni::map.iter().find(|(sys, _)|{ &name == sys }) {
            self.set_conf(*nr, CONF::PRINT);
        }
    }

    pub fn set_simple_by_name(&mut self, name: &str) {
        if let Some((_, nr)) = arch::sys_uni::map.iter().find(|(sys, _)|{ &name == sys }) {
            self.set_conf(*nr, CONF::SIMPLE);
        }
    }

    pub fn set_nopeek_by_name(&mut self, name: &str) {
        if let Some((_, nr)) = arch::sys_uni::map.iter().find(|(sys, _)|{ &name == sys }) {
            self.set_conf(*nr, CONF::NOPEEK);
        }
    }

    pub fn set_skip_by_include_name(&mut self, name: &str) {
        arch::sys_uni::map.iter().for_each(|(sys, nr)|{
            if sys.contains(name) { self.set_conf(*nr, CONF::SKIP); }
        })
    }

    pub fn set_not_skip_by_include_name(&mut self, name: &str) {
        arch::sys_uni::map.iter().for_each(|(sys, nr)|{
            if sys.contains(name) { self.set_conf(*nr, CONF::PRINT); }
        })
    }

    pub fn set_simple_by_include_name(&mut self, name: &str) {
        arch::sys_uni::map.iter().for_each(|(sys, nr)|{
            if sys.contains(name) { self.set_conf(*nr, CONF::SIMPLE); }
        })
    }

    pub fn set_nopeek_by_include_name(&mut self, name: &str) {
        arch::sys_uni::map.iter().for_each(|(sys, nr)|{
            if sys.contains(name) { self.set_conf(*nr, CONF::NOPEEK); }
        })
    }

    fn set_conf(&mut self, nr: NR, conf: CONF) {
        if let Some(e) = self.conf.iter_mut().find(|x|{x.nr == nr}) {
            e.conf = conf;
        } else {
            self.conf.push(PrintConf{nr, conf});
        }
    }
}

include!("config.inc.rs");

define_print_info_all_fmt_type!(U8HEX, U8DEC, U8OCT, U8);
define_print_info_all_fmt_type!(I8HEX, I8DEC, I8OCT, I8);
define_print_info_all_fmt_type!(U16HEX, U16DEC, U16OCT, U16);
define_print_info_all_fmt_type!(I16HEX, I16DEC, I16OCT, I16);
define_print_info_all_fmt_type!(U32HEX, U32DEC, U32OCT, U32);
define_print_info_all_fmt_type!(I32HEX, I32DEC, I32OCT, I32);
define_print_info_all_fmt_type!(U64HEX, U64DEC, U64OCT, U64);
define_print_info_all_fmt_type!(I64HEX, I64DEC, I64OCT, I64);

define_print_info_all_fmt_type!(INTHEX, INTDEC, INTOCT, INT);
define_print_info_all_fmt_type!(UINTHEX, UINTDEC, UINTOCT, UINT);
define_print_info_all_fmt_type!(LONGHEX, LONGDEC, LONGOCT, LONG);
define_print_info_all_fmt_type!(ULONGHEX, ULONGDEC, ULONGOCT, ULONG);
define_print_info_all_fmt_type!(USIZEHEX, USIZEDEC, USIZEOCT, USIZE);
define_print_info_all_fmt_type!(SSIZEHEX, SSIZEDEC, SSIZEOCT, SSIZE);

define_print_info_all_fmt_type!(OFFHEX, OFFDEC, OFFOCT, LONG);
define_print_info_all_fmt_type!(LOFFHEX, LOFFDEC, LOFFOCT, I64);

const LOFFLOW: TYPES = TYPES::U64LOW;
define_print_info_all_fmt_type!(LOFFHIGHHEX, LOFFHIGHDEC, LOFFHIGHOCT, U64HIGH);

define_print_info_all_fmt_type!(INTHEX_PTR, INTDEC_PTR, INTOCT_PTR, IntPtr);
define_print_info_all_fmt_type!(LOFFHEX_PTR, LOFFDEC_PTR, LOFFOCT_PTR, I64Ptr);

define_syscall_print_info!(UNDEFPRINT, UNDEF);
define_syscall_print_info!(SKIPPRINT, SKIP);
define_syscall_print_info!(UNKNOWN, U64HEX, U64HEX, U64HEX, U64HEX, U64HEX, U64HEX, U64HEX);

define_syscall_print_info!(SYS_ALIAS_INTDEC_INTDEC, INTDEC, INTDEC);
define_syscall_print_info!(SYS_ALIAS_INTDEC_INTDEC_INTDEC, INTDEC, INTDEC, INTDEC);

define_syscall_print_info!(ACCEPT, INTDEC, INTDEC, PTR, INTDEC_PTR);
define_syscall_print_info!(ACCEPT4, INTDEC, INTDEC, PTR, INTDEC_PTR, SocketFlag);
define_syscall_print_info!(ACCESS, INTDEC, StrPtr, INTOCT);
define_syscall_print_info!(ACCT, INTDEC, StrPtr);
define_syscall_print_info!(ADD_KEY, LONGDEC, StrPtr, StrPtr, PTR, USIZEDEC, LONGDEC);
define_syscall_print_info!(ADJTIMEX, INTDEC, PTR);
define_syscall_print_info!(ALARM, UINTDEC, UINTDEC);
define_syscall_print_info!(ARCH_PRCTL, LONGDEC, INTDEC, ULONGDEC);
define_syscall_print_info!(BDFLUSH, INTDEC, INTDEC, LONGDEC);
define_syscall_print_info!(BIND, INTDEC, INTDEC, SockaddrPtrLenArg3, INTDEC);
define_syscall_print_info!(BPF, INTDEC, INTDEC, PTR, UINTDEC); // TODO implement bpf_attr
define_syscall_print_info!(BRK, INTDEC, ULONGHEX);
define_syscall_print_info!(CAPGET, INTDEC, PTR, PTR); // TODO implement args
define_syscall_print_info!(CAPSET, INTDEC, PTR, PTR); // TODO implement args
define_syscall_print_info!(CHDIR, INTDEC, StrPtr);
define_syscall_print_info!(CHMOD, INTDEC, StrPtr, INTDEC, INTDEC);
define_syscall_print_info!(CHOWN, INTDEC, StrPtr, INTOCT);
define_syscall_print_info!(CHROOT, INTDEC, StrPtr);
define_syscall_print_info!(CLOCK_ADJTIME, INTDEC, Clockid, PTR);
define_syscall_print_info!(CLOCK_GETTIME, INTDEC, Clockid, PTR);
define_syscall_print_info!(CLOCK_NANOSLEEP, INTDEC, Clockid, INTDEC, TimespecPtr, PTR);
define_syscall_print_info!(CLOCK_SETTIME, INTDEC, Clockid, TimespecPtr);
define_syscall_print_info!(CLONE, LONGDEC, ULONGDEC, PTR, PTR, PTR, PTR);
define_syscall_print_info!(CLONE3, LONGDEC, PTR, USIZEDEC);
define_syscall_print_info!(CLOSE_RANGE, INTDEC, UINTDEC, UINTDEC, UINTDEC);
define_syscall_print_info!(COPY_FILE_RANGE, SSIZEDEC, INTDEC, LOFFDEC_PTR, INTDEC, LOFFDEC_PTR, USIZEDEC, UINTDEC);
define_syscall_print_info!(CREAT, INTDEC, StrPtr, INTOCT);
define_syscall_print_info!(CREATE_MODULE, PTR, StrPtr, USIZEDEC);
define_syscall_print_info!(DELETE_MODULE, INTDEC, StrPtr, INTDEC);
define_syscall_print_info!(DUP3, INTDEC, INTDEC, INTDEC, OpenFlag);
define_syscall_print_info!(EPOLL_CTL, INTDEC, INTDEC, EpollctlOp, INTDEC, EpolleventPtr);
define_syscall_print_info!(EPOLL_PWAIT, INTDEC, INTDEC, PTR, INTDEC, INTDEC);
define_syscall_print_info!(EPOLL_WAIT, INTDEC, INTDEC, PTR, INTDEC, INTDEC, PTR);
define_syscall_print_info!(EXECVE, INTDEC, StrPtr, ArgsPtr, ArgsPtr);
define_syscall_print_info!(EXECVEAT, INTDEC, DirFd, StrPtr, ArgsPtr, ArgsPtr, AtFlag);
define_syscall_print_info!(EXIT, NONE, INTDEC);
define_syscall_print_info!(FACCESSAT, INTDEC, DirFd, StrPtr, INTOCT);
define_syscall_print_info!(FACCESSAT2, INTDEC, DirFd, StrPtr, INTOCT, AccessatFlag);
define_syscall_print_info!(FADVISE64, INTDEC, INTDEC, LOFFDEC, USIZEDEC, INTHEX);
define_syscall_print_info_bits!(FADVISE64_64, INTDEC,
                                INTDEC, LOFFDEC, USIZEDEC, INTHEX,
                                INTDEC, U32HEX, U32HEX, U32HEX, U32HEX, INTHEX);
define_syscall_print_info_bits!(FALLOCATE, INTDEC,
                                INTDEC, INTDEC, LOFFDEC, LOFFDEC,
                                INTDEC, INTDEC, U32HEX, U32HEX, U32HEX, U32HEX);
define_syscall_print_info!(FCHDIR, INTDEC, UINTDEC);
define_syscall_print_info!(FCHMOD, INTDEC, INTDEC, INTOCT);
define_syscall_print_info!(FCHMODAT, INTDEC, DirFd, StrPtr, INTOCT, AtFlag);
define_syscall_print_info!(FCHOWN, INTDEC, INTDEC, INTDEC, INTDEC);
define_syscall_print_info!(FCHOWNAT, INTDEC, DirFd, StrPtr, INTDEC, INTDEC, AtFlag);
define_syscall_print_info!(FCNTL, INTDEC, UINTDEC, ULONGDEC);
define_syscall_print_info!(FCNTL64, INTDEC, UINTDEC, ULONGDEC);
define_syscall_print_info!(FGETXATTR, SSIZEDEC, INTDEC, StrPtr, PTR, USIZEDEC);
define_syscall_print_info!(FINIT_MODULE, INTDEC, INTDEC, StrPtr, INTHEX);
define_syscall_print_info!(FSTATFS, INTDEC, UINTDEC, PTR);
define_syscall_print_info!(FSTATFS64, INTDEC, UINTDEC, USIZEDEC, PTR);
define_syscall_print_info!(FUTEX, INTDEC, PTR, INTDEC, INTDEC, PTR, PTR, INTDEC);
define_syscall_print_info!(GETDENTS64, INTDEC, UINTDEC, PTR, UINTDEC);
define_syscall_print_info!(GETPID, PID);
define_syscall_print_info!(GETPGID, PID, PID);
define_syscall_print_info!(GETRANDOM, SSIZEDEC, PTR, USIZEDEC, INTDEC);
define_syscall_print_info!(GETTIMEOFDAY, INTDEC, PTR, PTR);
define_syscall_print_info!(GETXATTR, SSIZEDEC, StrPtr, StrPtr, PTR, USIZEDEC);
define_syscall_print_info!(INIT_MODULE, INTDEC, PTR, ULONGDEC, StrPtr);
define_syscall_print_info!(IOCTL, INTDEC, INTDEC, IoctlReqest, IoctlArg);
define_syscall_print_info!(LSEEK, OFFDEC, INTDEC, OFFDEC, LseekWhence);
define_syscall_print_info!(MADIVISE, INTDEC, PTR, INTDEC, MadviseAdvice);
define_syscall_print_info!(MKDIR, INTDEC, StrPtr, INTOCT);
define_syscall_print_info!(MKDIRAT, INTDEC, DirFd, StrPtr, INTOCT);
define_syscall_print_info!(MMAP, PTR, PTR, USIZEDEC, MmapProt, MmapFlag, INTDEC, OFFDEC);
define_syscall_print_info!(MPROTECT, INTDEC, PTR, USIZEDEC, MmapProt);
define_syscall_print_info!(MUNMAP, INTDEC, PTR, USIZEDEC);
define_syscall_print_info!(NANOSLEEP, INTDEC, TimespecPtr, PTR);
define_syscall_print_info!(NEWFSTATAT, INTDEC, DirFd, StrPtr, PTR, NewfstatatFlag);
define_syscall_print_info!(OLDOLDUNAME, INTDEC, PTR);
define_syscall_print_info!(OLDUNAME, INTDEC, PTR);
define_syscall_print_info!(OPEN, INTDEC, StrPtr, OpenFlag, INTOCT);
define_syscall_print_info!(OPENAT, INTDEC, DirFd, StrPtr, OpenFlag, INTOCT);
define_syscall_print_info!(OPENAT2, INTDEC, DirFd, StrPtr, PTR, SSIZEDEC);
define_syscall_print_info!(PIPE, INTDEC, PTR);
define_syscall_print_info!(PIPE2, INTDEC, PTR, FdFlag);
define_syscall_print_info!(PRLIMIT64, INTDEC, PID, RlimitResource, Rlimit64Ptr, PTR);
//define_syscall_print_info!(PREAD, SSIZEDEC, UINTDEC, PTR, USIZEDEC, OFFDEC);
define_syscall_print_info!(PREAD64, SSIZEDEC, UINTDEC, PTR, USIZEDEC, LOFFDEC);
define_syscall_print_info!(PREADV, SSIZEDEC, UINTDEC, PTR, ULONGDEC, LOFFLOW, LOFFHIGHDEC);
define_syscall_print_info!(PREADV2, SSIZEDEC, UINTDEC, PTR, ULONGDEC, LOFFLOW, LOFFHIGHDEC, INTHEX);
define_syscall_print_info!(PSELECT, INTDEC, INTDEC, FdsetPtrArg1, FdsetPtrArg1, FdsetPtrArg1, TimespecPtr, PTR);
//define_syscall_print_info!(PWRITE, SSIZEDEC, INTDEC, AsciiOrHexPtrLenArg3, USIZEDEC, OFFDEC);
define_syscall_print_info!(PWRITE64, SSIZEDEC, INTDEC, AsciiOrHexPtrLenArg3, USIZEDEC, LOFFDEC);
define_syscall_print_info!(PWRITEV, SSIZEDEC, INTDEC, AsciiOrHexPtrLenArg3, ULONGDEC, LOFFLOW, LOFFHIGHDEC);
define_syscall_print_info!(PWRITEV2, SSIZEDEC, INTDEC, AsciiOrHexPtrLenArg3, ULONGDEC, LOFFLOW, LOFFHIGHDEC, INTHEX);
define_syscall_print_info!(READ, SSIZEDEC, UINTDEC, PTR, USIZEDEC);
define_syscall_print_info!(READLINK, INTDEC, StrPtr, PTR, USIZEDEC);
define_syscall_print_info!(READLINKAT, DirFd, INTDEC, StrPtr, PTR, USIZEDEC);
define_syscall_print_info!(READV, SSIZEDEC, UINTDEC, PTR, INTDEC);
define_syscall_print_info!(RECVMSG, SSIZEDEC, INTDEC, PTR, SendFlag);
define_syscall_print_info!(RENAME, INTDEC, StrPtr, StrPtr);
define_syscall_print_info!(RENAMEAT, INTDEC, DirFd, StrPtr, DirFd, StrPtr);
define_syscall_print_info!(RENAMEAT2, INTDEC, DirFd, StrPtr, DirFd, StrPtr, RenameFlag);
define_syscall_print_info!(RSEQ, INTDEC, PTR, U32DEC, INTDEC, U32DEC);
define_syscall_print_info!(RT_SIGACTION, INTDEC, INTDEC, PTR, PTR, USIZEDEC);
define_syscall_print_info!(RT_SIGRETURN, INTDEC, ULONGDEC);
define_syscall_print_info!(RT_SIGPROCMASK, INTDEC, INTDEC, PTR, PTR, USIZEDEC);
define_syscall_print_info!(SENDMSG, SSIZEDEC, INTDEC, MsghdrPtr, SendFlag);
define_syscall_print_info!(SET_ROBUST_LIST, INTDEC, PTR, USIZEDEC);
define_syscall_print_info!(SET_THREAD_AREA, INTDEC, PTR);
define_syscall_print_info!(SET_TID_ADDRESS, LONGDEC, PTR);
define_syscall_print_info!(SETTIMEOFDAY, INTDEC, TimevalPtr, TimezonePtr);
define_syscall_print_info!(SIGALTSTACK, INTDEC, PTR, PTR);
define_syscall_print_info!(SOCKET, INTDEC, SocketDomain, SocketType, INTDEC);
define_syscall_print_info!(SOCKETCALL, INTDEC, SocketcallCall, SocketcallArgPtr);
define_syscall_print_info!(STATFS, INTDEC, StrPtr, PTR);
define_syscall_print_info!(STATFS64, INTDEC, StrPtr, USIZEDEC, PTR);
define_syscall_print_info!(STATX, INTDEC, INTDEC, StrPtr, INTDEC, INTDEC, PTR);
define_syscall_print_info!(SYSINFO, INTDEC, PTR);
define_syscall_print_info!(UGETRLIMIT, INTDEC, RlimitResource, PTR);
define_syscall_print_info!(UNAME, INTDEC, PTR);
define_syscall_print_info!(WAIT4, PID, PID, PTR, INTDEC, PTR);
define_syscall_print_info!(WRITE, SSIZEDEC, INTDEC, AsciiOrHexPtrLenArg3, USIZEDEC);
define_syscall_print_info!(WRITEV, SSIZEDEC, UINTDEC, IovecPtrLenArg3, INTDEC);

define_syscall_print_info_for_ret_args!(RET_ACCEPT, NONE, SockaddrPtrLenArg3Ptr, INTDEC_PTR);
define_syscall_print_info_for_ret_args!(RET_ADJTIMEX, TimexPtr);
define_syscall_print_info_for_ret_args!(RET_CLOCK_ADJTIME, NONE, TimexPtr);
define_syscall_print_info_for_ret_args!(RET_CLOCK_GETTIME, NONE, TimespecPtr);
define_syscall_print_info_for_ret_args!(RET_CLOCK_NANOSLEEP, NONE, NONE, TimespecPtr, TimespecPtr);
define_syscall_print_info_for_ret_args!(RET_COPY_FILE_RANGE, NONE, LOFFDEC_PTR, NONE, LOFFDEC_PTR);
define_syscall_print_info_for_ret_args!(RET_EPOLL_WAIT, NONE, EpolleventArrayPtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_FGETXATTR, NONE, NONE, AsciiOrHexPtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_GETDENTS64, NONE, Linuxdirent64PtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_GETTIMEOFDAY, TimevalPtr, TimezonePtr);
define_syscall_print_info_for_ret_args!(RET_IOCTL, NONE, NONE, IoctlArg);
define_syscall_print_info_for_ret_args!(RET_NANOSLEEP, TimespecPtr, TimespecPtr);
define_syscall_print_info_for_ret_args!(RET_NEWFSTATAT, NONE, NONE, StatPtr);
define_syscall_print_info_for_ret_args!(RET_OLDOLDUNAME, OldoldutsnamePtr);
define_syscall_print_info_for_ret_args!(RET_OLDUNAME, OldutsnamePtr);
define_syscall_print_info_for_ret_args!(RET_PIPE, IntArrayPtrLen2);
define_syscall_print_info_for_ret_args!(RET_READ, NONE, AsciiOrHexPtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_READLINK, NONE, StrPtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_READLINKAT, NONE, NONE, StrPtrLenArgR);
define_syscall_print_info_for_ret_args!(RET_READV, NONE, IovecPtrLenArg3BufLenArgR);
define_syscall_print_info_for_ret_args!(RET_RECVMSG, NONE, MsghdrPtrBufLenArgR);
define_syscall_print_info_for_ret_args!(RET_PRLIMIT64, NONE, NONE, NONE, Rlimit64Ptr);
define_syscall_print_info_for_ret_args!(RET_SOCKETCALL, NONE, SocketcallArgPtr);
define_syscall_print_info_for_ret_args!(RET_STATFS, NONE, StatfsPtr);
define_syscall_print_info_for_ret_args!(RET_STATFS64, NONE, NONE, Statfs64Ptr);
define_syscall_print_info_for_ret_args!(RET_STATX, NONE, NONE, NONE, NONE, StatxPtr);
define_syscall_print_info_for_ret_args!(RET_SYSINFO, SysinfoPtr);
define_syscall_print_info_for_ret_args!(RET_UGETRLIMIT, NONE, RlimitPtr);
define_syscall_print_info_for_ret_args!(RET_UNAME, UtsnamePtr);
define_syscall_print_info_for_ret_args!(RET_WAIT4, NONE, INTDEC_PTR);


impl SyscallPrinter for NR {
    fn get_print_info(&self) -> &'static [SyscallPrintInfoSet] {
        match self {
            NR::sys_unknown => &UNKNOWN,
            NR::sys_accept => &ACCEPT,
            NR::sys_accept4 => &ACCEPT4,
            NR::sys_access => &ACCESS,
            NR::sys_acct => &ACCT,
            NR::sys_add_key => &ADD_KEY,
            NR::sys_adjtimex => &ADJTIMEX,
            NR::sys_afs_syscall => &UNDEFPRINT,
            NR::sys_alarm => &ALARM,
            NR::sys_arch_prctl => &ARCH_PRCTL,
            NR::sys_arch_specific_syscall => &UNDEFPRINT,
            NR::sys_arm_fadvise64_64 => &UNDEFPRINT,
            NR::sys_bdflush => &BDFLUSH,
            NR::sys_bind | NR::sys_connect => &BIND,
            NR::sys_bpf => &BPF,
            NR::sys_brk => &BRK,
            NR::sys_capget => &CAPGET,
            NR::sys_capset => &CAPSET,
            NR::sys_chdir => &CHDIR,
            NR::sys_chmod => &CHMOD,
            NR::sys_chown | NR::sys_lchown => &CHOWN,
            NR::sys_chroot => &CHROOT,
            NR::sys_clock_adjtime => &CLOCK_ADJTIME,
            NR::sys_clock_getres | NR::sys_clock_getres_time64 | NR::sys_clock_gettime | NR::sys_clock_gettime64  => &CLOCK_GETTIME,
            NR::sys_clock_nanosleep => &CLOCK_NANOSLEEP,
            NR::sys_clock_settime | NR::sys_clock_settime64 => &CLOCK_SETTIME,
            NR::sys_clone => &CLONE,
            NR::sys_clone3 => &CLONE3,
            NR::sys_close => &SYS_ALIAS_INTDEC_INTDEC,
            NR::sys_close_range => &CLOSE_RANGE,
            NR::sys_copy_file_range => &COPY_FILE_RANGE,
            NR::sys_creat => &CREAT,
            NR::sys_create_module => &CREATE_MODULE,
            NR::sys_delete_module => &DELETE_MODULE,
            NR::sys_dup => &SYS_ALIAS_INTDEC_INTDEC,
            NR::sys_dup2 => &SYS_ALIAS_INTDEC_INTDEC_INTDEC,
            NR::sys_dup3 => &DUP3,
            NR::sys_epoll_create | NR::sys_epoll_create1 => &SYS_ALIAS_INTDEC_INTDEC,
            NR::sys_epoll_ctl => &EPOLL_CTL,
            NR::sys_epoll_pwait => &EPOLL_PWAIT,
            NR::sys_epoll_wait => &EPOLL_WAIT,
            NR::sys_eventfd => &SYS_ALIAS_INTDEC_INTDEC,
            NR::sys_eventfd2 => &SYS_ALIAS_INTDEC_INTDEC_INTDEC,
            NR::sys_execve => &EXECVE,
            NR::sys_execveat => &EXECVEAT,
            NR::sys_exit | NR::sys_exit_group => &EXIT,
            NR::sys_faccessat => &FACCESSAT,
            NR::sys_faccessat2 => &FACCESSAT2,
            NR::sys_fadvise64 => &FADVISE64,
            NR::sys_fadvise64_64 => &FADVISE64_64,
            NR::sys_fallocate => &FALLOCATE,
            NR::sys_fchdir => &FCHDIR,
            NR::sys_fchmod => &FCHMOD,
            NR::sys_fchmodat => &FCHMODAT,
            NR::sys_fchown => &FCHOWN,
            NR::sys_fchownat => &FCHOWNAT,
            NR::sys_fcntl => &FCNTL,
            NR::sys_fcntl64 => &FCNTL64,
            NR::sys_fdatasync => &SYS_ALIAS_INTDEC_INTDEC,
            NR::sys_fgetxattr => &FGETXATTR,
            NR::sys_finit_module => &FINIT_MODULE,
            NR::sys_fstatfs => &FSTATFS,
            NR::sys_fstatfs64 => &FSTATFS64,
            NR::sys_futex => &FUTEX,
            NR::sys_getdents64 => &GETDENTS64,
            NR::sys_getegid | NR::sys_getegid32 | NR::sys_geteuid | NR::sys_geteuid32 | NR::sys_getgid | NR::sys_getgid32 | NR::sys_getpgrp
                | NR::sys_getpid | NR::sys_getppid | NR::sys_gettid | NR::sys_getuid | NR::sys_getuid32 => &GETPID,
            NR::sys_getpgid | NR::sys_getsid | NR::sys_setpgid => &GETPGID,
            NR::sys_getrandom => &GETRANDOM,
            NR::sys_gettimeofday => &GETTIMEOFDAY,
            NR::sys_getxattr | NR::sys_lgetxattr=> &GETXATTR,
            NR::sys_init_module => &INIT_MODULE,
            NR::sys_ioctl => &IOCTL,
            NR::sys_listen => &SYS_ALIAS_INTDEC_INTDEC_INTDEC,
            NR::sys_lseek => &LSEEK,
            NR::sys_madvise => &MADIVISE,
            NR::sys_mkdir => &MKDIR,
            NR::sys_mkdirat => &MKDIRAT,
            NR::sys_mmap | NR::sys_mmap2 => &MMAP,
            NR::sys_mprotect => &MPROTECT,
            NR::sys_munmap => &MUNMAP,
            NR::sys_nanosleep => &NANOSLEEP,
            NR::sys_newfstatat => &NEWFSTATAT,
            NR::sys_oldolduname => &OLDOLDUNAME,
            NR::sys_olduname => &OLDUNAME,
            NR::sys_open => &OPEN,
            NR::sys_openat => &OPENAT,
            NR::sys_openat2 => &OPENAT2,
            NR::sys_pipe => &PIPE,
            NR::sys_pipe2 => &PIPE2,
            NR::sys_pread64 => &PREAD64,
            NR::sys_preadv => &PREADV,
            NR::sys_preadv2 => &PREADV2,
            NR::sys_prlimit64 => &PRLIMIT64,
            NR::sys_pselect6 => &PSELECT,
            NR::sys_pwrite64 => &PWRITE64,
            NR::sys_pwritev => &PWRITEV,
            NR::sys_pwritev2 => &PWRITEV2,
            NR::sys_read => &READ,
            NR::sys_readlink => &READLINK,
            NR::sys_readlinkat => &READLINKAT,
            NR::sys_readv => &READV,
            NR::sys_recvmsg => &RECVMSG,
            NR::sys_rseq => &RSEQ,
            NR::sys_rt_sigaction => &RT_SIGACTION,
            NR::sys_rt_sigreturn => &RT_SIGRETURN,
            NR::sys_rt_sigprocmask => &RT_SIGPROCMASK,
            NR::sys_rename => &RENAME,
            NR::sys_renameat => &RENAMEAT,
            NR::sys_renameat2 => &RENAMEAT2,
            NR::sys_sendmsg => &SENDMSG,
            NR::sys_set_robust_list => &SET_ROBUST_LIST,
            NR::sys_set_thread_area => &SET_THREAD_AREA,
            NR::sys_set_tid_address => &SET_TID_ADDRESS,
            NR::sys_settimeofday => &SETTIMEOFDAY,
            NR::sys_sigaltstack => &SIGALTSTACK,
            NR::sys_socket => &SOCKET,
            NR::sys_socketcall => &SOCKETCALL,
            NR::sys_statfs64 => &STATFS64,
            NR::sys_statfs => &STATFS,
            NR::sys_statx => &STATX,
            NR::sys_sysinfo => &SYSINFO,
            NR::sys_ugetrlimit => &UGETRLIMIT,
            NR::sys_umask => &SYS_ALIAS_INTDEC_INTDEC_INTDEC,
            NR::sys_uname => &UNAME,
            NR::sys_wait4 => &WAIT4,
            NR::sys_write => &WRITE,
            NR::sys_writev => &WRITEV,
            _ => &UNDEFPRINT,
        }
    }
    fn get_print_info_for_ret_args(&self) -> &'static [SyscallPrintInfoSet] {
        match self {
            NR::sys_accept | NR::sys_accept4 => &RET_ACCEPT,
            NR::sys_adjtimex => &RET_ADJTIMEX,
            NR::sys_clock_adjtime => &RET_CLOCK_ADJTIME,
            NR::sys_clock_gettime | NR::sys_clock_gettime64 | NR::sys_clock_getres | NR::sys_clock_getres_time64 => &RET_CLOCK_GETTIME,
            NR::sys_clock_nanosleep => &RET_CLOCK_NANOSLEEP,
            NR::sys_copy_file_range => &RET_COPY_FILE_RANGE,
            NR::sys_epoll_wait | NR::sys_epoll_pwait => &RET_EPOLL_WAIT,
            NR::sys_fgetxattr | NR::sys_getxattr | NR::sys_lgetxattr=> &RET_FGETXATTR,
            NR::sys_getdents64 => &RET_GETDENTS64,
            NR::sys_gettimeofday => &RET_GETTIMEOFDAY,
            NR::sys_ioctl => &RET_IOCTL,
            NR::sys_nanosleep => &RET_NANOSLEEP,
            NR::sys_newfstatat => &RET_NEWFSTATAT,
            NR::sys_olduname => &RET_OLDUNAME,
            NR::sys_oldolduname => &RET_OLDOLDUNAME,
            NR::sys_pipe | NR::sys_pipe2 => &RET_PIPE,
            NR::sys_prlimit64 => &RET_PRLIMIT64,
            NR::sys_pread64 | NR::sys_read => &RET_READ,
            NR::sys_readlink => &RET_READLINK,
            NR::sys_readlinkat => &RET_READLINKAT,
            NR::sys_readv | NR::sys_preadv | NR::sys_preadv2 => &RET_READV,
            NR::sys_recvmsg => &RET_RECVMSG,
            NR::sys_socketcall => &RET_SOCKETCALL,
            NR::sys_statfs64 | NR::sys_fstatfs64 => &RET_STATFS64,
            NR::sys_statfs | NR::sys_fstatfs => &RET_STATFS,
            NR::sys_statx => &RET_STATX,
            NR::sys_sysinfo => &RET_SYSINFO,
            NR::sys_ugetrlimit => &RET_UGETRLIMIT,
            NR::sys_uname => &RET_UNAME,
            NR::sys_wait4 => &RET_WAIT4,
            _ => &SKIPPRINT
        }
    }
}

