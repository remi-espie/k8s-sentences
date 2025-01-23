FROM clux/muslrust AS build

WORKDIR /app

COPY Cargo.toml ./

COPY src src

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch AS prod

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/nouns /nouns

COPY nouns.json nouns.json

CMD ["./nouns"]