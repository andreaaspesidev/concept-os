# Boards
This directory contains a folder for each supported board.
Into each folder, there are files used during the build process (i.e. linker scripts, ...)
and code needed to access the devices of that board.

**If a reader wonders why I have cloned the corresponding `stm32fx` crate into each board, it's because of the need of getting rid of `DEVICE_PERIPHERALS` global not-mangled. It is never used in this OS, and also makes impossible to link together crates that have both a dependency on this library.**

Such code is released under the original license.

*At the same time, that library it's not expected to be updated once working for the used peripherals*.