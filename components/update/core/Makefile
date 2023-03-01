COMPONENT_NAME:=update
current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
root_dir := $(shell dirname $(realpath ../../.))

.PHONY: build

clean:
	rm -rf build
	rm $(COMPONENT_NAME).hbf

build-f303re:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32f303re -f log-itm

build-l476rg:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32l476rg -f log-itm,multi-support

disassemble-l476rg: build
	arm-none-eabi-readelf -l build/stm32l476rg/image.elf > build/stm32l476rg/headers.disass
	arm-none-eabi-objdump -h build/stm32l476rg/image.elf > build/stm32l476rg/sections.disass
	arm-none-eabi-objdump -s -j .data build/stm32l476rg/image.elf > build/stm32l476rg/data.disass
	arm-none-eabi-objdump -s -j .rodata build/stm32l476rg/image.elf > build/stm32l476rg/rodata.disass
	arm-none-eabi-objdump -d build/stm32l476rg/image.elf --visualize-jumps > build/stm32l476rg/text.asm

dump-l476rg:
	arm-none-eabi-objcopy -O binary --only-section=.text build/stm32l476rg/image.elf build/stm32l476rg/image.text
	arm-none-eabi-objcopy -O binary --only-section=.rodata build/stm32l476rg/image.elf build/stm32l476rg/image.rodata
	arm-none-eabi-objcopy -O binary --only-section=.data build/stm32l476rg/image.elf build/stm32l476rg/image.data

dump-hbf: build
	../../../libs/hbf_lite/tests/simple_read/target/release/hbf_simple_read $(current_dir)/$(COMPONENT_NAME).hbf

size-l476rg: build
	size -A build/stm32l476rg/image.elf