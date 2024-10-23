FROM rust:1.72.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/sananmuunnos
COPY . .
RUN cargo install --path .

FROM node:18 as buildjs
WORKDIR /usr/src/sananmuunnos/sananmuunnos-js
COPY sananmuunnos-js .
RUN npm install; npm run build


FROM debian:bookworm-slim
RUN apt-get -y update; apt-get install -y libpq5
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

