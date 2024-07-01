#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use figment::{
    providers::{Format, Toml, Yaml},
    Figment,
};
use ratatui::style::Color;
use serde::de;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;

/// The `Base16PaletteError` enum represents errors that can occur while working
/// with the Base16 color palette configuration.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Base16PaletteError {
    /// This error occurs when the extraction of data from a file or
    /// configuration fails.
    ///
    /// This variant encapsulates a `figment::Error`, indicating that it
    /// originated from the Figment configuration library, which might be
    /// used to handle configuration data in various formats like JSON,
    /// TOML, YAML, etc.
    #[error("unable to extract data from file")]
    ExtractionFailed(#[from] figment::Error),
}

/// A `Base16Palette` defines a color palette based on the Base16 styling
/// guidelines. These color codes are typically used to create themes for syntax
/// highlighting, terminal emulators, and other developer tools. Each field
/// represents a different element of the user interface that can be customized.
/// Base16 aims to group similar language constructs with a single colour. For
/// example, floats, ints, and doubles would belong to the same colour group.
/// The colours for the default theme were chosen to be easily separable, but
/// scheme designers should pick whichever colours they desire, e.g. base0B
/// (green by default) could be replaced with red. There are, however, some
/// general guidelines below that stipulate which base0B should be used to
/// highlight each construct when designing templates for editors.
///
/// Colours base00 to base07 are typically variations of a shade and run from
/// darkest to lightest. These colours are used for foreground and background,
/// status bars, line highlighting and such. colours base08 to base0F are
/// typically individual colours used for types, operators, names and variables.
/// In order to create a dark theme, colours base00 to base07 should span from
/// dark to light. For a light theme, these colours should span from light to
/// dark.
#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Base16Palette {
    /// Name
    #[serde(skip, alias = "scheme")]
    pub name: &'static str,

    /// Author
    #[serde(skip)]
    pub author: &'static str,

    /// Default Background
    #[serde(skip)]
    pub slug: &'static str,

    /// Default Background
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base00: Color,

    /// Lighter Background (Used for status bars, line number and folding marks)
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base01: Color,

    /// Selection Background (Settings where you need to highlight text, such as
    /// find results)
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base02: Color,

    /// Comments, Invisibles, Line Highlighting
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base03: Color,

    /// Dark Foreground (Used for status bars)
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base04: Color,

    /// Default Foreground, Caret, Delimiters, Operators
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base05: Color,

    /// Light Foreground (Not often used, could be used for hover states or
    /// dividers)
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base06: Color,

    /// Light Background (Probably at most for cursor line background color)
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base07: Color,

    /// Variables, XML Tags, Markup Link Text, Markup Lists, Diff Deleted
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base08: Color,

    /// Integers, Boolean, Constants, XML Attributes, Markup Link Url
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base09: Color,

    /// Classes, Markup Bold, Search Text Background
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0a: Color,

    /// Strings, Inherited Class, Markup Code, Diff Inserted
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0b: Color,

    /// Support, Regular Expressions, Escape Characters, Markup Quotes
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0c: Color,

    /// Functions, Methods, Attribute IDs, Headings
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0d: Color,

    /// Keywords, Storage, Selector, Markup Bold, Diff Changed
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0e: Color,

    /// Deprecated, Opening/Closing Embedded Language Tags, e.g. `<?php ?>
    #[serde(deserialize_with = "deserialize_from_str")]
    pub base0f: Color,
}

impl Base16Palette {
    /// Loads a `Base16Palette` instance from a YAML file.
    ///
    /// Given a file path, this function uses Figment's `Yaml` provider to read
    /// and parse the YAML content into a `Base16Palette` instance. This
    /// allows for loading the color palette configuration from a
    /// YAML-formatted file.
    ///
    /// # Arguments
    ///
    /// * `file`: The file path pointing to the YAML configuration file. The
    ///   file path type is generic and can be any type that implements
    ///   `Into<PathBuf>`.
    ///
    /// # Returns
    ///
    /// If the function is successful, it returns `Ok(Base16Palette)`, the
    /// loaded palette instance. If an error occurs during reading or
    /// parsing the file, it returns a `Base16PaletteError`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_base16::Base16Palette;
    /// let palette_result = Base16Palette::from_yaml("path_to_file.yaml");
    /// ```
    pub fn from_yaml(file: impl Into<PathBuf>) -> Result<Self, Base16PaletteError> {
        Figment::new()
            .merge(Yaml::file(file.into()))
            .extract::<Base16Palette>()
            .map_err(Base16PaletteError::ExtractionFailed)
    }

