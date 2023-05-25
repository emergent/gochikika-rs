use anyhow::Context;
use ndarray::prelude::*;
use plotters::prelude::*;
use polars::prelude::*;
use std::ops::Range;

const FILENAME: &str = "./data/2020413_utf8.csv";
const OUT_FILENAME: &str = "./plotters-images/acf.png";

const N_LAGS: usize = 40;
const COL_NAME: &str = "気化器温度_PV";
const FONT_STYLE: (&str, f64) = ("Hiragino Maru Gothic Pro", 20.0);
const FIG_SIZE: (u32, u32) = (640, 480);

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_ignore_errors(true)
        .with_encoding(CsvEncoding::Utf8)
        .finish()?;

    let df = lf.select([col(COL_NAME)]).collect()?;

    // ndarrayに変換して操作する
    let x =
        df.column(COL_NAME)?.as_list().to_ndarray::<Float64Type>()?;
    let x = x.t();
    let x = x.row(0);
    let x = x.slice(s![..;60]); // 60秒ごとに間引く

    // 自己相関関数の計算
    let x_mean = x.mean().context("no value")?;
    let mut v = vec![1.];
    for i in 1..=N_LAGS {
        let a1 = x.slice(s![i..]).map(|&xi| xi - x_mean);
        let a2 = x.slice(s![..x.len() - i]).map(|&xi| xi - x_mean);
        let a3 = x.map(|&xi| (xi - x_mean) * (xi - x_mean)).sum();
        let acf = (a1 * a2).sum() / a3;
        v.push(acf);
    }

    println!("{:?}", v);
    plot(v)?;

    Ok(())
}

fn plot(v: Vec<f64>) -> anyhow::Result<()> {
    let root =
        BitMapBackend::new(OUT_FILENAME, FIG_SIZE).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Autocorrelation", FONT_STYLE.into_font())
        .build_cartesian_2d(-1..v.len() as i32 + 1, to_range(&v)?)?;

    // 点をプロット
    chart.draw_series(
        v.iter().enumerate().map(|(x, &y)| {
            Circle::new((x as i32, y), 2, BLUE.filled())
        }),
    )?;

    // 垂直の縦線を引く
    chart.draw_series(v.iter().enumerate().map(|(x, &y)| {
        PathElement::new(vec![(x as i32, 0.0), (x as i32, y)], BLUE)
    }))?;

    chart.configure_mesh().draw()?;

    root.present().context("Unable to write result to file")?;
    println!("Result has been saved to {}", OUT_FILENAME);

    Ok(())
}

fn to_range(v: &[f64]) -> anyhow::Result<Range<f64>> {
    let s = Series::from_vec("acf", v.to_owned());

    Ok(Range {
        start: s.min().context("no value")?,
        end: s.max().context("no value")?,
    })
}
