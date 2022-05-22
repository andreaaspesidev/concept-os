COMPONENT_NAME:="component4"
all: clean build disassemble dump output-reloc

clean:
	cargo clean
	rm -rf disassemble/*
	rm -rf output/*

build:
	cargo build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	cp target/thumbv7em-none-eabihf/release/$(COMPONENT_NAME) output/image.elf

disassemble: build
	arm-none-eabi-readelf -l output/image.elf > disassemble/headers
	arm-none-eabi-objdump -h output/image.elf > disassemble/sections
	arm-none-eabi-objdump -s -j .data output/image.elf > disassemble/data
	arm-none-eabi-objdump -s -j .rodata output/image.elf > disassemble/rodata
	arm-none-eabi-objdump -d output/image.elf --visualize-jumps > disassemble/text.asm

dump: build
	arm-none-eabi-objcopy -O binary --only-section=.text output/image.elf output/image.text
	arm-none-eabi-objcopy -O binary --only-section=.rodata output/image.elf output/image.rodata
	arm-none-eabi-objcopy -O binary --only-section=.data output/image.elf output/image.data

output-reloc: build
	python3 ../../../../scripts/relocations/elf_relocations.py output/image.elf output/out.map output/relocations.toml 