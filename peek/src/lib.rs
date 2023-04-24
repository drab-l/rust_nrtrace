//! Wrapper for ptrace C function
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use std::mem::MaybeUninit;

#[allow(unused_macros)]
macro_rules! LINE { () => { eprintln!("{}", line!()) } }


mod c {
    extern "C" {
        pub fn ptrace(request: types::SInt, pid: types::Pid, addr: *mut types::Void, data: *mut types::Void) -> types::SLong;
        pub fn getpid() -> types::Pid;
        pub fn fork() -> types::Pid;
        pub fn kill(pid: types::Pid, sig: types::SInt) -> types::SInt;
        pub fn process_vm_readv(pid: types::Pid, dst: *const iovec, dstcnt: types::ULong,
                                src: *const iovec, srccnt: types::ULong, flags: types::UInt) -> types::SSizeT;
    }

    #[repr(C)]#[allow(non_camel_case_types)]
    pub struct iovec {
        pub iov_base: *mut types::Void,
        pub iov_len: types::USizeT,
    }

    pub const ECHILD: types::SInt = 10;
    pub const SIGTRAP: types::SInt = 5;
    pub const SIGCONT: types::SInt = 18;
    pub const SIGSTOP: types::SInt = 19;
    pub const SIGTSTP: types::SInt = 20;
    pub const SIGTTIN: types::SInt = 21;
    pub const SIGTTOU: types::SInt = 22;
    pub const __AUDIT_ARCH_64BIT: u32 = 0x80000000;
include!("peek_const.inc");
}

#[repr(C)]#[derive(Clone, Copy)]#[allow(non_camel_case_types)]
struct ptrace_syscall_info_entry {
    nr: u64,
    args: [u64; 6],
}

#[repr(C)]#[derive(Clone, Copy)]#[allow(non_camel_case_types)]
struct ptrace_syscall_info_exit {
    rval: i64,
    is_error: u8,
}

#[repr(C)]#[derive(Clone, Copy)]#[allow(non_camel_case_types)]
struct ptrace_syscall_info_seccomp {
    nr: u64,
    args: [u64; 6],
    ret_data: u32,
}

#[repr(C)]
union ptrace_syscall_info_union {
    entry: ptrace_syscall_info_entry,
    exit: ptrace_syscall_info_exit,
    seccomp: ptrace_syscall_info_seccomp,
}

#[repr(C)]#[allow(non_camel_case_types)]
struct ptrace_syscall_info {
    op: u8,
    arch: u32,
    instruction_pointer: u64,
    stack_pointer: u64,
    u: ptrace_syscall_info_union,
}

impl ptrace_syscall_info {
    fn is64(&self) -> bool {
        (self.arch & c::__AUDIT_ARCH_64BIT) == c::__AUDIT_ARCH_64BIT
    }

    unsafe fn to_rust_entry(&self) -> SyscallInfoEntry {
        let args = SyscallArg{ nr: self.u.entry.nr, args: self.u.entry.args };
        let is64 = self.is64();
        SyscallInfoEntry{ args, is64 }
    }

    unsafe fn to_rust_exit(&self) -> SyscallInfoExit {
        let ret = if self.u.exit.is_error == 0 { SyscallRet::OK(self.u.exit.rval) } else { SyscallRet::ERR(self.u.exit.rval as i32) };
        SyscallInfoExit{ ret }
    }
}

