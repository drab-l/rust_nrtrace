simple systemcall tracer implemented in rust

Usage:

    nrtrace [Option] [CMD [ARGs...]]

Option:

    -p  : Trace target thread ids, separated comma. Don't trace other thread in same process.
    -e  : Print syscall names, separated comma. Default is all print.
    --ee: Print syscall inclusive names, separated comma. Default is all print.
    -E  : No print syscall names, separated comma.
    --EE: No print syscall inclusive names, separated comma.
    -s  : Simple print syscalll names, separated comma.
    --ss: Simple print syscall inclusive names, separated comma.
    -S  : change print format to nopeek tracee memory for spefified name's syscalls, separated comma.
    --SS: change print format to nopeek tracee memory for inclusive named syscalls, separated comma.
