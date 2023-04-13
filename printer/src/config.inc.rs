macro_rules! define_print_info_hex_type {
    ($name:ident, $type:ident) => {
        const $name: TYPES = TYPES::$type(FORMATS::HEX);
    };
}

macro_rules! define_print_info_dec_type {
    ($name:ident, $type:ident) => {
        const $name: TYPES = TYPES::$type(FORMATS::DEC);
    };
}

macro_rules! define_print_info_oct_type {
    ($name:ident, $type:ident) => {
        const $name: TYPES = TYPES::$type(FORMATS::OCT);
    };
}

macro_rules! define_print_info_all_fmt_type {
    ($hexname:ident, $decname:ident, $octname:ident, $type:ident) => {
        define_print_info_hex_type!($hexname, $type);
        define_print_info_dec_type!($decname, $type);
        define_print_info_oct_type!($octname, $type);
    };
}

macro_rules! define_syscall_print_info {
    ($name:ident, $ret:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [NONE; 6] };
    };
    ($name:ident, $ret:ident, $arg1:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, NONE, NONE, NONE, NONE, NONE] };
    };
    ($name:ident, $ret:ident, $arg1:ident, $arg2:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, NONE, NONE, NONE, NONE] };
    };
    ($name:ident, $ret:ident, $arg1:ident, $arg2:ident, $arg3:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, NONE, NONE, NONE] };
    };
    ($name:ident, $ret:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, NONE, NONE] };
    };
    ($name:ident, $ret:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident, $arg5:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, $arg5, NONE] };
    };
    ($name:ident, $ret:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident, $arg5:ident, $arg6:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, $arg5, $arg6] };
    };
}

macro_rules! define_syscall_print_info_for_ret_args {
    ($name:ident, $arg1:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, NONE, NONE, NONE, NONE, NONE] };
    };
    ($name:ident, $arg1:ident, $arg2:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, NONE, NONE, NONE, NONE] };
    };
    ($name:ident, $arg1:ident, $arg2:ident, $arg3:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, NONE, NONE, NONE] };
    };
    ($name:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, NONE, NONE] };
    };
    ($name:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident, $arg5:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, $arg5, NONE] };
    };
    ($name:ident, $arg1:ident, $arg2:ident, $arg3:ident, $arg4:ident, $arg5:ident, $arg6:ident) => {
        const $name: SyscallPrintInfoSet = SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, $arg5, $arg6] };
    };
}
