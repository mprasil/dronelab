FROM rust as build

WORKDIR /build

COPY . .

RUN rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.8

COPY --from=build /build/target/x86_64-unknown-linux-musl/release/dronelab /bin/dronelab
