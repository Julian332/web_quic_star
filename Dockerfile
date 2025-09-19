FROM lukemathwalker/cargo-chef:latest AS builder
WORKDIR /build

#COPY --from=planner /app/recipe.json .
#RUN cargo chef cook --release
COPY . .
RUN cargo build --release   --bin example_app
RUN mv ./target/release/example_app /build/exe
#EXPOSE 5090
#ENTRYPOINT ["/build/exe"]

FROM rust:1.83-slim AS runtime
#RUN apt install build-essential
#RUN apt-get install libpq-dev

WORKDIR /app
COPY --from=builder /build/exe /app/exe
COPY --from=builder /build/.env /app/.env

EXPOSE 5090
ENTRYPOINT ["/app/exe"]