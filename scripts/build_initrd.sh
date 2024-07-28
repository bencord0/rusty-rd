#!/bin/bash
set -ex
cargo build --release

make gen_init_cpio
./gen_init_cpio cpio_list > initrd.cpio
