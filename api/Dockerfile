ARG ARCH=
FROM ${ARCH}rust:1.60 as builder

WORKDIR /usr/src

RUN cargo new --bin oncall_bot
WORKDIR /usr/src/oncall_bot
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm src/*.rs


ADD . ./

RUN rm ./target/release/deps/oncall_bot*
RUN cargo build --release

ENV PORT=80 \
    DOTENV_FILE=/data/.env

CMD ["target/release/oncall_bot"]
