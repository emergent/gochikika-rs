# gochikika-rs

[ごきちか](https://gochikika.ntt.com/index.html)を、[Polars](https://www.pola.rs/)＋[Plotters](https://github.com/plotters-rs/plotters)でやるチャレンジ。

## 使用するデータ

プロジェクトルート直下に`data`ディレクトリを作成し、その中に[ここ](https://gochikika.ntt.com/Introduction/datasets.html)から取得したCSVを置いてください。

そのままでは列名が文字化けする（Shift-JISのため）ので、UTF-8に文字コード変換してファイル名も変更してください。

```shell
nkf -S -w  < 2020413_raw.csv > 2020413_raw_utf8.csv
nkf -S -w  < 2020413.csv > 2020413_utf8.csv
```