    /// Loads a `Base16Palette` instance from a TOML file.
    ///
    /// Given a file path, this function uses Figment's `Toml` provider to read
    /// and parse the TOML content into a `Base16Palette` instance. This
    /// allows for loading the color palette configuration from a
    /// TOML-formatted file.
    ///
    /// # Arguments
    ///
    /// * `file`: The file path pointing to the TOML configuration file. The
    ///   file path type is generic and can be any type that implements
    ///   `Into<PathBuf>`.
    ///
    /// # Returns
    ///
    /// If the function is successful, it returns `Ok(Base16Palette)`, the
    /// loaded palette instance. If an error occurs during reading or
    /// parsing the file, it returns a `Base16PaletteError`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui_base16::Base16Palette;
    /// let palette_result = Base16Palette::from_toml("path_to_file.toml");
    /// ```
    pub fn from_toml(file: impl Into<PathBuf>) -> Result<Self, Base16PaletteError> {
        Figment::new()
            .merge(Toml::file(file.into()))
            .extract::<Base16Palette>()
            .map_err(Base16PaletteError::ExtractionFailed)
    }
}

fn deserialize_from_str<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.starts_with('#') {
        Color::from_str(&s).map_err(de::Error::custom)
    } else {
        Color::from_str(&format!("#{s}")).map_err(de::Error::custom)
    }
}

macro_rules! palette {
    (
        $name:ident,
        scheme : $scheme:literal,
        author : $author:literal,
        slug : $slug:literal,
        base00 : $base00:literal,
        base01 : $base01:literal,
        base02 : $base02:literal,
        base03 : $base03:literal,
        base04 : $base04:literal,
        base05 : $base05:literal,
        base06 : $base06:literal,
        base07 : $base07:literal,
        base08 : $base08:literal,
        base09 : $base09:literal,
        base0a : $base0a:literal,
        base0b : $base0b:literal,
        base0c : $base0c:literal,
        base0d : $base0d:literal,
        base0e : $base0e:literal,
        base0f : $base0f:literal,
    ) => {
        pub const $name: $crate::Base16Palette = $crate::Base16Palette {
            name: $scheme,
            author: $author,
            slug: $slug,
            base00: ratatui::style::Color::from_u32($base00),
            base01: ratatui::style::Color::from_u32($base01),
            base02: ratatui::style::Color::from_u32($base02),
            base03: ratatui::style::Color::from_u32($base03),
            base04: ratatui::style::Color::from_u32($base04),
            base05: ratatui::style::Color::from_u32($base05),
            base06: ratatui::style::Color::from_u32($base06),
            base07: ratatui::style::Color::from_u32($base07),
            base08: ratatui::style::Color::from_u32($base08),
            base09: ratatui::style::Color::from_u32($base09),
            base0a: ratatui::style::Color::from_u32($base0a),
            base0b: ratatui::style::Color::from_u32($base0b),
            base0c: ratatui::style::Color::from_u32($base0c),
            base0d: ratatui::style::Color::from_u32($base0d),
            base0e: ratatui::style::Color::from_u32($base0e),
            base0f: ratatui::style::Color::from_u32($base0f),
        };
    };
}

palette! {
    CUPCAKE,
    scheme: "Cupcake",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/cupcake.yaml",
    base00: 0x00fbf1f2,
    base01: 0x00f2f1f4,
    base02: 0x00d8d5dd,
    base03: 0x00bfb9c6,
    base04: 0x00a59daf,
    base05: 0x008b8198,
    base06: 0x0072677E,
    base07: 0x00585062,
    base08: 0x00D57E85,
    base09: 0x00EBB790,
    base0a: 0x00DCB16C,
    base0b: 0x00A3B367,
    base0c: 0x0069A9A7,
    base0d: 0x007297B9,
    base0e: 0x00BB99B4,
    base0f: 0x00BAA58C,
}

