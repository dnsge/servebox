# Build stage
FROM rust:1.70 AS builder

# Install target platform
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

COPY Cargo.toml .
COPY Cargo.lock .

# Dummy build to cache dependencies
RUN mkdir ./src && echo 'fn main() { println!("Dummy build"); }' > ./src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release

# Copy over source files
RUN rm -rf ./src
COPY ./src ./src

# Fix last-modified attribute of main
RUN touch -a -m ./src/main.rs

# Compile binary
RUN cargo build --target x86_64-unknown-linux-musl --release

###
### Run stage
### 
FROM alpine:latest

WORKDIR /app
COPY --from=builder ./target/x86_64-unknown-linux-musl/release/servebox ./servebox

ENV RUST_LOG=info
ENV SERVEBOX_CONFIG_FILE=/config/servebox.toml

CMD ["./servebox"]
