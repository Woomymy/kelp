#!/usr/bin/env bash
ls -R ./target
mv ./target/x86_64-unknown-linux-musl/release/rpmbuild/RPMS/x86_64/* kelpdot.rpm
