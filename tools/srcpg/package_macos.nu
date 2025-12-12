#!/usr/bin/env nu

const CRATE = "srcpg"
const PG = "pg18"

let WORKSPACE_DIR = $"($env.FILE_PWD)/../.." | path expand
let CRATE_DIR = $"($WORKSPACE_DIR)/crates/($CRATE)" | path expand

cd $CRATE_DIR

cargo pgrx package

let RELEASE_DIR = $"($WORKSPACE_DIR)/target/release" | path expand
let PKG_ORIG = $"($CRATE)-($PG)"
let PKG = $"($CRATE)_0.0.1"

cd $RELEASE_DIR

rm -rf $PKG
mv $"($PKG_ORIG)/opt/homebrew" $PKG

tar -cf $"($PKG).tar" --no-mac-metadata --no-xattrs $"($PKG)" 
xz -fk $"($PKG).tar"
shasum -a 256 $"($PKG).tar.xz" | save --force $"($PKG).tar.xz.sha256"
gzip -fk $"($PKG).tar"
shasum -a 256 $"($PKG).tar.gz" | save --force $"($PKG).tar.gz.sha256"

