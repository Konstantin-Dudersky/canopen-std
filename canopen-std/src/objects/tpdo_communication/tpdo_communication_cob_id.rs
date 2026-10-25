use embedded_can::Id;

#[derive(Clone, Debug)]
pub enum TpdoCommunicationCobId {
    RelativeNodeId,
    Specific(Id),
}
