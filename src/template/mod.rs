pub mod template_header;

use std::{error::Error, fmt::Display, path::Path};

use serde::Serialize;
use template_header::TemplateHeader;

#[derive(Debug, Default, Serialize)]
pub struct Template {
    #[serde(rename = "TemplateHeader")]
    pub template_header: TemplateHeader,
    #[serde(skip_serializing)]
    pub template_text: String,
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let template_header: String = self
            .template_header
            .to_string()
            .split("\n")
            .map(|e| "\t".to_string() + e)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "Template Header:\n")?;
        write!(f, "{template_header}\n\n")?;

        let template_text = self
            .template_text
            .split("\n")
            .map(|e| "\t".to_string() + e)
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "Template Text:\n")?;
        write!(f, "{template_text}")?;

        Ok(())
    }
}

impl Template {
    pub fn from_path(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut template = Template::default();

        template.template_header = TemplateHeader::from_path(&file_path)?;

        let file_text = std::fs::read_to_string(&file_path)?;

        let text = TemplateHeader::get_raw_template_header(&file_text)?;

        match text {
            template_header::RawTemplateHeader::SingleLine(single_cap) => {
                let full_match = single_cap.extract::<1>().0;
                template.template_text = file_text.replace(&full_match, "").trim_start().to_string();
            },
            template_header::RawTemplateHeader::MultiLine(multi_cap) => {
                let full_match = multi_cap.extract::<1>().0;
                template.template_text = file_text.replace(&full_match, "").trim_start().to_string();
            },
        }

        Ok(template)
    }
}
