#!/bin/bash
KVER="$(uname -r)"

set -ex
cargo build --release

make gen_init_cpio

export KVER
./gen_init_cpio cpio_list > initrd.cpio
