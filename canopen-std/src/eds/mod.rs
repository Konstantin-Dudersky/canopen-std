//! EDS / DCF parsing (CiA 306) — INI-style device description files.
//!
//! An Electronic Data Sheet describes a device's object dictionary in an
//! INI-like text format: a `[<index>]` section per object and
//! `[<index>sub<subindex>]` sections per subindex, each carrying fields such
//! as `ParameterName`, `DataType`, `AccessType`, and `DefaultValue`. Indices
//! and subindices are hexadecimal. A Device Configuration File (DCF) shares
//! the grammar, adding concrete `ParameterValue` fields.
//!
//! [`Eds::parse`](crate::eds::Eds::parse) turns that text into a list of typed
//! [`ObjectDescription`](crate::eds::ObjectDescription)s, and
//! [`Eds::object_dictionary`](crate::eds::Eds::object_dictionary) builds a
//! ready-to-use core [`ObjectDictionary`](canopen_rs::ObjectDictionary) from
//! them — the same dictionary type an embedded node runs, populated here from a
//! file.
//!
//! ## Supported subset
//!
//! * The numeric basic data types (see [`DataType`](canopen_rs::DataType));
//!   `VISIBLE_STRING`,
//!   `OCTET_STRING`, and `DOMAIN` objects are skipped, as the core does not
//!   yet model variable-length values.
//! * `DefaultValue` / `ParameterValue` as decimal or `0x…` hex, including the
//!   `$NODEID` substitution used for COB-IDs (e.g. `$NODEID+0x200`), resolved
//!   against the file's `[DeviceComissioning] NodeID` when present.

mod eds_error;
mod section_comments;
mod section_device_info;
mod section_dummy_usage;
mod section_file_info;
mod sections_object_count;

pub use {
    eds_error::EdsError, section_comments::Comments, section_device_info::DeviceInfo,
    section_dummy_usage::DummyUsage, section_file_info::FileInfo,
    sections_object_count::ObjectCount as ObjectsCount,
};

type EdsFileSection = BTreeMap<String, String>;

use std::{collections::BTreeMap, fs, path::Path};

use tracing::warn;

use crate::{
    Error,
    core::object_dictionary::Address,
    objects::ObjectDictionary as ObjectDictionaryNew,
    types::{
        AccessType, ByteString, DataType, Entry, EntryArray, EntryRecord, MAX_STRING_LEN, NodeId,
        Object, ObjectArray, ObjectRecord, ObjectType, ObjectVar, Value,
    },
};

/// A parsed Electronic Data Sheet (or Device Configuration File).
#[derive(Debug, Clone)]
pub struct Eds {
    /// [DeviceInfo] section
    pub device_info: DeviceInfo,

    /// [FileInfo] section
    pub file_info: FileInfo,

    /// The described objects, ordered by address.
    pub objects: Vec<Object>,
}

