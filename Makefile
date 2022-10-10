all:
	cargo bootimage
	sudo qemu-system-x86_64 \
	-drive format=raw,file=target/x86_64-tiny_os/debug/bootimage-tiny_os.bin

precompile:
	rustup component add rust-src
	rustup component add llvm-tools-preview
	cargo install bootimage

