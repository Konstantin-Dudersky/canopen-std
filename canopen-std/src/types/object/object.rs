use std::fmt::{Display, Formatter};

use crate::types::{AccessType, EntryArray, Value};

use super::{DataType, Entry};

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Null,
    Domain,
    Deftype,
    Defstruct,
    Var {
        index: u16,
        name: String,
        data_type: DataType,
        entry: Entry,
    },
    Array {
        index: u16,
        name: String,
        entries: Vec<Entry>,
    },
    Record {
        index: u16,
        name: String,
        entries: Vec<Entry>,
    },
}

impl Object {
    pub fn add_entry(&mut self, entry: impl Into<Entry>) {
        let entries = match self {
            Object::Array { entries, .. } => entries,
            Object::Record { entries, .. } => entries,
            _ => return,
        };

        entries.push(entry.into());
    }

    pub fn get_entry(&self, subindex: u8) -> Option<Entry> {
        let entries = match self {
            Object::Var { entry, .. } => {
                if subindex == 0 {
                    return Some(entry.clone());
                } else {
                    return None;
                }
            }
            Object::Array { entries, .. } => entries,
            Object::Record { entries, .. } => entries,
            _ => return None,
        };

        if subindex == 0 {
            let value = entries.iter().max_by_key(|e| e.subindex)?.subindex;
            let entry = Entry::from(EntryArray {
                subindex,
                description: "Highest sub-index supported".into(),
                access_type: AccessType::Ro,
                pdo_mapping: false,
                value: Value::Unsigned8(value),
            });
            return Some(entry);
        }

        entries.iter().find(|o| o.subindex == subindex).cloned()
    }

    pub fn get_entry_mut(&mut self, subindex: u8) -> Option<&mut Entry> {
        let entries = match self {
            Object::Var { entry, .. } => {
                if subindex == 0 {
                    return Some(entry);
                } else {
                    return None;
                }
            }
            Object::Array { entries, .. } => entries,
            Object::Record { entries, .. } => entries,
            _ => return None,
        };

        entries.iter_mut().find(|o| o.subindex == subindex)
    }

    pub fn get_all_entries_without_highest_subindex(&self) -> Vec<Entry> {
        let entries = match self {
            Object::Var { entry, .. } => return vec![entry.clone()],
            Object::Array { entries, .. } => entries,
            Object::Record { entries, .. } => entries,
            _ => return vec![],
        };

        entries
            .iter()
            .filter(|o| o.subindex != 0)
            .cloned()
            .collect::<Vec<Entry>>()
    }

    pub fn to_eds(&self) -> String {
        match self {
            Object::Null => todo!(),
            Object::Domain => todo!(),
            Object::Deftype => todo!(),
            Object::Defstruct => todo!(),
            Object::Var {
                index,
                name,
                data_type,
                entry,
            } => {
                let data_type_index = data_type.index();
                let access_type = entry.access_type.to_eds();
                let default_value = entry.value;
                let pdo_mapping = if entry.pdo_mapping { 1 } else { 0 };

                format!(
                    r#"
[{index:X}]
ParameterName={name}
ObjectType=0x7
DataType=0x{data_type_index:02X}
AccessType={access_type}
DefaultValue={default_value}
PDOMapping={pdo_mapping}

"#
                )
            }
            Object::Array {
                index,
                name,
                entries,
            } => {
                let highest_subindex = entries.len() as u16;
                let sub_number = highest_subindex + 1;

                let entries_eds = entries
                    .iter()
                    .map(|entry| {
                        let subindex = entry.subindex;
                        let parameter_name = entry.description.clone().unwrap();
                        let data_type_index = entry.value.data_type().index();
                        let access_type = entry.access_type.to_eds();
                        let default_value = entry.value;
                        let pdo_mapping = if entry.pdo_mapping { "1" } else { "0" };
                        format!(
                            r#"
[{index:04X}sub{subindex}]
ParameterName={parameter_name}
DataType=0x{data_type_index:02X}
AccessType={access_type}
DefaultValue={default_value}
PDOMapping={pdo_mapping}
"#
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    r#"
[{index:04X}]
ParameterName={name}
ObjectType=0x8
SubNumber={sub_number}

[{index:04X}sub0]
ParameterName=Highest sub-index supported
DataType=0x05
AccessType=ro
DefaultValue={highest_subindex}
PDOMapping=0
{entries_eds}
"#
                )
            }
            Object::Record {
                index,
                name,
                entries,
            } => {
                let highest_subindex = entries.len() as u16;
                let sub_number = highest_subindex + 1;

                let entries_eds = entries
                    .iter()
                    .map(|entry| {
                        let subindex = entry.subindex;
                        let parameter_name = entry.description.clone().unwrap();
                        let data_type_index = entry.value.data_type().index();
                        let access_type = entry.access_type.to_eds();
                        let default_value = entry.value.to_string();
                        let pdo_mapping = if entry.pdo_mapping { "1" } else { "0" };
                        format!(
                            r#"
[{index:04X}sub{subindex}]
ParameterName={parameter_name}
DataType=0x{data_type_index:02X}
AccessType={access_type}
DefaultValue={default_value}
PDOMapping={pdo_mapping}
"#
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("");

                format!(
                    r#"
[{index:04X}]
ParameterName={name}
ObjectType=0x9
SubNumber={sub_number}

[{index:04X}sub0]
ParameterName=Highest sub-index supported
DataType=0x05
AccessType=ro
DefaultValue={highest_subindex}
PDOMapping=0
{entries_eds}
"#
                )
            }
        }
    }
}

impl Object {
    pub fn index(&self) -> u16 {
        match self {
            Object::Null => todo!(),
            Object::Domain => todo!(),
            Object::Deftype => todo!(),
            Object::Defstruct => todo!(),
            Object::Var { index, .. } => *index,
            Object::Array { index, .. } => *index,
            Object::Record { index, .. } => *index,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Object::Null => "",
            Object::Domain => "",
            Object::Deftype => "",
            Object::Defstruct => "",
            Object::Var { name, .. } => name,
            Object::Array { name, .. } => name,
            Object::Record { name, .. } => name,
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let index = self.index();
        let name = self.name();
        write!(f, "0x{:04X} - {}", index, name)?;

        match self {
            Object::Null => todo!(),
            Object::Domain => todo!(),
            Object::Deftype => todo!(),
            Object::Defstruct => todo!(),
            Object::Var {
                index,
                name,
                data_type,
                entry,
            } => write!(
                f,
                "\n    {} - {} - {}",
                entry.subindex,
                entry.description.as_deref().unwrap_or(""),
                entry.value
            )?,
            Object::Array {
                index,
                name,
                entries,
            } => {
                let mut entries = entries.clone();
                entries.sort_by_key(|e| e.subindex);
                for entry in entries {
                    write!(
                        f,
                        "\n    {} - {} - {}",
                        entry.subindex,
                        entry.description.as_deref().unwrap_or(""),
                        entry.value
                    )?
                }
            }
            Object::Record {
                index,
                name,
                entries,
            } => {
                let mut entries = entries.clone();
                entries.sort_by_key(|e| e.subindex);
                for entry in entries {
                    write!(
                        f,
                        "\n    {} - {} - {}",
                        entry.subindex,
                        entry.description.as_deref().unwrap_or(""),
                        entry.value
                    )?
                }
            }
        }
        write!(f, "")
    }
}
