CFLAGS=-std=c11 -g -static
rustfiles := src/**/*.rs

main: $(rustfiles)
	rustc src/main.rs

test: main
	./test/test.sh

clean:
	rm -f main tmp*

.PHONY: test clean
