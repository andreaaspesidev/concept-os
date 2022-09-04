# Update Process

In order to update the system, the single components must be updated one by one.
The update server is preinstalled into the default image of the system, along with the needed dependencies.
It's wired to listen to a bidirectional channel of communication, were waits for commands/data.

*Currently, the update process happens via UART. Will be tested also with a wireless channel in the future.*

## UART Channel

The update client is listening on the default UART connected to USB via the Serial-to-USB converter installed on the board.
We use a binary protocol as it's simple, very concise, and the communication is not intended for human interaction anyway. *Of course, this makes the protocol more difficult to be inspected during debug. A digital analyzer will be used for this purpose.*

**This is one of the many ways to design this task, and does not claim to be the smartest and/or most performing.**

In order to update a component, we need its HBF. This file contains almost everything, starting from the component executable code, the regions to be loaded, etc.

Before sending the HBF, some volatile data (metadata) is transmitted as handshake (**inspired from the TLS handshake**). 

This includes:

- The **authentication metadata** used to verify the identity of sender, and setup encrypted channel:
  1. Using the public key of the device, the sender signs the AES-128bit symmetric key that will be used for the next part of the transmission.
  2. The device validates the message using its private key. If the message is valid, then the key is saved and used to decrypt next packets. Otherwise, the communication channel is closed.
- The **update metadata**, containing the component dependencies with their max and min version.

*To tune the performance of the process, a different level of SRAM buffering can be selected.*

In order to update the device, the update client expect a public-key-signed message structured as:

  |   17    |       16        |...|       1        |    0    |
  |---------|-----------------|---|----------------|---------|
  |  CRC-8  | AES_KEY_BYTE_15 |...| AES_KEY_BYTE_0 | OP_TYPE |

were:

- `OP_TYPE` can be:
    - `0xCA`: Component Add/Update
    - `0xCB`: System Info
    - `0xCE`: Component Erase
- `CRC-8` is the Checksum byte computed using the simple algorithm `CRC-8-Dallas/Maxim`. *This field can be useful when we test more unreliable connections.*

To this message, if correct, the client responds with the hello packet:

  |   6   |    5    |  4  |  3  |  2  |  1  |  0  |
  |-------|---------|-----|-----|-----|-----|-----|
  | CRC-8 | OP_TYPE | 'H' | 'E' | 'L' | 'L' | 'O' |

This packet is encrypted with the provided AES key.

### Component Add/Update
In order to request the allocation space for the new component, the HBF header base + main must be stored in SRAM (**this two structures have fixed size**). See `toolchain/HubrisBinaryFormat.md` for details.

Communication from this point on is based on fixed size packets. The client responds only using single byte messages:

**Commands:**
- `0x01`: Send update metadata (ignored for now)
- `0x02`: Send component HBF header (base + main)
- `0x03`: Send component HBF remaining header (regions + interrupts + relocations)
- `0x04`: Send component HBF payload

**Errors:**
- `0xE1`: Failed on metadata check
- `0xE2`: Not enough space for the component.
- `0xE3`: Cannot start the component.
- `0xEF`: Update generic failure.

**Messages:**
- `0xFF`: Update success.

Each incoming packet is structured as follows:

  | <pkt_size> |   N    |...|   0   |
  |------------|--------|---|-------|
  |    CRC-8   | DATA_N |...| DATA0 |

The update client:

1. Asks for update metadata (`0x01`). Checks if they are satisfied by the currently installed components. If everything is okay, asks for header (`0x02`), otherwise responds with `0xE1`.
2. Expect the HBF header (base + main) to be transmitted. Validates the messages. If the validation fails (for CRC), asks back the header.
3. Request the system a new space allocation, large enough for this component needs. If this space cannot be found, fails with `0xE2`. Otherwise, starts copying the buffered header to flash. Then asks for the following data (`0x03`).
4. Simply validates the rest of the header, then writes it to flash. Multiple packets are accepted, as long as their encryption is okay. When the sender stops transmitting (we get all the bytes indicated in the HBF header), the client asks for the next part (`0x04`).
5. While saving the payload, the client reads the relocations contained in the header, and applies them when copying data from the buffer to the flash.
6. At the end of the process, the system is notified of the presence of the new component. If the component completes the start-up, then the client responds with update success (`0xFF`), otherwise `0xE3`.

A failure in any step leaves the system in the same condition before the beginning of the operation. *The allocated space will be marked as finalized only if every step is successful. An unexpected reboot will simply erase the block and the new component*.

For now, older versions of components are removed as soon as the new component is started. In the future, they could be kept as long as no additional free space is needed, and only then removed.

The kernel at boot will chose the newer version of each component to put in the task list.

### Component Erase
In order to remove a component, the only information needed is its component ID. Also, it's important not to remove the component if any dependencies are active.
*Currently this check is skipped, but will be introduced in the future when all this information will be moved into the HBF format itself.*

**Commands:**
- `0x01`: Send component ID
  Expect a packet structured as follows:
    |      6     |        5-2        |      1-0     |
    |------------|-------------------|--------------|
    |    CRC-8   | COMPONENT_VERSION | COMPONENT_ID |

    *All fields are encoded as Little-Endian*

**Errors:**
- `0xE1`: Cannot find the component
- `0xE2`: Cannot find the specified version
- `0xEF`: Update generic failure.

**Messages:**
- `0xFF`: Erase success.

### System Info
Returns a list of components of the system, with some associated status.

For each component, it will be returned the following structure:

  Offset| Size (bytes) | Field Name
  ------|--------------|-------------
  0x00  |      2       | COMPONENT_ID
  0x02  |      4       | COMPONENT_VERSION
  0x06  |      4       | ALLOCATED_FLASH
  0x0A  |      4       | ALLOCATED_RAM
  0x0E  |      2       | COMPONENT_STATUS
  0x10  |      1       | CRC-8

  Total size: 17 bytes

where:
- `STATUS` can be:
  | 15 |...| 7 | 6 | 5 | 4 | 3 | 2 | 1 |      0      |
  |----|---|---|---|---|---|---|---|---|-------------|
  | R  | R | R | R | R | R | R | R | R |  HBF_VALID  |

At the end of the components, the two bytes 0x0000 will be transmitted.
*In fact, no component with ID 0x0000 can exist: it's reserved for the kernel.*


# Notes
**At the moment, both the encryption and the compatibility check is missing in the first implementation.**