build:
	make clean
	dub test
	make clean
	dub build

clean:
	cd rust && make clean
	dub clean

init-rslib:
	echo cargo new rslib --lib  # only once!

