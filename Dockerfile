FROM rust:1.77-slim-bullseye as builder
WORKDIR /usr/src/

# RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src ./src 

RUN cargo build --release
RUN strip target/release/hxn


####
#Begin final image
####
FROM debian:bullseye-slim as release
WORKDIR /app
COPY --from=builder /usr/src/target/release/hxn .

RUN apt update
RUN apt install -y wget
RUN wget -q https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt remove wget -y
RUN apt install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb

RUN ln -s /opt/google/chrome/chrome /usr/bin/chrome
RUN mkdir /app/hxnshots

USER 1000
ENTRYPOINT ["/app/hxn"]