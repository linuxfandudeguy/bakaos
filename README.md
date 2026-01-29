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
You can install crates to add as addons by adding it as a line under `dependencies` in `Cargo.toml`, which can be imported into the shell by modifying the code in `src/main.rs`. 

```toml
[package]
name = "bakashell" # Dont mod this line
version = "0.1.0" # Dont mod this line
edition = "2021" # Dont mod this line

[dependencies]
rustyline = "17.0.2" # Dont mod this line 
duct = "1.1.1" # Dont mod this line
 glob = "0.3.3" # Dont mod this line
# insert a package here
```

You can also remove `--rm` from the command in the `run-os` function in the Makefile to have the OS be persistent, but it eats at disk storage.

```make
.PHONY: run-os
run-os: docker
	sudo docker run -it --rm $(IMAGE)
```
## Installing packages 

> you can only use your installed packages in admin mode :(

To install any package, do the following:

1. Enter admin mode

Run `admin` in the shell to follow these instructions, or else it wont work.
```sh
バカ:/$ admin
Admin mode enabled
バカ(admin)/$
```
2. Find the rpm for the package you want to install

Go onto an RPM mirror, such as [rpmfind](https://rpmfind.net/) and make sure the system is Fedora 43 and that the arch is `x86_64`, and search for the package you want.

3. Fetch the rpm
Instead of clicking the url of your rpm package, you should right click and choose "Copy link address".
Then, in the terminal, type (or copy and paste) the following:
```sh
curl (your rpm url here)  -O

```
For this example, I am using `nano`.
After running the command, it should output this:
```sh
バカ(admin)/$ curl https://rpmfind.net/linux/fedora/linux/releases/43/Everything/x86_64/os/Packages/n/nano-8.5-2.fc43.x86_64.rpm -O
curl: applet not found
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100  710k  100  710k    0     0   367k      0  0:00:01  0:00:01 --:--:--  373k
バカ(admin)/$
```

The `curl: applet not found` just means it checked `busybox` first.

4. Install using RPM 
To check what the file is called, just type `ls`.

```sh
バカ(admin)/$ ls
afs                         lib64                       run
bin                         media                       sbin
boot                        mnt                         srv
dev                         nano-8.5-2.fc43.x86_64.rpm  sys
etc                         opt                         tmp
home                        proc                        usr
lib                         root                        var
バカ(admin)/$
```

`nano-8.5-2.fc43.x86_64.rpm` is the rpm I will install.

Now, to install your rpm, run the following:

```sh
rpm -i (filename).rpm
```
This will output
```
バカ(admin)/$ rpm -i  nano-8.5-2.fc43.x86_64.rpm
rpm: no gzip/bzip2/xz magic
```
You can now run your command.



## Licence
MIT
