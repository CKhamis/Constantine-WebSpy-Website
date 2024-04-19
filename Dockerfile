FROM rust:1.77.1 as builder
COPY . .
RUN cargo build --release

FROM debian:bookworm
COPY --from=builder /target/release/webspy ./webspy
EXPOSE 8080
RUN apt-get install libssl-dev libssl1.0.0
CMD ["./webspy"]