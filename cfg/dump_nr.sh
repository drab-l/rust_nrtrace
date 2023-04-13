#!/bin/sh

PP () {
    clang --target=${TARGET} --include=sys/syscall.h -E  - "$@"
}

dump () {
    echo|PP -dM|grep _NR_|cut -d" " -f2|grep _NR_|xargs -I@ echo DDDD@:@|PP|grep "^DDDD"|sed 's@^DDDD@@;t t1;b;:t1;s@^__NR_@@;t;s@_*NR_*@_@;'|sort
}

arm_treat () {
    grep -v -eSYSCALL_BASE -eSYSCALL_MASK -earm_sync_file_range -eARM_BASE
}

cd `dirname $0`
ARCH=aarch64 ; TARGET=aarch64-linux-gnu ; dump > aarch64/NR
ARCH=arm ; TARGET=arm-linux-gnueabi ; dump|arm_treat > arm/NR
ARCH=x86_64 ; TARGET=x86_64-linux-gnu ; dump > x86_64/NR
ARCH=x86 ; TARGET=i386-linux-gnu ; dump > x86/NR
echo "unknown" > syscall_list
cat */NR|cut -d":" -f1|sort|uniq >> syscall_list
