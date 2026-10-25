use std::fmt::Display;

use bitvec::{field::BitField, order::Msb0, slice::BitSlice};

use crate::Error;

use super::super::{ByteString, byte_string::MAX_STRING_LEN};
use super::DataType;

/// A typed CANopen value.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Value {
    /// A `BOOLEAN`.
    Boolean(bool),
    /// An `INTEGER8`.
    Integer8(i8),
    /// An `INTEGER16`.
    Integer16(i16),
    /// An `INTEGER32`.
    Integer32(i32),
    /// An `UNSIGNED8`.
    Unsigned8(u8),
    /// An `UNSIGNED16`.
    Unsigned16(u16),
    /// An `UNSIGNED32`.
    Unsigned32(u32),
    /// A `REAL32`.
    Real32(f32),
    /// A `REAL64`.
    Real64(f64),
    /// An `INTEGER64`.
    Integer64(i64),
    /// An `UNSIGNED64`.
    Unsigned64(u64),
    /// A `VISIBLE_STRING` (ASCII/UTF-8 text).
    VisibleString(ByteString),
    /// An `OCTET_STRING` (arbitrary bytes).
    OctetString(ByteString),
    /// A `DOMAIN` (arbitrary bytes).
    Domain(ByteString),
    /// Значение относительного идентификатора узла ($NODEID+id).
    RelativeNodeId(u16),
}

impl Value {
    /// The data type of this value.
    pub const fn data_type(&self) -> DataType {
        match self {
            Value::Boolean(_) => DataType::Boolean,
            Value::Integer8(_) => DataType::Integer8,
            Value::Integer16(_) => DataType::Integer16,
            Value::Integer32(_) => DataType::Integer32,
            Value::Unsigned8(_) => DataType::Unsigned8,
            Value::Unsigned16(_) => DataType::Unsigned16,
            Value::Unsigned32(_) => DataType::Unsigned32,
            Value::Real32(_) => DataType::Real32,
            Value::Real64(_) => DataType::Real64,
            Value::Integer64(_) => DataType::Integer64,
            Value::Unsigned64(_) => DataType::Unsigned64,
            Value::VisibleString(_) => DataType::VisibleString,
            Value::OctetString(_) => DataType::OctetString,
            Value::Domain(_) => DataType::Domain,
            Value::RelativeNodeId(_) => DataType::Unsigned32,
        }
    }

    /// The encoded size of this value, in bits. For a variable-length value
    /// this is its current content length.
    pub const fn bit_size(&self) -> usize {
        match self {
            Value::VisibleString(s) | Value::OctetString(s) | Value::Domain(s) => s.len(),
            _ => self.data_type().bit_size(),
        }
    }

    pub const fn size(&self) -> usize {
        match self {
            Value::VisibleString(s) | Value::OctetString(s) | Value::Domain(s) => s.len(),
            _ => self.data_type().size(),
        }
    }

    pub fn encode(&self, buf: &mut BitSlice<u8, Msb0>, start: usize) -> Result<(), Error> {
        let stop = start + self.bit_size();
        match self {
            Value::Boolean(_) => todo!(),
            Value::Integer8(_) => todo!(),
            Value::Integer16(_) => todo!(),
            Value::Integer32(_) => todo!(),
            Value::Unsigned8(_) => todo!(),
            Value::Unsigned16(v) => buf[start..stop].store_le(*v),
            Value::Unsigned32(_) => todo!(),
            Value::Real32(_) => todo!(),
            Value::Real64(_) => todo!(),
            Value::Integer64(_) => todo!(),
            Value::Unsigned64(_) => todo!(),
            Value::VisibleString(_) => todo!(),
            Value::OctetString(_) => todo!(),
            Value::Domain(_) => todo!(),
            Value::RelativeNodeId(_) => todo!(),
        }

        Ok(())
    }

    /// Encode this value little-endian into `buf`, returning the number of
    /// bytes written.
    ///
    /// Returns [`Error::BadLength`] if `buf` is smaller than [`Value::size`].
    pub fn encode_le(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let n = self.size();
        if buf.len() < n {
            return Err(Error::BadLength);
        }
        match self {
            Value::Boolean(v) => buf[0] = *v as u8,
            Value::Integer8(v) => buf[0] = *v as u8,
            Value::Unsigned8(v) => buf[0] = *v,
            Value::Integer16(v) => buf[..2].copy_from_slice(&v.to_le_bytes()),
            Value::Unsigned16(v) => buf[..2].copy_from_slice(&v.to_le_bytes()),
            Value::Integer32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            Value::Unsigned32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            Value::Real32(v) => buf[..4].copy_from_slice(&v.to_le_bytes()),
            Value::Integer64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            Value::Unsigned64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            Value::Real64(v) => buf[..8].copy_from_slice(&v.to_le_bytes()),
            Value::VisibleString(s) | Value::OctetString(s) | Value::Domain(s) => {
                buf[..n].copy_from_slice(s.as_bytes())
            }
            Value::RelativeNodeId(_) => todo!(),
        }
        Ok(n)
    }

