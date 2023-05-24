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

x = df[df.columns[0]]  # x軸: 時刻
y = df[df.columns[1]]  # データの一列目. 「:」で行すべて選択、「0」で1列目を選択

from matplotlib.dates import DateFormatter

plt.rcParams["font.family"] = "Hiragino Maru Gothic Pro"
fig = plt.figure(figsize=(7, 4), dpi=100)
ax = fig.add_subplot(1, 1, 1)
ax.plot(x, y, color="#02A8F3", linewidth=1.0)
ax.xaxis.set_major_formatter(DateFormatter("%m/%d\n%H:%M"))  # 日付を2段組に
ax.set_xlim(x[0], x[-1])
ax.set_title("時系列プロット")
plt.show()
