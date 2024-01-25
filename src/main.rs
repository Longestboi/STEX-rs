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

    println!("{}", teste);
    
    Ok(())
}