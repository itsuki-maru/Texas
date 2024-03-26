## split

```shell
rustex split -t ./testfile/test1.txt -r "^第[1-9]章"
```

## groupby

```shell
rustex groupby -t ./testfile/test2.csv -c name
```

## sortcsv

```shell
rustex sortcsv -t ./testfile/test2.csv -c name
```

## agrregate

```shell
rustex aggregate -t ./testfile/test2.csv -k name -c score use
```