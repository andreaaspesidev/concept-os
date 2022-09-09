# Component Identifiers
At the moment, the system identifies components directly using a 16-bit integer, called `Component ID`. This is the same method internally used by Hubris kernel itself, but then Hubris supports a build-time ID-resolution/assignment procedure to avoid hard-coding ID to components.

For the sake of simplicity, Concept-OS uses directly IDs.

Below the fixed list of assignments for standard servers.

# Component ID Map

  Component ID | Component Name | Standard Priority | Description
  -------------|----------------|------------------ |---------------------------
  0 | Kernel | - | This identifier is assigned to the kernel itself.
  1 | Supervisor | - | This component is responsible for rebooting components when a crash occurs.
  2 | RCC | 0 (highest) | This component is in charge of managing clocks and resets of peripherals
  3 | UART | 1 | This component is in charge of supplying a UART channel via USB, used by the updater
  4 | STORAGE | 2 | This component contains all the procedures needed to manage flash and ram allocations.
  5 | UPDATER | 10 | This component is responsible for the update capability of the system

  < 2^10 -1 = 1023