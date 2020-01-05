use crate::*;
use std::io::Cursor;

pub(crate) fn read_csv(body: &str) -> Result<Vec<Word>, Error> {
    let mut rdr = csv::Reader::from_reader(Cursor::new(body));
    let returns = rdr.deserialize::<Word>().collect::<Result<Vec<_>, _>>()?;
    Ok(returns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    type RowArray<'a> = [[&'a str; 3]];
    #[test_case(
            &[
                ["abc","word_name","desc"]
            ],
            vec![
                Word::new("abc".to_string(),"word_name".to_string(),"desc".to_string())
            ]; "1 rows"
        )]
    #[test_case(&[] ,vec![]; "nothing rows")]
    fn read_csv_works(given: &RowArray, expected: Vec<Word>) -> Result<(), Error> {
        let src = &with_header(given);
        let actulal = read_csv(src)?;
        assert_eq!(&expected[..], &actulal[..]);
        Ok(())
    }

    fn with_header(rows: &RowArray) -> String {
        [&[["key", "name", "description"]], rows]
            .concat()
            .iter()
            .map(|rows| rows.join(","))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
