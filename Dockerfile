FROM rust:1.77.1 as builder
COPY . .
RUN cargo build

FROM debian:bookworm
COPY --from=builder /target/debug/webspy ./webspy
EXPOSE 8080
RUN apt-get update
RUN apt-get install libssl-dev -y
CMD ["./webspy"]
