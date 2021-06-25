#!/usr/bin/env bash
cargo install cargo-deb
cargo deb

VERSION="$(grep -i version Cargo.toml | head -n 1 | awk '{print $3}' | cut -d '"' -f 2)"
mv target/debian/*.deb "kelpdot.deb"
