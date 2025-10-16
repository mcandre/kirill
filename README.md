# kirill: JSON validator

# ABOUT

kirill scans projects large and small for JSON correctness:

* Check for basic JSON(5) syntactical validity
* Identify JSON(5) files recursively in large project directories
* Verify compliance with a [JSON Schema](https://json-schema.org/)

# EXAMPLES

```console
$ cd examples

$ kirill .
error: fruit-missing-end-brace.json: EOF while parsing an object at line 3 column 0
error: fruit-trailing-comma.json: trailing comma at line 3 column 1
error: fruit-unquoted-key.json: key must be a string at line 2 column 5
error: fruit-with-comment.json: expected value at line 1 column 1
error: settings.json: expected value at line 1 column 1

$ kirill --schema species.json zoo
error: zoo/bad-bear.json: Missing required property 'species'

$ kirill books.json5
error: books.json5: expected value at line 9 column 9

$ kirill --json5 books.json5
$
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
