use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
    // let pattern = std::env::args()
    //     .nth(1)
    //     .expect("no pattern given");
    // let path = std::env::args()
    //     .nth(2)
    //     .expect("no path given");
    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let args = Cli::parse();
    print!("pattern: {:?}, path: {:?}\n", args.pattern, args.path);

    // let result= std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => {
    //         content
    //         // println!("File content: {}", content);
    //     },
    //     Err(error) => {
    //         return Err(error.into());
    //         // panic!("Error reading file: {}", error);
    //         // std::process::exit(1);
    //     }
    // };

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    println!("File content: {}", content);
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // if content.is_empty() {
    //     println!("The file is empty.");
    // } else {
    //     println!("File read successfully.");
    // }
    Ok(())
}
