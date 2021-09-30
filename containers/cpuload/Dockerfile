FROM rust:1.55 as build

RUN USER=root cargo new --bin cpuload
WORKDIR /cpuload

# Cache deps
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/cpuload*
RUN cargo build --release

# ---

FROM debian:buster-slim
COPY --from=build /cpuload/target/release/cpuload .
CMD ["./cpuload"]
