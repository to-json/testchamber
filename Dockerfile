# syntax=docker/dockerfile:1

FROM rust:1.71.1-bookworm

WORKDIR '/test'
RUN apt update
RUN apt install libseccomp-dev
# RUN bash -c "pushd neovim; make"
# ADD ./nvim-mnml /root/.config/nvim
# ADD ./test.ts /test/test.ts
COPY . .
RUN cargo install --path .
RUN echo lol > ./lol.txt
CMD testchamber cat ./lol.txt
