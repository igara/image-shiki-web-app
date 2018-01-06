#!/bin/sh

if [ $WWW_ENV = "local" ]; then
    echo "rust_www: 開発環境用ビルドを開始します"
    cargo install cargo-watch
    cargo watch --poll -x 'run -- --some-arg'
elif [ $WWW_ENV = "production" ]; then
    echo "rust_www: 本番環境ビルドです"
    cargo run
fi
