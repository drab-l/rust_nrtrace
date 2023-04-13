#!/bin/sh

INC="${1}"

PP () {
    clang --target=${TARGET} -E --include="${INC}" -
}

cd `dirname $0`
ARCH=aarch64 ; TARGET=aarch64-linux-gnu ; echo|PP > /tmp/tmp.head.aarch64.c
ARCH=arm ; TARGET=arm-linux-gnueabi ; echo|PP > /tmp/tmp.head.arm.c
ARCH=x86_64 ; TARGET=x86_64-linux-gnu ; echo|PP > /tmp/tmp.head.x86_64.c
ARCH=x86 ; TARGET=i386-linux-gnu ; echo|PP > /tmp/tmp.head.x86.c
