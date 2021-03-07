#!/usr/bin/env bash
set -e
cargo aur
cat << EOF >> PKGBUILD
# Maintainer: Woomy4680-exe <github@woomy.ovh>
pkgname=kelpdot-bin
pkgver=@VERSION@
pkgrel=1
pkgdesc="Simple dotfiles manager"
url="https://woomy4680-exe.github.io/kelp-docs/"
license=("MIT")
arch=("x86_64")
provides=("kelpdot")
options=("strip")
source=("https://github.com/Woomy4680-exe/kelp/releases/download/\$pkgver/kelpdot-aur.tar.gz")
sha256sums=("@PKGSUM@")

package() {
    install -Dm755 kelpdot -t "$pkgdir/usr/bin/"
}
EOF
VERSION="$(grep -i version Cargo.toml | head -n 1 | awk '{print $3}' | cut -d '"' -f 2)"
sed -i "s/@PKGSUM@/$(sha256sum kelpdot-*-x86_64.tar.gz | awk '{print $1}')/g" "PKGBUILD"
sed -i "s/@VERSION@/${VERSION}/g" "PKGBUILD"
mv kelpdot-*.tar.gz kelpdot-aur.tar.gz
