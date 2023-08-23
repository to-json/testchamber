# syntax=docker/dockerfile:1

FROM rustlang/rust:nightly-bookworm

WORKDIR '/test'
RUN apt update
RUN apt install libseccomp-dev
COPY Cargo.toml Cargo.lock syscall.json /test/
# --deps-only no longer exists in cargo; this simulates building deps-only
# for caching purposes, so that later `docker build` runs needn't build all
# dependencies every time
RUN \
    mkdir /test/src && \
    echo 'fn main() {}' > /test/src/main.rs && \
    cargo build && \
    rm -Rvf /test/src && \
    mkdir /test/src
COPY src /test/src
# resume normalcy
RUN cargo build
RUN cp /test/target/debug/testchamber /test/testchamber
RUN echo lol > /test/lol.txt
CMD /test/testchamber cat /test/lol.txt
