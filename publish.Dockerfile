# syntax=docker.io/docker/dockerfile:1.3.0

FROM gcr.io/distroless/cc-debian10:latest-${TARGETARCH}

ARG TARGETARCH
ARG TARGETVARIANT

COPY "./outputs/built-${TARGETARCH}/discord-rcon" /discord-rcon
ADD --chmod=555 "https://files.anatawa12.com/tini-download/?arch=${TARGETARCH}&variant=${TARGETVARIANT}" /tini

USER nonroot

CMD ["/tini", "--", "/discord-rcon", "--env"]
