# ConceptOS
This repository contains the code of `ConceptOS`, a micro-kernel-based operating system for embedded devices based on [Hubris](https://hubris.oxide.computer/) and completely written in Rust. It was the product of my master's Thesis.

It's capable to allow OTA directly in NAND-type flash memory on the same bank from which the code is executing, without dedicated hardware support. Only a Flash IAP is required.

<p align="center">
   <img height="200px" src="docs/images/concept-os-logo.svg" >
</p>


It supports the following features:
- `Memory-isolated components`, hardware-enforced via Memory Protection Unit (MPU): a defective component cannot compromise the others.
- `Live update of components`: no reboot is needed and no specific time window/system state is required in order to apply the update.
- `Background updates`: the update is performed without disturbing the execution of the device’s main functionality.
- `State transfer between versions`: the old version of a component can transfer part/all of its state to the newer version.
- `A custom memory layout`, enforced by algorithms shipped with the device, that provides fragmentation control. No external service is needed during an update.
- `The ability to accept or reject an update`, based on a check of the new component’s dependencies performed on-board.
- `An automatic rollback procedure` at component-level, that must cover every possible update defect and restore the system’s integrity.

**Note: it's not production-ready.**

## Repository Organization
The repository is organized as follows:
- `analysis` contains code and data used to perform comparisons with legacy methods. Depending on the branch, can contain code used to generate graphs. In particular:
  - `analysis/hubris` is a fork of hubris OS.
  - `analysis/humility` is a fork of hubris' debugger (modified to support the target board).
- `app` depending on the branch, contains the code of the application being developed. It's possible to build/flash/debug the system using the provided Makefile. Depending on the branch, might contain also the measurements collected during the experiments.
- `boards` contains the board-specific configurations and code.
- `components` contains all the components used to develop the applications. Each component has its API code under `components/<component_name>/api` and its source code under `components/<component_name>/core`.
- `docs` contains useful reference documentation to better understand the design choices taken.
- `libs` is a series of `#[no_std]` libraries shared by components and/or toolchain.
- `sys` is the core of the system. Under `sys/kern` is provided the code of the kernel, the ABI under `sys/abi` and the user library (used by components) is under `sys/userlib`.
- `toolchain` contains all the modules and code related to build/debug/update the system.
- `utils` depending on the branch, contains additional modules/code to support the operations.

There are four main branches:
- `main` is the default branch, providing the minimum functionality of the system.
- `bthermo` contains the code of the thermostat appliance used for the thesis/article.
- `bthermo-resources` is related to resource-consumption evaluation.
- `bthermo-performance` is related to performance-consumption evaluation.

## Running the System
The toolchain is intended to work on Linux (in particular Ubuntu). In order to run the system, first build the toolchain using
```bash
   cd toolchain
   make build
```
Then use the Makefile provided with each app.
```bash
   cd app/<app>
   make build
   make flash
```

*It's possible to exploit docker to build and flash the system, but the support is experimental and might require a linux virtual machine*.