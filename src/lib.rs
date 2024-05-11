#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use figment::{
    providers::{Format, Toml, Yaml},
    Figment,
};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::path::PathBuf;
use thiserror::Error;

/// The `Base16PaletteError` enum represents errors that can occur while working
/// with the Base16 color palette configuration.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Base16PaletteError {
    /// This error occurs when the extraction of data from a file or configuration fails.
    ///
    /// This variant encapsulates a `figment::Error`, indicating that it originated
    /// from the Figment configuration library, which might be used to handle configuration
    /// data in various formats like JSON, TOML, YAML, etc.
    #[error("unable to extract data from file")]
    ExtractionFailed(#[from] figment::Error),
}

/// A `Base16Palette` defines a color palette based on the Base16 styling guidelines.
/// These color codes are typically used to create themes for syntax highlighting,
/// terminal emulators, and other developer tools. Each field represents a different
/// element of the user interface that can be customized.
#[serde_as]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Base16Palette {
    /// Default Background
    #[serde_as(as = "DisplayFromStr")]
    pub base00: Color,

    /// Lighter Background (Used for status bars, line number and folding marks)
    #[serde_as(as = "DisplayFromStr")]
    pub base01: Color,

    /// Selection Background (Settings where you need to highlight text, such as find results)
    #[serde_as(as = "DisplayFromStr")]
    pub base02: Color,

    /// Comments, Invisibles, Line Highlighting
    #[serde_as(as = "DisplayFromStr")]
    pub base03: Color,

    /// Dark Foreground (Used for status bars)
    #[serde_as(as = "DisplayFromStr")]
    pub base04: Color,

    /// Default Foreground, Caret, Delimiters, Operators
    #[serde_as(as = "DisplayFromStr")]
    pub base05: Color,

    /// Light Foreground (Not often used, could be used for hover states or dividers)
    #[serde_as(as = "DisplayFromStr")]
    pub base06: Color,

    /// Light Background (Probably at most for cursor line background color)
    #[serde_as(as = "DisplayFromStr")]
    pub base07: Color,

    /// Variables, XML Tags, Markup Link Text, Markup Lists, Diff Deleted
    #[serde_as(as = "DisplayFromStr")]
    pub base08: Color,

    /// Integers, Boolean, Constants, XML Attributes, Markup Link Url
    #[serde_as(as = "DisplayFromStr")]
    pub base09: Color,

    /// Classes, Keywords, Storage, Selector, Markup Italic, Diff Changed
    #[serde_as(as = "DisplayFromStr")]
    pub base0a: Color,

    /// Strings, Inherited Class, Markup Code, Diff Inserted
    #[serde_as(as = "DisplayFromStr")]
    pub base0b: Color,

    /// Support, Regular Expressions, Escape Characters, Markup Quotes
    #[serde_as(as = "DisplayFromStr")]
    pub base0c: Color,

    /// Functions, Methods, Attribute IDs, Headings
    #[serde_as(as = "DisplayFromStr")]
    pub base0d: Color,

    /// Keywords, Storage, Selector, Markup Bold, Diff Renamed
    #[serde_as(as = "DisplayFromStr")]
    pub base0e: Color,

    /// Deprecated, Opening/Closing Embedded Language Tags
    #[serde_as(as = "DisplayFromStr")]
    pub base0f: Color,
}

impl Base16Palette {
    /// Loads a `Base16Palette` instance from a YAML file.
    ///
    /// Given a file path, this function uses Figment's `Yaml` provider to read and parse
    /// the YAML content into a `Base16Palette` instance. This allows for loading the
    /// color palette configuration from a YAML-formatted file.
    ///
    /// # Arguments
    ///
    /// * `file`: The file path pointing to the YAML configuration file. The file path
    /// type is generic and can be any type that implements `Into<PathBuf>`.
    ///
    /// # Returns
    ///
    /// If the function is successful, it returns `Ok(Base16Palette)`, the loaded palette instance.
    /// If an error occurs during reading or parsing the file, it returns a `Base16PaletteError`.
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
    /// Given a file path, this function uses Figment's `Toml` provider to read and parse
    /// the TOML content into a `Base16Palette` instance. This allows for loading the
    /// color palette configuration from a TOML-formatted file.
    ///
    /// # Arguments
    ///
    /// * `file`: The file path pointing to the TOML configuration file. The file path
    /// type is generic and can be any type that implements `Into<PathBuf>`.
    ///
    /// # Returns
    ///
    /// If the function is successful, it returns `Ok(Base16Palette)`, the loaded palette instance.
    /// If an error occurs during reading or parsing the file, it returns a `Base16PaletteError`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_from_yaml() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("./.config/github.yaml");
        let _ = Base16Palette::from_yaml(file).unwrap();
    }
}
