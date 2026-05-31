# Maintainer: Lordseriouspig <lordseriouspig@gmail.com>
pkgname=starcli
pkgver=1.0.3
pkgrel=1
pkgdesc="StarCLI is a fully-featured implementation of the Stardance API, allowing for easier and streamlined interaction with HackClub's Stardance."
arch=('x86_64' 'aarch64')
url="https://github.com/lordseriouspig/starcli"
license=('GPL')
depends=("glibc")
makedepends=('rust' 'cargo' 'zip')
source=("https://github.com/lordseriouspig/starcli/archive/refs/tags/v${pkgver//_/-}.tar.gz")
sha256sums=('SKIP')

build() {
	cd "$srcdir/$pkgname-${pkgver//_/-}"
	cargo build --release
}

package() {
    cd "$srcdir/$pkgname-${pkgver//_/-}"
    install -Dm755 "target/release/star" "$pkgdir/usr/bin/star"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 "README.MD" "$pkgdir/usr/share/doc/$pkgname/README.MD"
    install -Dm644 "CHANGELOG.md" "$pkgdir/usr/share/doc/$pkgname/CHANGELOG.md"
}
