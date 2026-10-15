use bitvec::{
    order::{Lsb0, Msb0},
    view::BitView,
};

use crate::{
    core::object_dictionary::ObjectDictionary, objects::RpdoMapping, Address, Entry, Error, Value,
};

/// Расшифровываем данные RPDO
fn decode_rpdo(
    rpdo_maping: RpdoMapping,
    rpdo_data: &[u8],
    od: &ObjectDictionary,
) -> Result<Vec<(Address, Value)>, Error> {
    if rpdo_data.len() * 8 < rpdo_maping.bit_size() {
        return Err(Error::BadLength);
    }

    let bits = rpdo_data.view_bits::<Msb0>();
    let mut start_bit = 0;

    let mut values = vec![];

    for map in rpdo_maping.items {
        let address = Address::new(map.index, map.subindex);
        let od_entry = od.entry(address).unwrap();

        let od_size = od_entry.value.data_type().bit_size();
        let rpdo_size = map.length as usize;
        if od_size != rpdo_size {
            return Err(Error::TypeMismatch);
        }

        let bit_slice = &bits[start_bit..(start_bit + od_size)];
        let value = Value::decode(od_entry.value.data_type(), bit_slice)?;
        start_bit += od_size;

        values.push((address, value));
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use canopen_rs::NodeId;

    use pretty_assertions::assert_eq;

    use crate::{
        objects::{self, ObjectTrait},
        AccessType, Address, Value,
    };

    use super::*;

    #[test]
    fn test_decode_rpdo() {
        let node = NodeId::new(0x02).unwrap();
        let rpdo_data: Vec<u8> = vec![0b1_0001011, 0b1_0000000];
        let mut od = ObjectDictionary::new(node);

        // Заполняем словарь объектов
        let entries = [
            objects::RpdoMapping {
                sequence_number: 0,
                items: vec![
                    objects::PdoMappingItem {
                        index: 0x2000,
                        subindex: 1,
                        length: 1,
                    },
                    objects::PdoMappingItem {
                        index: 0x2000,
                        subindex: 2,
                        length: 8,
                    },
                ],
            }
            .entries(),
            vec![
                Entry {
                    address: Address::new(0x2000, 1),
                    value: Value::Boolean(false),
                    access: AccessType::Rw,
                },
                Entry {
                    address: Address::new(0x2000, 2),
                    value: Value::Unsigned8(0),
                    access: AccessType::Rw,
                },
            ],
        ]
        .concat();
        for entry in entries {
            od.insert(entry.address, entry).unwrap();
        }

        let rpdo_mapping = od.read_rpdo_mapping(0).unwrap();

        let left = decode_rpdo(rpdo_mapping, &rpdo_data, &od).unwrap();

        let right = vec![
            (Address::new(0x2000, 1), Value::Boolean(true)),
            (Address::new(0x2000, 2), Value::Unsigned8(0x17)),
        ];

        assert_eq!(left, right);
    }
}
