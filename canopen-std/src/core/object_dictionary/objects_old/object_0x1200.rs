use bitvec::{field::BitField, order::Lsb0, view::BitView};
use embedded_can::{ExtendedId, Id, StandardId};

use super::{Address, Error, NodeId, ObjectDictionary};

/// COB-ID base for SDO client → server (request) frames: `0x600 + node id`.
pub const SDO_REQUEST_COB_BASE: u16 = 0x600;

/// COB-ID base for SDO server → client (response) frames: `0x580 + node id`.
pub const SDO_RESPONSE_COB_BASE: u16 = 0x580;

/// SDO server parameter
pub struct Object0x1200 {
    /// COB-ID client -> server (rx)
    pub cob_id_client_to_server: Id,

    /// COB-ID server -> client (tx)
    pub cob_id_server_to_client: Id,

    /// Node-ID of the SDO client
    pub _node_id_sdo_server: NodeId,
}

impl ObjectDictionary {
    /// SDO server parameter
    pub fn read_object_0x1200(&self) -> Result<Object0x1200, Error> {
        // COB-ID client -> server (rx) ------------------------------------------------------------
        let address = Address::new(0x1200, 1);
        let entry = self.entries.get(&address);
        let cob_id_client_to_server = match entry {
            Some(v) => {
                let v = v.value.get_unsigned32()?;
                decode_cob_id(v)?
            }
            None => default_cob_id_client_to_server(self.node_id)?,
        };

        // COB-ID server -> client (tx) ------------------------------------------------------------
        let address = Address::new(0x1200, 2);
        let entry = self.entries.get(&address);
        let cob_id_server_to_client = match entry {
            Some(v) => {
                let v = v.value.get_unsigned32()?;
                decode_cob_id(v)?
            }
            None => default_cob_id_server_to_client(self.node_id)?,
        };

        let _node_id_sdo_server = NodeId::BROADCAST;

        let res = Object0x1200 {
            cob_id_client_to_server,
            cob_id_server_to_client,
            _node_id_sdo_server,
        };

        Ok(res)
    }
}

fn default_cob_id_client_to_server(node_id: NodeId) -> Result<Id, Error> {
    let id =
        StandardId::new(node_id.raw() as u16 + SDO_REQUEST_COB_BASE).ok_or(Error::InvalidCobId)?;

    Ok(id.into())
}

fn default_cob_id_server_to_client(node_id: NodeId) -> Result<Id, Error> {
    let id =
        StandardId::new(node_id.raw() as u16 + SDO_RESPONSE_COB_BASE).ok_or(Error::InvalidCobId)?;

    Ok(id.into())
}

fn decode_cob_id(raw: u32) -> Result<Id, Error> {
    let bits = raw.view_bits::<Lsb0>();

    let frame = bits[29];
    let id = match frame {
        true => {
            let id_raw: u32 = bits[0..29].load_le();
            let id = ExtendedId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Extended(id)
        }
        false => {
            let id_raw: u16 = bits[0..11].load_le();
            let id = StandardId::new(id_raw).ok_or(Error::InvalidCobId)?;
            Id::Standard(id)
        }
    };

    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_decode_cob_id_client_to_server() {
        assert_eq!(
            decode_cob_id(0x0000_0602).unwrap(),
            Id::Standard(StandardId::new(0x602).unwrap())
        );
        assert_eq!(
            decode_cob_id(0x2000_0602).unwrap(),
            Id::Extended(ExtendedId::new(0x602).unwrap())
        );
        assert_eq!(
            decode_cob_id(0x2100_0602).unwrap(),
            Id::Extended(ExtendedId::new(0x100_0602).unwrap())
        );
    }
}
