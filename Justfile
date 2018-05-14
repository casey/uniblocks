default: codegen

run:
	cargo run

check:
	cargo check

get:
	curl https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt > Blocks.txt

codegen:
	(cd codegen && cargo run) | rustfmt > src/block.rs
	cargo run
