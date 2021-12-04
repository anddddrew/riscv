from rust:latest as builder

LABEL maintainer="git@nijmeh.xyz"
LABEL description="RISC-V core in rust."

COPY . /usr/src/app
WORKDIR /usr/src/app

RUN rustup toolchain install nightly && \
  rustup default nightly

ENV CARGO_MAKE_VERSION 0.33.0
ENV CARGO_MAKE_TMP_DIR /tmp/setup-rust-cargo-make
RUN mkdir ${CARGO_MAKE_TMP_DIR} && \
    wget -qO ${CARGO_MAKE_TMP_DIR}/cargo-make.zip https://github.com/sagiegurari/cargo-make/releases/download/${CARGO_MAKE_VERSION}/cargo-make-v${CARGO_MAKE_VERSION}-x86_64-unknown-linux-musl.zip && \
    unzip -d ${CARGO_MAKE_TMP_DIR} ${CARGO_MAKE_TMP_DIR}/cargo-make.zip && \
    mv ${CARGO_MAKE_TMP_DIR}/cargo-make-v${CARGO_MAKE_VERSION}-x86_64-unknown-linux-musl/cargo-make /usr/local/bin

RUN cargo build --release

