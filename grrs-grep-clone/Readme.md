Following https://rust-cli.github.io/book/tutorial/index.html
to build grrs a grep clone

End output 
```
$ cat test.txt
foo: 10
bar: 20
baz: 30
$ grrs foo test.txt
foo: 10
$ grrs --help
[some help text explaining the available options]
```