FROM archlinux:base-devel

COPY pacman.conf /etc/pacman.conf

RUN pacman -Syu --noconfirm && \
  pacman -S --noconfirm --needed --overwrite '*' bash rustup gcc llvm pkgconf

COPY run.bash /run.bash

ENTRYPOINT ["/run.bash"]
