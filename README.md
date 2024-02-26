# Rust CLI Tools
Basic Rust project that mirrors the functionality of some Unix commands, created in order to learn about the Rust programming language.

## Platform
This project is a console application, made for all platforms that support Rust.

## Functionalities
Currently, this project mirrors the following Unix commands:
- [`echo [...args]`](https://linux.die.net/man/1/echo)
- [`cat [file1] [file2] [...]`](https://linux.die.net/man/1/cat)
- [`ls [dir_name]`](https://linux.die.net/man/1/ls)
- [`find <directory_path> <file_to_find>`](https://linux.die.net/man/1/find)
- [`grep <pattern> <file_path>`](https://linux.die.net/man/1/grep)

## Getting Started
This step describes the prerequisites for building and running this project:
- [Rust compiler](https://www.rust-lang.org/en-US/tools/install)

## Building
The following command will install all required dependencies required for this project:
```sh
cargo build
```

## Running
The following command runs the project:
```sh
cargo run <unix_command> [...args]
```

## Running tests
The following comand runs all unit tests:
```sh
cargo test
```

## WIP
The following features are work-in-progress(WIP). Priority is sorted in descending order, where items on top of the list are the most important ones:
- Remove `.unwrap()` instances from `grep.rs`

## Authors
- André Wlodkovski - [@andrewlod](https://github.com/andrewlod)

## License
This project is licensed under the [MIT License](https://opensource.org/license/mit) - see the [LICENSE](LICENSE) for more details.