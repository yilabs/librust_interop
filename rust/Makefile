
RUST_INTEROP_D = rust_interop.d

rust_interop.h:
	cbindgen --lang C --cpp-compat --config cbindgen.toml --crate rust_interop --output $@
	dstep $@ -o $*.d

build:
	cargo build
	make rust_interop.h

clean:
	$(RM) -f rust_interop.h $(RUST_INTEROP_D)
	cargo clean

