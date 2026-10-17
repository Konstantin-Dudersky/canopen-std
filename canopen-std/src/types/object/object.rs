use super::{DataType, Entry};

#[derive(Debug, Clone)]
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
        data_type: DataType,
        entries: Vec<Entry>,
    },
    Record {
        index: u16,
        name: String,
        data_type: DataType,
        entries: Vec<Entry>,
    },
}

impl Object {
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
                let default_value = entry.value.clone();
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
                data_type,
                entries,
            } => {
                let highest_subindex = entries.len() as u16;
                let sub_number = highest_subindex + 1;
                let data_type_index = data_type.index();

                let entries_eds = entries
                    .iter()
                    .enumerate()
                    .map(|(subindex, entry)| {
                        let subindex = subindex + 1;
                        let parameter_name = entry.description.clone().unwrap();
                        let access_type = entry.access_type.to_eds();
                        let default_value = entry.value.clone();
                        let pdo_mapping = if entry.pdo_mapping { "1" } else { "0" };
                        format!(
                            r#"
[{index:04X}sub{subindex}]
ParameterName={parameter_name}
ObjectType=0x7
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
ObjectType=0x7
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
                data_type: _,
                entries,
            } => {
                let highest_subindex = entries.len() as u16;
                let sub_number = highest_subindex + 1;

                let entries_eds = entries
                    .iter()
                    .enumerate()
                    .map(|(subindex, entry)| {
                        let subindex = subindex + 1;
                        let parameter_name = entry.description.clone().unwrap();
                        let data_type_index = entry.data_type.unwrap().index();
                        let access_type = entry.access_type.to_eds();
                        let default_value = entry.value.clone();
                        let pdo_mapping = if entry.pdo_mapping { "1" } else { "0" };
                        format!(
                            r#"
[{index:04X}sub{subindex}]
ParameterName={parameter_name}
ObjectType=0x7
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
ObjectType=0x7
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
}
