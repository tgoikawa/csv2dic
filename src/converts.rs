use crate::*;
use async_std::fs::File;
use async_std::io::{Read, Write};
use async_std::path::Path;
use async_std::prelude::*;
use std::convert::AsRef;

pub fn convert(src: &str) -> Result<String, Error> {
    let words = csv_src::read_csv(src)?;
    dest::render(&words, template::default())
}

pub async fn convert_to<R: ?Sized + Read + Unpin, W: ?Sized + Write + Unpin>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), Error>
where
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).await?;
    let result = convert(&buffer)?;
    writer.write_all(result.as_bytes()).await?;
    Ok(())
}

pub async fn convert_to_file<P: AsRef<Path>>(input: P, output: P) -> Result<(), Error> {
    convert_to(
        &mut File::open(input).await?,
        &mut File::create(output).await?,
    )
    .await
}
