#! /bin/sh
#

set -e

MSRV=$(grep rust-version Cargo.toml | sed -e 's/"$//' -e 's/.*"//')
for f in README.md .github/workflows/tests.yml; do
  if ! grep $MSRV $f > /dev/null 2>&1; then
    echo "MSRV $MSRV not found in $f"
    exit 1
  fi
done
