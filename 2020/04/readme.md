I used some unix utils to normalise the inputs to one line per documents.

```
tr ' ' '\n' < input.txt | tr '\n' ' ' | sed -e 's/  /\n/g' > normalized.txt
```