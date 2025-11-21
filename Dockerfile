FROM lukemathwalker/cargo-chef:latest AS builder
WORKDIR /build

#COPY --from=planner /app/recipe.json .
#RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/web3_quick /build/exe


FROM rust:1.83-slim AS runtime

WORKDIR /app
COPY --from=builder /build/exe /app/exe
COPY --from=builder /build/.env /app/.env
COPY --from=builder /build/env_prod.env /app/env_prod.env

EXPOSE 5090
ENTRYPOINT ["/app/exe"]