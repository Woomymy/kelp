#!/usr/bin/env bash
set -e
cargo aur
cat << EOF > PKGBUILD
# Maintainer: Woomymy <woomy@woomy.be>
pkgname=kelpdot-bin
pkgver=@VERSION@
pkgrel=1
pkgdesc="Simple dotfiles manager"
url="https://kelp.woomy.be/"
license=("MIT")
arch=("x86_64")
provides=("kelpdot")
options=("strip")
source=("https://github.com/Woomymy/kelp/releases/download/\$pkgver/kelpdot-aur.tar.gz")
sha256sums=("@PKGSUM@")

package() {
    install -Dm755 kelpdot -t "\$pkgdir/usr/bin/"
}
EOF
VERSION="$(grep -i version Cargo.toml | head -n 1 | awk '{print $3}' | cut -d '"' -f 2)"
sed -i "s/@PKGSUM@/$(sha256sum kelpdot-*-x86_64.tar.gz | awk '{print $1}')/g" "PKGBUILD"
sed -i "s/@VERSION@/${VERSION}/g" "PKGBUILD"
mv kelpdot-*.tar.gz kelpdot-aur.tar.gz
