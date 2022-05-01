#!/bin/bash

echo Run this file only on dev server
diesel migration redo --database-url postgres://actix:actix@localhost:5432/actix
cargo test


# sed -i -e 's/\r$//' test.sh