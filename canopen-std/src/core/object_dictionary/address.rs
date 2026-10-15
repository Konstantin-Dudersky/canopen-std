/// The address of an object dictionary entry: a 16-bit index and 8-bit
/// subindex.
///
/// Subindex `0` conventionally holds the "number of entries" for `RECORD` and
/// `ARRAY` objects; simple `VAR` objects use subindex `0` for the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address {
    /// 16-bit object index.
    pub index: u16,
    /// 8-bit subindex.
    pub subindex: u8,
}

impl Address {
    /// Construct an address from an index and subindex.
    pub const fn new(index: u16, subindex: u8) -> Self {
        Self { index, subindex }
    }
}
