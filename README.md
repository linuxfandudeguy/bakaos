# バカOS

バカOS is a Docker based operating system running on Fedora 43, with utils from `busybox`.

> note: admin mode is more like advanced mode 

## Installation

Requirements:
- `cargo`  (`1.93.0 (083ac5135 2025-12-15)` is preferred)
- `rustc` (`1.93.0 (254b59607 2026-01-19)` is preferred)
- `make`
- `docker.io`
1. Clone the repo

```sh
git clone https://github.com/linuxfandudeguy/bakaos
```
2. Build the shell and container and pretty much start the OS

```sh
make run-os
```
## Customization
You can install crates to add as addons by adding it as a line under `dependencies` in `Cargo.toml`, which can be imported into the shell. (`src/main.rs`)

```toml
[package]
name = "bakashell" # Dont mod this line
version = "0.1.0" # Dont mod this line
edition = "2021" # Dont mod this line

[dependencies]
rustyline = "17.0.2" # Dont mod this line 
                     # insert a package here
```

You can also remove `--rm` from the command in the `run-os` function in the Makefile to have the OS be persistent, but it eats at disk storage.

```make
.PHONY: run-os
run-os: docker
	sudo docker run -it --rm $(IMAGE)
```

## Licence
MIT
