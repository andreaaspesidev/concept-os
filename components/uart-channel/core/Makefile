COMPONENT_NAME:=uart-channel
current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
root_dir := $(shell dirname $(realpath ../../.))

.PHONY: build

clean:
	rm -rf build
	rm $(COMPONENT_NAME).cbf

build-f303re:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).cbf -b stm32f303re -f log-itm

build-l432kc:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).cbf -b stm32l432kc -f log-semihosting

build-l476rg:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).cbf -b stm32l476rg -f log-itm

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

dump-cbf:
	../../../libs/cbf_lite/tests/simple_read/target/release/cbf_simple_read $(current_dir)/$(COMPONENT_NAME).cbf

size: build
	size -A build/image.elf