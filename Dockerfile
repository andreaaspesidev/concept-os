FROM ubuntu:latest

# Install generic
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y build-essential curl gdb-multiarch

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Install python
RUN apt-get install -y python3 python3-pip

# Install additional packages
RUN apt-get install pkg-config libudev-dev binutils-arm-none-eabi

RUN $HOME/.cargo/bin/rustup install nightly
RUN $HOME/.cargo/bin/rustup target add thumbv7em-none-eabihf 
RUN $HOME/.cargo/bin/rustup component add rust-src

RUN cp $HOME/.bashrc $HOME/.profile

ENTRYPOINT ["tail", "-f", "/dev/null"]
