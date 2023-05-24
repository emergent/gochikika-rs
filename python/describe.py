import warnings

warnings.filterwarnings("ignore")

import datetime

import numpy as np
import pandas as pd

FILE_PATH = "../data/2020413_raw_utf8.csv"
df = pd.read_csv(
    FILE_PATH, index_col=0, parse_dates=True, encoding="utf8"
)

df = df["気化器温度_PV"]

print(df.head())
print(df.mean())
print(df.var())
