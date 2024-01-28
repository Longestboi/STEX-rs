#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("Template info header not present in file.")]
    TemplateHeaderNotPresent,
    // #[error("Could not parse template info header.")]
    // TemplateHeaderNotParsed
}