FROM archlinux:base-devel

COPY pacman.conf /etc/pacman.conf

# install dependencies
RUN pacman -Syu --noconfirm && \
  pacman -S --noconfirm --needed --overwrite '*' \
    bash \
    gcc \
    llvm \
    pkgconf \
    rustup \
    sudo

# Create builder user with UID 1001 to match GitHub Actions runner
# This ensures the builder user can access mounted volumes from GitHub Actions
RUN useradd -m -u 1001 -s /bin/bash builder && \
  echo "builder ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/builder && \
  chmod 0440 /etc/sudoers.d/builder

COPY run.bash /run.bash


ENTRYPOINT ["/run.bash"]
