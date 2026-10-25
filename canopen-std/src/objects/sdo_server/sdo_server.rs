use canopen_rs::NodeId;
use embedded_can::Id;

/// 0x1200: SDO server parameter
pub struct SdoServer {
    /// COB-ID client -> server (rx)
    pub cob_id_client_to_server: Id,

    /// COB-ID server -> client (tx)
    pub cob_id_server_to_client: Id,

    /// Node-ID of the SDO client
    pub _node_id_sdo_server: NodeId,
}
