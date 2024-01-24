use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(untagged)]
pub enum StringOrSequence {
    String(String),
    Sequence(Vec<String>),
    #[default]
    None,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TemplateCorefile(pub Option<String>);

impl Default for TemplateCorefile {
    fn default() -> Self {
        Self(None)
    }
}