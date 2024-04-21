from rust:1.77 as builder
WORKDIR /usr/src/

# RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src ./src 

RUN cargo build --release
RUN strip target/release/hxn

FROM gcr.io/distroless/cc-debian12 as release
WORKDIR /app
COPY --from=builder /usr/src/target/release/hxn .
USER 1000

ENTRYPOINT ["/app/hxn"]
