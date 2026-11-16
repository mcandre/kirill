# kirill: JSON validator

[![Crate Downloads](https://badgen.net/crates/d/kirill?label=crate+downloads)](https://crates.io/crates/kirill) [![API Docs](https://badgen.net/static/docs/API/3eca2a?icon=docs.rs)](https://docs.rs/kirill/latest/kirill/) [![CI/CD](https://badgen.net/github/checks/mcandre/kirill?icon=github)](https://github.com/mcandre/kirill/actions) [![License](https://badgen.net/static/license/BSD/ab2b29)](LICENSE.md)

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

# DOWNLOAD

```sh
cargo install kirill
```

## Prerequisites

* [cargo](https://doc.rust-lang.org/cargo/)

## Postinstall

Register `~/.cargo/bin` to `PATH` environment variable.

For details on tuning kirill, see [CONFIGURATION](CONFIGURATION.md).

For details on building from source, see [DEVELOPMENT](DEVELOPMENT.md).

# RESOURCES

Prior art, personal plugs, and tools for managing data.

* [jq](https://jqlang.github.io/jq/) - styles and transforms JSON documents
* [mcandre/linters](https://github.com/mcandre/linters) - a collection of file analyzers

✉️
