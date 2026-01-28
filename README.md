# バカOS

バカOS is a Docker based operating system running on Fedora 43, with utils from `busybox`.

## Installation

Requirements:
- `cargo`  (`1.93.0 (083ac5135 2025-12-15)` is preferred)
- `rustc` (`1.93.0 (254b59607 2026-01-19)` is preferred)
- `make`
- `docker.io`
1. Clone the repo

```bash
git clone https://github.com/linuxfandudeguy/bakaos
```
2. Build the shell and container and pretty much start the OS

```bash
make run-os
```
