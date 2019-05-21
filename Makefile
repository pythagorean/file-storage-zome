NIGHTLY = nightly-2019-01-24

all: dna ui

clean: dna-clean ui-clean

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

build: dna-build ui-build

update: dna-update ui-update

dna: dna-build

dna-build:
	(cd dna-src; hc package)

dna-fmt:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) do fmt, tomlfmt)

dna-lint:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) clippy)

dna-start: dna
	(cd dna-src; hc run)

dna-update:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) update)

dna-clean:
	(cd dna-src/zomes/*/code; cargo +$(NIGHTLY) clean && rm -f Cargo.lock)
	(cd dna-src; rm -rf dist)

ui: ui-build

ui-build:
	(cd ui; rustup run stable wasm-pack build --target web)
	(cd ui/pkg; \
		tar cf snippets.tar `grep "^import" file_storage_zome_client.js | cut -d \' -f2`; \
		rm -rf snippets; \
		tar xf snippets.tar; \
		rm snippets.tar)

ui-fmt:
	(cd ui; cargo +stable do fmt, tomlfmt)

ui-lint:
	(cd ui; cargo +stable clippy)

ui-update:
	(cd ui; cargo +stable update)

ui-clean:
	(cd ui; rm -rf pkg target Cargo.lock)

ui-start: all
	(cd ui; http-server) &
	fswatch -o ui/src | xargs -n 1 -I{} make ui-build
