use canopen_rs::NodeId;
use embedded_can::{Id, StandardId};

#[derive(Clone, Debug, PartialEq)]
pub enum PdoCommunicationCobId {
    /// COB-ID рассчитывается на основе ID узла и номера PDO (seq_number). Можно использовать на
    /// первых 4 PDO.
    ///
    /// Например:
    /// - RPDO с индексом 0x1401; seq_number = 1
    /// - адрес NodeId = 0x17
    /// - COB-ID = 0x300 + 0x17 = 0x317
    RelativeSelfNodeId,

    /// Явно заданный COB-ID
    Specific(Id),
}

impl PdoCommunicationCobId {
    /// Отправлять данные в RPDO другого узла
    pub const fn connect_to_rpdo_to_another_node(
        another_node_id: NodeId,
        seq_number: u16,
    ) -> PdoCommunicationCobId {
        let offset: u32 = match seq_number {
            0 => 0x200,
            1 => 0x300,
            2 => 0x400,
            3 => 0x500,
            _ => {
                panic!("Max seq_number for RPDO currently 3");
            }
        };
        let id = offset + another_node_id.raw() as u32;
        let id = Id::Standard(StandardId::new(id as u16).unwrap());
        PdoCommunicationCobId::Specific(id)
    }

    /// Получать данные из TPDO другого узла
    pub const fn connect_to_tpdo_from_another_node(
        another_node_id: NodeId,
        seq_number: u16,
    ) -> PdoCommunicationCobId {
        let offset: u32 = match seq_number {
            0 => 0x180,
            1 => 0x280,
            2 => 0x380,
            3 => 0x480,
            _ => {
                panic!("Max seq_number for TPDO currently 3");
            }
        };
        let id = offset + another_node_id.raw() as u32;
        let id = Id::Standard(StandardId::new(id as u16).unwrap());
        PdoCommunicationCobId::Specific(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_rpdo_to_another_node() {
        let another_node_id = NodeId::new(0x17).unwrap();
        let seq_number = 1;
        let id =
            PdoCommunicationCobId::connect_to_rpdo_to_another_node(another_node_id, seq_number);
        assert_eq!(
            id,
            PdoCommunicationCobId::Specific(Id::Standard(StandardId::new(0x317).unwrap()))
        );
    }

    #[test]
    fn test_tpdo_from_another_node() {
        let another_node_id = NodeId::new(0x17).unwrap();
        let seq_number = 1;
        let id =
            PdoCommunicationCobId::connect_to_tpdo_from_another_node(another_node_id, seq_number);
        assert_eq!(
            id,
            PdoCommunicationCobId::Specific(Id::Standard(StandardId::new(0x297).unwrap()))
        );
    }
}
