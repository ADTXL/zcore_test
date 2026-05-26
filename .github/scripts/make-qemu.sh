#!/usr/bin/env bash

wget https://download.qemu.org/qemu-$1.tar.xz
tar -xJf qemu-$1.tar.xz
cd qemu-$1
./configure --target-list=aarch64-softmmu
make -j > /dev/null 2>&1
