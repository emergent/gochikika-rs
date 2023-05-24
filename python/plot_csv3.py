import polars as pl

FILENAME = "../data/2020413_utf8.csv"
df = pl.scan_csv(
    FILENAME,
    has_header=True,
    try_parse_dates=True,
    encoding="utf8",
    ignore_errors=True,
).collect()

import matplotlib.pyplot as plt
from matplotlib.dates import DateFormatter


def plot_figs(df):
    color_iter = itertools.cycle(
        ["#02A8F3", "#33B490", "#FF5151", "#B967C7"]
    )
    cols = df.columns[1:]  # 時刻の列以外の列を取り出す
    n_figs = len(cols)
    fig = plt.figure(figsize=(6, 2.0 * n_figs), dpi=100)
    x = df[df.columns[0]]  # 時刻の列
    for i, col_name in enumerate(cols):  # 変更
        y = df[col_name]  # 変更
        title = col_name  # 変更
        ax = fig.add_subplot(n_figs, 1, i + 1)
        ax.plot(x, y, color=next(color_iter), linewidth=1.0)
        ax.xaxis.set_major_formatter(DateFormatter("%H:%M"))
        ax.set_xlim(x[0], x[-1])
        ax.set_title(title, fontsize=12)
    fig.tight_layout()
    plt.show()
    plt.close()


plot_list = [
    "",
    "気化器液面レベル_PV",
    "気化器液面レベル_SV",
    "気化器液面レベル_MV",
    "コンプレッサー出口温度_PV",
    "コンプレッサー出口温度_SV",
]
plot_figs(df.select(map(pl.col, plot_list)))
