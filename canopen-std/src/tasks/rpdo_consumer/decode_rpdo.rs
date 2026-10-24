use bitvec::{order::Msb0, view::BitView};

use crate::{
    objects::{ObjectDictionary, RpdoMapping},
    types::Value,
    Error,
};

/// Расшифровываем данные RPDO
fn decode_rpdo(
    rpdo_maping: RpdoMapping,
    rpdo_data: &[u8],
    od: &ObjectDictionary,
) -> Result<Vec<(u16, u8, Value)>, Error> {
    if rpdo_data.len() * 8 < rpdo_maping.bit_size() {
        return Err(Error::BadLength);
    }

    let bits = rpdo_data.view_bits::<Msb0>();
    let mut start_bit = 0;

    let mut values = vec![];

    for map in rpdo_maping.items {
        let od_entry = od.get_entry(map.index, map.subindex).unwrap();

        let od_size = od_entry.value.data_type().bit_size();
        let rpdo_size = map.length as usize;
        if od_size != rpdo_size {
            return Err(Error::TypeMismatch);
        }

        let bit_slice = &bits[start_bit..(start_bit + od_size)];
        let value = Value::decode(od_entry.value.data_type(), bit_slice)?;
        start_bit += od_size;

        values.push((map.index, map.subindex, value));
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use canopen_rs::NodeId;

    use pretty_assertions::assert_eq;

    use crate::{
        objects_old::{self},
        types::{AccessType, EntryRecord, Object, ObjectRecord},
    };

    use super::*;

    #[test]
    fn test_decode_rpdo() {
        let node = NodeId::new(0x02).unwrap();
        let rpdo_data: Vec<u8> = vec![0b1_0001011, 0b1_0000000];
        let mut od = ObjectDictionary::new(node);

        // Заполняем словарь объектов
        let objects: Vec<Object> = vec![
            RpdoMapping {
                sequence_number: 0,
                items: vec![
                    objects_old::PdoMappingItem {
                        index: 0x2000,
                        subindex: 1,
                        length: 1,
                    },
                    objects_old::PdoMappingItem {
                        index: 0x2000,
                        subindex: 2,
                        length: 8,
                    },
                ],
            }
            .into(),
            ObjectRecord {
                index: 0x2000,
                name: "Test".to_string(),
                entries: vec![
                    EntryRecord {
                        subindex: 1,
                        description: "".to_string(),
                        access_type: AccessType::Rw,
                        pdo_mapping: false,
                        value: Value::Boolean(false),
                    },
                    EntryRecord {
                        subindex: 2,
                        description: "".to_string(),
                        access_type: AccessType::Rw,
                        pdo_mapping: false,
                        value: Value::Unsigned8(0),
                    },
                ],
            }
            .into(),
        ];
        for entry in objects {
            od.insert_object(entry);
        }

        let rpdo_mapping = od.read_rpdo_mapping(0).unwrap();

        let left = decode_rpdo(rpdo_mapping, &rpdo_data, &od).unwrap();

        let right = vec![
            (0x2000, 1, Value::Boolean(true)),
            (0x2000, 2, Value::Unsigned8(0x17)),
        ];

        assert_eq!(left, right);
    }
}
