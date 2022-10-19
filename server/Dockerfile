################
##### Builder
FROM rust as builder
WORKDIR /rust

RUN USER=root cargo new app
COPY Cargo.toml Rocket.toml /rust/app/
WORKDIR /rust/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY src /rust/app/src/
RUN touch /rust/app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM alpine as runner
COPY --from=builder /rust/app/target/x86_64-unknown-linux-musl/release/graphql_server /usr/local/bin
WORKDIR /rust/app
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/usr/local/bin/graphql_server"]