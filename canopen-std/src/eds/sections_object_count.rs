use crate::types::Object;

pub struct ObjectCount {}

impl ObjectCount {
    pub fn to_eds_str(&self, objects: &[Object]) -> String {
        let mut mandatory = vec![];
        let mut optional = vec![];
        let mut manufacturer = vec![];

        for obj in objects {
            match obj.index() {
                0x1000 | 0x1001 | 0x1018 => mandatory.push(obj.index()),
                (0x1000..=0x1FFF) | (0x6000..=0xFFFF) => optional.push(obj.index()),
                (0x2000..=0x5FFF) => manufacturer.push(obj.index()),
                _ => unimplemented!(),
            }
        }

        let mut result = String::new();

        result.push_str("\n[MandatoryObjects]\n");
        result.push_str(&format!("SupportedObjects={}\n", mandatory.len()));
        for (count, index) in mandatory.iter().enumerate() {
            let count = count + 1;
            result.push_str(&format!("{count}=0x{index:04X}\n"));
        }

        result.push_str("\n\n[OptionalObjects]\n");
        result.push_str(&format!("SupportedObjects={}\n", optional.len()));
        for (count, index) in optional.iter().enumerate() {
            let count = count + 1;
            result.push_str(&format!("{count}=0x{index:04X}\n"));
        }

        result.push_str("\n\n[ManufacturerObjects]\n");
        result.push_str(&format!("SupportedObjects={}\n", manufacturer.len()));
        for (count, index) in manufacturer.iter().enumerate() {
            let count = count + 1;
            result.push_str(&format!("{count}=0x{index:04X}\n"));
        }

        result
    }
}
