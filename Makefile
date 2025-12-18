CARGO = cargo

CARGO_FLAGS =

DL = $(shell ls ./**/*.dl)
RS = $(shell ls ./**/*.rs)
TOML = $(shell ls ./**/*.toml)

.PHONY: build
build:
	$(CARGO) $(CARGO_FLAGS) --locked build

.PHONY: check
check:
	$(CARGO) check --locked $(CARGO_FLAGS)

.PHONY: entr
entr:
	ls Makefile $(DL) $(RS) $(TOML) | \
	  entr -c -s "make -j fmt check lint && make build && make test"

.PHONY: fmt
fmt:
	$(CARGO) fmt $(CARGO_FLAGS)

.PHONY: lint
lint:
	$(CARGO) clippy $(CARGO_FLAGS) -- \
	  -D warnings \
	  -D clippy::unnecessary_wraps

# requires: apt-get install -y musl-tools
# requires: rustup target add x86_64-unknown-linux-musl
.PHONY: static
static:
	$(CARGO) build $(CARGO_FLAGS) \
	  --bins \
	  --locked \
	  --release \
	  --target=x86_64-unknown-linux-musl

.PHONY: test
test:
	$(CARGO) build --locked $(CARGO_FLAGS) --workspace \
		--exclude treeedbgen-souffle-python \
		--exclude treeedbgen-souffle-ruby
	$(CARGO) test --locked $(CARGO_FLAGS) --workspace \
		--exclude treeedbgen-souffle-python \
		--exclude treeedbgen-souffle-ruby

.PHONY: all
all: build check fmt lint test
