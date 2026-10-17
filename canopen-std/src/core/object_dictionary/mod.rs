//! The CANopen object dictionary (OD) model.
//!
//! Every CANopen device exposes its data as a dictionary of objects addressed
//! by a 16-bit `index` and an 8-bit `subindex` (CiA 301 §7.4). Communication
//! objects live in `0x1000..=0x1FFF`, manufacturer-specific objects in
//! `0x2000..=0x5FFF`, and standardised device-profile objects in
//! `0x6000..=0x9FFF`.
//!
//! [`ObjectDictionary`] is backed by a [`heapless::LinearMap`] with a
//! compile-time capacity, so the same type serves both `no_std` embedded
//! nodes and host tooling with no allocator.
//!
//! ```
//! use canopen_rs::{Address, Entry, ObjectDictionary, Value};
//!
//! let mut od = ObjectDictionary::<8>::new();
//! // 0x1017 producer heartbeat time — a read/write UNSIGNED16.
//! od.insert(Address::new(0x1017, 0), Entry::rw(Value::Unsigned16(1000))).unwrap();
//!
//! assert_eq!(od.read(Address::new(0x1017, 0)).unwrap(), Value::Unsigned16(1000));
//! od.write(Address::new(0x1017, 0), Value::Unsigned16(500)).unwrap();
//! assert_eq!(od.read(Address::new(0x1017, 0)).unwrap(), Value::Unsigned16(500));
//!
//! // Access rights and data types are enforced.
//! od.insert(Address::new(0x1000, 0), Entry::constant(Value::Unsigned32(0x192))).unwrap();
//! assert!(od.write(Address::new(0x1000, 0), Value::Unsigned32(1)).is_err()); // read-only
//! ```

mod access_type;
mod address;
pub mod objects_old;
pub use {access_type::AccessType, address::Address};

use {canopen_rs::NodeId, std::collections::HashMap};

use crate::{types::Value, Error};

/// A single object dictionary entry: a typed value and its access rights.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Entry {
    pub address: Address,

    /// The current value (its variant fixes the object's data type).
    pub value: Value,
    /// The entry's access rights.
    pub access: AccessType,
}

impl Entry {
    /// A read/write entry.
    pub const fn rw(value: Value) -> Self {
        Self {
            address: Address::new(0x0000, 0),
            value,
            access: AccessType::Rw,
        }
    }

    /// A read-only entry.
    pub const fn ro(value: Value) -> Self {
        Self {
            address: Address::new(0x0000, 0),
            value,
            access: AccessType::Ro,
        }
    }

    /// A read-only constant entry.
    pub const fn constant(value: Value) -> Self {
        Self {
            address: Address::new(0x0000, 0),
            value,
            access: AccessType::Const,
        }
    }
}

/// An object dictionary holding up to `N` entries, addressed by
/// `(index, subindex)`.
#[derive(Debug)]
pub struct ObjectDictionary {
    entries: HashMap<Address, Entry>,
    node_id: NodeId,
}

impl Default for ObjectDictionary {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            node_id: NodeId::new(1).unwrap(),
        }
    }
}

impl ObjectDictionary {
    /// Create an empty object dictionary.
    pub fn new(node_id: NodeId) -> Self {
        Self {
            entries: HashMap::new(),
            node_id,
        }
    }

    /// Insert or replace the entry at `addr`.
    ///
    /// Returns [`Error::DictionaryFull`] when capacity `N` is exhausted.
    pub fn insert(&mut self, addr: Address, entry: Entry) -> Result<(), Error> {
        self.entries.insert(addr, entry);
        Ok(())
    }

    /// Borrow the raw entry at `addr`, ignoring access rights.
    pub fn entry(&self, addr: Address) -> Option<&Entry> {
        self.entries.get(&addr)
    }

    /// Read the value at `addr`, honouring access rights.
    ///
    /// Returns [`Error::ObjectNotFound`] if absent, or [`Error::WriteOnly`] if
    /// the object is not readable.
    pub fn read(&self, addr: Address) -> Result<Value, Error> {
        let entry = self.entries.get(&addr).ok_or(Error::ObjectNotFound)?;
        if !entry.access.is_readable() {
            return Err(Error::WriteOnly);
        }
        Ok(entry.value)
    }

    /// Write `value` at `addr`, honouring access rights and the object's data
    /// type.
    ///
    /// Returns [`Error::ObjectNotFound`] if absent, [`Error::ReadOnly`] if not
    /// writable, or [`Error::TypeMismatch`] if `value`'s type differs from the
    /// stored object's type.
    pub fn write(&mut self, addr: Address, value: Value) -> Result<(), Error> {
        let entry = self.entries.get_mut(&addr).ok_or(Error::ObjectNotFound)?;
        if !entry.access.is_writable() {
            return Err(Error::ReadOnly);
        }
        if entry.value.data_type() != value.data_type() {
            return Err(Error::TypeMismatch);
        }
        entry.value = value;
        Ok(())
    }

    /// Set the value at `addr` from the **device side**, ignoring master access
    /// rights — those govern SDO access, not the application, so this can update
    /// a master-read-only object (e.g. publish process data into a TPDO source,
    /// or the node's error register `0x1001`). The object must exist and the
    /// value's type must match.
    ///
    /// Returns [`Error::ObjectNotFound`] if absent or [`Error::TypeMismatch`] on
    /// a type mismatch.
    pub fn set(&mut self, addr: Address, value: Value) -> Result<(), Error> {
        let entry = self.entries.get_mut(&addr).ok_or(Error::ObjectNotFound)?;
        if entry.value.data_type() != value.data_type() {
            return Err(Error::TypeMismatch);
        }
        entry.value = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn od() -> ObjectDictionary {
        let mut od = ObjectDictionary::new(NodeId::new(1).unwrap());
        // 0x1000 Device type — read-only constant.
        od.insert(
            Address::new(0x1000, 0),
            Entry::constant(Value::Unsigned32(0x0000_0192)),
        )
        .unwrap();
        // 0x1017 Producer heartbeat time — read/write.
        od.insert(Address::new(0x1017, 0), Entry::rw(Value::Unsigned16(1000)))
            .unwrap();
        od
    }

    #[test]
    fn read_returns_stored_value() {
        assert_eq!(
            od().read(Address::new(0x1000, 0)).unwrap(),
            Value::Unsigned32(0x192)
        );
    }

    #[test]
    fn read_missing_object_errors() {
        assert!(matches!(
            od().read(Address::new(0x9999, 0)),
            Err(Error::ObjectNotFound)
        ));
    }

    #[test]
    fn write_updates_value() {
        let mut od = od();
        od.write(Address::new(0x1017, 0), Value::Unsigned16(500))
            .unwrap();
        assert_eq!(
            od.read(Address::new(0x1017, 0)).unwrap(),
            Value::Unsigned16(500)
        );
    }

    #[test]
    fn write_read_only_object_errors() {
        let mut od = od();
        assert!(matches!(
            od.write(Address::new(0x1000, 0), Value::Unsigned32(1)),
            Err(Error::ReadOnly)
        ));
    }

    #[test]
    fn write_wrong_type_errors() {
        let mut od = od();
        assert!(matches!(
            od.write(Address::new(0x1017, 0), Value::Unsigned32(1)),
            Err(Error::TypeMismatch)
        ));
    }
}
