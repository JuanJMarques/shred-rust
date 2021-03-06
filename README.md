# shred-rust
An implementation of shred command in rust.

![Build Status][bi] ![MIT/Apache][li]  ![LoC][lo]

[bi]: https://github.com/JuanJMarques/shred-rust/actions/workflows/CI.yml/badge.svg
[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[lo]: https://tokei.rs/b1/github/JuanJMarques/shred-rust?category=code

This is a version of [shred command](https://linux.die.net/man/1/shred) implemented in rust.

## Shred
Overwrite a file to hide its contents, and optionally delete it

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
    shred.exe [FLAGS] [OPTIONS] <FILES>...

FLAGS:
    -h, --help         Prints help information
    -r, --recursive    Recursively deletes the files in directories
    -u, --remove       Truncate and remove file after overwriting.
    -V, --version      Prints version information
    -v, --verbose      Show verbose information about shredding progress.
    -z, --zero         Add a final overwrite with zeros to hide shredding.

OPTIONS:
    -s, --size <size>           Shred this many bytes (suffixes like K, M, G accepted).
    -t, --threads <threads>     Number of threads to execute in parallel [default: 1]
    -n, --iterations <times>    Overwrite N times. [default: 3]

ARGS:
    <FILES>...    Sets the files to to shred

Process finished with exit code 0

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
