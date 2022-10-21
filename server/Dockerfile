################
##### Builder
FROM rust as builder

# pre build app
WORKDIR /usr/src
RUN USER=root cargo new app
COPY Cargo.toml Rocket.toml /usr/src/app/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# build app
COPY src /usr/src/app/src/
RUN touch /usr/src/app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM gcr.io/distroless/static-debian11 as runner

# get binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/graphql_server /

# set run env
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# run it
CMD ["/graphql_server"]