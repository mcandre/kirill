# DEVELOPMENT GUIDE

kirill follows standard, cargo based operations for compiling and unit testing Rust code.

For advanced operations, such as linting and generating install media artifacts, we further supplement with some software industry tools.

# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [awscli](https://aws.amazon.com/cli/)
* [bash](https://www.gnu.org/software/bash/) 4+
* [Docker](https://www.docker.com/)
* POSIX compliant [findutils](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/find.html)
* [jq](https://jqlang.github.io/jq/) 1.6+
* POSIX compliant [make](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/make.html)
* [rustup](https://rustup.rs/) 1.28.1+
* [Rust](https://www.rust-lang.org/en-US/)
* [cross](https://crates.io/crates/cross) 4e64366af6095c84fa4f54a0fa5a2ba7d9a271aa
* GNU [tar](https://www.gnu.org/software/tar/) as `gtar`
* Provision additional dev tools with `make -f install.mk`

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* Apple Silicon macOS users may want to apply `DOCKER_DEFAULT_PLATFORM=linux/amd64`, in order to account for images commonly lacking `linux/arm64` buildx platforms
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after provisioning)

# INSTALL BINARIES FROM SOURCE

```sh
make install
```

# UNINSTALL BINARIES

```sh
make uninstall
```

# SECURITY AUDIT

```sh
make audit
```

# LINT

```sh
make lint
```

# TEST

```sh
make test
```

# CROSSCOMPILE BINARIES

```sh
make crit
```

# ARCHIVE BINARIES

```sh
make port
```

# PACKAGE BINARIES

```sh
make package
```

# UPLOAD BINARIES

```sh
make upload
```

# BUILD IMAGES

```sh
make docker-build
```

# TEST PUSH IMAGES

```sh
make docker-test
```

# PUSH IMAGES

```sh
make docker-push
```

# PUBLISH CRATE

```sh
make publish
```

# CLEAN

```sh
make clean
```