palette! {
    DEFAULT_DARK,
    scheme: "Default Dark",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/default-dark.yaml",
    base00: 0x00181818,
    base01: 0x00282828,
    base02: 0x00383838,
    base03: 0x00585858,
    base04: 0x00b8b8b8,
    base05: 0x00d8d8d8,
    base06: 0x00e8e8e8,
    base07: 0x00f8f8f8,
    base08: 0x00ab4642,
    base09: 0x00dc9656,
    base0a: 0x00f7ca88,
    base0b: 0x00a1b56c,
    base0c: 0x0086c1b9,
    base0d: 0x007cafc2,
    base0e: 0x00ba8baf,
    base0f: 0x00a16946,
}

palette! {
    DEFAULT_LIGHT,
    scheme: "Default Light",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/default-light.yaml",
    base00: 0x00f8f8f8,
    base01: 0x00e8e8e8,
    base02: 0x00d8d8d8,
    base03: 0x00b8b8b8,
    base04: 0x00585858,
    base05: 0x00383838,
    base06: 0x00282828,
    base07: 0x00181818,
    base08: 0x00ab4642,
    base09: 0x00dc9656,
    base0a: 0x00f7ca88,
    base0b: 0x00a1b56c,
    base0c: 0x0086c1b9,
    base0d: 0x007cafc2,
    base0e: 0x00ba8baf,
    base0f: 0x00a16946,
}

palette! {
    EIGHTIES,
    scheme: "Eighties",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/eighties.yaml",
    base00: 0x002d2d2d,
    base01: 0x00393939,
    base02: 0x00515151,
    base03: 0x00747369,
    base04: 0x00a09f93,
    base05: 0x00d3d0c8,
    base06: 0x00e8e6df,
    base07: 0x00f2f0ec,
    base08: 0x00f2777a,
    base09: 0x00f99157,
    base0a: 0x00ffcc66,
    base0b: 0x0099cc99,
    base0c: 0x0066cccc,
    base0d: 0x006699cc,
    base0e: 0x00cc99cc,
    base0f: 0x00d27b53,
}

palette! {
    MOCHA,
    scheme: "Mocha",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/mocha.yaml",
    base00: 0x003b3228,
    base01: 0x00534636,
    base02: 0x00645240,
    base03: 0x007e705a,
    base04: 0x00b8afad,
    base05: 0x00d0c8c6,
    base06: 0x00e9e1dd,
    base07: 0x00f5eeeb,
    base08: 0x00cb6077,
    base09: 0x00d28b71,
    base0a: 0x00f4bc87,
    base0b: 0x00beb55b,
    base0c: 0x007bbda4,
    base0d: 0x008ab3b5,
    base0e: 0x00a89bb9,
    base0f: 0x00bb9584,
}

palette! {
    OCEAN,
    scheme: "Ocean",
    author: "Chris Kempson (http://chriskempson.com)",
    slug: "https://github.com/chriskempson/base16-default-schemes/blob/master/ocean.yaml",
    base00: 0x002b303b,
    base01: 0x00343d46,
    base02: 0x004f5b66,
    base03: 0x0065737e,
    base04: 0x00a7adba,
    base05: 0x00c0c5ce,
    base06: 0x00dfe1e8,
    base07: 0x00eff1f5,
    base08: 0x00bf616a,
    base09: 0x00d08770,
    base0a: 0x00ebcb8b,
    base0b: 0x00a3be8c,
    base0c: 0x0096b5b4,
    base0d: 0x008fa1b3,
    base0e: 0x00b48ead,
    base0f: 0x00ab7967,
}

