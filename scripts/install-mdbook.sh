#!/usr/bin/env bash
# Downloads latest bin version of mdbook
VERSION="0.4.7"
URL="https://github.com/rust-lang/mdBook/releases/download/v${VERSION}/mdbook-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
echo $USER
wget "${URL}" -O "/tmp/mdbook.tar.gz"
tar xf "/tmp/mdbook.tar.gz" -C "/usr/bin"

