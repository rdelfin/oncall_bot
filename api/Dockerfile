ARG ARCH=
FROM ${ARCH}rust:1.60 as builder

WORKDIR /usr/src/oncall_bot
COPY . .

RUN cargo install --path .

ENV PORT=80 \
    DOTENV_FILE=/data/.env

CMD ["oncall_bot"]
