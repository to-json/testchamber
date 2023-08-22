# syntax=docker/dockerfile:1

FROM rustlang/rust:nightly-bookworm

WORKDIR '/test'
RUN apt update
RUN apt install libseccomp-dev
COPY Cargo.toml Cargo.lock syscall.json /test/
RUN \
    mkdir /test/src && \
    echo 'fn main() {}' > /test/src/main.rs && \
    cargo build && \
    rm -Rvf /test/src && \
    mkdir /test/src
COPY src /test/src
RUN cargo build
RUN cp /test/target/debug/testchamber /test
RUN echo lol > /test/lol.txt
CMD /test/testchamber cat ./lol.txt
