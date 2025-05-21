# Release Notes

## Version 1.3.0

### utf8 サブコマンドの追加

SHIFT-JISの文字コードを使用するファイルをUTF-8に変換するコマンド。主にWindows ExcelなどをCSVファイルにエクスポートしたものを後続処理するために使用。

```bash
texas utf8 -t ./testfile/test7-shift-jis.csv > utf8.csv
# UTF8なので標準出力も特に問題なく可能
texas utf8 -t ./testfile/test7-shift-jis.csv
```

### shiftjis サブコマンドの追加

文字コードが UTF-8 のテキストファイルを SHIFT-JIS に変換するコマンド。主にUTF-8で処理したテキストファイル（CSV）をExcelでそのまま開けるようにするなどの用途で使用。デフォルトでは標準出力だが、SHIFT-JISで端末上にに出力することはない（文字化けやエラーが発生）ので、基本的にはファイルに対してリダイレクトして使用する。

```bash
texas shiftjis -t ./testfile/test8-utf8.csv > shift-jis.csv
```

## Version 1.2.0

### サブコマンドの追加

- 指定列を加算して、新しい列を作成する`sumcol` サブコマンドを追加
    - 指定列が文字列であった場合は加算ではなく、文字列を結合したものが値となる。

```bash
texas sumcol -t ./testfile/test2.csv -c 加算対象1 加算対象2 -s 列名
```

### 標準出力への変更

ファイル分割系以外のコマンド処理で、結果をファイル出力から標準出力に変更。

## Version 1.1.1

- `csvtree` サブコマンドの出力結果に累積加算の項目を追加
- `csvtree` サブコマンドのlong引数を修正

## Version 1.1.0

### サブコマンドの追加

- `csvtree` サブコマンドを追加

### ヘルプの修正

- 各サブコマンドのヘルプにexampleを追加

### 修正

- redサブコマンドの出力ファイル名拡張子が `.txx` となっていたため `.txt` に修正

## Version 1.0.0

- Release