use embedded_can::Id;

#[derive(Clone, Debug)]
pub enum PdoCommunicationCobId {
    RelativeNodeId,
    Specific(Id),
}
