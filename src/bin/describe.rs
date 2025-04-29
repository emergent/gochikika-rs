use polars::prelude::*;

const FILENAME: &str = "./data/2020413_raw_utf8.csv";

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new(FILENAME)
        .with_try_parse_dates(true)
        .has_header(true)
        .with_ignore_errors(true)
        .with_encoding(CsvEncoding::Utf8)
        .finish()?;

    let df = lf.clone().collect()?;

    let names = df.get_column_names();
    println!("{:?}", names);

    println!("{}", df.describe(None)?);
    println!("{}", df.mean()); // 平均
    println!("{}", df.var(0)); // 分散
    println!("{}", df.std(0)); // 標準偏差
    println!("{}", df.min()); // 最小値
    println!("{}", df.max()); // 最大値
    println!("{}", df.median()); // 中央値

    println!(
        "{}",
        df.quantile(0.05, QuantileInterpolOptions::Nearest)?
    ); // 分位点

    for name in names {
        let series = df.column(name)?;

        // 平均
        let Some(mean) = series.mean() else { continue };

        // 分散
        let var = series
            .iter()
            .filter(|x| x.clone().try_extract::<f64>().is_ok())
            .map(|x| {
                let x = x.try_extract::<f64>().unwrap();
                (x - mean) * (x - mean)
            })
            .sum::<f64>();

        println!("{}\tmean: {}\tvar: {}", name, mean, var);

        // 歪度
        let Some(skew) = series.skew(false)? else {
            continue;
        };
        // 尖度
        let Some(kurt) = series.kurtosis(false, false)? else {
            continue;
        };
        println!("{}\tskew: {}\tkurt: {}", name, skew, kurt);

        // 最頻値
        let mode = polars::prelude::mode::mode(series)?;
        println!("{}", mode);
    }

    // 共分散
    let cov = lf
        .clone()
        .select([cov(col("気化器圧力_PV"), col("気化器圧力_SV"), 1)
            .alias("cov")])
        .collect()?;
    println!("{}", cov);

    // 相関係数
    let corr = lf
        .select([pearson_corr(
            col("気化器圧力_PV"),
            col("気化器圧力_SV"),
            1,
        )
        .alias("corr")])
        .collect()?;
    println!("{}", corr);

    Ok(())
}
