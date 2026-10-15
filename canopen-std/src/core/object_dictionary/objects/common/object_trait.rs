use crate::Entry;

pub trait ObjectTrait {
    fn entries(self) -> Vec<Entry>;
}
