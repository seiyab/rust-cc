CFLAGS=-std=c11 -g -static

main: src/main.rs
	rustc src/main.rs

test: main
	./test/test.sh

clean:
	rm -f main tmp*

.PHONY: test clean
