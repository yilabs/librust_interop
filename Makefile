build:
	make clean
	dub test
	make clean
	dub build

clean:
	cd rust && make clean
	dub clean
	$(RM) librust_interop_d.a  rust_interop_d-test-library

init-rslib:
	echo cargo new rslib --lib  # only once!

