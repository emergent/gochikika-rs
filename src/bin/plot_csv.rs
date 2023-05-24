use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use plotters::prelude::*;
use polars::prelude::*;
use std::ops::Range;

const FILENAME: &str = "./data/2020413_utf8.csv";
const OUT_FILENAME: &str = "./plotters-images/plot_csv.png";
const FONT_STYLE: (&str, f64) = ("Hiragino Maru Gothic Pro", 20.0);
const FIG_SIZE: (u32, u32) = (640, 300);

fn value_to_utc(value: AnyValue) -> anyhow::Result<DateTime<Utc>> {
    let x = NaiveDateTime::from(&value);
    let d = x
        .and_local_timezone(Utc)
        .single()
        .context("couldn't convert to UTC timestamp.")?;
    Ok(d)
}

fn df_to_utc(df: &DataFrame) -> anyhow::Result<DateTime<Utc>> {
    value_to_utc(df.column("")?.get(0)?)
}

fn get_date_range(
    lf: &LazyFrame,
) -> anyhow::Result<Range<DateTime<Utc>>> {
    let from = lf.clone().select([col("")]).min().collect()?;
    let to = lf.clone().select([col("")]).max().collect()?;

    Ok(Range {
        start: df_to_utc(&from)?,
        end: df_to_utc(&to)?,
    })
}

fn get_value_range(series: &Series) -> anyhow::Result<Range<f64>> {
    Ok(Range {
        start: series.min().context("couldn't find a min value")?,
        end: series.max().context("couldn't find a max value")?,
    })
}

fn plot_figs(lf: &LazyFrame, col_name: &str) -> anyhow::Result<()> {
    let date_range = get_date_range(lf)?;

    let df = lf.clone().select([col(""), col(col_name)]).collect()?;

    let xs = &df[0];
    let ys = &df[col_name];

    let root =
        BitMapBackend::new(OUT_FILENAME, FIG_SIZE).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(col_name, FONT_STYLE.into_font())
        .build_cartesian_2d(date_range, get_value_range(ys)?)?;

    chart.draw_series(LineSeries::new(
        xs.iter().zip(ys.iter()).map(|(x, y)| {
            (value_to_utc(x).unwrap(), y.try_extract::<f64>().unwrap())
        }),
        BLUE.filled(),
    ))?;

    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| {
            format!("{:02}:{:02}", v.hour(), v.minute())
        })
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    root.present().context("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir")?;
    println!("Result has been saved to {}", OUT_FILENAME);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_ignore_errors(true)
        .with_encoding(CsvEncoding::Utf8)
        .finish()?;

    plot_figs(&lf, "コンプレッサー出口温度_PV")?;

    Ok(())
}