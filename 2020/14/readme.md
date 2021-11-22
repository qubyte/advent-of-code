Processed the input with:

```shell
tr '\n' ',' < input.txt | sed -e 's/,mask = /\n/g' -e 's/mask = //' -e 's/mem\[//g' -e 's/] = /=/g'  -e 's/,$//g' > processed.txt
```
