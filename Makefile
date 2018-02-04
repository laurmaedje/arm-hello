kernel    := arm-hello
target    := arm-unknown-none
qemu      := qemu-system-aarch64
qemuflags := -machine virt -m 1024M -cpu cortex-a53 -nographic -s

.PHONY: all clean run kernel

all: kernel

clean:
    cargo clean

run: kernel
    $(qemu) $(qemuflags) -kernel target/$(target)/debug/$(kernel)

kernel: 
    RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)
