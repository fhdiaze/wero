FROM rust:slim-buster AS build

RUN USER=root cargo new --bin wero
WORKDIR /wero

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/wero*
RUN cargo build --release

FROM rust

ENV WERO_DB__CONNECTION_STRING
ENV WERO_SERVER__PORT=80

COPY --from=build /wero/target/release/wero .
COPY ./config ./config

CMD ["./wero"]