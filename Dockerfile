FROM fedora:43
RUN dnf install -y git zig python3.12 ruby fastfetch vim gcc
COPY busybox-1.37.0-3.fc43.x86_64.rpm /root/
RUN rpm -ivh /root/busybox-1.37.0-3.fc43.x86_64.rpm && rm -f /root/busybox-1.37.0-3.fc43.x86_64.rpm
COPY target/release/bakashell /usr/local/bin/bakashell
RUN chmod +x /usr/local/bin/bakashell
CMD ["/usr/local/bin/bakashell"]
