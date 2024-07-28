#!/bin/bash

set -ex
qemu-system-x86_64 \
    -append quiet \
    -kernel /boot/gentoo/$(uname -r)/linux \
    -initrd ./initrd.cpio
