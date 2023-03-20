FROM rust:slim-buster

EXPOSE 3333

WORKDIR reee/
COPY . .

RUN cargo install --path .

CMD ["reee", "0.0.0.0:3333"]