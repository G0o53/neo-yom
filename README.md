# neo-yom
![MIT License Badge](https://img.shields.io/badge/license-MIT-blue)

`neo-yom` is the simple, and easy way to turn [`YOM`](https://github.com/G0o53/yom), into a (dead simple) interactive shell!
To install, simply make sure you have [Rust](https://rustup.rs) for the toolchain, and [huh](https://github.com/G0o53/huh) as the package manager,
on your system. Then just do
```bash
huh 'G0o53/neo-yom'
```
as easy as that!

Then to use, simply put inside a `yom` script (e.g., `~/.yomrc`)
```bash
hook 'core' '~/.huh/neo-yom'
```
run it, and then you have an interactive shell ready to be used!
> [!NOTE]
> This doesn't support arrow keys and will type the literal text of an arrow key.
