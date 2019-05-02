NIGHTLY = nightly-2019-01-24

all: dna ui

clean: dna-clean ui-clean

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

build: dna-build ui-build

update: dna-update ui-update

dna: dna-build

dna-build: dna
	(cd dna-src; hc package)

dna-fmt:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) fmt)

dna-lint:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) clippy)

dna-start: dna
	(cd dna-src; hc run)

dna-update:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) update)

dna-clean:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) clean && rm -f Cargo.lock)

ui: ui-build

ui-build:
	(cd ui; wasm-pack build --target web)

ui-fmt:
	(cd ui; cargo fmt)

ui-lint:
	(cd ui; cargo clippy)

ui-update:
	(cd ui; cargo update)

ui-clean:
	(cd ui; rm -rf pkg target Cargo.lock)

ui-start: all
	(cd ui; http-server) &
	fswatch -o ui/src | xargs -n 1 -I{} make ui-build
