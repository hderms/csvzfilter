# csvzgrep

# Installation
install rust/cargo

`cargo install --path .`


# Usage
```csvgrep --file /tmp/foo --pattern "1" --columns "foo"```


```text
Program which streams through a gzipped CSV, greps for rows which match some criteria and then streams those to stdout as plaintext CSV

Usage: csvgrep [OPTIONS] --file <FILE> --pattern <PATTERN>

Options:
  -f, --file <FILE>        path of the file
  -p, --pattern <PATTERN>  pattern to search by
  -c, --columns <COLUMNS>  fields we should search by name
  -h, --help               Print help
  -V, --version            Print version
```