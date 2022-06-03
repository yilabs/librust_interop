
all: build

RUST_INTEROP_D = rust_interop_h.d

rust_interop.h:
	cbindgen --lang C --cpp-compat --config cbindgen.toml --crate rust_interop --output $@
	# dub run 
	dstep $@ -o $(RUST_INTEROP_D)

build:
	rustup toolchain install nightly  # needed for cargo-expand
	rustup default nightly
	cargo install cbindgen  # --force
	cargo install cargo-expand
	cargo build --release
	make -B rust_interop.h
	mv --force ./target/release/librust_interop.a ../
	cargo clean  # rust will generate .d files


view_macro_expansion:
	cargo rustc --profile=check -- -Zunpretty=expanded

clean:
	$(RM) -f rust_interop.h $(RUST_INTEROP_D)
	cargo clean

