//! CANopen basic data types and their little-endian value encoding.
//!
//! CANopen is little-endian on the wire (CiA 301 §7.1). This module models the
//! numeric basic data types plus the variable-length ones — `VISIBLE_STRING`,
//! `OCTET_STRING`, and `DOMAIN`, carried by a bounded [`ByteString`] — and
//! converts between typed [`Value`]s and their byte encoding. Variable-length
//! values (over four bytes) travel by segmented SDO transfer.
//!
//! ```
//! use canopen_std::{DataType, Value};
//!
//! let mut buf = [0u8; 8];
//! let n = Value::Unsigned32(0xDEAD_BEEF).encode_le(&mut buf).unwrap();
//! assert_eq!(&buf[..n], &[0xEF, 0xBE, 0xAD, 0xDE]); // little-endian
//!
//! let value = Value::decode_le(DataType::Unsigned32, &buf[..n]).unwrap();
//! assert_eq!(value, Value::Unsigned32(0xDEAD_BEEF));
//! ```

use crate::Error;

/// The maximum length, in bytes, of a variable-length value (`VISIBLE_STRING`,
/// `OCTET_STRING`, `DOMAIN`).
///
/// Chosen to keep [`Value`] `Copy` and reasonably small while covering the
/// typical string objects (device name `0x1008`, versions `0x1009` / `0x100A`).
/// It also bounds the SDO segmented-transfer buffers, so it is the largest
/// object value the sans-I/O SDO server and client will transfer.
pub const MAX_STRING_LEN: usize = 32;

/// A bounded, `Copy` byte string backing the variable-length CANopen types
/// (`VISIBLE_STRING`, `OCTET_STRING`, `DOMAIN`): up to [`MAX_STRING_LEN`] bytes.
#[derive(Clone, Copy)]
pub struct ByteString {
    data: [u8; MAX_STRING_LEN],
    len: u8,
}

impl ByteString {
    /// An empty byte string.
    pub const fn new() -> Self {
        Self {
            data: [0; MAX_STRING_LEN],
            len: 0,
        }
    }

    /// Build from a byte slice, returning [`Error::BadLength`] if it is longer
    /// than [`MAX_STRING_LEN`].
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > MAX_STRING_LEN {
            return Err(Error::BadLength);
        }
        let mut data = [0u8; MAX_STRING_LEN];
        data[..bytes.len()].copy_from_slice(bytes);
        Ok(Self {
            data,
            len: bytes.len() as u8,
        })
    }

    /// Build from a string slice (a `VISIBLE_STRING`), returning
    /// [`Error::BadLength`] if it is longer than [`MAX_STRING_LEN`].
    // Inherent (fallible, bounded) constructor, not the `FromStr` trait.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, Error> {
        Self::from_bytes(s.as_bytes())
    }

    /// The bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }

    /// Interpret the bytes as UTF-8 text, if valid.
    pub fn as_str(&self) -> Option<&str> {
        core::str::from_utf8(self.as_bytes()).ok()
    }

    /// The length in bytes.
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Whether the string is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Default for ByteString {
    fn default() -> Self {
        Self::new()
    }
}

// Equality and Debug compare/show only the meaningful bytes, ignoring the
// unused tail of the fixed buffer.
impl PartialEq for ByteString {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for ByteString {}

impl core::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.as_str() {
            Some(s) => write!(f, "ByteString({s:?})"),
            None => write!(f, "ByteString({:?})", self.as_bytes()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DataType, Value};

    #[test]
    fn sizes_match_spec() {
        assert_eq!(DataType::Boolean.size(), 1);
        assert_eq!(DataType::Unsigned16.size(), 2);
        assert_eq!(DataType::Unsigned32.size(), 4);
        assert_eq!(DataType::Real64.size(), 8);
    }

    #[test]
    fn data_type_index_roundtrips() {
        for dt in [
            DataType::Boolean,
            DataType::Unsigned32,
            DataType::Real64,
            DataType::Integer64,
            DataType::Unsigned64,
            DataType::VisibleString,
            DataType::OctetString,
            DataType::Domain,
        ] {
            assert_eq!(DataType::from_index(dt.index()), Some(dt));
        }
        assert_eq!(DataType::Unsigned32.index(), 0x0007);
        assert_eq!(DataType::VisibleString.index(), 0x0009);
        // A genuinely unmodelled type (e.g. TIME_OF_DAY 0x0C) is rejected.
        assert_eq!(DataType::from_index(0x0C), None);
    }

    #[test]
    fn variable_length_types_have_no_fixed_size() {
        for dt in [
            DataType::VisibleString,
            DataType::OctetString,
            DataType::Domain,
        ] {
            assert!(dt.is_variable());
            assert_eq!(dt.fixed_size(), None);
            assert_eq!(dt.size(), 0);
        }
        assert!(!DataType::Unsigned32.is_variable());
        assert_eq!(DataType::Unsigned32.fixed_size(), Some(4));
    }

    #[test]
    fn string_values_round_trip() {
        let mut buf = [0u8; MAX_STRING_LEN];
        for v in [
            Value::VisibleString(ByteString::from_str("canopen-rs").unwrap()),
            Value::OctetString(ByteString::from_bytes(&[0, 1, 2, 0xFF]).unwrap()),
            Value::Domain(ByteString::from_bytes(&[]).unwrap()), // empty
        ] {
            let n = v.encode_le(&mut buf).unwrap();
            assert_eq!(n, v.size());
            let back = Value::decode_le(v.data_type(), &buf[..n]).unwrap();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn bytestring_rejects_oversized_input() {
        let too_long = [0u8; MAX_STRING_LEN + 1];
        assert!(matches!(
            ByteString::from_bytes(&too_long),
            Err(Error::BadLength)
        ));
        assert!(matches!(
            Value::decode_le(DataType::VisibleString, &too_long),
            Err(Error::BadLength)
        ));
    }

    #[test]
    fn bytestring_equality_ignores_unused_tail() {
        let a = ByteString::from_str("hi").unwrap();
        let b = ByteString::from_bytes(b"hi").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.as_str(), Some("hi"));
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn u32_is_little_endian() {
        let mut buf = [0u8; 8];
        let n = Value::Unsigned32(0x1234_5678).encode_le(&mut buf).unwrap();
        assert_eq!(n, 4);
        assert_eq!(&buf[..4], &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn decode_roundtrips() {
        let cases = [
            Value::Boolean(true),
            Value::Integer8(-5),
            Value::Unsigned16(0xBEEF),
            Value::Integer32(-123456),
            Value::Unsigned32(0xDEAD_BEEF),
            Value::Unsigned64(0x0102_0304_0506_0708),
        ];
        let mut buf = [0u8; 8];
        for v in cases {
            let n = v.encode_le(&mut buf).unwrap();
            let back = Value::decode_le(v.data_type(), &buf[..n]).unwrap();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn encode_rejects_short_buffer() {
        let mut buf = [0u8; 2];
        assert!(matches!(
            Value::Unsigned32(0).encode_le(&mut buf),
            Err(Error::BadLength)
        ));
    }

    #[test]
    fn decode_rejects_wrong_length() {
        assert!(matches!(
            Value::decode_le(DataType::Unsigned32, &[0, 0]),
            Err(Error::BadLength)
        ));
    }
}
