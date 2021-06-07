# See: https://crates.io/crates/cargo-chef

FROM rustlang/rust:nightly-alpine as planner

ADD \
    Cargo.toml \
    Cargo.lock \
    /workspace/

WORKDIR /workspace

# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN apk add --no-cache --update musl-dev
RUN cargo install cargo-chef
RUN cargo chef prepare  --recipe-path recipe.json

FROM rustlang/rust:nightly-alpine as cacher

WORKDIR /workspace

RUN apk add --no-cache --update musl-dev
RUN cargo install cargo-chef
COPY --from=planner /workspace/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly-alpine as builder

COPY . /workspace
WORKDIR /workspace

# Copy over the cached dependencies
COPY --from=cacher /workspace/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN apk add --no-cache --update mariadb-connector-c-dev musl-dev
# https://users.rust-lang.org/t/sigsegv-with-program-linked-against-openssl-in-an-alpine-container/52172
RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --release --bin air-balloon-api

FROM alpine:3.12 as runtime
WORKDIR /
COPY --from=builder /workspace/target/release/air-balloon-api /usr/local/bin
# https://github.com/mischov/meeseeks/issues/98#issuecomment-636615680
RUN apk add --no-cache --update mariadb-connector-c libgcc
ENTRYPOINT ["/usr/local/bin/air-balloon-api"]
