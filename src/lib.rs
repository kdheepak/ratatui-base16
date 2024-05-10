use std::path::PathBuf;

use figment::{
    providers::{Format, Yaml},
    Figment,
};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Base16PaletteError {
    #[error("unable to extract data from file")]
    ExtractionFailed(#[from] figment::Error),
}

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

    /// Deprecated, Opening/Closing Embedded Language Tags e.g., `<? ?>`
    #[serde_as(as = "DisplayFromStr")]
    pub base0f: Color,
}

impl Base16Palette {
    pub fn from_yaml(file: impl Into<PathBuf>) -> Result<Self, Base16PaletteError> {
        Figment::new()
            .merge(Yaml::file(file.into()))
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
