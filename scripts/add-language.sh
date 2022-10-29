#!/usr/bin/env bash

set -eu

old=rust
new="${1}"
Old=Rust
New="${2}"

sed_recurse() { ag -g '.*' -0 | xargs -0 sed -E -i "$@"; }

cp -r treeedb-${old} treeedb-${new}
pushd treeedb-${new}
sed_recurse "s/${Old}/${New}/g"
sed_recurse "s/${old}/${new}/g"
popd

cp -r treeedbgen-souffle-${old} treeedbgen-souffle-${new}
pushd treeedbgen-souffle-${new}
sed_recurse "s/${Old}/${New}/g"
sed_recurse "s/${old}/${new}/g"
popd
