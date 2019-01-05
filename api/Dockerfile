
FROM buildpack-deps:stretch as builder

ENV \
  RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH \
  URL="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
  RUST_VERSION="nightly-2019-01-01"
RUN set -eux; \
  wget "${URL}"; \
  chmod +x rustup-init; \
  ./rustup-init -y --no-modify-path --default-toolchain "${RUST_VERSION}"; \
  rm rustup-init; \
  chmod -R a+w "${RUSTUP_HOME}" "${CARGO_HOME}";

WORKDIR /build
COPY ./ ./
RUN cargo build --release

FROM debian:7-slim

ARG BINARY=api
ARG PORT=3000
ARG TIMEOUT=1000
ARG DEBIAN_FRONTEND=noninteractive

ENV BINARY=${BINARY} PORT=${PORT} TIMEOUT=${TIMEOUT}

WORKDIR /app

RUN \
  echo "deb http://ftp.debian.org/debian stable main contrib non-free" >> /etc/apt/sources.list; \
  apt-key adv --keyserver keyserver.ubuntu.com --recv-keys EF0F382A1A7B6500; \
  apt-get update; \
  apt-get install -y libc6; \
  apt-get clean; rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/doc/*

COPY --from=builder /build/target/release/* ./

CMD ./${BINARY}
