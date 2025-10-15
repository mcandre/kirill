# kirill: JSON Schema validator

# ABOUT

kirill scans projects for degenerate JSON files.

# EXAMPLES

```console
$ cd examples

$ kirill --basic .
error: fruit-missing-end-brace.json: EOF while parsing an object at line 3 column 0
error: fruit-trailing-comma.json: trailing comma at line 3 column 1
error: fruit-unquoted-key.json: key must be a string at line 2 column 5
error: fruit-with-comment.json: expected value at line 1 column 1
error: settings.json: expected value at line 1 column 1

$ kirill --schema species.json zoo
error: zoo/bad-bear.json: Missing required property 'species'
```

See `kirill -h` for more options.

# LICENSE

BSD-2-Clause

# RUNTIME REQUIREMENTS

(None)

## Recommended

* [jq](https://jqlang.github.io/jq/) 1.6+

# CONTRIBUTING

For more information on developing kirill itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# SEE ALSO

* [sail](https://github.com/mcandre/sail) identifies C/C++ source code
* [stank](https://github.com/mcandre/stank) identifies shell scripts
