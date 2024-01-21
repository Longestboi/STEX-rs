use std::error::Error;

mod replace;
mod template_header;

use replace::Replacer;
use template_header::TemplateHeader;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::current_dir()?.as_path().join("Unity.shader");

    let teste = TemplateHeader::from_path(&path);//(&testing);

    let _temp = Replacer::replace_token_in_file(
        path,
        "{{% CoreShader %}}",
        "fn testing() { return; }",
    );

    println!("{:?}", teste);

    // println!("{}", _temp?);

    Ok(())
}
