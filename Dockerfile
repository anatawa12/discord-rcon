FROM rust:1 as builder

ADD . /project

RUN cd /project && \
    cargo build --release

FROM gcr.io/distroless/base-debian10

COPY --from=builder /project/target/release/discord-rcon /discord-rcon

USER nonroot

CMD ["/discord-rcon"]
