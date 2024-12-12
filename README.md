# bitly-shortener

**DISCLAIMER**: This repository is now archived. If you want to contribute,
please send me a message at `winlogon.exe@matrix.org` and I will unarchive it.

This application implements the [Bitly API](https://dev.bitly.com/) as a Rust
CLI utility, which tries to Keep it Simple.

## Table of Contents

  - [Installation](#installation)
  - [Usage](#usage)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)

## Installation

To use this application, you must install Rust and its package manager, Cargo.
Consult the official [Rust installation
manual](https://www.rust-lang.org/tools/install) for guidance.

Once Rust and Cargo are installed, you can build and install this program using
the following command:

``` console
$ cargo build --release
```

## Usage

To use the API, either set as the `BITLY_TOKEN` environment variable as your
access token, or, create an `api_token.txt` in your current working directory
containing your access token which you can obtain
[here](https://app.bitly.com/settings/api/). There are several subcommands
available:

  - `create` - Generate a Bitlink for a lengthy URL.
  - `delete` - Remove a Bitlink.
  - `retrieve` - Retrieve information about a Bitlink.
  - `shorten` - Shorten a lengthy URL.
  - `update` - Update a Bitlink.

For each of these subcommands, you need to provide a bitlink.

### Example

``` console
$ ./bitly-shortener-cli shorten 'https://example.com/long-link-here'
```

## License

This project is licensed under the [GPL-3.0](LICENSE.md).

### Contributing

Feel free to contribute to open issues, propose new features, fix bugs, and add
documentation!

This project follows the [Rust Code
Style](https://doc.rust-lang.org/style-guide/index.html); it also follows the
[Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) and
[KISS](https://en.wikipedia.org/wiki/KISS_principle).

## Acknowledgments

I'd like to give credit to the following libraries and tools used in this
project:

  - [Clap](https://crates.io/crates/clap) - for command-line argument parsing in
    Rust.
  - [Serde](https://crates.io/crates/serde) and
    [serde\_json](https://crates.io/crates/serde_json) for serializing and
    deserializing.
  - [Reqwest](https://crates.io/crates/reqwest) for making HTTP requests in
    order to use the API.
