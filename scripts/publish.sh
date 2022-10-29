#!/usr/bin/env bash

set -e

# TODO(#23): Upload Java crates

# See #22 for discussion on the rate limit.

cargo publish --package treeedb
sleep 60
cargo publish --package treeedbgen
sleep 60
cargo publish --package treeedbgen-souffle
sleep 60
for lang in c csharp javascript rust souffle swift; do
  cargo publish --package treeedb-${lang}
  sleep 60
  cargo publish --package treeedbgen-souffle-${lang}
  sleep 60
done
