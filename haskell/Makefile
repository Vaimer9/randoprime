BIN=main

all: build

run: build
	bin/$(BIN)

build: src/*.hs
	ghc src/*.hs -o bin/$(BIN)
	mv src/*.o src/*.hi bin/

clean:
	rm bin/*
