ifeq ($(OS),Windows_NT)
	LIB_EXT = dll
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		LIB_EXT = so
	endif
	ifeq ($(UNAME_S),Darwin)
		LIB_EXT = dylib
	endif
endif

all:
	@echo $(OSFLAG)

.PHONY: clean
clean:
	rm -rf ./lua/libjira_nvim.so ./lua/deps

.PHONY: compile
compile:
	cargo build --release

.PHONY: out
out:
	mkdir -p lua/deps
	cp ./target/release/libjira_nvim.$(LIB_EXT) ./lua/libjira_nvim.so
	cp ./target/release/deps/*.rlib ./lua/deps/

.PHONY: build
build: clean compile out

