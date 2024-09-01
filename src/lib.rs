//! This crate provides functions to create human readable hex
//! dumps of binary data.
//!
#![no_std]
const HEX_DIGIT: &[u8; 16] = b"0123456789abcdef";

extern crate alloc;
use alloc::{
    string::String,
    vec::Vec,
};



/// Write a simple hex dump of the given data to the given target.
/// The dump contains pairs of hex digits, separated by spaces, no
/// line breaks, decorations, etc.
///
/// # Examples
/// ```
/// let data = [0x00, 0x01, 0x02, 0x03];
/// let mut target = Vec::new();
/// qdhex::write_bare_dump_to_vec(&data, &mut target);
/// assert_eq!(target, b"00 01 02 03");
/// ```
pub fn write_bare_dump_to_vec(data: &[u8], target: &mut Vec<u8>) {
    for byte in data {
        target.push(HEX_DIGIT[(byte >> 4) as usize]);
        target.push(HEX_DIGIT[(byte & 0x0f) as usize]);
        target.push(b' ');
    }

    target.pop();
}

/// Create a simple hex dump of the given data. The dump contains pairs of
/// hex digits, separated by spaces, no line breaks, decorations, etc.
///
/// # Examples
/// ```
/// let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
/// let target = qdhex::bare_dump(&data);
/// assert_eq!(target, b"00 01 02 03 04 05");
/// ```
pub fn bare_dump(data: &[u8]) -> Vec<u8> {
    let mut target = Vec::with_capacity(data.len() * 3 + 1);
    write_bare_dump_to_vec(data, &mut target);
    target
}

/// Create a simple hex dump of the given data. The dump contains pairs of
/// hex digits, separated by spaces, no line breaks, decorations, etc.
///
/// # Examples
/// ```
/// let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
/// let target = qdhex::bare_dump_string(&data);
/// assert_eq!(target, "00 01 02 03 04 05");
/// ```
///
pub fn bare_dump_string(data: &[u8]) -> String {
    let vec = bare_dump(data);
    // SAFETY: The dump is always valid UTF-8 since it only contains ASCII characters
    unsafe { String::from_utf8_unchecked(vec) }
}

/// Write a formatted multi-line hex dump of the given data to the given target.
/// Dump lines are prefixed with the given offset.  Each line contains up to 16
/// bytes, separated by spaces, followed by a space and the ASCII representation.
///
/// # Examples
/// ```
/// let data = &b"baadfood\xba\xad\xf0\x0dASDFasdf;lkj."[..];
/// let mut target = Vec::new();
/// qdhex::write_formatted_dump_to_vec(0x1000, &data, &mut target);
/// assert_eq!(target, br"1000 62 61 61 64 66 6f 6f 64 ba ad f0 0d 41 53 44 46 baadfood....ASDF
/// 1010 61 73 64 66 3b 6c 6b 6a 2e                      asdf;lkj.
/// ");
/// ```
pub fn write_formatted_dump_to_vec(offset: u32, data: &[u8], target: &mut Vec<u8>) {
    let mut line_offset = offset;

    for chunk in data.chunks(16) {
        // Write the line offset
        for i in 0..4 {
            target.push(HEX_DIGIT[((line_offset >> (4 * (3 - i))) & 0x0f) as usize]);
        }
        target.push(b' ');

        // Write the hex representation
        write_bare_dump_to_vec(chunk, target);

        // Pad the last line with spaces
        for _ in chunk.len()..16 {
            target.push(b' ');
            target.push(b' ');
            target.push(b' ');
        }

        // Write the ASCII representation
        target.push(b' ');
        for byte in chunk {
            if *byte >= 0x20 && *byte <= 0x7e {
                target.push(*byte);
            } else {
                target.push(b'.');
            }
        }

        target.push(b'\n');

        line_offset += 16;
    }
}

/// Create a formatted multi-line hex dump of the given data.
/// Dump lines are prefixed with the given offset.  Each line contains up to 16
/// bytes, separated by spaces, followed by a space and the ASCII representation.
///
/// # Examples
/// ```
/// let data = &b"baadfood\xba\xad\xf0\x0dASDFasdf;lkj."[..];
/// let target = qdhex::formatted_dump(0x1000, &data);
/// assert_eq!(target, br"1000 62 61 61 64 66 6f 6f 64 ba ad f0 0d 41 53 44 46 baadfood....ASDF
/// 1010 61 73 64 66 3b 6c 6b 6a 2e                      asdf;lkj.
/// ");
/// ```
pub fn formatted_dump(offset: u32, data: &[u8]) -> Vec<u8> {
    let lines = (data.len() + 15) / 16;
    let size = lines * 70;
    let mut target = Vec::with_capacity(size);
    write_formatted_dump_to_vec(offset, data, &mut target);
    target
}

/// Create a formatted multi-line hex dump of the given data.
/// Dump lines are prefixed with the given offset.  Each line contains up to 16
/// bytes, separated by spaces, followed by a space and the ASCII representation.
///
/// # Examples
/// ```
/// let data = &b"baadfood\xba\xad\xf0\x0dASDFasdf;lkj."[..];
/// let target = qdhex::formatted_dump_string(0x1000, &data);
/// assert_eq!(target, "1000 62 61 61 64 66 6f 6f 64 ba ad f0 0d 41 53 44 46 baadfood....ASDF\n1010 61 73 64 66 3b 6c 6b 6a 2e                      asdf;lkj.\n");
/// ```
pub fn formatted_dump_string(offset: u32, data: &[u8]) -> String {
    let vec = formatted_dump(offset, data);
    // SAFETY: The dump is always valid UTF-8 since it only contains ASCII characters
    unsafe { String::from_utf8_unchecked(vec) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bare_dump() {
        let data: [u8; 9] = [0x12, 0x34, 0x56, 0x78, 0xab, 0xcd, 0xef, 0xa0, 0x0b];
        let mut target = Vec::new();
        write_bare_dump_to_vec(&data, &mut target);
        assert_eq!(target, b"12 34 56 78 ab cd ef a0 0b");

        let data: &[u8] = &b"Hello, World!"[..];
        let mut target = Vec::new();
        write_bare_dump_to_vec(&data, &mut target);
        assert_eq!(target, b"48 65 6c 6c 6f 2c 20 57 6f 72 6c 64 21");

        // Test with empty data
        let data: &[u8] = &b""[..];
        let mut target = Vec::new();
        write_bare_dump_to_vec(&data, &mut target);
        assert_eq!(target, b"");

        // Test with one byte
        let data: &[u8] = &b"\xab"[..];
        let mut target = Vec::new();
        write_bare_dump_to_vec(&data, &mut target);
        assert_eq!(target, b"ab");

        // Verify that the target is not cleared
        let data: &[u8] = &b"\xab\x30"[..];
        let mut target = b"Hello, World!".to_vec();
        write_bare_dump_to_vec(&data, &mut target);
        assert_eq!(target, b"Hello, World!ab 30");
    }
}
