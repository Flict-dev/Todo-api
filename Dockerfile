FROM debian:bullseye-slim
FROM rust


COPY Cargo.toml Cargo.toml
COPY /src/ /src/

RUN cargo build --release
CMD ["target/release/todo_actix"] 
