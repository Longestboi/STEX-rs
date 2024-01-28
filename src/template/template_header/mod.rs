pub mod error;
pub mod types;

use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, path::Path, str::FromStr};
use toml::{Serializer as TomlSerializer, Table};

use error::Errors;
use types::{StringOrSequence, TemplateCorefile};

const MULTILINE_INFO_REGEX: &str = r"/\*(\s*\[TemplateHeader\]\s.*)\*/";
const SINGLELINE_INFO_REGEX: &str = r"^(//.*\[TemplateHeader\].*(?:\n//.*)*)";

#[derive(Debug)]
pub enum RawTemplateHeader<'a> {
    SingleLine(regex::Captures<'a>),
    MultiLine(regex::Captures<'a>),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateHeader {
    pub shader_file: TemplateCorefile,
    pub output_path: StringOrSequence,
}

impl TemplateHeader {
    pub fn from_path(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file_string = std::fs::read_to_string(file_path)?;

        TemplateHeader::from_str(&file_string)
    }

    pub fn get_raw_template_header(s: &str) -> Result<RawTemplateHeader, Box<dyn Error>> {
        let multiline_toml_pat = RegexBuilder::new(MULTILINE_INFO_REGEX)
            .dot_matches_new_line(true)
            .build()?;
        let multisingle_toml_pat = RegexBuilder::new(SINGLELINE_INFO_REGEX)
            .multi_line(true)
            .build()?;

        if let Some(single_line_regex_capture) = multisingle_toml_pat.captures(s) {
            return Ok(RawTemplateHeader::SingleLine(single_line_regex_capture));
        } else if let Some(multi_line_regex_capture) = multiline_toml_pat.captures(s) {
            return Ok(RawTemplateHeader::MultiLine(multi_line_regex_capture));
        }

        Err(Box::new(Errors::TemplateHeaderNotPresent))
    }

    pub fn extract_toml_from_raw_header(s: &str) -> Result<String, Box<dyn Error>> {
        let raw_header = Self::get_raw_template_header(s)?;

        match raw_header {
            RawTemplateHeader::SingleLine(raw_single) => {
                let (_, [raw_toml]) = raw_single.extract();

                return Ok(raw_toml
                    .split("\n")
                    .map(|e| e[2..].trim())
                    .collect::<Vec<&str>>()
                    .join("\n"));
            }
            RawTemplateHeader::MultiLine(raw_multi) => {
                let (_, [raw_toml]) = raw_multi.extract();

                return Ok(raw_toml
                    .trim()
                    .split("\n")
                    .map(|e| e.trim())
                    .collect::<Vec<&str>>()
                    .join("\n"));
            }
        }

    }
}

impl FromStr for TemplateHeader {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toml = Self::extract_toml_from_raw_header(s)?;
        let parsed_toml = toml.parse::<Table>()?;

        let mut serializer_output = String::new();
        let toml_serializer = TomlSerializer::new(&mut serializer_output);

        let template_header_section = parsed_toml.get("TemplateHeader");

        template_header_section.serialize(toml_serializer)?;

        match toml::from_str::<TemplateHeader>(&serializer_output) {
            Ok(e) => return Ok(e),
            Err(e) => return Err(Box::new(e)),
        }
    }
}

impl Display for TemplateHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let shader_file_path = match self.shader_file.0.clone() {
            Some(e) => e,
            None => "None".into(),
        };

        write!(f, "shader_file path: {}\n", shader_file_path)?;

        let output_paths = match self.output_path.clone() {
            types::StringOrSequence::String(string) => string,
            types::StringOrSequence::Sequence(e) => e.join(", "),
            types::StringOrSequence::None => "None".into(),
        };

        write!(f, "output_paths: {}", output_paths)?;

        Ok(())
    }
}
