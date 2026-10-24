#![allow(clippy::module_inception)]

mod entry;
mod entry_array;
mod entry_record;

pub use {entry::Entry, entry_array::EntryArray, entry_record::EntryRecord};

use super::{AccessType, Value};
