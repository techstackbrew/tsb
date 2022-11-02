#! /bin/bash

VERSION="v0.1.0"

echo "Releasing $VERSION..."

# download targets
# rustup target add aarch64-apple-darwin
# rustup target add x86_64-apple-darwin

# aarch64-apple-darwin
# cargo build --target aarch64-apple-darwin --release
tar -cvzf dist/tsb-$VERSION-aarch64-apple-darwin.tar.gz LICENSE README.md target/aarch64-apple-darwin/release/tsb

# aarch64-unknown-linux-gnu
# cross build --target aarch64-unknown-linux-gnu --release
tar -cvzf dist/tsb-$VERSION-aarch64-unknown-linux-gnu.tar.gz LICENSE README.md target/aarch64-unknown-linux-gnu/release/tsb

# x86_64-apple-darwin
# cargo build --target x86_64-apple-darwin --release
tar -cvzf dist/tsb-$VERSION-x86_64-apple-darwin.tar.gz LICENSE README.md target/x86_64-apple-darwin/release/tsb

# x86_64-unknown-linux-gnu
# cross build --target x86_64-unknown-linux-gnu --release
tar -cvzf dist/tsb-$VERSION-x86_64-unknown-linux-gnu.tar.gz LICENSE README.md target/x86_64-unknown-linux-gnu/release/tsb
