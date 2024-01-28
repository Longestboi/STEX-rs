mod replace;
mod template;

mod core;
mod daemon;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author = "testing", version = "1", long_about=None)]
struct Args {
    #[arg(short='D', long)]
    daemon: bool,
    #[arg(short='C', long)]
    core_search_location: Option<String>,
    #[arg(short='F', long)]
    force: bool,
    // Template
    #[arg(short='t', long)]
    template: Option<String>,
    #[arg(short='o', long)]
    output: Option<String>,
    #[arg(short='I', long)]
    ignore_header: bool,
    #[arg(short='c', long)]
    core_override: Option<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    if !args.daemon {
        match core::core(&args) {
            Ok(_) => (),
            Err(er) => println!("Error: {}", er),
        };
    } else {
        daemon::daemon(&args)?;
    }
    Ok(())
}