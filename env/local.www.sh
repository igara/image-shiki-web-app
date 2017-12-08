#!/bin/sh

cargo install cargo-watch
cargo watch --poll -x 'run -- --some-arg'
