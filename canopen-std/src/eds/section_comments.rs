pub struct Comments {}

impl Comments {
    pub fn to_eds_str(&self) -> String {
        let comments = r#"
[Comments]
Lines=0
"#;
        comments.to_string()
    }
}
