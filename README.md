# ConceptOS
This repository contains the code of ConceptOS, a micro-kernel-based operating system for embedded devices based on [Hubris](https://hubris.oxide.computer/) and completely written in Rust. It was the product of my master's Thesis.

<div style="display:flex;flex-direction:column;height:200px;">
   <img src="docs/images/concept-os-logo.svg">
</div>

It's capable to allow OTA directly in NAND-type flash memory on the same bank from which the code is executing, without dedicated hardware support. Only a Flash IAP is required.

It supports the following features:
- **Memory-isolated components**, hardware-enforced via Memory Protection Unit (MPU): a defective component cannot compromise the others.
- **Live update of components**: no reboot is needed and no specific time window/system state is required in order to apply the update.
- **Background updates**: the update is performed without disturbing the execution of the device’s main functionality.
- **State transfer between versions**: the old version of a component can transfer part/all of its state to the newer version.
- **A custom memory layout**, enforced by algorithms shipped with the device, that provides fragmentation control. No external service is needed during an update.
- **The ability to accept or reject an update**, based on a check of the new component’s dependencies performed on-board.
- **An automatic rollback procedure** at component-level, that must cover every possible update defect and restore the system’s integrity.

**Note: it's not production-ready.**