pub mod csv;
pub mod json;
pub mod table;
pub mod yaml;

use crate::error::Result;
use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            "yaml" => Ok(Self::Yaml),
            other => Err(format!(
                "Unknown output format: {other}. Use: table, json, csv, yaml"
            )),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Table => write!(f, "table"),
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::Yaml => write!(f, "yaml"),
        }
    }
}

impl OutputFormat {
    pub fn auto_detect() -> Self {
        if is_terminal::is_terminal(std::io::stdout()) {
            Self::Table
        } else {
            Self::Json
        }
    }
}

pub trait Tabular {
    fn headers() -> Vec<String>;
    fn row(&self) -> Vec<String>;
}

pub fn render<T: Tabular + Serialize>(
    items: &[T],
    format: OutputFormat,
    compact: bool,
) -> Result<String> {
    match format {
        OutputFormat::Table => Ok(table::render(items)),
        OutputFormat::Json => Ok(json::render(items, compact)),
        OutputFormat::Csv => csv::render(items),
        OutputFormat::Yaml => yaml::render(items),
    }
}

pub fn render_single<T: Serialize>(
    item: &T,
    format: OutputFormat,
    compact: bool,
) -> Result<String> {
    match format {
        OutputFormat::Table | OutputFormat::Json => Ok(json::render_single(item, compact)),
        OutputFormat::Yaml => yaml::render_single(item),
        OutputFormat::Csv => Ok(json::render_single(item, compact)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_output_format() {
        assert_eq!(
            "table".parse::<OutputFormat>().unwrap(),
            OutputFormat::Table
        );
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert_eq!("csv".parse::<OutputFormat>().unwrap(), OutputFormat::Csv);
        assert_eq!("yaml".parse::<OutputFormat>().unwrap(), OutputFormat::Yaml);
        assert_eq!("JSON".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
    }

    #[test]
    fn parse_invalid_format() {
        assert!("xml".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn display_format() {
        assert_eq!(OutputFormat::Table.to_string(), "table");
        assert_eq!(OutputFormat::Json.to_string(), "json");
    }
}
