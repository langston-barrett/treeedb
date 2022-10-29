#!/usr/bin/env bash

set -e

cargo publish --package treeedb
sleep 10
cargo publish --package treeedbgen
sleep 10
cargo publish --package treeedbgen-souffle
sleep 10
for lang in c csharp java javascript rust souffle swift; do
  cargo publish --package treeedb-${lang}
  sleep 10
  cargo publish --package treeedbgen-souffle-${lang}
  sleep 10
done
