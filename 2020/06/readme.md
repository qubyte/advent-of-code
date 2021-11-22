Used to normalise the input:

```shell
sed 's/^$/:/g' < ../input.txt | tr '\n' ',' | sed 's/,:,/\n/g' > collected.txt
```

(I had to trim off a trailing comma)