# Serial Channel
Multiple components might require to access the same channel at the same time.
This is not a problem if the communication is only one-way, but it becomes a problem once this is no longer true.
In order to have bidirectional communication with multiple components apps, we have to mux data on the same physical channel.

This happens for example when we have the update client running in parallel with an application. Both might need to access for instance a serial port, as it could be connected to a remote Bluetooth endpoint.

Many implementations can be done, and this has nothing to do with the kernel itself, as no specific support is needed. In the following sections, it's presented a very simple methods to share a Serial Channel between multiple components, but with a maximum number of components at the same time.

The idea is to append in both direction a header to the message:

```
----------------------------------
| MSG_BYTE_0 |  ... | MSG_BYTE_N |
----------------------------------
```
The `N` number of bytes is always known both when a component asks to receive data and of course when it tries to write it.

The message then will be sent in the channel as:
```
-----------------------------------------------------------
| COMPONENT_ID | MSG_LEN | MSG_BYTE_0 |  ... | MSG_BYTE_N |
-----------------------------------------------------------
```
Where:
- `COMPONENT_ID` is a `16-bits` encoded unsigned integer (big endian).
- `MSG_LEN` is a `16-bits` encoded unsigned integer (big endian).
