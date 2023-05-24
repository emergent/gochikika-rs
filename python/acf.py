# Warningsの消去
import warnings

warnings.filterwarnings("ignore")

# モジュールの読み込み
from datetime import datetime, timedelta
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import statsmodels.api as sm

plt.rcParams["font.family"] = "Hiragino sans"

# データの読み込み
fname = "../data/2020413.csv"
df = pd.read_csv(
    fname,
    index_col=0,
    parse_dates=True,
    na_values="None",
    encoding="shift-jis",
)

# 例として1つ抜き出してみる
col_name = "気化器温度_PV"

acf = sm.tsa.stattools.acf(df[col_name])
print(acf)

nlags = 40
x = df[col_name].values
x_mean = np.mean(x)

acf_np = [
    np.sum((x - x_mean) * (x - x_mean) / np.sum((x - x_mean) ** 2))
]
for i in range(1, nlags + 1):
    _acf = np.sum((x[i:] - x_mean) * (x[:-i] - x_mean)) / np.sum(
        (x - x_mean) ** 2
    )
    acf_np.append(_acf)

acf_np = np.array(acf_np)
print(acf_np)  # 一致した結果が出てくる


""" fig = plt.figure(figsize=(8, 5))
ax1 = fig.add_subplot(111)
sm.graphics.tsa.plot_acf(df.loc[::60, col_name], lags=40, ax=ax1) #グラフを自動作成
ax1.set_xlabel('ラグ数')
ax1.set_ylabel('自己相関')
plt.show()
plt.close('all')
 """
