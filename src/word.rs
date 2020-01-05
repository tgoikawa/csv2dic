use crate::*;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidationError};
lazy_static! {
    static ref KEY_PATTERN: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

#[derive(Debug, Deserialize, Validate, PartialEq)]
pub(crate) struct Word {
    #[validate(length(min = 1), regex(path = "KEY_PATTERN"))]
    key: String,
    #[validate(length(min = 1))]
    name: String,
    #[validate(length(min = 1))]
    description: String,
}

impl Word {
    pub fn new(key: String, name: String, description: String) -> Word {
        Word {
            key,
            name,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case("key", "name", "des")]
    #[test_case("k", "n", "d")]
    fn validate_works(key: &str, name: &str, description: &str) -> Result<(), Error> {
        let word = Word::new(key.to_string(), name.to_string(), description.to_string());
        word.validate()?;
        Ok(())
    }

    #[test_case(
        "key",
        "name",
        "",
        [
            ("description",&vec![
                validation_error("length",Option::None,&[
                    ("min",json!(1)),
                    ("value",json!("")),
                ]),
            ])
        ].iter().cloned().collect::<HashMap<_,_>>()
    )]
    #[test_case(
        "",
        "name",
        "desc",
        [
            ("key",&vec![
                validation_error("length",Option::None,&[
                    ("min",json!(1)),
                    ("value",json!("")),
                ]),
                validation_error("regex",Option::None,&[
                    ("value",json!("")),
                ]),
            ])
        ].iter().cloned().collect::<HashMap<_,_>>()
    )]
    #[test_case(
        "k",
        "",
        "desc",
        [
            ("name",&vec![
                validation_error("length",Option::None,&[
                    ("min",json!(1)),
                    ("value",json!("")),
                ]),
            ])
        ].iter().cloned().collect::<HashMap<_,_>>()
    )]
    fn validte_error_works(
        key: &str,
        name: &str,
        description: &str,
        expected: HashMap<&str, &Vec<ValidationError>>,
    ) {
        let word = Word::new(key.to_string(), name.to_string(), description.to_string());
        let result = word.validate();
        assert!(result.is_err());
        assert_eq!(expected, result.err().unwrap().field_errors());
    }

    fn validation_error(
        code: &str,
        message: Option<&str>,
        params: &[(&str, Value)],
    ) -> ValidationError {
        ValidationError {
            code: code.to_string().into(),
            message: message.map(|m| m.to_string().into()),
            params: params
                .iter()
                .map(|p| -> (Cow<'static, str>, Value) { (p.0.to_string().into(), p.1.clone()) })
                .collect::<HashMap<_, _>>(),
        }
    }
}
