# Components
Each component that composes the system is described by a TOML file.
This file must contain all the information that cannot be inferred from the ELF file of the component.

In particular, the information needed are:
- `Component ID`: Integer 1-65535 indicating the ID of the component. 0 is reserved to the kernel
- `Component Version`: Integer 0-65535 indicating the component major version, for compatibility checks
- `Component Priority`: Component scheduled priority 0-255
- `Component Flags`: Flags associated to the component.
    - `START_AT_BOOT`: if the component should start at boot
- `Component Min RAM`: Minumum SRAM needed by the component (stack)
- (optional) `Regions`: list of memory regions required by the component
- (optional) `Interrupts`: list of irq and masks required by the component

## Regions
For each region, the following values must be specified:
- `Region Base Address`: Address of start of region. Must be a multiple of the Region Size due to limitations of the MPU (natural alignment).
- `Region Size`: Size of region, in bytes (on ARMv7-M, it must be a power of two greater than 16: 2^5, 2^6, ...)
- `Region Attributes`: 
    - `READ`: Region can be read by components that include it.
    - `WRITE`: Region can be written by components that include it.
    - `EXECUTE`: Region can contain executable code for components that include it.
    - `DEVICE`: Region contains memory mapped registers. This affects cache behavior on devices that include it, and discourages the kernel from using memcpy in the region.
    - `DMA`: Region can be used for DMA or communication with other processors. This heavily restricts how this memory can be cached and will hurt performance if overused. This is ignored for DEVICE memory, which is already not cached.

## Interrupts
For each interrupt, the following values must be specified:
- `IRQ Number`: Interrupt number
- `Notification Mask`: Interrupt notification to the component

## Dependencies
Each component dependency is inserted here to ensure the component will work in the target
system. Three fields are specified:
- `Component ID` is the identifier of the component
- `Min Version` is the min version of the component (use 0 to disable)
- `Max Version` is the max version of the component (use 0 to disable)

## Example
```toml
# This an example of a component configuration
# file generated during component compilation

[component]
id = 1
version = 1
priority = 1
flags = []
min_ram = 1024

[[regions]]
base_address = '0x08000000'
size = '0x1000'
attributes = [
    'READ',
    'WRITE',
]

[[regions]]
base_address = '0x08001000'
size = '0x2000'
attributes = ['DMA']

[[interrupts]]
irq = 1
notification_mask = '0x00000001'

[[interrupts]]
irq = 2
notification_mask = '0x00000002'

```

Note: a more advanced version can be used now for components, by using the peripheral keyword
to make the component more board-independent and allow the support of multiple boards. 
Here is an example:

```toml
[component]
id = 3
version = 1
priority = 10
flags = ['START_AT_BOOT']
min_ram = 1024
peripherals = ["usart2","gpioa","dma1"]
interrupts = { "usart2.irq" = 1, "dma1.irq6" = 2 }


# RCC
[[dependencies]]
component_id = 2
min_version = 1
max_version = 1
```