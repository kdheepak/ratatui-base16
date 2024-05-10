# ratatui-base16

ratatui-base16 is a base16 palette library for [Ratatui](https://github.com/ratatui-org/ratatui) crate for Rust.

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
