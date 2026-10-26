use std::time::Duration;

use canopen_rs::NodeId;
use embedded_can::{Id, StandardId};
use tracing::error;

use crate::Error;

use super::super::PdoCommunicationCobId;
use super::TpdoTransmissionType;

/// 0x1800..=0x19FF: TPDO communication parameter
#[derive(Clone, Debug)]
pub struct TpdoCommunication {
    pub seq_number: u16,

    /// 1 = PDO exists
    pub pdo_exist: bool,

    /// COB-ID used by TPDO
    pub cob_id: PdoCommunicationCobId,

    /// transmission type
    pub transmission_type: TpdoTransmissionType,

    /// inhibit time
    pub inhibit_time: Duration,

    /// event timer
    pub event_timer: Duration,

    /// SYNC start value
    pub sync_start_value: u8,
}

impl TpdoCommunication {
    pub const SUBINDEX_COB_ID: u8 = 1;
    pub const SUBINDEX_TRANSMISSION_TYPE: u8 = 2;
    pub const SUBINDEX_INHIBIT_TIME: u8 = 3;
    pub const SUBINDEX_EVENT_TIMER: u8 = 5;
    pub const SUBINDEX_SYNC_START_VALUE: u8 = 6;

    pub const INDEX_BASE: u16 = 0x1800;

    pub const fn index(&self) -> u16 {
        Self::INDEX_BASE + self.seq_number
    }

    pub fn name(&self) -> String {
        format!("TPDO communication parameter #{}", self.seq_number + 1)
    }

    pub fn get_cob_id_offset(&self) -> Result<u32, Error> {
        let offset = match self.seq_number {
            0 => 0x180,
            1 => 0x280,
            2 => 0x380,
            3 => 0x480,
            _ => {
                error!("Max sequence_number for RPDO currently 3");
                return Err(Error::InvalidCobId);
            }
        };
        Ok(offset)
    }

    pub fn get_cob_id_for_node(&self, node_id: NodeId) -> Result<Id, Error> {
        match self.cob_id {
            PdoCommunicationCobId::RelativeSelfNodeId => {
                let cob_id_base = self.get_cob_id_offset()?;
                let cob_id = cob_id_base + node_id.raw() as u32;
                let id = StandardId::new(cob_id as u16).ok_or(Error::InvalidCobId)?;
                Ok(id.into())
            }
            PdoCommunicationCobId::Specific(id) => Ok(id),
        }
    }
}

impl Default for TpdoCommunication {
    fn default() -> Self {
        Self {
            seq_number: 0,
            pdo_exist: false,
            cob_id: PdoCommunicationCobId::RelativeSelfNodeId,
            transmission_type: TpdoTransmissionType::SyncronousAcyclic,
            inhibit_time: Duration::default(),
            event_timer: Duration::default(),
            sync_start_value: 0,
        }
    }
}
