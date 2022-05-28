# This layer contains common dependencies needed by build and server
FROM rust:1.61-slim AS dependencies
LABEL MAINTAINER "Jacob Gardner <jacob.gardner@wonderlic.com>"
RUN apt update 
RUN apt install -y libfontconfig-dev

FROM dependencies AS builder

RUN apt install -y git build-essential llvm-dev libclang-dev clang libfreetype-dev 
ADD . /workspace
WORKDIR /workspace

RUN cargo build --bin server --release --no-default-features

FROM dependencies

WORKDIR /workspace

COPY --from=builder /workspace/assets /workspace/assets
COPY --from=builder /workspace/target/release/server /workspace

ENTRYPOINT [ "/workspace/server" ]
# CMD ["./server"]