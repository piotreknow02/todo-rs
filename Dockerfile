FROM alpine:latest AS base
WORKDIR /app/

FROM rust:1.67-alpine AS build
WORKDIR /build/
COPY . .
RUN cargo build --release

FROM base AS final
COPY --from=build  /build/target/release/go-todo /app/go-todo
ENTRYPOINT [ "/app/go-todo" ]