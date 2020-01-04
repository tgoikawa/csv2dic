use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Word {
    key: String,
    name: String,
    description: String,
}

impl Word {
    pub fn new(key: &str, name: &str, description: &str) -> Word {
        Word {
            key: key.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
