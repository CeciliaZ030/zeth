use std::cmp::min;

pub fn string_to_bytes32(input: &[u8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let len = min(input.len(), 32);
    bytes[..len].copy_from_slice(&input[..len]);
    bytes
}