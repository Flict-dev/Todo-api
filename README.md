# To Do API

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Flict-dev/Todo-api/Tests?label=build) ![License](https://img.shields.io/github/license/Flict-dev/Todo-api)


## Usage:zap:
Use the virtual variable RUST_LOG for logging
```
# Copy example .env file
cp ./config/.env.example .env

# Run postgres
docker-compose up -d postgres

# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL=postgres://actix:actix@localhost:7878/actix diesel migration run

# Run unit tests
cargo test

# Run the server (Add --release for an optimized build)
cargo run

# Check it
curl http://localhost:7878/api/spec/v2
```
## Swagger schema:memo:
[here](https://app.swaggerhub.com/apis/Flict-dev/Todo/0.5)

---

## Docker:whale2:
```
cd docker
docker-compose up
```

## Build🛠️
```bash
cargo build --release
```

## Tests:test_tube:
```bash
./test.sh 
```
or
```bash
cargo test
```


