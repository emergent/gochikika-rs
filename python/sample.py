import polars as pl

# Eager
df = (
    pl.read_csv("../data/sample.csv")
    .groupby("b")
    .agg([pl.count().alias("d"), pl.col("c").mean().alias("e")])
    .sort("d")
)
print(df)

# Lazy
lf = (
    pl.scan_csv("../data/sample.csv")
    .groupby("b")
    .agg([pl.count().alias("d"), pl.col("c").mean().alias("e")])
    .sort("d")
)
print(lf.collect())

lf.show_graph(optimized=True, show=False, output_path="optimized.png")
lf.show_graph(
    optimized=False, show=False, output_path="no_optimized.png"
)