impl Eds {
    /// Parse EDS/DCF text.
    pub fn parse(text: &str, node_id: NodeId) -> Result<Eds, EdsError> {
        let sections = parse_ini(text);

        let device_info = sections.get("deviceinfo");
        let device_info = match device_info {
            Some(s) => s.into(),
            None => DeviceInfo::default(),
        };

        let node_for_expr = node_id.raw();

        let mut objects = Vec::new();
        for (name, fields) in &sections {
            let Some(address) = parse_object_section(name) else {
                continue;
            };

            let object_type = fields.get("objecttype");

            let name = fields.get("parametername").cloned().unwrap_or_default();

            let pdo_mapping = fields
                .get("pdomapping")
                .map(|v| v.trim() != "0")
                .unwrap_or(false);

            let access_type = match fields.get("accesstype") {
                Some(a) => AccessType::try_from(a.as_str())?,
                None => AccessType::Ro,
            };

            let datatype = fields.get("datatype");
            let datatype = match datatype {
                Some(dt) => Some(DataType::try_from(dt.as_str())?),
                None => None,
            };

            let default_value = match datatype {
                Some(datatype) => {
                    let raw_value = fields
                        .get("parametervalue")
                        .or_else(|| fields.get("defaultvalue"))
                        .map(String::as_str)
                        .unwrap_or("");
                    let default_value = value_from_str(datatype, raw_value, node_for_expr)
                        .ok_or_else(|| EdsError::InvalidValue {
                            section: name.clone(),
                            value: raw_value.to_string(),
                        })?;
                    Some(default_value)
                }
                None => None,
            };

            // TODO: ObjectType не обязателен для Var объектов

            match object_type {
                Some(object_type) => {
                    let object_type = ObjectType::try_from(object_type.as_str());
                    let Ok(object_type) = object_type else {
                        warn!("Invalid ObjectType for address {}", address);
                        continue;
                    };

                    let object: Object = match object_type {
                        ObjectType::Var => {
                            let Some(value) = default_value else {
                                warn!("No value");
                                continue;
                            };

                            ObjectVar {
                                index: address.index,
                                name,
                                access_type,
                                pdo_mapping,
                                value,
                            }
                            .into()
                        }
                        ObjectType::Array => ObjectArray {
                            index: address.index,
                            name,
                            entries: vec![],
                        }
                        .into(),
                        ObjectType::Record => ObjectRecord {
                            index: address.index,
                            name,
                            entries: vec![],
                        }
                        .into(),
                    };

                    objects.push(object);
                }

                // ObjectType не задан - значит добавляем Entry к существующему объекту
                None => {
                    let object = objects.iter_mut().find(|o| o.index() == address.index);
                    let Some(object) = object else {
                        warn!("No object found for index {}", address.index);
                        continue;
                    };
                    match object {
                        Object::Array { .. } => {
                            let Some(value) = default_value else {
                                warn!("No value");
                                continue;
                            };

                            let entry = EntryArray {
                                subindex: address.subindex,
                                description: name,
                                access_type,
                                pdo_mapping,
                                value,
                            };
                            object.add_entry(entry);
                        }
                        Object::Record { .. } => {
                            let Some(value) = default_value else {
                                continue;
                            };
                            let entry = EntryRecord {
                                subindex: address.subindex,
                                description: name,
                                access_type,
                                pdo_mapping,
                                value,
                            };
                            object.add_entry(entry);
                        }
                        _ => {
                            warn!("Unknown object type: {:?}", object_type);
                        }
                    }
                }
            }
        }

        objects.sort_by_key(|o| o.index());

        Ok(Eds {
            device_info,
            file_info: FileInfo::default(),
            objects,
        })
    }

    /// Read and parse an EDS/DCF file from `path`.
    pub fn from_file<P: AsRef<Path>>(path: P, node_id: NodeId) -> Result<Eds, EdsError> {
        Self::parse(&fs::read_to_string(path)?, node_id)
    }

    pub fn get_entry(&self, address: Address) -> Option<Entry> {
        let object = self.objects.iter().find(|o| o.index() == address.index)?;
        object.get_entry(address.subindex)
    }

    pub fn object_dictionary(&self, node_id: NodeId) -> Result<ObjectDictionaryNew, EdsError> {
        let mut od = ObjectDictionaryNew::new(node_id);
        for obj in &self.objects {
            od.insert_object(obj.clone());
        }
        Ok(od)
    }

    pub fn to_eds_str(&self) -> Result<String, Error> {
        let mut eds_str = String::new();

        let obj_dict = self.object_dictionary(NodeId::new(1).unwrap()).unwrap();

        let file_info = self.file_info.to_eds_str(&obj_dict)?;
        eds_str.push_str(&file_info);

        let device_info = &self.device_info.to_eds_str(&obj_dict)?;
        eds_str.push_str(device_info);

        let dummy_usage = DummyUsage {}.to_eds_str();
        eds_str.push_str(&dummy_usage);

        let comments = Comments {}.to_eds_str();
        eds_str.push_str(&comments);

        let object_count = ObjectsCount {}.to_eds_str(&self.objects);
        eds_str.push_str(&object_count);

        eds_str.push('\n');

        let mut objects = self.objects.clone();
        objects.sort_by_key(|obj| obj.index());

        for object in objects.iter() {
            eds_str.push_str(&object.to_eds_str());
        }

        Ok(eds_str)
    }

