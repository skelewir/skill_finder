.PHONY: build clean

build:
	cargo build --release
	mkdir -p scripts
	cp target/release/skill-finder.exe scripts/

clean:
	cargo clean
	rm -rf scripts/
