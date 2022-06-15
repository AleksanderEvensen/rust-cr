#[allow(non_snake_case)]
mod Console;
mod demos;
use demos::{basic_rendering, image, main};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = String::from("main"))]
    demo: String,
}

fn main() {
    let args = Args::parse();

    match args.demo.to_lowercase().as_str() {
        "main" => main::run(),
        "render" => basic_rendering::run(),
        "image" => image::run(),
        _ => println!("Unknown demo: {}", args.demo),
    }
}
