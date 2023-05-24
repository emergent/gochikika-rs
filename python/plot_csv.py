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

fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
ax.plot(x, y)
plt.show()
