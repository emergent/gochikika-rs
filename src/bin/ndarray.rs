use anyhow::Context;
use ndarray::prelude::*;

fn main() -> anyhow::Result<()> {
    let a = arr1(&[1., 2., 4., 5., 7., 9.]);
    let a = a.view();
    let a_mean = a.mean().context("no value")?;
    let a_var = a.var(0.);
    println!("mean: {}", a_mean);

    let a2 = a.slice(s![2..]).map(|&ai| ai - a_mean);
    let a3 = a.slice(s![..a.len() - 2]).map(|&ai| ai - a_mean);

    println!("{:?}", a2);
    println!("{:?}", a3);

    let a_product = a2 * a3;
    println!("{:?}", a_product);
    println!("{:?}", a_product.sum() / a_var);

    Ok(())
}
