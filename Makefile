.PHONY: buildw-nodejs buildw-web clean skip-gitignore build

build: clean build-nodejs
	cp ./wrapper/* ./dist/

build-nodejs:
	rm -rf ./dist/raw
	wasm-pack build --target nodejs --release --out-name index --out-dir dist/raw
	rm -f ./dist/raw/.gitignore

clean:
	rm -rf ./dist