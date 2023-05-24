use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use plotters::prelude::*;
use polars::prelude::*;
use std::ops::Range;

const FILENAME: &str = "./data/2020413_utf8.csv";
const FONT_STYLE: (&str, f64) = ("Hiragino Maru Gothic Pro", 20.0);
const FIG_SIZE: (u32, u32) = (640, 300);
const OUT_DIR: &str = "plotters-images";

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_ignore_errors(true)
        .with_encoding(CsvEncoding::Utf8)
        .finish()?;

    let plot_list = [
        ("気化器液面レベル_PV", "level_pv"),
        ("気化器液面レベル_SV", "level_sv"),
        ("気化器液面レベル_MV", "level_mv"),
        ("コンプレッサー出口温度_PV", "comp_pv"),
        ("コンプレッサー出口温度_SV", "comp_sv"),
    ];

    for (col_name, file_name) in plot_list {
        plot_figs(&lf, col_name, file_name)?;
    }

    Ok(())
}

/// `LazyFrame`から指定した列のグラフを出力する
fn plot_figs(
    lf: &LazyFrame,
    col_name: &str,
    file_name: &str,
) -> anyhow::Result<()> {
    let date_range = get_date_range(lf)?;

    let df = lf.clone().select([col(""), col(col_name)]).collect()?;

    let xs = &df[0];
    let ys = &df[col_name];

    let name = format!("{}/plot_fig_{}.png", OUT_DIR, file_name);
    let root = BitMapBackend::new(&name, FIG_SIZE).into_drawing_area();
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
        .y_label_formatter(&|v| format!("{}", v))
        .draw()?;

    root.present().context("Unable to write result to file")?;
    println!("Result has been saved to {}", name);

    Ok(())
}

/// `AnyValue`の日時情報を`DateTime<Utc>`に変換する関数
fn value_to_utc(value: AnyValue) -> anyhow::Result<DateTime<Utc>> {
    let x = NaiveDateTime::from(&value);
    let d = x
        .and_local_timezone(Utc)
        .single()
        .context("couldn't convert to UTC timestamp.")?;
    Ok(d)
}

/// `DataFrame`から日時を1件抽出し`DateTime<Utc>`にして返す関数
fn df_to_utc(df: &DataFrame) -> anyhow::Result<DateTime<Utc>> {
    value_to_utc(df.column("")?.get(0)?)
}

/// `LazyFrame`の日時列の一番古い・新しい時刻で`Range`を作る
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

/// `f64`の`Series`の上限＆下限をRangeにして返す
fn get_value_range(series: &Series) -> anyhow::Result<Range<f64>> {
    Ok(Range {
        start: series.min().context("couldn't find a min value")?,
        end: series.max().context("couldn't find a max value")?,
    })
}