    pub fn save_in_folder(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref().join(&self.file_info.file_name);

        let contents = self.to_eds_str()?;
        fs::write(path, contents).map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}

// --- INI + value helpers ---------------------------------------------------

/// Parse INI text into `section -> (key -> value)`, with section names and
/// keys lower-cased for case-insensitive lookup and values kept verbatim.
fn parse_ini(text: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut sections = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        if let Some(inner) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            let name = inner.trim().to_ascii_lowercase();
            sections.entry(name.clone()).or_insert_with(BTreeMap::new);
            current = Some(name);
        } else if let (Some(section), Some((key, value))) = (&current, line.split_once('=')) {
            sections
                .get_mut(section)
                .expect("current section was inserted")
                .insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }
    sections
}

/// Parse an object/subindex section name (`"1018"` or `"1018sub2"`) into an
/// [`Address`]; `None` for a non-object section such as `"deviceinfo"`.
fn parse_object_section(name: &str) -> Option<Address> {
    if let Some((index, subindex)) = name.split_once("sub") {
        Some(Address::new(
            u16::from_str_radix(index, 16).ok()?,
            u8::from_str_radix(subindex, 16).ok()?,
        ))
    } else {
        Some(Address::new(u16::from_str_radix(name, 16).ok()?, 0))
    }
}

/// Evaluate a sum of `$NODEID` and integer literal terms, e.g. `"$NODEID+0x200"`
/// or `"1280+$NODEID"`. Literals may be decimal or `0x…` hex.
fn eval_int_expr(s: &str, node_id: u8) -> Option<i128> {
    let mut sum: i128 = 0;
    for term in s.split('+') {
        let term = term.trim();
        if term.is_empty() {
            continue;
        } else if term.eq_ignore_ascii_case("$nodeid") {
            sum += node_id as i128;
        } else if let Some(hex) = term.strip_prefix("0x").or_else(|| term.strip_prefix("0X")) {
            sum += i128::from_str_radix(hex, 16).ok()?;
        } else {
            sum += term.parse::<i128>().ok()?;
        }
    }
    Some(sum)
}

