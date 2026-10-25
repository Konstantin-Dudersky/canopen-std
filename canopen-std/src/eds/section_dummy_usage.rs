pub struct DummyUsage {}

impl DummyUsage {
    pub fn to_eds_str(&self) -> String {
        let dummy_usage = "
[DummyUsage]
Dummy0001=1
Dummy0002=1
Dummy0003=1
Dummy0004=1
Dummy0005=1
Dummy0006=1
Dummy0007=1

";
        dummy_usage.to_string()
    }
}
