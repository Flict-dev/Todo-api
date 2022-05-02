#!/bin/bash

echo Run this file only on dev server
cargo test -- --show-output
diesel migration redo --database-url postgres://actix:actix@localhost:5432/actix


# sed -i -e 's/\r$//' test.sh