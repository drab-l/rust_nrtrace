use std::io::Result;

mod history;

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

mod c {
    use types::SigHandler;

    extern "C" {
        pub fn signal(signum: types::SInt, sighander: types::SigHandler) -> SigHandler;
        pub fn _exit(status: types::SInt);
    }
    pub const SIGINT: types::SInt = 2;
}

fn signal(signum: types::SInt, sighandler: types::SigHandler) {
    unsafe { c::signal(signum, sighandler); }
}

extern fn sighandle_exit (_: types::SInt) {
    unsafe { c::_exit(1); }
}

fn event_loop(printer: printer::Printer) -> Result<()> {
    let mut history = history::HistMap::new();
    let log = printer;
    loop {
        match peek::wait_event() {
            Ok((pid, peek::ChildEventKind::ForkStop)) => {
                peek::treat_stopped_clone_process(pid)?;
            },
            Ok((pid, peek::ChildEventKind::ExitDone)) => {
                history.clear(pid);
            },
            Ok((pid, peek::ChildEventKind::SigExited)) => {
                history.clear(pid);
            },
            Ok((pid, peek::ChildEventKind::SyscallStop)) => {
                if let Ok(e) = peek::peek_syscall_info(pid) {
                    if let Some(e) = history.update(pid, e) {
                        log.output_and_cont(pid, e).unwrap();
                    }
                }
            },
            _ => { break; },
        }
    }
    Ok(())
}

pub struct Tracer {
    out_path: Option<String>,
    printer: printer::Printer,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer{ out_path:None, printer:printer::Printer::new() }
    }
    pub fn set_output(&mut self, path: &str) {
        self.out_path = Some(path.to_owned());
    }

    pub fn start(mut self) -> Result<()> {
        signal(c::SIGINT, sighandle_exit);
        if let Some(out) = self.out_path {
            self.printer.file(out);
        }
        event_loop(self.printer)
    }

    pub fn set_skip_for_default(&mut self) {
        self.printer.set_skip_for_default()
    }

    pub fn attach_running_process(&self, pid: types::Pid) -> Result<()> {
        peek::peek_attach_running_process(pid)?;
        Ok(())
    }

    pub fn set_print_skip_by_name(&mut self, name: &str) {
        self.printer.set_skip_by_name(name)
    }

    pub fn set_print_not_skip_by_name(&mut self, name: &str) {
        self.printer.set_not_skip_by_name(name)
    }

    pub fn set_print_simple_by_name(&mut self, name: &str) {
        self.printer.set_simple_by_name(name)
    }

    pub fn set_print_nopeek_by_name(&mut self, name: &str) {
        self.printer.set_nopeek_by_name(name)
    }

    pub fn set_print_skip_by_include_name(&mut self, name: &str) {
        self.printer.set_skip_by_include_name(name)
    }

    pub fn set_print_not_skip_by_include_name(&mut self, name: &str) {
        self.printer.set_not_skip_by_include_name(name)
    }

    pub fn set_print_simple_by_include_name(&mut self, name: &str) {
        self.printer.set_simple_by_include_name(name)
    }

    pub fn set_print_nopeek_by_include_name(&mut self, name: &str) {
        self.printer.set_nopeek_by_include_name(name)
    }

    pub fn attach_exec_child<T>(&self, cmd: String, args: T) -> Result<types::Pid>
    where
        T: Iterator<Item = String>
    {
        let pid = peek::peek_attach_exec_child(cmd, args)?;
        Ok(pid)
    }

}

