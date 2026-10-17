#![allow(clippy::module_inception)]

mod object;
mod object_array;
mod object_record;
mod object_var;

pub use {
    object::Object, object_array::ObjectArray, object_record::ObjectRecord, object_var::ObjectVar,
};

use super::{AccessType, DataType, Entry, EntryArray, EntryRecord};
