# nrtrace
Simple systemcall tracer implemented in rust.

It is like the strace implemented in rust, but the output format is different from strace.

Supported architectures are x86, x86_64, arm and aarch64. But, for some systemcalls, argument parsing is not implemented and arguments are only displayed in hexadecimal.

The original is the following C implementation.Therefore, the constraints on the ptrace factor are similar.

https://github.com/drab-l/c_nrtrace

Usage:

    nrtrace [Option] [CMD [ARGs...]]

Option:

    -p <tid,...>     : Trace target thread ids, separated comma. Don't trace other thread in same process.
    -e <syscall,...> : Print syscall names, separated comma. Default is all print.
    --ee <name,...>  : Print syscall inclusive names, separated comma. Default is all print.
    -E <syscall,...> : No print syscall names, separated comma.
    --EE <name,...>  : No print syscall inclusive names, separated comma.
    -s <syscall,...> : Simple print syscalll names, separated comma.
    --ss <name,...>  : Simple print syscall inclusive names, separated comma.
    -S <syscall,...> : change print format to nopeek tracee memory for spefified name's syscalls, separated comma.
    --SS <name,...>  : change print format to nopeek tracee memory for inclusive named syscalls, separated comma.
