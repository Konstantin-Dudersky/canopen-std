use std::time::Duration;

use bitvec::{field::BitField, order::Lsb0, view::BitView};
use canopen_rs::NodeId;

use super::{Address, Error, ObjectDictionary};

/// Consumer heartbeat time
#[derive(Debug)]
pub struct Object0x1016 {
    pub items: Vec<Object0x1016Item>,
}

impl ObjectDictionary {
    pub fn read_object_0x1016(&self) -> Result<Object0x1016, Error> {
        // Определяем количество элементов
        let address = Address::new(0x1016, 0);
        let entry = self.entries.get(&address);
        let Some(highest_subindex) = entry else {
            return Ok(Object0x1016 { items: vec![] });
        };
        let highest_subindex = highest_subindex.value.get_unsigned8()?;

        let mut items: Vec<Object0x1016Item> = vec![];

        for i in 1..=highest_subindex {
            let address = Address::new(0x1016, i);
            let entry = self.entries.get(&address);

            let Some(entry) = entry else {
                continue;
            };
            let value = entry.value.get_unsigned32()?;
            let item = decode_item(value);

            let Some(item) = item else {
                continue;
            };
            items.push(item);
        }

        Ok(Object0x1016 { items })
    }
}

#[derive(Debug)]
pub struct Object0x1016Item {
    pub node_id: NodeId,
    pub heartbeat_time: Duration,
}

fn decode_item(raw: u32) -> Option<Object0x1016Item> {
    let bits = raw.view_bits::<Lsb0>();
    let node_id = NodeId::new(bits[16..24].load_le()).ok()?;
    let heartbeat_time = Duration::from_millis(bits[0..16].load_le());
    Some(Object0x1016Item {
        node_id,
        heartbeat_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode_item() {
        let item = decode_item(0x000105DC).unwrap();
        assert_eq!(item.node_id, NodeId::new(0x01).unwrap());
        assert_eq!(item.heartbeat_time, Duration::from_millis(1_500));
    }
}
