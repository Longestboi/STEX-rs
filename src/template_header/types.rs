use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub enum StringOrSequence {
    String(String),
    Sequence(Vec<String>),
    #[default]
    None,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateLanguage(pub Option<String>);

impl Default for TemplateLanguage {
    fn default() -> Self {
        Self(None)
    }
}

impl TemplateLanguage {
    pub fn new(string: impl TryInto<String>) -> Self {

        let mut template_lang = Self::default();
        
        if let Ok(ok_string) = string.try_into() as Result<String, _> {
            template_lang.0 = Some(ok_string);
        }

        template_lang   
    }
}