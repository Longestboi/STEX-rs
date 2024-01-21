mod error;
mod types;

use regex::{Regex, RegexBuilder};
use serde::{Deserialize, Deserializer, Serialize};
use std::{error::Error, path::Path, str::FromStr};
use toml::{Serializer as TomlSerializer, Table};

use error::Errors;
use types::{StringOrSequence, TemplateLanguage};

#[derive(Debug, Serialize, Clone, Default)]
pub struct TemplateHeader {
    pub language: TemplateLanguage,
    pub output_path: StringOrSequence,
}

impl<'de> Deserialize<'de> for TemplateHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut template_header = TemplateHeader::default();

        let toml: toml::value::Value = toml::value::Value::deserialize(deserializer)?;

        // Parse out the language field
        if let Some(language_value) = toml.get("language") {
            if let Some(language) = language_value.as_str() {
                template_header.language = TemplateLanguage::new(language);
            };
        }

        // Parse out the output_path field
        if let Some(output_path) = toml.get("output_path") {
            match output_path.type_str() {
                "array" => {
                    let out_paths_raw = output_path.as_array()
                        .expect("Somehow, the array could not be unwraped, despite it being establised as an array");

                    let mut out_paths: Vec<String> = Vec::new();

                    for path in out_paths_raw.iter() {
                        if let Some(iter_path) = path.as_str() {
                            out_paths.push(iter_path.to_string());
                        } else {
                            return Err(Errors::Message(format!(
                                "Could not handle output path with type: '{}'",
                                path.type_str()
                            )))
                            .map_err(serde::de::Error::custom);
                        }
                    }

                    template_header.output_path = StringOrSequence::Sequence(out_paths);
                }
                "string" => {
                    if let Some(str_path) = output_path.as_str() {
                        template_header.output_path = StringOrSequence::String(str_path.into())
                    }
                }
                _ => {
                    return Err(Errors::Message(format!(
                        "Could not handle output path with type: '{}'",
                        output_path.type_str()
                    )))
                    .map_err(serde::de::Error::custom)
                }
            }
        }

        Ok(template_header)
    }
}

impl TemplateHeader {
    pub fn from_path(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let file_string = std::fs::read_to_string(file_path)?;

        TemplateHeader::from_str(&file_string)
    }
}

impl FromStr for TemplateHeader {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let multiline_toml_pat = Regex::new("\\/\\*([\\s\\S]*?)\\*\\/")?;
        let multisingle_toml_pat = RegexBuilder::new("^(\\/\\/.*(?:\n\\/\\/.*)*)").multi_line(true).build()?;

        let mut toml: Option<String> = None;

        if let Some(single_line_regex_capture) = multisingle_toml_pat.captures(s) {
            let (_, [raw_toml]) = single_line_regex_capture.extract();

            toml = Some(raw_toml
                .split("\n")
                .map(|e| e[2..].trim())
                .collect::<Vec<&str>>()
                .join("\n")
            );

        } else if let Some(multi_line_regex_capture) = multiline_toml_pat.captures(s) {
            let (_, [raw_toml]) = multi_line_regex_capture.extract();
            
            toml = Some(
                raw_toml
                .trim()
                .split("\n")
                .map(|e| e.trim())
                .collect::<Vec<&str>>()
                .join("\n"),
            );
        }
        
        if let Some(toml_inner) = toml {
            let parsed_toml = match toml_inner.parse::<Table>() {
                Ok(e) => e,
                Err(e) => return Err(Box::new(e)),
            };

            let mut serializer_output = String::new();
            let toml_serializer = TomlSerializer::new(&mut serializer_output);
            
            let template_header_section = parsed_toml.get("TemplateHeader");

            match template_header_section.serialize(toml_serializer) {
                Ok(_) => match toml::from_str::<TemplateHeader>(&serializer_output) {
                    Ok(e) => return Ok(e),
                    Err(e) => return Err(Box::new(e)),
                },
                Err(e) => return Err(Box::new(e)),
            }
        }

        Err(Box::new(Errors::Message(
            "Could not find Template Header".into(),
        )))
    }
}
