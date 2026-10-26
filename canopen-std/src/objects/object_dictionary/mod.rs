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

use crate::{
    Error,
    types::{Entry, NodeId, Object, Value},
};

/// An object dictionary holding up to `N` entries, addressed by
/// `(index, subindex)`.
#[derive(Debug)]
pub struct ObjectDictionary {
    objects: Vec<Object>,
    pub(crate) node_id: NodeId,
}

impl Default for ObjectDictionary {
    fn default() -> Self {
        Self {
            objects: vec![],
            node_id: NodeId::new(1).unwrap(),
        }
    }
}

impl ObjectDictionary {
    /// Create an empty object dictionary.
    pub fn new(node_id: NodeId) -> Self {
        Self {
            objects: vec![],
            node_id,
        }
    }

    /// Insert or replace the entry at `addr`.
    pub fn insert_object(&mut self, object: impl Into<Object>) {
        let new_object = object.into();

        // Удаляем существующий объект с таким же индексом, если он есть
        self.objects.retain(|obj| obj.index() != new_object.index());

        self.objects.push(new_object);
        self.objects.sort_by_key(|o| o.index());
    }

    /// Borrow the raw entry at `addr`, ignoring access rights.
    pub fn get_entry(&self, index: u16, subindex: u8) -> Option<Entry> {
        let object = self.get_object(index)?;
        object.get_entry(subindex)
    }

    pub fn get_object(&self, index: u16) -> Option<&Object> {
        self.objects.iter().find(|o| o.index() == index)
    }

    /// Read the value at `addr`, honouring access rights.
    ///
    /// Returns [`Error::ObjectNotFound`] if absent, or [`Error::WriteOnly`] if
    /// the object is not readable.
    pub fn read(&self, index: u16, subindex: u8) -> Result<Value, Error> {
        let object = self.get_object(index).ok_or(Error::ObjectNotFound)?;
        let entry = object.get_entry(subindex).ok_or(Error::ObjectNotFound)?;
        if !entry.access_type.is_readable() {
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
    pub fn write(&mut self, index: u16, subindex: u8, value: Value) -> Result<(), Error> {
        let object = self
            .objects
            .iter_mut()
            .find(|o| o.index() == index)
            .ok_or(Error::ObjectNotFound)?;
        let entry = object
            .get_entry_mut(subindex)
            .ok_or(Error::ObjectNotFound)?;

        if !entry.access_type.is_writable() {
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
    pub fn set(&mut self, index: u16, subindex: u8, value: Value) -> Result<(), Error> {
        let object = self
            .objects
            .iter_mut()
            .find(|o| o.index() == index)
            .ok_or(Error::ObjectNotFound)?;
        let entry = object
            .get_entry_mut(subindex)
            .ok_or(Error::ObjectNotFound)?;

        if entry.value.data_type() != value.data_type() {
            return Err(Error::TypeMismatch);
        }
        entry.value = value;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{AccessType, ObjectVar};

    use super::*;

    fn od() -> ObjectDictionary {
        let mut od = ObjectDictionary::new(NodeId::new(1).unwrap());
        // 0x1000 Device type — read-only constant.
        od.insert_object(ObjectVar {
            index: 0x1000,
            name: "Device type".into(),
            access_type: AccessType::Const,
            pdo_mapping: false,
            value: Value::Unsigned32(0x0000_0192),
        });
        // 0x1017 Producer heartbeat time — read/write.
        od.insert_object(ObjectVar {
            index: 0x1017,
            name: "Producer heartbeat time".into(),
            access_type: AccessType::Rw,
            pdo_mapping: false,
            value: Value::Unsigned16(1000),
        });
        od
    }

    #[test]
    fn read_returns_stored_value() {
        assert_eq!(od().read(0x1000, 0).unwrap(), Value::Unsigned32(0x192));
    }

    #[test]
    fn read_missing_object_errors() {
        assert!(matches!(od().read(0x9999, 0), Err(Error::ObjectNotFound)));
    }

    #[test]
    fn write_updates_value() {
        let mut od = od();
        od.write(0x1017, 0, Value::Unsigned16(500)).unwrap();
        assert_eq!(od.read(0x1017, 0).unwrap(), Value::Unsigned16(500));
    }

    #[test]
    fn write_read_only_object_errors() {
        let mut od = od();
        assert!(matches!(
            od.write(0x1000, 0, Value::Unsigned32(1)),
            Err(Error::ReadOnly)
        ));
    }

    #[test]
    fn write_wrong_type_errors() {
        let mut od = od();
        assert!(matches!(
            od.write(0x1017, 0, Value::Unsigned32(1)),
            Err(Error::TypeMismatch)
        ));
    }
}
