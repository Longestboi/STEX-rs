pub mod template_header;

use std::{error::Error, path::Path};

use serde::Serialize;
use template_header::TemplateHeader;

#[derive(Debug, Default, Serialize)]
pub struct Template {
    #[serde(rename = "TemplateHeader")]
    pub template_header: TemplateHeader,
    #[serde(skip_serializing)]
    pub template_text: String,
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
                template.template_text = file_text.replace(&full_match, "");
            },
            template_header::RawTemplateHeader::MultiLine(multi_cap) => {
                let full_match = multi_cap.extract::<1>().0;
                template.template_text = file_text.replace(&full_match, "");
            },
        }

        Ok(template)
    }
}
