# syntax=docker.io/docker/dockerfile:1.3.0

FROM rust:1 as builder

WORKDIR /project/

COPY Cargo* ./
COPY src src

RUN cargo build --release

FROM gcr.io/distroless/cc-debian10:latest-${TARGETARCH}

ARG TARGETARCH
ARG TARGETVARIANT

COPY --from=builder /project/target/release/discord-rcon /discord-rcon
ADD --chmod=555 "https://files.anatawa12.com/tini-download/?arch=${TARGETARCH}&variant=${TARGETVARIANT}" /tini

USER nonroot

CMD ["/tini", "/discord-rcon"]
