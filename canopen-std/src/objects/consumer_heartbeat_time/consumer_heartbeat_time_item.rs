use std::time::Duration;

use bitvec::{field::BitField, order::Lsb0, view::BitView};

use crate::{
    Error,
    types::{NodeId, Value},
};

#[derive(Debug)]
pub struct ConsumerHeartbeatTimeItem {
    pub node_id: NodeId,
    pub heartbeat_time: Duration,
}

impl TryFrom<Value> for ConsumerHeartbeatTimeItem {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let value = value.get_unsigned32()?;

        let bits = value.view_bits::<Lsb0>();
        let node_id = NodeId::new(bits[16..24].load_le())?;
        let heartbeat_time = Duration::from_millis(bits[0..16].load_le());

        Ok(ConsumerHeartbeatTimeItem {
            node_id,
            heartbeat_time,
        })
    }
}

impl From<&ConsumerHeartbeatTimeItem> for Value {
    fn from(item: &ConsumerHeartbeatTimeItem) -> Self {
        let mut value = 0u32;

        let bits = value.view_bits_mut::<Lsb0>();
        bits[16..24].store_le(item.node_id.raw());
        bits[0..16].store_le(item.heartbeat_time.as_millis() as u16);

        Value::Unsigned32(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode_item() {
        let item: ConsumerHeartbeatTimeItem = Value::Unsigned32(0x000105DC).try_into().unwrap();
        assert_eq!(item.node_id, NodeId::new(0x01).unwrap());
        assert_eq!(item.heartbeat_time, Duration::from_millis(1_500));
    }

    #[test]
    fn test_encode_item() {
        let item: ConsumerHeartbeatTimeItem = ConsumerHeartbeatTimeItem {
            node_id: NodeId::new(0x01).unwrap(),
            heartbeat_time: Duration::from_millis(1_500),
        };
        assert_eq!(Value::from(&item), Value::Unsigned32(0x000105DC));
    }
}
