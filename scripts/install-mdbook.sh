#!/usr/bin/env bash
# Downloads latest bin version of mdbook
VERSION="0.4.7"
URL="https://github.com/rust-lang/mdBook/releases/download/v${VERSION}/mdbook-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
wget "${URL}" -O - | sudo tar -C "/usr/bin" xzvf -

