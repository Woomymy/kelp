#!/usr/bin/env bash
ls -R ./target
mv ./target/release/rpmbuild/RPMS/x86_64/* kelpdot.rpm
