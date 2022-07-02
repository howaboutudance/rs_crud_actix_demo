FROM docker.io/library/rust:1 as build

RUN USER=root cargo new --bin rs_crud_actix
WORKDIR /rs_crud_actix

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN cargo build --release

FROM docker.io/library/debian:buster-slim as app

COPY --from=build /rs_crud_actix/target/release/rs_crud_actix .
ENTRYPOINT ["./rs_crud_actix"]