/// Parse a default/configured value string into a typed [`Value`].
fn value_from_str(data_type: DataType, s: &str, node_id: u8) -> Option<Value> {
    let s = s.trim();
    // Variable-length types take the literal as their content (VISIBLE_STRING
    // is text; OCTET_STRING / DOMAIN keep the raw bytes), truncated to the
    // buffer capacity.
    if data_type.is_variable() {
        let bytes = s.as_bytes();
        let end = bytes.len().min(MAX_STRING_LEN);
        let bs = ByteString::from_bytes(&bytes[..end]).ok()?;
        return Some(match data_type {
            DataType::OctetString => Value::OctetString(bs),
            DataType::Domain => Value::Domain(bs),
            _ => Value::VisibleString(bs),
        });
    }
    // Floating-point types parse the literal directly.
    match data_type {
        DataType::Real32 => {
            return Some(Value::Real32(if s.is_empty() {
                0.0
            } else {
                s.parse().ok()?
            }));
        }
        DataType::Real64 => {
            return Some(Value::Real64(if s.is_empty() {
                0.0
            } else {
                s.parse().ok()?
            }));
        }
        _ => {}
    }
    // Integer types evaluate a (possibly `$NODEID`-relative) expression.
    let n = if s.is_empty() {
        0
    } else {
        eval_int_expr(s, node_id)?
    };
    Some(match data_type {
        DataType::Boolean => Value::Boolean(n != 0),
        DataType::Integer8 => Value::Integer8(n as i8),
        DataType::Integer16 => Value::Integer16(n as i16),
        DataType::Integer32 => Value::Integer32(n as i32),
        DataType::Unsigned8 => Value::Unsigned8(n as u8),
        DataType::Unsigned16 => Value::Unsigned16(n as u16),
        DataType::Unsigned32 => Value::Unsigned32(n as u32),
        DataType::Integer64 => Value::Integer64(n as i64),
        DataType::Unsigned64 => Value::Unsigned64(n as u64),
        // Reals handled above; `#[non_exhaustive]` requires this catch-all.
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // A compact but representative EDS: device info, node id, a VAR, a RECORD
    // with subindices, a $NODEID COB-ID, a hex value, a float, and a skipped
    // string object.
    const SAMPLE: &str = "\
[DeviceInfo]
VendorName=Acme Robotics
ProductName=Widget 3000

[DeviceComissioning]
NodeID=0x10

[MandatoryObjects]
SupportedObjects=2
1=0x1000
2=0x1018

[1000]
ParameterName=Device type
ObjectType=0x7
DataType=0x0007
AccessType=ro
DefaultValue=0x00040192
PDOMapping=0

[1008]
ParameterName=Device name
ObjectType=0x7
DataType=0x0009
AccessType=const
DefaultValue=Widget

[1017]
ParameterName=Producer heartbeat time
ObjectType=0x7
DataType=0x0006
AccessType=rw
DefaultValue=1000
PDOMapping=1

[1018]
ParameterName=Identity object
ObjectType=0x9
SubNumber=2

[1018sub0]
ParameterName=Highest sub-index supported
DataType=0x0005
AccessType=const
DefaultValue=1

[1018sub1]
ParameterName=Vendor-ID
DataType=0x0007
AccessType=ro
DefaultValue=0x000000AB

[1400]
ParameterName=RPDO communication parameter
ObjectType=0x9
SubNumber=2

[1400sub0]
ParameterName=Highest sub-index supported
DataType=0x0005
AccessType=const
DefaultValue=1

[1400sub1]
ParameterName=COB-ID used by RPDO
DataType=0x0007
AccessType=rw
DefaultValue=$NODEID+0x200

[6000]
ParameterName=Scale factor
ObjectType=0x7
DataType=0x0008
AccessType=rw
DefaultValue=1.5
";

    fn sample() -> Eds {
        Eds::parse(SAMPLE, NodeId::new(0x10).unwrap()).unwrap()
    }

    #[test]
    fn reads_device_metadata_and_node_id() {
        let eds = sample();
        assert_eq!(eds.device_info.vendor_name, "Acme Robotics");
    }

    #[test]
    fn parses_var_with_hex_default() {
        let obj = sample().get_entry(Address::new(0x1000, 0)).unwrap().clone();
        assert_eq!(obj.description, Some("Device type".into()));
        assert_eq!(obj.value.data_type(), DataType::Unsigned32);
        assert_eq!(obj.access_type, AccessType::Ro);
        assert_eq!(obj.value, Value::Unsigned32(0x0004_0192));
    }

    #[test]
    fn parses_record_subindices() {
        let eds = sample();
        assert_eq!(
            eds.get_entry(Address::new(0x1018, 0)).unwrap().value,
            Value::Unsigned8(1)
        );
        assert_eq!(
            eds.get_entry(Address::new(0x1018, 1)).unwrap().value,
            Value::Unsigned32(0x0000_00AB)
        );
    }

    #[test]
    fn resolves_nodeid_expression() {
        // $NODEID (0x10) + 0x200 = 0x210.
        let obj = sample().get_entry(Address::new(0x1400, 1)).unwrap().clone();
        assert_eq!(obj.value, Value::Unsigned32(0x210));
    }

    #[test]
    fn parses_float_and_pdo_flag() {
        let eds = sample();
        assert_eq!(
            eds.get_entry(Address::new(0x6000, 0)).unwrap().value,
            Value::Real32(1.5)
        );
        assert!(eds.get_entry(Address::new(0x1017, 0)).unwrap().pdo_mapping);
        assert!(!eds.get_entry(Address::new(0x1000, 0)).unwrap().pdo_mapping);
    }

    #[test]
    fn parses_visible_string_object() {
        // 0x1008 Device name is VISIBLE_STRING (0x09), default "Widget".
        let obj = sample().get_entry(Address::new(0x1008, 0)).unwrap().clone();
        assert_eq!(obj.value.data_type(), DataType::VisibleString);
        match obj.value {
            Value::VisibleString(bs) => assert_eq!(bs.as_str(), Some("Widget")),
            other => panic!("expected VISIBLE_STRING, got {other:?}"),
        }
    }

    #[test]
    fn builds_object_dictionary() {
        let eds = sample();
        let od = eds.object_dictionary(NodeId::new(1).unwrap()).unwrap();
        assert_eq!(od.read(0x1000, 0).unwrap(), Value::Unsigned32(0x0004_0192));
        assert_eq!(od.read(0x1017, 0).unwrap(), Value::Unsigned16(1000));
    }

    #[test]
    fn unknown_access_type_errors() {
        let text = "[2000]\nDataType=0x0005\nAccessType=bogus\n";
        assert!(matches!(
            Eds::parse(text, NodeId::new(0x10).unwrap()),
            Err(EdsError::UnknownAccessType(_))
        ));
    }
}
