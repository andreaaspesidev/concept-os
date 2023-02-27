/**
 * Simple implementation of CRC-8-Dallas/Maxim.
 * Maybe in the future we can exploit hardware CRC unit to offload CPU.
 */

/// Optimized Dallas (now Maxim) iButton 8-bit CRC calculation.
/// Polynomial: x^8 + x^5 + x^4 + 1 (0x8C)
/// Initial value: 0x0
pub fn crc8_update(crc: &mut u8, byte: u8) {
    let mut tmp = (*crc) ^ byte;
    for _ in 0..8u8 {
        if tmp & 0x01 == 1 {
            tmp = (tmp >> 1) ^ 0x8C;
        } else {
            tmp >>= 1;
        }
    }
    *crc = tmp;
}