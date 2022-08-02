# This layer contains common dependencies needed by build and server
FROM rust:1.61-bullseye AS builder
LABEL MAINTAINER "Jacob Gardner <jacob.gardner@wonderlic.com>"

RUN apt update && \
    apt install -y git build-essential llvm-dev libclang-dev clang libfreetype-dev libfontconfig-dev
ADD . /workspace
# ADD Docker-Cargo.toml /workspace/.cargo/config.toml
WORKDIR /workspace

RUN cargo build --bin server --release 
# RUN cargo build --bin server --release --no-default-features
RUN cargo build --bin cli --release --no-default-features


# FROM bitnami/minideb:bullseye
FROM gcr.io/distroless/base-debian11

WORKDIR /workspace


COPY --from=builder /workspace/assets /workspace/assets
COPY --from=builder /workspace/target/release/server /workspace
COPY --from=builder /workspace/target/release/cli /workspace

COPY --from=builder /etc/fonts/fonts.conf /etc/fonts/fonts.conf

# Distroless doesn't have a package manager so we have to manually pull over the
# dependencies.  We determined what libraries were in use from ldd
COPY --from=builder [\
    "/usr/lib/x86_64-linux-gnu/libfontconfig.so.1", \
    "/usr/lib/x86_64-linux-gnu/libfreetype.so.6", \
    "/lib/x86_64-linux-gnu/libexpat.so.1", \
    "/usr/lib/x86_64-linux-gnu/libuuid.so.1", \ 
    "/usr/lib/x86_64-linux-gnu/libpng16.so.16", \
    "/lib/x86_64-linux-gnu/libz.so.1", \ 
    "/usr/lib/x86_64-linux-gnu/libbrotlidec.so.1", \ 
    "/usr/lib/x86_64-linux-gnu/libbrotlicommon.so.1", \
    "/usr/lib/x86_64-linux-gnu/libstdc++.so.6", \
    "/lib/x86_64-linux-gnu/libgcc_s.so.1", \
    "/usr/lib/" ]

ENV RUST_LOG warn,server=info,pdf_render=infgit sto

ENTRYPOINT [ "/workspace/server" ]
# CMD ["./server"]