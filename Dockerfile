from rust:1.77 as builder
WORKDIR /usr/src/

COPY Cargo.toml Cargo.lock ./
COPY src ./src 

RUN cargo build --release
RUN strip target/release/hxn

FROM alpine:latest as release
WORKDIR /app
COPY --from=builder /app/target/release/hxn .

ENTRYPOINT ["/app/hxn"]
