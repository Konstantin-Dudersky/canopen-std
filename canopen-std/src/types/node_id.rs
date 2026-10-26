use crate::Error;

/// A CANopen node identifier.
///
/// Valid *device* node ids are `1..=127`. The value `0` is reserved for
/// broadcast in NMT and LSS; use [`NodeId::BROADCAST`] for it and
/// [`NodeId::new`] for a checked device id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u8);

impl NodeId {
    /// The broadcast node id (`0`), used by NMT and LSS.
    pub const BROADCAST: NodeId = NodeId(0);

    /// Create a checked device node id in `1..=127`.
    ///
    /// Returns [`Error::InvalidNodeId`] for `0` or any value above `127`.
    pub fn new(id: u8) -> Result<Self, Error> {
        if (1..=127).contains(&id) {
            Ok(NodeId(id))
        } else {
            Err(Error::InvalidNodeId)
        }
    }

    /// The raw node id byte.
    pub const fn raw(self) -> u8 {
        self.0
    }

    pub fn checked_add(self, other: u8) -> Result<Self, Error> {
        let value = self
            .0
            .checked_add(other)
            .filter(|&v| (1..=127).contains(&v))
            .ok_or(Error::InvalidNodeId)?;
        Ok(NodeId(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn node_id_accepts_valid_range() {
        assert_eq!(NodeId::new(1).unwrap().raw(), 1);
        assert_eq!(NodeId::new(127).unwrap().raw(), 127);
    }

    #[test]
    fn node_id_rejects_out_of_range() {
        assert_matches!(NodeId::new(0), Err(Error::InvalidNodeId));
        assert_matches!(NodeId::new(128), Err(Error::InvalidNodeId));
    }

    #[test]
    fn broadcast_is_zero() {
        assert_eq!(NodeId::BROADCAST.raw(), 0);
    }
}
