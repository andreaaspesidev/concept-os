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