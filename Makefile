CROSS = aarch64-unknown-linux-gnu
PROJECT = $(shell basename ${PWD})

CC = $(CROSS)-gcc
OBJCOPY = $(CROSS)-objcopy

debug:
	cargo build
	$(OBJCOPY) -O binary target/aarch64-unknown-none/debug/$(PROJECT) images/kernel8.img

release:
	cargo build --release
	$(OBJCOPY) -O binary target/aarch64-unknown-none/release/$(PROJECT) images/kernel8.img

clean:
	cargo clean
	rm images/kernel8.img

.PHONY: kernel clean
