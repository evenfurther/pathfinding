#! /bin/sh
#

set -e

MSRV=$(awk -F '"' '/^rust-version =/ {print $2}' < Cargo.toml)
if ! grep $MSRV README.md > /dev/null 2>&1; then
  echo "MSRV $MSRV not found in README.md"
  exit 1
fi
