FROM node:24.6.0-bookworm-slim AS node-build
COPY ./mini-santa-frontend-riotjs /project-code/
RUN cd /project-code && npm install && npm run build 

FROM rust:1.89.0-slim-bookworm AS rust-prebuild
RUN rustup target add x86_64-unknown-linux-gnu

FROM rust-prebuild AS rust-sources
COPY ./src /project-code/src
COPY ./Cargo.lock ./Cargo.toml /project-code/

FROM rust-sources AS rust-sources-plus-deps
RUN cd /project-code && cargo fetch --target x86_64-unknown-linux-gnu

FROM rust-sources-plus-deps AS rust-build
RUN cd /project-code && cargo build --target x86_64-unknown-linux-gnu -r --offline  


FROM debian:bookworm-slim AS server-preready

RUN apt-get update -y && apt-get install logrotate -y

FROM server-preready AS server-ready

COPY ./bash-scripts /executables/bash-scripts
COPY --from=rust-build /project-code/target/x86_64-unknown-linux-gnu/release/mini-santa /executables/mini-santa
COPY ./app /executables/app
COPY ./Config.toml /executables/
COPY --from=node-build /project-code/dist /executables/app/static
COPY --from=node-build /project-code/dist/index.html /executables/app/templates
RUN mkdir -p "/executables/db" && rm -f "/executables/app/static/index.html"

EXPOSE 8080

WORKDIR /executables
ENTRYPOINT [ "sh",  "/executables/bash-scripts/entrypoint.sh"]
CMD [ "" ]