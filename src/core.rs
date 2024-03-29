use std::{fs::File, io::Write, path::Path};
use crate::{replace::Replacer, template::Template};
use path_dedot::*;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Template file not specified")]
    TemplateFileNotSpecified,
    #[error("Template file could not be found")]
    TemplateFileNotFound,
    #[error("Output not specified")]
    OutputNotSpecified,
    #[error("Output not specified in template")]
    OutputNotSpecifiedTemplate,
    #[error("Core file not specified")]
    CoreFileNotSpecified,
    #[error("Core file not specified in template")]
    CoreFileNotSpecifiedTemplate,
    #[error("Core file '{0}' could not be found")]
    CoreFileNotFound(String),
    #[error("Unknown Error")]
    UnknownError,
}

pub fn core(args: &super::Args) -> Result<(), Box<dyn std::error::Error>> {

    // Was a template File Specified?
    let template_file = match &args.template {
        Some(e) => e.clone(),
        None => return Err(Box::new(Error::TemplateFileNotSpecified)),
    };
    
    // Does the file Exist?
    let template_file_path = Path::new(&template_file);
    if !template_file_path.is_file() {
        return Err(Box::new(Error::TemplateFileNotFound));
    }

    // Is metadata present?
    let template = match Template::from_path(template_file_path) {
        Ok(e) => e,
        Err(err) => {
            // Is ignore header passed?
            if !&args.ignore_header {
                return Err(err);
            }

            // Is an output specified?
            if args.output.is_none() {
                return Err(Box::new(Error::OutputNotSpecified));
            }
            
            // Is a core override specified?
            if args.core_override.is_none() {
                return Err(Box::new(Error::CoreFileNotSpecified));
            }

            Template::default()
        },
    };

    let full_template_path = std::fs::canonicalize(template_file_path)?;

    // Use specified output, if specified
    let output_paths = match args.output.as_ref() {
        Some(e) => vec![e.clone()],
        None => {
            use crate::template::template_header::types as THTypes;
            match &template.template_header.output_path {
                THTypes::StringOrSequence::String(e) => {
                    vec![format!("{}/{}", full_template_path.to_str().unwrap(), e.clone())]
                },
                THTypes::StringOrSequence::Sequence(seq) => {
                    seq.clone().iter().map(| f | format!("{}/{}", full_template_path.to_str().unwrap(), f)).collect::<Vec<String>>()
                },
                THTypes::StringOrSequence::None => {
                    return Err(Box::new(Error::OutputNotSpecifiedTemplate));
                }
            }
        },
    };

    // Get core search path
    let core_path = match args.core_override.as_ref() {
        Some(e) => e.clone(),
        None => {
            match template.template_header.shader_file.0 {
                Some(e) => e,
                None => return Err(Box::new(Error::CoreFileNotSpecifiedTemplate)),
            }
        },
    };

    // 
    let core_search_path = match args.core_search_location.as_ref() {
        Some(e) => {
            let mut string_path = e.clone();
            
            if !string_path.ends_with("/") {
                string_path.push('/')
            }
            
            string_path
        },
        None => "./".into(),
    };

    let full_core_path = format!("{core_search_path}{}", core_path);

    let core_string = match std::fs::read_to_string(&full_core_path) {
        Ok(e) => e,
        Err(_) => {
            return Err(Box::new(Error::CoreFileNotFound(full_core_path)));
        },
    };

    let replaced = match Replacer::replace_token_in_string(template.template_text, "<% CoreShader %>", core_string) {
        Ok(e) => e,
        Err(e) => {
            return Err(e);
        },
    };

    // Do output stuff here
    for i in output_paths {

        let parsed_output_path = Path::new(&i).parse_dot()?;
        
        let template_parent = match Path::new(&template_file).file_name() {
            Some(e) => {
                match e.to_str() {
                    Some(e) => e,
                    None => return Err(Box::new(Error::UnknownError)),
                }
            },
            None => return Err(Box::new(Error::UnknownError)),
        };
        
        let poutput = match parsed_output_path.to_str() {
            Some(e) => e,
            None => return Err(Box::new(Error::UnknownError)),
        };

        let mut file = File::create(&parsed_output_path)?;
        
        file.write_all(replaced.as_bytes())?;
        println!("Output template '{}' to {}", template_parent, poutput);
    }

    Ok(())
}