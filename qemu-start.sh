#!/bin/bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-solas_os/debug/bootimage-solas-os.bin
