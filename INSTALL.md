# INSTALLATION REQUIREMENTS

* GNU or BSD [findutils](https://en.wikipedia.org/wiki/Find_(Unix))
* [git](https://git-scm.com/) 2.46.1+
* [jq](https://jqlang.github.io/jq/) 1.6+
* POSIX compatible [sh](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/sh.html)

# INSTALL

1. Clone the project to a local directory.

```console
$ git clone https://github.com/mcandre/kirill.git "$HOME/kirill"
```

2. Add .../kirill/bin to `PATH`.

For example, `$HOME/.zshrc` (zsh):

```sh
# ...
export PATH="$PATH:$HOME/kirill/bin"
```

# UNINSTALL

1. Remove .../kirill/bin from `PATH`.

For example, `$HOME/.zshrc` (zsh):

```sh
# ...
```

2. Remove local clone directory.

```console
$ rm -rf "$HOME/kirill"
```
