FROM debian:bullseye AS base
WORKDIR /app/

FROM rust:1.67-bullseye AS build
WORKDIR /build/
COPY . .
RUN cargo build --release

FROM base AS final
COPY --from=build  /build/target/release/todo-rs /app/todo-rs
CMD [ "/app/todo-rs" ]