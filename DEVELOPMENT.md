# BUILDTIME REQUIREMENTS

* GNU or BSD [findutils](https://en.wikipedia.org/wiki/Find_(Unix))
* [git](https://git-scm.com/) 2.46.1+
* [Go](https://go.dev/) 1.23.2+
* [jq](https://jqlang.github.io/jq/) 1.6+
* [Python](https://www.python.org/) 3.12.1+
* POSIX compatible [sh](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/sh.html)
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
$ ./build [lint]
```
