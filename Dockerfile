FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /floatconv
WORKDIR /floatconv/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /floatconv/fuzz/target/x86_64-unknown-linux-gnu/release/floatconv-fuzz /