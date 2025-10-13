# INSTALLATION REQUIREMENTS

* [bash](https://www.gnu.org/software/bash/)
* [GNU](https://www.gnu.org/)/[BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) [findutils](https://en.wikipedia.org/wiki/Find_(Unix))
* [git](https://git-scm.com/) 2.46.1+
* [jq](https://jqlang.github.io/jq/) 1.6+

# INSTALL

1. Clone the project to a local directory.

```console
$ git clone https://github.com/mcandre/kirill.git "$HOME/kirill"
```

2. Add .../kirill/bin to `PATH`.

For example, `$HOME/.bashrc` (bash):

```sh
# ...
export PATH="$PATH:$HOME/kirill/bin"
```

# UNINSTALL

1. Remove .../kirill/bin from `PATH`.

For example, `$HOME/.bashrc` (bash):

```sh
# ...
```

2. Remove local clone directory.

```console
$ rm -rf "$HOME/kirill"
```
