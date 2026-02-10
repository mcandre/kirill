# kirill: JSON validator

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/kirill)](https://hub.docker.com/r/n4jm4/kirill) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/kirill?label=crate%20downloads)](https://crates.io/crates/kirill) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/kirill/total?logo=github)](https://github.com/mcandre/kirill/releases) [![docs.rs](https://img.shields.io/docsrs/kirill)](https://docs.rs/kirill/latest/kirill/) [![Test](https://github.com/mcandre/kirill/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test.yml) [![Test-Futureproof-Dependencies](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-dependencies.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-dependencies.yml) [![Test-Futureproof-Language](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-language.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-language.yml) [![Test-Futureproof-OS](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-os.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test-futureproof-os.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md)

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

# INSTALLATION

See [INSTALL.md](INSTALL.md).

## Recommended

* [jq](https://jqlang.github.io/jq/) 1.6+

# SEE ALSO

* [sail](https://github.com/mcandre/sail) identifies C/C++ source code
* [stank](https://github.com/mcandre/stank) identifies shell scripts
* [todolint](https://github.com/mcandre/todolint) identifies bugs by code comments
