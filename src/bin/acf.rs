use anyhow::Context;
use ndarray::prelude::*;
use polars::prelude::*;

const FILENAME: &str = "./data/2020413_utf8.csv";
const OUT_FILENAME: &str = "./plotters-images/acf.png";

const N_LAGS: usize = 40;
const COL_NAME: &str = "気化器温度_PV";
//const COL_NAME: &str = "蒸留塔第5トレイ温度_PV";

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_null_values(Some(NullValues::AllColumnsSingle(
            "None".into(),
        )))
        .finish()?;

    let df = lf.clone().select([col(COL_NAME)]).collect()?;

    // ndarrayに変換して操作する
    let x =
        df.column(COL_NAME)?.as_list().to_ndarray::<Float64Type>()?;
    let x = x.t();
    let x = x.row(0);
    let x = x.slice(s![..;60]);

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

use plotters::prelude::*;
use std::ops::Range;

fn plot(v: Vec<f64>) -> anyhow::Result<()> {
    let root = BitMapBackend::new(OUT_FILENAME, (1024, 768))
        .into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Autocorrelation", ("Hiragino sans", 20.0).into_font())
        .build_cartesian_2d(-1..v.len() as i32 + 1, to_range(&v)?)?;

    chart.draw_series(
        v.iter().enumerate().map(|(x, &y)| {
            Circle::new((x as i32, y), 2, BLUE.filled())
        }),
    )?;

    chart.draw_series(v.iter().enumerate().map(|(x, &y)| {
        PathElement::new(vec![(x as i32, 0.0), (x as i32, y)], BLUE)
    }))?;

    chart.configure_mesh().draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
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

// 気化器温度_PV   mean: 120.18179555484978        var: 4337.590769644328
