use std::error::Error;

mod replace;
mod template;

use template::Template;

fn main() {
    match testing() {
        Ok(_) => return,
        Err(e) => {
            println!("{}", e);
        },
    }
}

fn testing() -> Result<(), Box<dyn Error>> {
    let teste = Template::from_path("./Unity.shader")?;

    // toml::to_string_pretty::<TemplateHeader>(&teste.template_header);

    println!("{}", toml::to_string_pretty::<Template>(&teste)?);

    println!("{:?}", teste.template_header);
    
    Ok(())
}