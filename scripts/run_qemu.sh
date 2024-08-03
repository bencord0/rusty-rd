#!/bin/bash
KVER="$(uname -r)"

set -ex
qemu-system-x86_64 \
    -append quiet \
    -kernel "/boot/gentoo/${KVER}/linux" \
    -initrd ./initrd.cpio
