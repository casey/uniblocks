default: run

run:
	cargo run

check:
	cargo check

codegen:
	curl https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt > codegen/Blocks.txt
	cd codegen && cargo run
