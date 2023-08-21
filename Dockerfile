# syntax=docker/dockerfile:1

FROM rustlang/rust:nightly-bookworm

WORKDIR '/test'
RUN apt update
RUN apt install libseccomp-dev
COPY . .
RUN cargo install --path .
RUN echo lol > ./lol.txt
CMD testchamber cat ./lol.txt
