# Kernel Update Support
Currently, Hubris kernel is build by generating at compile time the memory structures 
needed for it to work correctly. This is no longer possible if we enable system update.

We must be able not only to reconstruct this state, but to keep it synchronized whenever
we add/update/remove a task.

Let's start by an implementation analysis of the current structures

## Legacy Structures

### Read-Only Structures (saved in Flash)
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
            ...
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
        ...
    ];
    ```
- `HUBRIS_TASK_IRQ_LOOKUP`: maps tasks to IRQ number
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

### Mutable Structures (saved in SRAM)
These are the actual structures loaded in SRAM of the kernel, and used during kernel operations.
The kernel populates there during start-up procedure, by reading `HUBRIS_TASK_DESCS` and `HUBRIS_REGION_DESCS` from flash.

We have two structures:
- `HUBRIS_TASK_TABLE_SPACE`: an array of `HUBRIS_TASK_COUNT` un-init elements, each with enough space to store a `Task` struct.
    - Maintain a pointer to the read-only entry TaskDesk.
    - Maintain a pointer to an array of pointers of RegionDesc
- `HUBRIS_REGION_TABLE_SPACE`: an array of `HUBRIS_TASK_COUNT` elements. Each of these elements is another array of 8 elements, containing pointers to `RegionDesc`, presumably in flash memory.

## New Structures
In this new edition, the read-only structures are distributed in the HBF of the various components, and we can find only the necessary data:
- `HUBRIS_TASK_DESCS` as it is, but now we must pre-allocate all the space, putting a maximum number of components the system could have in its lifetime. This datum is fixed in the ABI. This number will be called `HUBRIS_MAX_SUPPORTED_TASKS`.
- `HUBRIS_REGION_DESCS` theoretically, but it's not wise as we would introduce a further upper limit. As regions comes with HBF descriptors, we can work around all original usages of this old structure.
- `HUBRIS_TASK_IRQ_LOOKUP`: can be derived from HBF, but it's no use in practice, as in the IRQ handle the following is used.
- `HUBRIS_IRQ_TASK_LOOKUP`: we know the IRQs with the associated masks for each component, in the HBFs. Also in this case we have to pre-allocate memory, but in this case we even know the maximum limit on the IRQs of the system (an IRQ can be managed by a single component currently). As it could be wise to lower this limit, another ABI constraint is set: `HUBRIS_MAX_IRQS`.

A big difference is that in the original version tasks are not identified by the kernel using their IDs, but using their index in the `HUBRIS_TASK_DESCS` table. This is needed to fast access them during the syscalls. Now, due to the volatile nature of components, even during their life, they can assume at least two IDs. 

For this reason, we identify -from now on- components using their ID. To obtain still a fast access, an IndexMap (hash map) is used, with key the ID itself. A more simple solution where the ID is the index could still be used, implementing an ad-hoc structure now would be too time-consuming.

We then have two mutable structures:
- `TASK_MAP`: given the component id as key, returns its structure `Task`.
- `IRQ_TO_TASK`: given the irq number as key, returns its owner (component id + notification mask)

## Component Identifiers
Components need to identify themselves in order to correctly communicate. Currently, differently from the original Hubris implementation, IDs are fixed during system development and not resolved "dynamically". To ease the developer, the API of each component contains also the component ID.

As in original Hubris, during runtime not all the 16-bits of the ID are used as they are. Currently, only the lower 10-bits are the original one, while the upper 6-bits identify the component generation.

### Component Generation
Each component starts with a generation number of 0. This number is incremented each time the component crashes, and it's restarted by the supervisor component.

Generation is important to avoid dangling calls, where a caller is about to invoke a component that is actually crashed: some mutable state is lost, without that the caller can possibly know - without the generation number. Each of these calls is faulted, and the caller must explicitely issue a syscall to get the new generation number for that component.

### Component ID Evolution
Mature components will always be identified by the ID contained in their HBF descriptor (called **Nominal ID**), but a young component, introduced when the system is still live, will obtain the fixed ID `1023`.
*With `1024` the maximum theoretically supported number of components of the system (2^10-1).*

After the initial setup (state transfer), the nominal ID is assigned, after the other eventual component active with that ID is terminated.

The start-up procedure works this way:
- Flash memory is scanned searching for components
- Each component HBF is used to populate the two structures.
- Those components are initialized as mature, so they get their nominal ID, with generation 0.

When a new component is saved into the system, a kipc call `load_component` is invoked by the updater component pointing to that component's block:
1. If the system contains only mature components (i.e. do not exist a component with ID `1023`), the kipc call can proceed, otherwise it's rejected.
2. The new component is added to `TASK_MAP` using the ID `1023`. It immediately has full access to hardware (regions and IRQs), and can be scheduled. It has 30000 kernel ticks to issue the kipc call `activate`, or will be deleted and the eventual old component version re-executed.
3. If a component with that nominal ID (the old version) exists in the system:
    1. If the old task specified a handler for the state transfer, then it's set as new PC for the execution. If not, continue to 4.
    3. If the new component is willing to accept the state transfer, can get information on its availability with the kipc `get_state_availability`.
    4. The new component can wait for a state transfer using the standard `recv` syscall. It may schedule a timer in order to protect itself from a faulty old component.
4. In order to get its nominal ID, the component issue a kipc call `activate`: regardless of the state of the old component, the old component -if exists- get terminated, and the ID is assigned to the new one.

### State Transfer
In order to allow state transfer, two conditions must be satisfied:
- The old component before the update should have registered a handler using the kipc `set_update_handler`. In this callback, it should perform a `send` syscall to ID `1023`.
- The new component must put itself into `recv` syscall, and read state before calling kipc `activate`.

The old component can refuse a state transfer by avoiding registering a handler, and the new component can just call immediately kipc `activate` if it's not interested in the old state.
*Calling `activate` from a mature component has in fact no effect*.