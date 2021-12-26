FROM rust:1.57.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/sananmuunnos
COPY . .
RUN cargo install --path .

FROM node:16 as buildjs
WORKDIR /usr/src/sananmuunnos/sananmuunnos-js
COPY sananmuunnos-js .
RUN npm install; npm run build


FROM gcr.io/distroless/cc-debian11
COPY --from=build /usr/local/cargo/bin/sananmuunnos-webapp /sananmuunnos-webapp
COPY --from=buildjs /usr/src/sananmuunnos/sananmuunnos-js/dist /static
COPY --from=build /usr/src/sananmuunnos/Rocket.toml /

COPY sanat.xml /
ENV ROCKET_ENV production
ENV ROCKET_ADDRESS "0.0.0.0"
ENV STATIC_DIR /static
# COPY --from=build /usr/src/rosvosektori/static /static/
WORKDIR /
ENTRYPOINT ["/sananmuunnos-webapp"]

