#!/usr/bin/env sh

diesel migration redo --database-url postgres://actix:actix@localhost:5432/actix

cargo test -- --show-output

