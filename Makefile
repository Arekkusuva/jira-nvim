.PHONY: clean
clean:
	rm -rf ./lua/libjira_nvim.so ./lua/deps

.PHONY: compile
compile:
	cargo build --release

.PHONY: out
out:
	mkdir -p lua/deps
	cp ./target/release/libjira_nvim.so ./lua/libjira_nvim.so
	cp ./target/release/deps/*.rlib ./lua/deps/

.PHONY: build
build: clean compile out

