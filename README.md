# kirill: JSON validator

[![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/kirill?label=crate%20downloads)](https://crates.io/crates/kirill) [![docs.rs](https://img.shields.io/docsrs/kirill)](https://docs.rs/kirill/latest/kirill/) [![Test](https://github.com/mcandre/kirill/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

# ABOUT

kirill scans projects large and small for JSON correctness:

* Check for basic JSON(5) syntactical validity
* Identify JSON(5) files recursively in large project directories
* Verify compliance with a [JSON Schema](https://json-schema.org/)

# EXAMPLES

```console
% cd examples

% kirill .
error: fruit-missing-end-brace.json: EOF while parsing an object at line 3 column 0
error: fruit-trailing-comma.json: trailing comma at line 3 column 1
error: fruit-unquoted-key.json: key must be a string at line 2 column 5
error: fruit-with-comment.json: expected value at line 1 column 1
error: settings.json: expected value at line 1 column 1

% kirill --schema species.json zoo
error: zoo/bad-bear.json: Missing required property 'species'

% kirill books.json5
error: books.json5: expected value at line 9 column 9

% kirill --json5 books.json5
%
```

For feature details, see our [usage guide](USAGE.md).

# DOWNLOAD

```sh
cargo install kirill
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

For details on building from source, see our [development guide](DEVELOPMENT.md).

# RESOURCES

Prior art, personal plugs, and tools for managing data.

* [jq](https://jqlang.github.io/jq/) - styles and transforms JSON documents
* [mcandre/linters](https://github.com/mcandre/linters) - a collection of file analyzers