    /// TODO: проверить на настоящих данных - возможно заменить Msb0 и load_be
    pub fn decode(data_type: DataType, buf: &BitSlice<u8, Msb0>) -> Result<Value, Error> {
        let value = match data_type {
            DataType::Boolean => {
                let v = buf[0];
                Value::Boolean(v)
            }
            DataType::Integer8 => {
                let v = buf.load_be();
                Value::Integer8(v)
            }
            DataType::Integer16 => todo!(),
            DataType::Integer32 => todo!(),
            DataType::Unsigned8 => {
                let v = buf.load_be();
                Value::Unsigned8(v)
            }
            DataType::Unsigned16 => todo!(),
            DataType::Unsigned32 => todo!(),
            DataType::Real32 => todo!(),
            DataType::Real64 => todo!(),
            DataType::Integer64 => todo!(),
            DataType::Unsigned64 => todo!(),
            DataType::VisibleString => todo!(),
            DataType::OctetString => todo!(),
            DataType::Domain => todo!(),
            DataType::PdoCommunicationParameter => todo!(),
            DataType::PdoMapping => todo!(),
            DataType::SdoParameter => todo!(),
            DataType::Identity => todo!(),
        };

        Ok(value)
    }

    /// Decode a little-endian value of `data_type` from `bytes`.
    ///
    /// For a fixed-size type `bytes` must be exactly [`DataType::fixed_size`]
    /// long; for a variable-length type it may be any length up to
    /// [`MAX_STRING_LEN`]. Otherwise returns [`Error::BadLength`].
    pub fn decode_le(data_type: DataType, bytes: &[u8]) -> Result<Value, Error> {
        match data_type.fixed_size() {
            Some(fixed) if bytes.len() != fixed => return Err(Error::BadLength),
            None if bytes.len() > MAX_STRING_LEN => return Err(Error::BadLength),
            _ => {}
        }
        let v = match data_type {
            DataType::Boolean => Value::Boolean(bytes[0] != 0),
            DataType::Integer8 => Value::Integer8(bytes[0] as i8),
            DataType::Unsigned8 => Value::Unsigned8(bytes[0]),
            DataType::Integer16 => Value::Integer16(i16::from_le_bytes([bytes[0], bytes[1]])),
            DataType::Unsigned16 => Value::Unsigned16(u16::from_le_bytes([bytes[0], bytes[1]])),
            DataType::Integer32 => {
                Value::Integer32(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            DataType::Unsigned32 => {
                Value::Unsigned32(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            DataType::Real32 => {
                Value::Real32(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            DataType::Integer64 => Value::Integer64(i64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
            DataType::Unsigned64 => Value::Unsigned64(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
            DataType::Real64 => Value::Real64(f64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
            DataType::VisibleString => Value::VisibleString(ByteString::from_bytes(bytes)?),
            DataType::OctetString => Value::OctetString(ByteString::from_bytes(bytes)?),
            DataType::Domain => Value::Domain(ByteString::from_bytes(bytes)?),
            DataType::PdoCommunicationParameter => todo!(),
            DataType::PdoMapping => todo!(),
            DataType::SdoParameter => todo!(),
            DataType::Identity => todo!(),
        };
        Ok(v)
    }

    #[track_caller]
    pub fn get_unsigned8(&self) -> Result<u8, Error> {
        match self {
            Value::Unsigned8(v) => Ok(*v),
            _ => {
                let loc = std::panic::Location::caller();
                Err(Error::TypeMismatch2 {
                    value: *self,
                    location: loc.to_string(),
                })
            }
        }
    }

    #[track_caller]
    pub fn get_unsigned16(&self) -> Result<u16, Error> {
        match self {
            Value::Unsigned16(v) => Ok(*v),
            _ => {
                let loc = std::panic::Location::caller();
                Err(Error::TypeMismatch2 {
                    value: *self,
                    location: loc.to_string(),
                })
            }
        }
    }

    #[track_caller]
    pub fn get_unsigned32(&self) -> Result<u32, Error> {
        match self {
            Value::Unsigned32(v) => Ok(*v),
            _ => {
                let loc = std::panic::Location::caller();
                Err(Error::TypeMismatch2 {
                    value: *self,
                    location: loc.to_string(),
                })
            }
        }
    }

    #[track_caller]
    pub fn get_visible_string(&self) -> Result<ByteString, Error> {
        match self {
            Value::VisibleString(v) => Ok(*v),
            _ => {
                let loc = std::panic::Location::caller();
                Err(Error::TypeMismatch2 {
                    value: *self,
                    location: loc.to_string(),
                })
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(_) => todo!(),
            Value::Integer8(_) => todo!(),
            Value::Integer16(_) => todo!(),
            Value::Integer32(_) => todo!(),
            Value::Unsigned8(v) => write!(f, "{}", v),
            Value::Unsigned16(v) => write!(f, "{}", v),
            Value::Unsigned32(v) => write!(f, "{}", v),
            Value::Real32(v) => write!(f, "{}", v),
            Value::Real64(_) => todo!(),
            Value::Integer64(_) => todo!(),
            Value::Unsigned64(_) => todo!(),
            Value::VisibleString(v) => write!(f, "{}", v.as_str().unwrap()),
            Value::OctetString(_) => todo!(),
            Value::Domain(_) => todo!(),
            Value::RelativeNodeId(v) => write!(f, "$NODEID+0x{:x}", v),
        }
    }
}
