mod c {
    extern "C" {
        pub fn waitpid(pid: types::Pid, status: *mut types::SInt, options: types::SInt) -> types::Pid;
    }
    pub const __WALL: types::SInt = 0x40000000;
    pub const WSTOPPED: types::SInt = 0x00000002;
}

pub fn wait_one(pid: types::Pid) -> std::io::Result<types::SInt> {
    let mut status: types::SInt = 0;
    match unsafe { c::waitpid(pid, &mut status, c::__WALL) } {
        -1 => Err(std::io::Error::last_os_error()),
        0 => Ok(0),
        _ => Ok(status),
    }
}

pub fn wait_one_stop(pid: types::Pid) -> std::io::Result<types::SInt> {
    let mut status: types::SInt = 0;
    match unsafe { c::waitpid(pid, &mut status, c::WSTOPPED) } {
        -1 => Err(std::io::Error::last_os_error()),
        0 => Ok(0),
        _ => Ok(status),
    }
}

pub fn wait_any() -> std::io::Result<(types::Pid, types::SInt)> {
    let mut status: types::SInt = 0;
    match unsafe { c::waitpid(-1 as types::Pid, &mut status, c::__WALL) } {
        -1 => Err(std::io::Error::last_os_error()),
        0 => Ok((0, 0)),
        r => Ok((r, status)),
    }
}

