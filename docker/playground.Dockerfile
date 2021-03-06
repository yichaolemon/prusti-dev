FROM fpoli/prusti-base:latest
MAINTAINER Vytautas Astrauskas "vastrauskas@gmail.com"

# Install Prusti.
# TODO: Download prusti-dev in /tmp/prusti-dev
ADD . /tmp/prusti-dev
RUN cd /tmp/prusti-dev && \
    cargo build --release && \
	mkdir -p /usr/local/prusti/lib && \
	cp rust-toolchain /usr/local/prusti/rust-toolchain && \
	cp target/release/cargo-prusti /usr/local/prusti/cargo-prusti && \
	cp target/release/prusti-rustc /usr/local/prusti/prusti-rustc && \
	cp target/release/prusti-driver /usr/local/prusti/prusti-driver && \
	cp target/release/libprusti_contracts.rlib /usr/local/prusti/libprusti_contracts.rlib
ADD bin/prusti /usr/local/bin/prusti
ADD bin/cargo-prusti /usr/local/bin/cargo-prusti

# Set up workdir.
ENV USER root
RUN cd / && \
    cargo new playground && \
    sed -i '1s/^/extern crate prusti_contracts;\n/;s/println.*$/assert!(true)/' /playground/src/main.rs
WORKDIR /playground

ADD docker/entrypoint.sh /root/
ENTRYPOINT ["/root/entrypoint.sh"]

# Prepare env variables to run Prusti
ENV RUSTC_WRAPPER /usr/local/bin/prusti

# Continue compilation after the verification succeeds
ENV PRUSTI_FULL_COMPILATION true

# Prusti configuration
ENV PRUSTI_ENCODE_UNSIGNED_NUM_CONSTRAINT true

# Reduce log level
ENV RUST_LOG warn

# Pre-build dependencies
# ADD Cargo.toml /playground/Cargo.toml
RUN cargo build
RUN cargo build --release
RUN rm src/*.rs
