all:
	cargo bootimage
	sudo qemu-system-x86_64 \
	-drive format=raw,file=target/x86_64-tiny_os/debug/bootimage-tiny_os.bin

run:
	cargo run

test:
	cargo test

test_should_panic:
	cargo test --test should_panic

precompile:
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install bootimage

