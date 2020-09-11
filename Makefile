build:
	cargo build --release

update: build
	cp target/release/delete_known_host ~/.local/bin
	cp target/release/g ~/.local/bin
	cp target/release/h ~/.local/bin
	cp target/release/p ~/.local/bin