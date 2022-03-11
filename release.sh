#! /bin/sh
#
# Usage: ./release.sh [arguments to cargo release]

set -e

git changelog -n CHANGELOG.md
git commit -am "Prepare ChangeLog for next release"
cargo release --execute "$@"
tag=$(git tag --list --sort=-v:refname | head -n 1)
prev_tag=$(git tag --list --sort=-v:refname | head -n 2 | tail -n 1)
changelog=$(mktemp)
sed -e "/^$prev_tag/,\$d" CHANGELOG.md | sed -e '1,4d' -e '/^$/d' > "$changelog"
gh release create $tag -F "$changelog"
rm "$changelog"
