FROM rust:latest

COPY ./ ./

RUN apt-get update && apt-get install -y cmake && apt-get clean

# install sqlx-cli and run "sqlx database setup" in project directory before running the docker image or the following steps will not compile and will result in error

RUN cargo build --release

CMD ["./target/release/bismarck"]