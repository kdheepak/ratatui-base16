# ratatui-base16

[![Crates.io](https://img.shields.io/crates/v/ratatui-base16.svg)](https://crates.io/crates/ratatui-base16)
[![Documentation](https://docs.rs/ratatui-base16/badge.svg)](https://docs.rs/ratatui-base16)
[![LICENSE](https://img.shields.io/crates/l/ratatui-base16.svg)](./LICENSE)

ratatui-base16 is a base16 palette library for the [Ratatui](https://github.com/ratatui-org/ratatui) crate in Rust.

## Getting Started

To get started, first add it to your Cargo.toml dependencies:

```shell
$ cargo add ratatui-base16
```

Creating a `Base16Palette` is simple. Here's a quick example:

```rust
use ratatui_base16::Base16Palette;
let palette = Base16Palette::from_yaml("./config/dracula.yaml");
```

## Contributing

Please feel free to fork the repository, make your changes, and submit a pull request.

## License

See [LICENSE](LICENSE) for details.
