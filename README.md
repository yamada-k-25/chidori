# chidori

## Overview

chidori is an operating system that reimplements [OS in 1,000 lines](https://operating-system-in-1000-lines.vercel.app/ja/)  in Rust.

## Setup

### macOS

```console
brew install llvm lld qemu
```

```console
$ export PATH="$PATH:$(brew --prefix)/opt/llvm/bin"
$ which llvm-objcopy
/opt/homebrew/opt/llvm/bin/llvm-objcopy
```

## Build

```shell
cargo build --target riscv32i-unknown-none-elf
```

## Run

```shell
qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel target/riscv32i-unknown-none-elf/debug/chidori
```
