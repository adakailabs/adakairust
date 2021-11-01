.PHONY: doc
doc:
	cargo doc --no-deps --lib
	cp -r target/doc/* docs


