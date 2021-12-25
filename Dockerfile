FROM rust:1.57.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/sananmuunnos
COPY . .
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11
COPY --from=build /usr/local/cargo/bin/sananmuunnos-webapp /sananmuunnos-webapp
COPY sanat.xml /
# COPY --from=build /usr/src/rosvosektori/static /static/
WORKDIR /
ENTRYPOINT ["/sananmuunnos-webapp"]

