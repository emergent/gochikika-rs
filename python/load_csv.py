import polars as pl

FILENAME = "../data/2020413_raw_utf8.csv"

lf = pl.scan_csv(
    FILENAME,
    has_header=True,
    try_parse_dates=True,
    encoding="utf8",
    ignore_errors=True,
)

print(lf.collect().head())
