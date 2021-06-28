#!/usr/bin/env bash
ls -R ./target
sudo chown $USER:$USER ./target/release/rpmbuild/RPMS/x86_64/*
sudo chmod 755 ./target/release/rpmbuild/RPMS/x86_64/*
mv ./target/release/rpmbuild/RPMS/x86_64/* ./kelpdot.rpm
