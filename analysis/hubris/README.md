# Hubris
This folder is forked from the original hubris repository (https://github.com/oxidecomputer/hubris).

## How to flash
First compile the Humility debugger (forked from this repository)
```bash
    cd ../humility
    cargo build --release
```
then add it to path
```bash
    cd ../hubris
    export PATH="$(dirname -- "$(readlink -f -- "../humility/target/release/humility")"):$PATH"
```

## Before flashing notes
When using the dual-bank update, to save yourself a headache, check the active bank is Bank1,
by reading the option bytes with:
```bash
openocd -f interface/stlink.cfg -f target/stm32l4x.cfg -c "init" -c "stm32l4x option_read 0 0x20"
```

`0xfffff8aa` -> second bank active
`0xffeff8aa` -> first bank active

As the system will always flash in the first bank, be sure to write the option bytes. See more [here](https://stackoverflow.com/questions/48927028/openocd-how-to-write-option-bytes-to-stm32f4). Or just erase all memory with
```bash
openocd -f interface/stlink.cfg -f target/stm32l4x.cfg -c "init" -c "halt" -c "stm32l4x mass_erase 0" -c "exit"
```
Then proceed flashing the new image, and force `optr.bfb2 = 0` via code.

