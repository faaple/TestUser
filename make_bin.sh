#!/bin/bash

APP_NAME=user_lib
USER_BIN=target/riscv64gc-unknown-none-elf/release/"$APP_NAME"

cargo build --release
rust-objcopy --binary-architecture=riscv64 "$USER_BIN" --strip-all -O binary "$USER_BIN".bin
rm bin/"$APP_NAME".bin
cp "$USER_BIN".bin bin/"$APP_NAME".bin
