#!/bin/bash

APPS=(hello1 hello2)
BASE=(0x80400000 0x80420000)
BIN_DIR=target/riscv64gc-unknown-none-elf/release/


for i in "${!APPS[@]}"; do
    cargo rustc --bin "${APPS[$i]}" --release -- -Clink-args=-Ttext="${BASE[$i]}"
    rust-objcopy --binary-architecture=riscv64 "$BIN_DIR"/"${APPS[$i]}" --strip-all -O binary "$BIN_DIR"/"${APPS[$i]}".bin
    rm bin/"${APPS[$i]}".bin
    cp "$BIN_DIR"/"${APPS[$i]}".bin bin/"${APPS[$i]}".bin
done
