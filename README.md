## split

```shell
rustex split -t ./testfile/test1.txt -r "^第[1-9]章"
```

## sortcsv

```shell
# 昇順
rustex sortcsv -t ./testfile/test2.csv -c id
# 降順
rustex sortcsv -t ./testfile/test2.csv -c id -r
```

## groupby

```shell
rustex groupby -t ./testfile/test2.csv -c name
```

## agrregate

```shell
rustex aggregate -t ./testfile/test2.csv -k name -c score use
```

## head

```shell
# 10行出力
rustex head -t ./testfile/test2.csv -l 10
# CSVファイルのヘッダとインデックス番号を出力
rustex head -t ./testfile/test2.csv -c
```

## excol

```shell
# nameとscore列のみ抽出
rustex excol -t ./testfile/test2.csv -c name score
```

## clean

```shell
# 先頭文字が2か3で始まる行を削除
rustex clean -t ./testfile/test2.csv -r "^[2-3],"
```

## collect

```shell
# maruという文字を含むテキストファイルを収集
rustex collect -t ./test -r "maru"
# maruという文字を含むテキストファイルを収集し、collectというフォルダに出力
rustex collect -t ./test -r "^maru" ./collect
```

## grep

```shell
# 「これは」で始まる行のみを残す
rustex grep -t ./testfile/test1.txt -r ^これは
# CSVのヘッダー行を残して「1,」で始まる行を残す
rustex grep -t ./testfile/test2.csv -r ^1, -c
```

## blocksplit

```shell
# idの値毎にファイルに分割して出力
rustex blocksplit -t ./testfile/test3-blocksplit.txt -c id
```

## red

```shell
# 「Rust」という文字を「Rust言語」に置換
rustex red -t ./testfile/test4-red.txt -r "Rust" -s "Rust言語"
```

## sum

```shell
# score列の値を全て加算
rustex sum -t ./testfile/test2.csv -c score
```

## ctoj

```shell
rustex ctoj -t ./testfile/test2.csv
```

## lastrow

```shell
rustex lastrow -t ./testfile/test1.txt
```