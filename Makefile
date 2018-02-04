.PHONY: qemu

kernel: 
	RUST_TARGET_PATH=$(shell pwd) xargo build --target arm-unknown-none

qemu:  
	qemu-system-aarch64 -machine virt -m 1024M -cpu cortex-a53 -nographic -s -kernel target/arm-unknown-none/debug/aarch64-hello