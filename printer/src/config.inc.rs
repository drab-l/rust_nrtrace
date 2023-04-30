macro_rules! define_print_info_hex_type {
    ($name:ident, $type:ident) => {
        #[allow(dead_code)]
        const $name: TYPES = TYPES::$type(FORMATS::HEX);
    };
}

macro_rules! define_print_info_dec_type {
    ($name:ident, $type:ident) => {
        #[allow(dead_code)]
        const $name: TYPES = TYPES::$type(FORMATS::DEC);
    };
}

macro_rules! define_print_info_oct_type {
    ($name:ident, $type:ident) => {
        #[allow(dead_code)]
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
    ($name:ident, $ret:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [NONE; 6] }];
    };
    ($name:ident, $ret:expr, $arg1:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, NONE, NONE, NONE, NONE, NONE] }];
    };
    ($name:ident, $ret:expr, $arg1:expr, $arg2:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, NONE, NONE, NONE, NONE] }];
    };
    ($name:ident, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, NONE, NONE, NONE] }];
    };
    ($name:ident, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, NONE, NONE] }];
    };
    ($name:ident, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, $arg5, NONE] }];
    };
    ($name:ident, $ret:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:$ret, args: [$arg1, $arg2, $arg3, $arg4, $arg5, $arg6] }];
    };
}

macro_rules! define_syscall_print_info_bits {
    ($name:ident, $ret:expr,
        $arg1_a64: expr, $arg2_a64: expr, $arg3_a64: expr, $arg4_a64: expr,
        $arg1_a32: expr, $arg2_a32: expr, $arg3_a32: expr, $arg4_a32: expr, $arg5_a32: expr, $arg6_a32: expr
        ) => {
        const $name: [SyscallPrintInfoSet; 2] = [
            SyscallPrintInfoSet { ret:$ret, args: [$arg1_a64, $arg2_a64, $arg3_a64, $arg4_a64, NONE, NONE] },
            SyscallPrintInfoSet { ret:$ret, args: [$arg1_a32, $arg2_a32, $arg3_a32, $arg4_a32, $arg5_a32, $arg6_a32] },
        ];
    }
}

macro_rules! define_syscall_print_info_for_ret_args {
    ($name:ident, $arg1:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, NONE, NONE, NONE, NONE, NONE] }];
    };
    ($name:ident, $arg1:expr, $arg2:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, NONE, NONE, NONE, NONE] }];
    };
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, NONE, NONE, NONE] }];
    };
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, NONE, NONE] }];
    };
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, $arg5, NONE] }];
    };
    ($name:ident, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
        const $name: [SyscallPrintInfoSet; 1] = [SyscallPrintInfoSet { ret:NONE, args: [$arg1, $arg2, $arg3, $arg4, $arg5, $arg6] }];
    };
}
