# kirill: JSON validator

[![CloudFlare R2 install media downloads](https://img.shields.io/badge/Packages-F38020?logo=Cloudflare&logoColor=white)](#download)[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/kirill)](https://hub.docker.com/r/n4jm4/kirill) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/kirill?label=crate%20downloads)](https://crates.io/crates/kirill) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/kirill/total?logo=github)](https://github.com/mcandre/kirill/releases) [![docs.rs](https://img.shields.io/docsrs/kirill)](https://docs.rs/kirill/latest/kirill/) [![Test](https://github.com/mcandre/kirill/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/kirill/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-0)](LICENSE.md)

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

# DOWNLOAD

<table>
  <thead>
    <tr>
      <th>OS</th>
      <th colspan=2>Package</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Alpine Linux 3.23+</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/alpine-linux/kirill-0.0.12-r1.x86_64.apk">Intel</a></td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/alpine-linux/kirill-0.0.12-r1.aarch64.apk">ARM</a></td>
    </tr>
    <tr>
      <td>Fedora 43+</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/fedora/kirill-0.0.12-1.x86_64.rpm">Intel</a></td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/fedora/kirill-0.0.12-1.aarch64.rpm">ARM</a></td>
    </tr>
    <tr>
      <td>FreeBSD 13</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/freebsd-amd64/kirill-0.0.12_1.pkg">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>macOS 26 Tahoe+</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/macos/kirill-x86_64-0.0.12-1.pkg">Intel</a></td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/macos/kirill-arm64-0.0.12-1.pkg">ARM</a></td>
    </tr>
    <tr>
      <td>NetBSD 10.1</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/netbsd-x86_64/kirill-0.0.12nb1.tgz">Intel</a></td>
      <td></td>
    </tr>
    <tr>
      <td>Ubuntu 24.04 Noble+</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/ubuntu/kirill_0.0.12-1_amd64.deb">Intel</a></td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/ubuntu/kirill_0.0.12-1_arm64.deb">ARM</a></td>
    </tr>
    <tr>
      <td>Windows 11+</td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/windows/kirill-0.0.12.1-x64.msi">Intel</a></td>
      <td><a href="https://pub-65957f5e8c044d56838ccc7a26b4b179.r2.dev/kirill-0.0.12/windows/kirill-0.0.12.1-arm64.msi">ARM</a></td>
    </tr>
  </tbody>
</table>

# SYSTEM REQUIREMENTS

## Bitness

64

For more host platforms and installation methods, see our [install guide](INSTALL.md).

# RESOURCES

Prior art, personal plugs, and tools for managing data.

* [jq](https://jqlang.github.io/jq/) - styles and transforms JSON documents
* [mcandre/linters](https://github.com/mcandre/linters) - a collection of file analyzers
