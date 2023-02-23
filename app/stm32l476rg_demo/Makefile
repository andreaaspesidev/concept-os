current_dir := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: build

build:
	../../toolchain/modules/system_builder/system_builder --app-config $(current_dir)/App.toml --output-path $(current_dir)/App

flash:
	../../toolchain/modules/update_cli/update-cli-uart flash-system --app-config $(current_dir)/App.toml --image-path $(current_dir)/App.ihex

debug:
	../../toolchain/modules/update_cli/update-cli-uart debug --app-config $(current_dir)/App.toml

gdb:
	../../toolchain/modules/update_cli/update-cli-uart gdb --app-config $(current_dir)/App.toml

disassemble:
	arm-none-eabi-readelf -l build/kernel.elf > build/headers.disass
	arm-none-eabi-objdump -h build/kernel.elf > build/sections.disass
	arm-none-eabi-objdump -s -j .data build/kernel.elf > build/data.disass
	arm-none-eabi-objdump -s -j .rodata build/kernel.elf > build/rodata.disass
	arm-none-eabi-objdump -d build/kernel.elf --visualize-jumps > build/text.asm

dump:
	arm-none-eabi-objcopy -O binary --only-section=.text build/kernel.elf build/image.text
	arm-none-eabi-objcopy -O binary --only-section=.rodata build/kernel.elf build/image.rodata
	arm-none-eabi-objcopy -O binary --only-section=.data build/kernel.elf build/image.data

#flash-erase:
#	openocd -f interface/stlink.cfg -f target/stm32f3x.cfg -c "init" -c "halt" -c "wait_halt" -c "stm32f3x mass_erase 0" -c "exit"

#flash:
#	openocd -f interface/stlink.cfg -f target/stm32f3x.cfg -c "program App.elf.ihex verify reset exit"

# itmdump -f /tmp/itm.fifo -s 0
# gdb-multiarch
# mkfifo /tmp/itm.fifo
# make connect