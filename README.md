# shred-rust
An implementation of shred command in rust.

![Build Status][bi] ![MIT/Apache][li]  ![LoC][lo]

[bi]: https://github.com/JuanJMarques/shred-rust/actions/workflows/CI.yml/badge.svg
[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[lo]: https://tokei.rs/b1/github/JuanJMarques/shred-rust?category=code

This is a version of [shred command] [shred-man] implemented in rust.

## Shred
Overwrite a file to hide its contents, and optionally delete it

[shred-man]: https://linux.die.net/man/1/shred

## Build
```commandline
git clone https://github.com/JuanJMarques/shred-rust.git
cd shred-rust
cargo build --release
```

## Usage
```
shred 1.0.0

USAGE:
    shred [FLAGS] [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -u, --remove     Truncate and remove file after overwriting.
    -V, --version    Prints version information
    -v, --verbose    Show verbose information about shredding progress.
    -z, --zero       Add a final overwrite with zeros to hide shredding.

OPTIONS:
    -s, --size <size>           Shred this many bytes (suffixes like K, M, G accepted).
    -n, --iterations <times>    Overwrite N times instead of the default (3).

ARGS:
    <FILE>    Sets the file to use
```
## Contribution

Contribution is highly welcome! If you'd like another feature, just create an issue.
You can also help out if you want to; just pick a "help wanted" issue.
If you need any help, feel free to ask!

All contributions are assumed to be dual-licensed under
MIT/Apache-2.

## License

`shred` is distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