palette! {
    DRACULA,
    scheme: "Dracula",
    author: "Mike Barkmin (http://github.com/mikebarkmin) based on Dracula Theme (http://github.com/dracula)",
    slug: "https://github.com/dracula/base16-dracula-scheme/blob/master/dracula.yaml",
    base00: 0x00282936,
    base01: 0x003a3c4e,
    base02: 0x004d4f68,
    base03: 0x00626483,
    base04: 0x0062d6e8,
    base05: 0x00e9e9f4,
    base06: 0x00f1f2f8,
    base07: 0x00f7f7fb,
    base08: 0x00ea51b2,
    base09: 0x00b45bcf,
    base0a: 0x0000f769,
    base0b: 0x00ebff87,
    base0c: 0x00a1efe4,
    base0d: 0x0062d6e8,
    base0e: 0x00b45bcf,
    base0f: 0x0000f769,
}

palette! {
    GITHUB_LIGHT,
    scheme: "Github",
    author: "Defman21",
    slug: "https://github.com/Defman21/base16-github-scheme/blob/master/github.yaml",
    base00: 0x00ffffff,
    base01: 0x00f5f5f5,
    base02: 0x00c8c8fa,
    base03: 0x00969896,
    base04: 0x00e8e8e8,
    base05: 0x00333333,
    base06: 0x00ffffff,
    base07: 0x00ffffff,
    base08: 0x00ed6a43,
    base09: 0x000086b3,
    base0a: 0x00795da3,
    base0b: 0x00183691,
    base0c: 0x00183691,
    base0d: 0x00795da3,
    base0e: 0x00a71d5d,
    base0f: 0x00333333,
}

palette! {
    ROSE_PINE_DAWN,
    scheme: "Rosé Pine Dawn",
    author: "Emilia Dunfelt <edun@dunfelt.se>",
    slug: "https://github.com/edunfelt/base16-rose-pine-scheme/blob/main/rose-pine-dawn.yaml",
    base00: 0x00faf4ed,
    base01: 0x00fffaf3,
    base02: 0x00f2e9de,
    base03: 0x009893a5,
    base04: 0x00797593,
    base05: 0x00575279,
    base06: 0x00575279,
    base07: 0x00cecacd,
    base08: 0x00b4637a,
    base09: 0x00ea9d34,
    base0a: 0x00d7827e,
    base0b: 0x00286983,
    base0c: 0x0056949f,
    base0d: 0x00907aa9,
    base0e: 0x00ea9d34,
    base0f: 0x00cecacd,
}

palette! {
    ROSE_PINE_MOON,
    scheme: "Rosé Pine Moon",
    author: "Emilia Dunfelt <edun@dunfelt.se>",
    slug: "https://github.com/edunfelt/base16-rose-pine-scheme/blob/main/rose-pine-moon.yaml",
    base00: 0x00232136,
    base01: 0x002a273f,
    base02: 0x00393552,
    base03: 0x006e6a86,
    base04: 0x00908caa,
    base05: 0x00e0def4,
    base06: 0x00e0def4,
    base07: 0x0056526e,
    base08: 0x00eb6f92,
    base09: 0x00f6c177,
    base0a: 0x00ea9a97,
    base0b: 0x003e8fb0,
    base0c: 0x009ccfd8,
    base0d: 0x00c4a7e7,
    base0e: 0x00f6c177,
    base0f: 0x0056526e,
}

palette! {
    ROSE_PINE,
    scheme: "Rosé Pine",
    author: "Emilia Dunfelt <edun@dunfelt.se>",
    slug: "https://github.com/edunfelt/base16-rose-pine-scheme/blob/main/rose-pine.yaml",
    base00: 0x00191724,
    base01: 0x001f1d2e,
    base02: 0x0026233a,
    base03: 0x006e6a86,
    base04: 0x00908caa,
    base05: 0x00e0def4,
    base06: 0x00e0def4,
    base07: 0x00524f67,
    base08: 0x00eb6f92,
    base09: 0x00f6c177,
    base0a: 0x00ebbcba,
    base0b: 0x0031748f,
    base0c: 0x009ccfd8,
    base0d: 0x00c4a7e7,
    base0e: 0x00f6c177,
    base0f: 0x00524f67,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_yaml() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("./.config/dracula.yaml");
        let _ = Base16Palette::from_yaml(file).unwrap();

        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("./.config/github.yaml");
        let _ = Base16Palette::from_yaml(file).unwrap();
    }
}
