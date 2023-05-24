use polars::prelude::*;

const FILENAME: &str = "./data/2020413_raw_utf8.csv";

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_ignore_errors(true)
        .with_encoding(CsvEncoding::Utf8)
        .finish()?;
    let df = lf.collect()?;

    println!("{}", df.head(Some(5)));
    Ok(())
}
