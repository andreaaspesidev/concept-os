COMPONENT_NAME:="allocator"
current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
root_dir := $(shell dirname $(realpath ../../.))

.PHONY: build

clean:
	rm -rf build
	rm allocator.hbf

build:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/allocator.hbf -b stm32f303e

build-verbose:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/allocator.hbf -b stm32f303e -v


disassemble: build
	arm-none-eabi-readelf -l build/image.elf > build/headers.disass
	arm-none-eabi-objdump -h build/image.elf > build/sections.disass
	arm-none-eabi-objdump -s -j .data build/image.elf > build/data.disass
	arm-none-eabi-objdump -s -j .rodata build/image.elf > build/rodata.disass
	arm-none-eabi-objdump -d build/image.elf --visualize-jumps > build/text.asm

dump: build
	arm-none-eabi-objcopy -O binary --only-section=.text build/image.elf build/image.text
	arm-none-eabi-objcopy -O binary --only-section=.rodata build/image.elf build/image.rodata
	arm-none-eabi-objcopy -O binary --only-section=.data build/image.elf build/image.data