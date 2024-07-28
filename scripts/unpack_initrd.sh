#!/bin/bash
set -ex

mkdir -pv unpacked
cd unpacked
lsinitrd --unpack ../initrd.cpio
