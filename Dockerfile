FROM rust:slim-buster AS build
EXPOSE 3333

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /reee
COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest
WORKDIR /opt/reee
COPY --from=build /reee/target/x86_64-unknown-linux-musl/release/reee ./

CMD ["./reee", "0.0.0.0:3333"]
