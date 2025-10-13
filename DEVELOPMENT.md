# BUILDTIME REQUIREMENTS

* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [bash](https://www.gnu.org/software/bash/) 4+
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [findutils](https://en.wikipedia.org/wiki/Find_(Unix))
* [git](https://git-scm.com/) 2.46.4+
* [Go](https://go.dev/) 1.24.6+
* [jq](https://jqlang.github.io/jq/) 1.6+
* [Python](https://www.python.org/) 3.13.7+
* [ShellCheck](https://www.shellcheck.net/) 0.10.0+
* [Snyk](https://snyk.io/)
* Provision additional dev tools with `./install`

## Recommended

* [ASDF](https://asdf-vm.com/) 0.10 (run `asdf reshim` after provisioning)
* [direnv](https://direnv.net/) 2

# AUDIT

```console
$ ./build audit
```

# LINT

```console
$ ./build lint
```

# TEST

```console
$ ./build [unit_test]
```
