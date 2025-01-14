
extern crate structopt;
mod imagix;

use ::imagix::error::ImagixError;
use ::imagix::resize::{process_resize_request, Mode, SizeOption};
use ::imagix::stats::get_stats;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "resize", about = "This is a tool for resizing images.",
help = "Specify subcommand resize or stats. For help type image-cli resize --help or image-cli stats --help")]
enum Commandline {
    #[structopt(help = "Specify size(small/medium/large), mode(single/all) and src folder")]
    Resize {
        #[structopt(short, long, help = "The mode of resizing")]
        mode: Mode,
        #[structopt(short, long, help = "The size of the image")]
        size: SizeOption,
        #[structopt(long, parse(from_os_str))]
        srcfolder: PathBuf,
    },
    #[structopt(name = "stats", about = "Get stats of images")]
    Stats {
        #[structopt(long, parse(from_os_str))]
        srcfolder: PathBuf,
    },
}

fn main() {
    let args = Commandline::from_args();
    match args {
        Commandline::Resize { size, mode, mut srcfolder } => {
            match process_resize_request(size, mode, &mut srcfolder) {
                Ok(_) => println!("Images resized successfully"),
                Err(e) => match e {
                    ImagixError::FileIOError(e) => eprintln!("File operator failed: {}", e),
                    ImagixError::UserInputError(e) => eprintln!("User input error: {}", e),
                    ImagixError::ImageResizingError(e) => eprintln!("Image resize error: {}", e),

                    _ => println!("Format error: {}", e),
                }            
            }
        }
        Commandline::Stats { srcfolder } => {
            match get_stats(srcfolder) {
                Ok(_) => println!("Stats generated successfully"),
                Err(e) => match e {
                    ImagixError::FileIOError(e) => eprintln!("File operator failed: {}", e),
                    ImagixError::UserInputError(e) => eprintln!("User input error: {}", e),

                    _ => println!("Format error: {}", e),
                }
            }
        }
    }
}