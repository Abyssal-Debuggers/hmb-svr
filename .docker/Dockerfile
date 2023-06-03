FROM rust:1.69-slim as planner

WORKDIR /app

RUN apt update && \
    apt install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo install cargo-chef --version 0.1.61 --target=x86_64-unknown-linux-musl --locked

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM planner as cacher

WORKDIR /app

RUN cargo install cargo-chef --version 0.1.61 --target=x86_64-unknown-linux-musl --locked

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --target=x86_64-unknown-linux-musl --recipe-path recipe.json

FROM planner as builder

WORKDIR /app

COPY . .

# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.17

WORKDIR /usr/app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/application /usr/app

ENV DATABASE_OPTION=postgres://postgres:postgres@localhost/postgres
ENV SERVER_PORT=80
EXPOSE 80

CMD ["/usr/app/application"]