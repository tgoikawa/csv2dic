use crate::*;
use std::io::prelude::*;
use std::io::Cursor;

pub(crate) fn read_csv<R: Read>(reader: R) -> Result<Vec<Word>, Error> {
    let mut rdr = csv::Reader::from_reader(reader);
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
                Word::new("abc","word_name","desc")
            ]; "1 rows"
        )]
    #[test_case(&[] ,vec![]; "nothing rows")]
    fn read_csv_works(given: &RowArray, expected: Vec<Word>) -> Result<(), Error> {
        let src = &with_header(given);
        let cursor = Cursor::new(src);
        let actulal = read_csv(cursor)?;
        assert_eq!(&expected[..], &actulal[..]);
        Ok(())
    }
    #[test_case("read io error occured")]
    fn read_csv_error_works(expected: &str) {
        let result = read_csv(&mut MockErrorRead {});
        assert!(result.is_err());
        assert_eq!(expected, format!("{}", result.err().unwrap()));
    }

    fn with_header(rows: &RowArray) -> String {
        [&[["key", "name", "description"]], rows]
            .concat()
            .iter()
            .map(|rows| rows.join(","))
            .collect::<Vec<_>>()
            .join("\n")
    }

    struct MockErrorRead;
    use std::io;
    impl Read for MockErrorRead {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "read io error occured",
            ))
        }
    }
}
