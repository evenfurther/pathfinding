#! /bin/sh
#

set -e

MSRV=$(awk -F '"' '/^rust-version =/ {print $2}' < Cargo.toml)
if ! grep "Rust $MSRV" src/lib.rs > /dev/null 2>&1; then
  echo "MSRV $MSRV not found in src/lib.rs"
  exit 1
fi
