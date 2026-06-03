# NOSONAR(docker:S6471): this GitHub Action builder runs as root so the entrypoint can
# replace /usr/bin/makepkg and execute pacman/package-build commands in CI.
FROM archlinux:base-devel

COPY pacman.conf /etc/pacman.conf

# install dependencies
RUN pacman -Syu --noconfirm && \
  pacman -S --noconfirm --needed --overwrite '*' \
    bash \
    gcc \
    llvm \
    pkgconf \
    rustup

COPY run.bash /run.bash

ENTRYPOINT ["/run.bash"]
