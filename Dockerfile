FROM clux/muslrust:stable as builder
COPY static /build/static
WORKDIR /build
COPY . .
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release

FROM gcr.io/distroless/static-debian11:nonroot
WORKDIR /app
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rust-api /app/rust-api
COPY --from=builder /build/static /app/static
EXPOSE 8080
CMD ["/app/rust-api"]
