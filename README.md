# kirill: a JSON document integrity checker

# ABOUT

kirill hunts down rogue JSON's. kirill is designed to identify JSON documents in large project directories. kirill can also feed these file paths to external linters.

# EXAMPLES

```console
$ cd examples

$ kirill .
./grocer-missing-document-envelope.json
./fruit-unquoted-key.json
./fruit.json
./.jsfmtrc
./grocer.json
./.eslintrc
./.jshintrc
./fruit-with-comment.json
./fruit-missing-end-brace.json
./.jslintrc
./fruit-trailing-comma.json

$ kirill -print0 . | xargs -0 -n 1 -t jq -r input_filename
jq -r input_filename ./grocer-missing-document-envelope.json
./grocer-missing-document-envelope.json
jq -r input_filename ./fruit-unquoted-key.json
parse error: Invalid numeric literal at line 2, column 11
jq -r input_filename ./fruit.json
./fruit.json
jq -r input_filename ./.jsfmtrc
./.jsfmtrc
jq -r input_filename ./grocer.json
./grocer.json
jq -r input_filename ./.eslintrc
./.eslintrc
jq -r input_filename ./.jshintrc
./.jshintrc
jq -r input_filename ./fruit-with-comment.json
parse error: Invalid numeric literal at line 1, column 3
jq -r input_filename ./fruit-missing-end-brace.json
parse error: Unfinished JSON term at EOF at line 3, column 0
jq -r input_filename ./.jslintrc
./.jslintrc
jq -r input_filename ./fruit-trailing-comma.json
parse error: Expected another key-value pair at line 3, column 1
```

# LICENSE

BSD-2-Clause

# RUNTIME REQUIREMENTS

* GNU or BSD [findutils](https://en.wikipedia.org/wiki/Find_(Unix))
* [jq](https://jqlang.github.io/jq/) 1.6+
* POSIX compatible [sh](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/sh.html)

# INSTALL

For more information on installing or uninstalling kirill, see [INSTALL.md](INSTALL.md).

# CONTRIBUTING

For more information on developing kirill itself, see [DEVELOPMENT.md](DEVELOPMENT.md).

# SEE ALSO

* [sail](https://github.com/mcandre/sail) identifies C/C++ source code
* [stank](https://github.com/mcandre/stank) identifies shell scripts