fn ptrace(request: types::SInt, pid: types::Pid, addr: *mut types::Void, data: *mut types::Void) -> Result<()> {
    match unsafe { c::ptrace(request, pid, addr, data) } {
        -1 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}

fn getpid() -> types::Pid {
    unsafe { c::getpid() }
}

#[allow(unused_macros)]
macro_rules! const_ptr {
    ($v:expr, $t:ty) => {
        ($v as *const $t)
    };
}

#[allow(unused_macros)]
macro_rules! any_ptr {
    ($v:expr, $t:ty) => {
        ($v as *mut $t)
    };
}

#[allow(unused_macros)]
macro_rules! void_ptr {
    ($v:expr) => {
        any_ptr!($v, types::Void)
    };
}

#[allow(unused_macros)]
macro_rules! NULL {
    () => {
        void_ptr!(0)
    };
}

#[allow(unused_macros)]
macro_rules! null_ptr {
    ($t:ty) => {
        any_ptr!(0, $t)
    };
}

fn wait_one_stop_no_status(pid: types::Pid) -> Result<()> {
    waiter::wait_one_stop(pid)?;
    Ok(())
}

fn ptrace2(request: types::SInt, pid: types::Pid) -> Result<()> {
    ptrace(request, pid, NULL!(), NULL!())
}

fn ptrace_syscall(pid: types::Pid) -> Result<()> {
    ptrace2(c::PTRACE_SYSCALL, pid)
}

fn ptrace_syscall_sig(pid: types::Pid, sig: types::SInt) -> Result<()> {
    ptrace(c::PTRACE_SYSCALL, pid, NULL!(), void_ptr!(sig))
}

fn ptrace_listen(pid: types::Pid) -> Result<()> {
    ptrace2(c::PTRACE_LISTEN, pid)
}

fn ptrace_interrupt(pid: types::Pid) -> Result<()> {
    ptrace2(c::PTRACE_INTERRUPT, pid)
}

fn ptrace_geteventmsg_get_child_pid(parent: types::Pid) -> Result<types::Pid> {
    let mut pid = MaybeUninit::<types::ULong>::uninit();
    ptrace(c::PTRACE_GETEVENTMSG, parent, NULL!(), void_ptr!(pid.as_mut_ptr()))?;
    let pid = unsafe { pid.assume_init() };
    Ok(pid as types::Pid)
}

fn ptrace_attach(pid: types::Pid) -> Result<()> {
    const OPT: i32 = c::PTRACE_O_TRACEEXEC | c::PTRACE_O_TRACEFORK | c::PTRACE_O_TRACEVFORK | c::PTRACE_O_TRACECLONE | c::PTRACE_O_TRACESYSGOOD;
    ptrace(c::PTRACE_SEIZE, pid, NULL!(), void_ptr!(OPT))
}

fn ptrace_get_syscall_info(pid: types::Pid) -> Result<ptrace_syscall_info> {
    type T = ptrace_syscall_info;
    let mut r = MaybeUninit::<T>::uninit();
    ptrace(c::PTRACE_GET_SYSCALL_INFO, pid, void_ptr!(std::mem::size_of::<T>()), void_ptr!(r.as_mut_ptr()))?;
    Ok(unsafe { r.assume_init() })
}

unsafe fn peek_readv(pid: types::Pid, addr: types::Ptr, dst: *mut u8, size: usize) -> Result<usize> {
    let local = c::iovec{iov_base: dst.cast::<types::Void>(), iov_len: size};
    let remote = c::iovec{iov_base: addr as *mut types::Void, iov_len: size};
    match c::process_vm_readv(pid, &local, 1, &remote, 1, 0) {
        res if res < 0 => Err(Error::last_os_error()),
        res => Ok(res as usize),
    }
}

unsafe fn peek_buf(pid: types::Pid, addr: types::Ptr, dst: *mut u8, size: usize) -> Result<usize> {
    Ok(peek_readv(pid, addr, dst, size).unwrap())
}

fn sigstop_self() -> std::io::Result<()> {
    match unsafe { c::kill(getpid(), c::SIGSTOP) } {
        -1 => Err(Error::last_os_error()),
        _ => Ok(())
    }
}

fn sigcont_process(pid: types::Pid) -> std::io::Result<()> {
    match unsafe { c::kill(pid, c::SIGCONT) } {
        -1 => Err(Error::last_os_error()),
        _ => Ok(())
    }
}

fn is_stopped_status(status: types::SInt) -> bool {
    (status & 0x7f) == 0x7f
}

fn is_exited_status(status: types::SInt) -> bool {
    (status & 0x7f) == 0
}

fn is_sigexited_status(status: types::SInt) -> bool {
    ((status + 1) & 0x7f) >= 2
}

fn exit_status(status: types::SInt) -> types::SInt {
    (status & 0xff00) >> 8
}

fn signal_status(status: types::SInt) -> types::SInt {
    (status & 0xff00) >> 8
}

fn is_fork_stopped_status(status: types::SInt) -> bool {
    const FORKED: types::SInt = c::SIGTRAP | (c::PTRACE_EVENT_FORK << 8);
    const VFORKED: types::SInt = c::SIGTRAP | (c::PTRACE_EVENT_VFORK << 8);
    const CLONED: types::SInt = c::SIGTRAP | (c::PTRACE_EVENT_CLONE << 8);
    let sig = status >> 8;
    is_stopped_status(status) && (sig == FORKED || sig == VFORKED || sig == CLONED)
}

fn is_syscall_stopped_status(status: types::SInt) -> bool {
    const SYSCALLED: types::SInt = c::SIGTRAP | 0x80;
    is_stopped_status(status) && (exit_status(status) == SYSCALLED)
}

fn is_event_stop(status: types::SInt) -> bool {
    (status >> 16) == c::PTRACE_EVENT_STOP
}

fn is_stop_signal(sig: types::SInt) -> bool {
    sig == c::SIGSTOP || sig == c::SIGTSTP || sig == c::SIGTTIN || sig == c::SIGTTOU
}

fn is_ptrace_event_stop(status: types::SInt) -> bool {
    let sig = signal_status(status);
    is_event_stop(status) && !is_stop_signal(sig)
}

fn is_group_stop(status: types::SInt) -> bool {
    let sig = signal_status(status);
    is_event_stop(status) && is_stop_signal(sig)
}

fn is_exec_stop(status: types::SInt) -> bool {
    const EXECED: types::SInt = c::SIGTRAP | (c::PTRACE_EVENT_EXEC << 8);
    let sig = status >> 8;
    is_stopped_status(status) && (sig == EXECED)
}

/// Peek event types
pub enum ChildEventKind {
    /// Stop event for process fork/vfork/clone
    ForkStop,
    /// Event for process exit
    ExitDone,
    /// Stop event for process exit by signal
    SigExited,
    /// Stop event for syscall-enter or syscall-exit
    SyscallStop,
}

/// Wait for peek event
pub fn wait_event() -> Result<(types::Pid, ChildEventKind)> {
    loop {
        let r = waiter::wait_any();
        if r.is_err() {
            let r = Error::last_os_error();
            if r.raw_os_error().unwrap() == c::ECHILD {
                return Err(r);
            }
            continue;
        }
        let (pid, status) = r.unwrap();
        if is_fork_stopped_status(status) {
            return Ok((pid, ChildEventKind::ForkStop));
        } else if is_group_stop(status) {
            ptrace_listen(pid)?;
        } else if is_ptrace_event_stop(status) {
            ptrace_syscall(pid)?;
        } else if is_exec_stop(status) {
            ptrace_syscall(pid)?;
        } else if is_exited_status(status) {
            return Ok((pid, ChildEventKind::ExitDone));
        } else if is_sigexited_status(status) {
            return Ok((pid, ChildEventKind::SigExited));
        } else if is_syscall_stopped_status(status) {
            return Ok((pid, ChildEventKind::SyscallStop));
        } else if is_stopped_status(status) {
            ptrace_syscall_sig(pid, signal_status(status))?;
        }
    }
}

struct SyscallArg {
    nr: u64,
    args: [u64; 6],
}

enum SyscallRet {
    OK(i64),
    ERR(i32),
}

/// Syscall-enter information
pub struct SyscallInfoEntry {
    args: SyscallArg,
    is64: bool
}

/// Syscall-exit information
pub struct SyscallInfoExit {
    ret: SyscallRet,
}

/// Syscall-enter or Syscall-exit information
pub enum SyscallInfo {
    ENTRY(SyscallInfoEntry),
    EXIT(SyscallInfoExit),
}

/// Summery of syscall-enter and syscall-exit information
pub struct SyscallSummery {
    args: SyscallArg,
    ret: Option<SyscallRet>,
    uni: arch::sys_uni::NR,
    is64: bool,
}

/// Syscall argument's number specifier
pub enum Arg { ONE, TWO, THR, FUR, FIV, SIX }

impl SyscallSummery {
    /// Create from syscall-enter information
    /// # Arguments
    /// * `entry` - syscall-enter information
    pub fn new_from_entry(entry: SyscallInfoEntry) -> Self {
        let args = entry.args;
        let ret = None;
        let nr = args.nr;
        let is64 = entry.is64;
        let uni = if is64 { arch::sys_uni::a64::to_uni(nr) } else { arch::sys_uni::a32::to_uni(nr) };
        SyscallSummery{ args, ret, uni, is64 }
    }

    /// Override by syscall-enter information and forget syscall-exit summery
    /// # Arguments
    /// * `entry` - syscall-enter information
    pub fn renew_from_entry(&mut self, entry: SyscallInfoEntry) {
        self.args = entry.args;
        self.ret = None;
        self.is64 = entry.is64;
        self.uni = if self.is64 { arch::sys_uni::a64::to_uni(self.args.nr) } else { arch::sys_uni::a32::to_uni(self.args.nr) };
    }

    /// Create dummy summery, used for socketcall etc
    pub fn new_dummy_entry(is64: bool, uni: arch::sys_uni::NR, nr: u64, args: [u64; 6], ret:i64) -> Self {
        let args = SyscallArg{nr, args};
        let ret = Some(if ret >= 0 && ret < -4096 { SyscallRet::OK(ret) } else { SyscallRet::ERR(ret as i32) });
        SyscallSummery{ args, ret, uni, is64 }
    }

    /// Add summery from syscall-exit information
    /// # Arguments
    /// * `exit` - syscall-exit information
    pub fn add_exit(&mut self, exit: SyscallInfoExit) {
        self.ret = Some(exit.ret);
    }

    /// Get raw syscall number
    pub fn sysnum(&self) -> u64 {
        self.args.nr
    }

    pub fn is_entry(&self) -> bool {
        self.ret.is_none()
    }

    pub fn is_exit(&self) -> bool {
        self.ret.is_some()
    }

    /// Get syscall succeeded return value
    pub fn return_value(&self) -> Result<u64> {
        match self.ret {
            Some(SyscallRet::OK(r)) => Ok(r as u64),
            Some(SyscallRet::ERR(r)) => Err(Error::from_raw_os_error(r)),
            _ => Err(Error::from(ErrorKind::Other)),
        }
    }

    /// Get raw syscall name
    pub fn sysname(&self) -> &str {
        arch::sys_uni::to_str(self.uni)
    }

    /// Get common syscall name
    pub fn uni_sysnum(&self) -> arch::sys_uni::NR {
        self.uni
    }

    pub fn args(&self) -> &[u64; 6] {
        &self.args.args
    }

    pub fn argn(&self, n: Arg) -> u64 {
        match n {
            s => self.args.args[s as usize],
        }
    }

    pub fn is_64(&self) -> bool {
        self.is64
    }
}

/// Execute process and trace it
/// # Arguments
/// * `cmd` - Execute command name
/// * `args` - Execute command's arguments
pub fn peek_attach_exec_child<T>(cmd: String, args: T) -> Result<types::Pid>
where
    T: Iterator<Item = String>
{
    match unsafe { c::fork() } {
        -1 => Err(Error::last_os_error()),
        0 => {
            sigstop_self().unwrap();
            Command::new(cmd).args(args).exec();
            panic!();
        },
        pid => {
            wait_one_stop_no_status(pid).unwrap();
            ptrace_attach(pid).unwrap();
            sigcont_process(pid).unwrap();
            Ok(pid)
        },
    }
}

/// Start trace of the runnning process
/// # Arguments
/// * `pid` - A target process ID
pub fn peek_attach_running_process(pid: types::Pid) -> Result<()> {
    ptrace_attach(pid)?;
    ptrace_interrupt(pid)?;
    Ok(())
}

/// Get cloned/forked/vforked child process ID and restart parent process
/// # Arguments
/// * `pid` - Parent process ID
pub fn treat_stopped_clone_process(parent: types::Pid) -> Result<types::Pid>
{
    let pid = ptrace_geteventmsg_get_child_pid(parent);
    ptrace_syscall(parent)?;
    let pid = pid?;
    Ok(pid)
}

/// Restart syscall-stopped target process
/// # Arguments
/// * `pid` - A target process ID
pub fn cont_process(pid: types::Pid) -> Result<()> {
    ptrace_syscall(pid)
}

/// Peek syscall info from syscall-stopped target process
/// # Arguments
/// * `pid` - A peek target process ID
pub fn peek_syscall_info(pid: types::Pid) -> Result<SyscallInfo> {
    let r = ptrace_get_syscall_info(pid)?;
    unsafe {
        match r.op as types::SInt {
            c::PTRACE_SYSCALL_INFO_ENTRY => Ok(SyscallInfo::ENTRY(r.to_rust_entry())),
            c::PTRACE_SYSCALL_INFO_EXIT => Ok(SyscallInfo::EXIT(r.to_rust_exit())),
            _ => Err(Error::from(ErrorKind::Other)),
        }
    }
}

/// Peek specfied type's data from target process
/// # Arguments
/// * `pid` - A peek target process ID
/// * `addr` - A peek target address
pub fn peek_data<T>(pid: types::Pid, addr: types::Ptr) -> Result<T> {
    let size = std::mem::size_of::<T>();
    let mut buf = MaybeUninit::<T>::uninit();
    unsafe { peek_buf(pid, addr, buf.as_mut_ptr().cast::<u8>(), size)?; }
    Ok(unsafe { buf.assume_init() })
}

/// Peek null terminated string
/// # Arguments
/// * `pid` - A peek target process ID
/// * `addr` - A peek target address
pub fn peek_until_null(pid: types::Pid, addr: types::Ptr) -> Result<Vec<u8>> {
    let mut addr = addr;
    let mut res = vec![];
    loop {
        const PEEK_SIZE: usize = 32;
        let mut buf = Vec::<u8>::with_capacity(PEEK_SIZE);
        unsafe {
            let len = peek_buf(pid, addr, buf.as_mut_ptr(), buf.capacity())?;
            buf.set_len(len);
        }
        let error = buf.len() != buf.capacity();
        addr += buf.len();
        if let Some(len) = buf.iter().position(|x| *x == 0) {
            buf.truncate(len);
            res.append(&mut buf);
            break
        } else {
            res.append(&mut buf);
        }
        if error { break; }
    }
    Ok(res)
}

/// Peek data from target process to `Vec<u8>`
/// # Arguments
/// * `pid` - A peek target process ID
/// * `addr` - A peek target address
/// * `dst` - Destination vector that capacity shall be allocated
/// * `size` - Peek size as bytes. If larger than the dst capacity, the dst capacity size is used
/// insted of this
pub fn peek_vec(pid: types::Pid, addr: types::Ptr, dst:&mut Vec<u8>, size: usize) -> Result<()> {
    let size = std::cmp::min(size, dst.capacity());
    unsafe {
        let ptr = dst.as_mut_ptr();
        peek_buf(pid, addr, ptr, size)?;
        dst.set_len(size);
    }
    Ok(())
}

