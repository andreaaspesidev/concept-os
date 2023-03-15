# Component Identifiers
At the moment, the system identifies components directly using a 16-bit integer, called `Component ID`. This is the same method internally used by Hubris kernel itself, but then Hubris supports a build-time ID-resolution/assignment procedure to avoid hard-coding ID to components.

For the sake of simplicity, Concept-OS uses directly IDs.

Below the fixed list of assignments for the standard components.

# Component ID Map

  Component ID | Component Name | Standard Priority | Description
  -------------|----------------|------------------ |---------------------------
  0 | Supervisor | - | This component is responsible for rebooting components when a crash occurs.
  2 | RCC | 0 (highest) | This component is in charge of managing clocks and resets of peripherals
  3 | UART | 15 | This component is in charge of supplying a UART channel, used by the updater and eventually the application
  4 | STORAGE | 25 | This component contains all the procedures needed to manage flash and ram allocations.
  5 | UPDATE | 30 | This component is responsible for the update capability of the system

The ID must be < 2^10 -1 = 1023