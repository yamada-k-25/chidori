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
