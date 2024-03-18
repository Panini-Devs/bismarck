FROM rust:latest

COPY ./ ./

# install sqlx-cli and run "sqlx database setup" in project directory before running the docker image or the following steps will not compile and will result in error

RUN cargo build --release

CMD ["./target/release/bismarck"]