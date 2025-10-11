FROM node:24.6.0-bookworm-slim AS node-build
COPY ./mini-santa-frontend-riotjs/package.json /mini-santa-frontend-riotjs/package-lock.json /mini-santa-frontend-riotjs/webpack.config.js /mini-santa-frontend-riotjs/LICENSE /mini-santa-frontend-riotjs/readme.md /project-code/
RUN cd /project-code && npm install
COPY /mini-santa-frontend-riotjs/src /project-code/src/
RUN cd /project-code && npm run build 

FROM rust:1.89.0-slim-bookworm AS rust-prebuild
RUN rustup target add x86_64-unknown-linux-gnu

FROM rust-prebuild AS rust-sources
COPY ./Cargo.lock ./Cargo.toml /project-code/
COPY ./src /project-code/src

FROM rust-sources AS rust-sources-plus-deps
RUN cd /project-code && cargo fetch --target x86_64-unknown-linux-gnu

FROM rust-sources-plus-deps AS rust-build
RUN cd /project-code && cargo build --target x86_64-unknown-linux-gnu -r --offline  

FROM debian:bookworm-slim AS server-preready

RUN apt-get update -y && apt-get install logrotate gettext-base curl -y

FROM server-preready AS server-ready

COPY ./bash-scripts /executables/bash-scripts
COPY ./app /executables/app
RUN mkdir -p "/executables/db" && rm -f "/executables/app/static/index.html"
COPY ./Config.toml.template /executables/
COPY --from=rust-build /project-code/target/x86_64-unknown-linux-gnu/release/mini-santa /executables/mini-santa
COPY --from=node-build /project-code/dist /executables/app/static
COPY --from=node-build /project-code/dist/index.html /executables/app/templates

EXPOSE 8080

WORKDIR /executables
ENTRYPOINT [ "sh",  "/executables/bash-scripts/entrypoint.sh"]
CMD [ "" ]