use std::env::Args;
use std::sync::atomic::{AtomicBool, Ordering};

mod tracer;
use tracer::Tracer;

#[allow(unused_macros)]
macro_rules! LINE { () => { println!("{}", line!()) } }

fn print_usage(bin: &str) -> ! {
    println!(
        r#"trace and print tracee syscall.

Usage:
    {} [Option] [CMD ARGs...]
Option:
    -p: tracee process ids, separated comma.
    -e: print only specified name's syscalls, separated comma. default all print.
    --ee: print only inclusive named syscalls, separated comma. default all print.
    -E: not print spefified name's syscalls, separated comma.
    --EE: not print inclusive named syscalls, separated comma.
    -s: change print format to simple for spefified name's syscalls, separated comma.
    --ss: change print format to simple for inclusive named syscalls, separated comma.
"#,
        bin
    );
    std::process::exit(1);
}

fn parse_opt_cb<T>(tracer: &mut Tracer, value: &str, args: &mut Args, expect: &str, cb: T) -> bool
where
    T: Fn(&mut Tracer, &str),
{
    if !value.starts_with(expect) {
        false
    } else if value.len() == expect.len() && args.len() == 0 {
        false
    } else {
        if value.len() > expect.len() {
            cb(tracer, &value[expect.len()..]);
        } else {
            cb(tracer, &args.next().unwrap());
        }
        true
    }
}

fn parse_opt_comma_separated_cb<T>(tracer: &mut Tracer, value: &str, args: &mut Args, expect: &str, cb: T) -> bool
where
    T: Fn(&mut Tracer, &str),
{
    parse_opt_cb(tracer, value, args, expect, |t: &mut Tracer, v: &str| {
        v.split(',').for_each(|x| {
            cb(t, x);
        });
    })
}

fn set_print_skip_for_default_once(tracer: &mut Tracer) {
    static CALLED: AtomicBool = AtomicBool::new(false);
    if !CALLED.load(Ordering::Acquire) {
        tracer.set_skip_for_default();
        CALLED.store(true, Ordering::Relaxed);
    }
}

fn set_print_not_skip_named_syscall(tracer: &mut Tracer, value: &str) {
    set_print_skip_for_default_once(tracer);
    tracer.set_print_not_skip_by_name(value);
}

fn set_print_not_skip_included_name_syscall(tracer: &mut Tracer, value: &str) {
    set_print_skip_for_default_once(tracer);
    tracer.set_print_not_skip_by_include_name(value);
}

fn set_print_skip_named_syscall(tracer: &mut Tracer, value: &str) {
    tracer.set_print_skip_by_name(value);
}

fn set_print_skip_included_name_syscall(tracer: &mut Tracer, value: &str) {
    tracer.set_print_skip_by_include_name(value);
}

fn set_print_simple_named_syscall(tracer: &mut Tracer, value: &str) {
    tracer.set_print_simple_by_name(value);
}

fn set_print_simple_included_name_syscall(tracer: &mut Tracer, value: &str) {
    tracer.set_print_simple_by_include_name(value);
}

fn collect_pid_for_attach(tracer: &mut Tracer, value: &str) {
    tracer.attach_running_process(value.parse::<types::Pid>().unwrap()).unwrap_or_else(|_|{});
}

fn set_output(tracer: &mut Tracer, value: &str) {
    tracer.set_output(value);
}

fn parse_opt(tracer: &mut Tracer) {
    let mut args = std::env::args();
    let bin = args.next().unwrap();
    while args.len() > 0 {
        let head = args.next().unwrap();
        if head == "-h" {
            print_usage(&bin);
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "-p", collect_pid_for_attach) {
            continue;
        } else if parse_opt_cb(tracer, &head, &mut args, "-o", set_output) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "-e", set_print_not_skip_named_syscall) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "--ee", set_print_not_skip_included_name_syscall) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "-E", set_print_skip_named_syscall) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "--EE", set_print_skip_included_name_syscall) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "-s", set_print_simple_named_syscall) {
            continue;
        } else if parse_opt_comma_separated_cb(tracer, &head, &mut args, "--ss", set_print_simple_included_name_syscall) {
            continue;
        }
        tracer.attach_exec_child(head, args).unwrap();
        break;
    }
}

pub fn start() {
    let mut tracer = Tracer::new();
    parse_opt(&mut tracer);
    tracer.start().unwrap();
}

