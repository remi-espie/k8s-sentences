FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml ./

COPY src src

RUN cargo build --release

FROM debian:bookworm-slim AS prod

RUN  apt-get update && apt-get install -y libssl-dev curl

COPY --from=build /app/target/release/aggregator /aggregator

CMD ["./aggregator"]