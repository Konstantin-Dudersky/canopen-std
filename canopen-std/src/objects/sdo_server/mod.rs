use bitvec::{field::BitField, order::Lsb0, view::BitView};
use canopen_rs::NodeId;
use embedded_can::{ExtendedId, Id, StandardId};

use crate::{
    sdo::{default_cob_id_client_to_server, default_cob_id_server_to_client},
    Error,
};

use super::ObjectDictionary;

/// SDO server parameter
pub struct SdoServer {
    /// COB-ID client -> server (rx)
    pub cob_id_client_to_server: Id,

    /// COB-ID server -> client (tx)
    pub cob_id_server_to_client: Id,

    /// Node-ID of the SDO client
    pub _node_id_sdo_server: NodeId,
}

impl ObjectDictionary {
    /// SDO server parameter
    pub fn read_sdo_server(&self) -> Result<SdoServer, Error> {
        // COB-ID client -> server (rx) ------------------------------------------------------------
        let entry = self.get_entry(0x1200, 1);
        let cob_id_client_to_server = match entry {
            Some(v) => {
                let v = v.value.get_unsigned32()?;
                decode_cob_id(v)?
            }
            None => default_cob_id_client_to_server(self.node_id)?,
        };

        // COB-ID server -> client (tx) ------------------------------------------------------------
        let entry = self.get_entry(0x1200, 1);
        let cob_id_server_to_client = match entry {
            Some(v) => {
                let v = v.value.get_unsigned32()?;
                decode_cob_id(v)?
            }
            None => default_cob_id_server_to_client(self.node_id)?,
        };

        let _node_id_sdo_server = NodeId::BROADCAST;

        let res = SdoServer {
            cob_id_client_to_server,
            cob_id_server_to_client,
            _node_id_sdo_server,
        };

        Ok(res)
    }
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
