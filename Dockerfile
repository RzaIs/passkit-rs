FROM rust:1.70-alpine3.17 as builder

RUN apk update && apk add pkgconfig openssl openssl-dev musl-dev

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /rust/src

COPY . .

RUN RUSTFLAGS="-C target-feature=-crt-static" cargo build --target x86_64-unknown-linux-musl --release

######## Start a new stage from scratch #######
FROM alpine:3.17
COPY --from=builder /rust/src/target/x86_64-unknown-linux-musl/release/passkit /

RUN apk update && apk add libgcc

CMD [ "./passkit" ]