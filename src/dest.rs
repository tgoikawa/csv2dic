use crate::*;
pub(crate) fn render(words: &Vec<Word>, template: &str) -> Result<String, Error> {
    let mut replace = String::new();
    replace.push_str("|key|name|summary|\n|---|---|---|\n");
    for word in words {
        replace.push('|');
        replace.push_str(word.key());
        replace.push('|');
        replace.push_str(word.name());
        replace.push('|');
        replace.push_str(word.summary());
        replace.push_str("|\n");
    }
    let result = template.replace("$dictionary", &replace);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    #[test_case(
        vec![],
        template::default(),
        include_str!("../resource/tests/expects/dest/case1.md")
        )]
    #[test_case(
        vec![
        Word::new("key1".to_string(),"name1".to_string(),"summary1".to_string()),
        Word::new("key2".to_string(),"name2".to_string(),"summary2".to_string())
        ],
        template::default(),
        include_str!("../resource/tests/expects/dest/case2.md")
        )]
    fn render_works(words: Vec<Word>, template: &str, expected: &str) -> Result<(), Error> {
        let actual = render(&words, template)?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
