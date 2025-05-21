# Texas

テキストファイル処理に特化したコマンドラインツール。CSVファイルの集計処理も可能。

## split

指定した正規表現に一致する文字列で複数ファイルに分割する。サブコマンドに`split`を設定して使用。また`-t`オプションでターゲットとなるテキストファイルを指定し、`-r`オプションで正規表現を指定する。

```shell
texas split -t ./testfile/test1.txt -r "^第[1-9]章"
```

## sortcsv

列を指定し、CSVファイルをソートする。`-r`オプションで降順ソート。

```shell
# 昇順
texas sortcsv -t ./testfile/test2.csv -c id
# 降順
texas sortcsv -t ./testfile/test2.csv -c id -r
```

## groupby

CSVファイルの列内データの出現回数をカウントする。

```shell
texas groupby -t ./testfile/test2.csv -c name
```

## aggregate

CSVファイルのキー列と集計対象列（複数指定可能）を指定して合計とデータの個数を集計し、標準出力する。`-c`オプションでカンマ区切りで出力。

```shell
texas aggregate -t ./testfile/test2.csv -k name -c score use
```

## head

テキストファイルの先頭行を表示する。`-c`オプションでCSVファイルのヘッダとインデックス番号を表示する。

```shell
# 10行出力
texas head -t ./testfile/test2.csv -l 10
# CSVファイルのヘッダとインデックス番号を出力
texas head -t ./testfile/test2.csv -c
```

## excol

CSVファイルの指定した列のみを抽出する。

```shell
# nameとscore列のみ抽出
texas excol -t ./testfile/test2.csv -c name score
```

## clean

指定した正規表現に一致する文字列を含む行を削除する。サブコマンドに`clean`を設定して使用。また`-t`オプションでターゲットとなるテキストファイルを指定し、`-r`オプションで正規表現を指定する。

```shell
# 先頭文字が2か3で始まる行を削除
texas clean -t ./testfile/test2.csv -r "^[2-3],"
```

## collect

指定した正規表現に一致する文字列を含むファイルを収集する。サブコマンドに`collect`を設定して使用。また`-t`オプションでターゲットとなるテキストファイルが保存されているディレクトリを指定し、`-r`オプションで正規表現を指定する。

```shell
# maruという文字を含むテキストファイルを収集
texas collect -t ./test -r "maru"
# maruという文字を含むテキストファイルを収集し、collectというフォルダに出力
texas collect -t ./test -r "^maru" ./collect
```

## grep

指定した正規表現に一致する文字列を含む行を抽出してファイルに出力する。サブコマンドに`grep`を設定して使用。また`-t`オプションでターゲットとなるテキストファイルを指定し、`-r`オプションで正規表現を指定する。`-o`オプションで出力するファイル名を任意で指定可能。

```shell
# 「これは」で始まる行のみを残す
texas grep -t ./testfile/test1.txt -r ^これは
# CSVのヘッダー行を残して「1,」で始まる行を残す
texas grep -t ./testfile/test2.csv -r ^1, -c
```

## blocksplit

CSVファイルの列を指定し、値が一致する部分を塊でファイルに分割。サブコマンドに`blocksplit`を設定して使用。`-t`オプションでターゲットとなるテキストファイルを指定し、`-c 列番号`でCSVファイルの列を指定する。なお、CSVファイルはソート済みである必要がある。

```shell
# idの値毎にファイルに分割して出力
texas blocksplit -t ./testfile/test3-blocksplit.txt -c id
```

## red

指定した正規表現に一致する文字列を置換する。サブコマンドに`red`を設定して使用。また`-t`オプションでターゲットとなるテキストファイルが保存されているディレクトリを指定し、`-r`オプションで正規表現、`-s`で置換後の文字列を指定する。`-o`オプションで出力するファイル名を任意で指定可能。

```shell
# 「Rust」という文字を「Rust言語」に置換
texas red -t ./testfile/test4-red.txt -r "Rust" -s "Rust言語"
```

## sum

CSVファイルの指定列の値を合計する。

```shell
# score列の値を全て加算
texas sum -t ./testfile/test2.csv -c score
```

## ctoj

CSVファイルをJSONに変換して標準出力する。同じ列名が存在する場合は配列として出力。

```shell
texas ctoj -t ./testfile/test2.csv
```

## lastrow

テキストファイルの最終行のみをファイルに抜き出す。

```shell
texas lastrow -t ./testfile/test1.txt
```

## wc

テキストファイルの行数をカウントする。

```shell
# 行数をカウント（最後の改行は含まない）
texas wc -t ./testfile/test1.txt -l
# 文字数をカウント（改行コードなどは含まない）
texas wc -t ./testfile/test1.txt -m
```

## csvtree

CSVファイルをネストした2グループにカテゴライズされるデータを、JSONツリーの構造に変換し、出現回数をカウントアップと計算可能な値を累積加算してJSONとして出力するコマンド。

```shell
texas csvtree -t ./testfile/test6.csv -c category -k name -C origin grade size -S size
```

## sumcol

指定列を加算して、新しい列を作成する`sumcol` サブコマンド。指定列が文字列であった場合は加算ではなく、文字列を結合したものが値となる。

```bash
texas sumcol -t ./testfile/test2.csv -c score use -s score+use 
```

## utf8

文字コードが SHIFT-JIS のテキストファイルを UTF-8 に変換するコマンド。

```bash
texas utf8 -t ./testfile/test7-shift-jis.csv > utf8.csv
# UTF8なので標準出力も特に問題なく可能
texas utf8 -t ./testfile/test7-shift-jis.csv
```

## shiftjis

文字コードが UTF-8 のテキストファイルを SHIFT-JIS に変換するコマンド。なお、標準出力の文字コードは通常 UTF-8 である。このため、Linux環境下では、文字化けが発生したり、Windows環境下では標準出力（コンソール）では、非UTF-8のバイト列を直接出力できないため、エラーとなる。基本的にはファイルに対してリダイレクトするコマンド。

```bash
texas shiftjis -t ./testfile/test8-utf8.csv > shift-jis.csv
```