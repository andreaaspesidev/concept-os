COMPONENT_NAME:=test_a
current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
root_dir := $(shell dirname $(realpath ../../.))

.PHONY: build

clean:
	rm -rf build
	rm $(COMPONENT_NAME).hbf

build-f303re:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32f303re -f log-itm

build-verbose-f303re:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32f303re -f log-itm -v

build-l432kc:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32l432kc -f log-semihosting

build-verbose-l432kc:
	ROOT_DIR=$(root_dir) ../../../toolchain/modules/component_builder/component_builder -s $(current_dir) -o $(current_dir)/$(COMPONENT_NAME).hbf -b stm32l432kc -f log-semihosting -v


disassemble-f303re: build
	arm-none-eabi-readelf -l build/stm32f303re/image.elf > build/stm32f303re/headers.disass
	arm-none-eabi-objdump -h build/stm32f303re/image.elf > build/stm32f303re/sections.disass
	arm-none-eabi-objdump -s -j .data build/stm32f303re/image.elf > build/stm32f303re/data.disass
	arm-none-eabi-objdump -s -j .rodata build/stm32f303re/image.elf > build/stm32f303re/rodata.disass
	arm-none-eabi-objdump -d -r build/stm32f303re/image.elf --visualize-jumps > build/stm32f303re/text.asm

dump: build
	arm-none-eabi-objcopy -O binary --only-section=.text build/image.elf build/image.text
	arm-none-eabi-objcopy -O binary --only-section=.rodata build/image.elf build/image.rodata
	arm-none-eabi-objcopy -O binary --only-section=.data build/image.elf build/image.data

dump-hbf: build
	../../../libs/hbf_lite/tests/simple_read/target/release/hbf_simple_read $(current_dir)/$(COMPONENT_NAME).hbf

size: build
	size -A build/image.elf