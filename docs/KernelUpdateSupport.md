# Kernel Update Support
Currently, Hubris kernel is build by generating at compile time the memory structures 
needed for it to work correctly. This is no longer possible if we enable system update.

We must be able not only to reconstruct this state, but to keep it synchronized whenever
we add/update/remove a task.

Let's start by an implementation analysis of the current structures

## Static ReadOnly Structures
We have a bunch of arrays involved in the process:
- `HUBRIS_TASK_DESCS` is an array of exactly `HUBRIS_TASK_COUNT` `TaskDesc`. This structure is generated at build time, with a new entry for each task. In particular:
    - `entry_point`: Address of the task's entry point. 
    This is the first instruction that will be executed whenever the task is (re)started. It must be within one of the task's memory regions (the kernel *will* check this).
    - `initial_stack`: Address of the task's initial stack pointer, to be loaded at (re)start. 
    It must be pointing into or *just past* one of the task's memory regions (the kernel *will* check this).
    - `priority`: Initial priority of this task.
    - `flags`: Collection of boolean flags controlling task behavior.
    - `index`: Index in the task table of this task. Used to check by the kernel. (**will be removed**)
    - `regions`: Array of 8 indexes pointing to another structure where regions are defined.

    Regarding the available data, we can currently deduce from the HBF/Allocation all the fields, except for index.

    ```rust
        static HUBRIS_TASK_DESCS: [abi::TaskDesc; HUBRIS_TASK_COUNT] = [
            abi::TaskDesc {
                regions: [
                    7,
                    8,
                    2,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                entry_point: 0x08005001,
                initial_stack: 0x20000f80,
                priority: 0,
                index: 0,
                flags: unsafe { abi::TaskFlags::from_bits_unchecked(1) },
            },
            abi::TaskDesc {
                regions: [
                    9,
                    10,
                    1,
                    3,
                    4,
                    0,
                    0,
                    0,
                ],
                entry_point: 0x08006001,
                initial_stack: 0x20002380,
                priority: 1,
                index: 1,
                flags: unsafe { abi::TaskFlags::from_bits_unchecked(1) },
            },
            abi::TaskDesc {
                regions: [
                    11,
                    12,
                    5,
                    6,
                    0,
                    0,
                    0,
                    0,
                ],
                entry_point: 0x08008001,
                initial_stack: 0x20006000,
                priority: 1,
                index: 2,
                flags: unsafe { abi::TaskFlags::from_bits_unchecked(1) },
            },
            abi::TaskDesc {
                regions: [
                    13,
                    14,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                entry_point: 0x08010001,
                initial_stack: 0x20001800,
                priority: 2,
                index: 3,
                flags: unsafe { abi::TaskFlags::from_bits_unchecked(1) },
            },
            abi::TaskDesc {
                regions: [
                    15,
                    16,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                entry_point: 0x08004e81,
                initial_stack: 0x20002900,
                priority: 5,
                index: 4,
                flags: unsafe { abi::TaskFlags::from_bits_unchecked(1) },
            },
        ];
    ```

- `HUBRIS_REGION_DESCS` is an array of n regions. Each region `RegionDesc` has the following fields:
    - `base`: Address of start of region. The platform likely has alignment requirements for this; it must meet them. (For example, on ARMv7-M, it must be naturally aligned for the size.)
    - `size`: Size of region, in bytes. The platform likely has alignment requirements for this; it must meet them. (For example, on ARMv7-M, it must be a power of two greater than 16.)
    - `attributes`: Flags describing what can be done with this region.

    This array can be virtually constructed iterating over the HBFs. Fortunately, is not the one actually
    stored and used in sRAM.

    ```rust
    static HUBRIS_REGION_DESCS: [abi::RegionDesc; 17] = [
        abi::RegionDesc {
            base: 0x00000000,
            size: 0x00000020,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(0) },
        },
        abi::RegionDesc {
            base: 0x40004400,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x40021000,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x48000000,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x40020000,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x40022000,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x08040000,
            size: 0x00040000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(11) },
        },
        abi::RegionDesc {
            base: 0x08005000,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(5) },
        },
        abi::RegionDesc {
            base: 0x20000c00,
            size: 0x00000400,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(7) },
        },
        abi::RegionDesc {
            base: 0x08006000,
            size: 0x00002000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(5) },
        },
        abi::RegionDesc {
            base: 0x20002000,
            size: 0x00000800,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(7) },
        },
        abi::RegionDesc {
            base: 0x08008000,
            size: 0x00008000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(5) },
        },
        abi::RegionDesc {
            base: 0x20004000,
            size: 0x00004000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(7) },
        },
        abi::RegionDesc {
            base: 0x08010000,
            size: 0x00004000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(5) },
        },
        abi::RegionDesc {
            base: 0x20001000,
            size: 0x00001000,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(7) },
        },
        abi::RegionDesc {
            base: 0x08004e80,
            size: 0x00000080,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(5) },
        },
        abi::RegionDesc {
            base: 0x20002800,
            size: 0x00000100,
            attributes: unsafe { abi::RegionAttributes::from_bits_unchecked(7) },
        },
    ];
    ```
- `HUBRIS_TASK_IRQ_LOOKUP`: maps tasks to IQR number
    ```rust
        pub const HUBRIS_TASK_IRQ_LOOKUP: PerfectHashMap::<abi::InterruptOwner, &'static [abi::InterruptNum]> = PerfectHashMap {
                m: 0x5a125382,
                values: &[
                    (abi::InterruptOwner { task: 1, notification: 0b10 }, &[abi::InterruptNum(16)]),
                    (abi::InterruptOwner { task: 1, notification: 0b1 }, &[abi::InterruptNum(38)]),
                ],
            };
    ```
- `HUBRIS_IRQ_TASK_LOOKUP`: maps IRQ to tasks
    ```rust
        pub const HUBRIS_IRQ_TASK_LOOKUP: PerfectHashMap::<abi::InterruptNum, abi::InterruptOwner> = PerfectHashMap {
            m: 0xda497a16,
            values: &[
                (abi::InterruptNum::invalid(), abi::InterruptOwner::invalid()),
                (abi::InterruptNum(38), abi::InterruptOwner { task: 1, notification: 0b1 }),
                (abi::InterruptNum(16), abi::InterruptOwner { task: 1, notification: 0b10 }),
            ],
        };
    ```

## Static Mutable Structures
These are the actual structures loaded in SRAM of the kernel, and used during kernel operations.
The kernel populates there during start-up procedure, by reading `HUBRIS_TASK_DESCS` and `HUBRIS_REGION_DESCS` from flash.

We have two structures:
- `HUBRIS_TASK_TABLE_SPACE`: an array of `HUBRIS_TASK_COUNT` un-init elements, each with enough space to store a `Task` struct.
    - Maintain a pointer to the read-only entry TaskDesk.
    - Maintain a pointer to an array of pointers of RegionDesc
- `HUBRIS_REGION_TABLE_SPACE`: an array of `HUBRIS_TASK_COUNT` elements. Each of these elements is another array of 8 elements, containing pointers to `RegionDesc`, presumably in flash memory.

## Needed Changes
1. The first thing we must change is the size of these structures. The downside is that now we have to reserve memory also for tasks we have not in the system, but we could have. So there is a maximum number of tasks, fixed at the first deploy of the system.

...