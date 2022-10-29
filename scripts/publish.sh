#!/usr/bin/env bash

set -e

cargo publish --package treeedb
cargo publish --package treeedbgen
cargo publish --package treeedbgen-souffle
for lang in c csharp java javascript rust souffle swift; do
  cargo publish --package treeedb-${lang}
  cargo publish --package treeedbgen-souffle-${lang}
done
