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

macro_rules! palette {
    (
        $name:ident,
        $schema:literal,
        $author:literal,
        $slug:literal,
        $base00:literal,
        $base01:literal,
        $base02:literal,
        $base03:literal,
        $base04:literal,
        $base05:literal,
        $base06:literal,
        $base07:literal,
        $base08:literal,
        $base09:literal,
        $base0a:literal,
        $base0b:literal,
        $base0c:literal,
        $base0d:literal,
        $base0e:literal,
        $base0f:literal,
    ) => {
        pub const $name: $crate::Base16Palette = $crate::Base16Palette {
            name: $schema,
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
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    ///   file path
    /// type is generic and can be any type that implements `Into<PathBuf>`.
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
    ///   file path
    /// type is generic and can be any type that implements `Into<PathBuf>`.
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

palette! {
    DRACULA,
    "Dracula",
    "Mike Barkmin (http://github.com/mikebarkmin) based on Dracula Theme (http://github.com/dracula)",
    "",
    0x00282936,
    0x003a3c4e,
    0x004d4f68,
    0x00626483,
    0x0062d6e8,
    0x00e9e9f4,
    0x00f1f2f8,
    0x00f7f7fb,
    0x00ea51b2,
    0x00b45bcf,
    0x0000f769,
    0x00ebff87,
    0x00a1efe4,
    0x0062d6e8,
    0x00b45bcf,
    0x0000f769,
}

palette! {
    GITHUB,
    "Github",
    "Defman21",
    "",
    0x00ffffff,
    0x00f5f5f5,
    0x00c8c8fa,
    0x00969896,
    0x00e8e8e8,
    0x00333333,
    0x00ffffff,
    0x00ffffff,
    0x00ed6a43,
    0x000086b3,
    0x00795da3,
    0x00183691,
    0x00183691,
    0x00795da3,
    0x00a71d5d,
    0x00333333,
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
