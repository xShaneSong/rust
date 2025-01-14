use clap::{App, Arg, SubCommand};
use crate::imagix::{resize::resize_image, stats::image_stats};

fn main() {
    let matches = App::new("image-cli")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("CLI for image processing")
        .subcommand(
            SubCommand::with_name("resize")
                .about("Resize an image")
                .arg(Arg::with_name("input").required(true).help("Input image file"))
                .arg(Arg::with_name("output").required(true).help("Output image file"))
                .arg(Arg::with_name("width").required(true).help("Width of the output image"))
                .arg(Arg::with_name("height").required(true).help("Height of the output image")),
        )
        .subcommand(
            SubCommand::with_name("stats")
                .about("Get image statistics")
                .arg(Arg::with_name("file").required(true).help("Image file")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("resize") {
        let input = matches.value_of("input").unwrap();
        let output = matches.value_of("output").unwrap();
        let width: u32 = matches.value_of("width").unwrap().parse().unwrap();
        let height: u32 = matches.value_of("height").unwrap().parse().unwrap();
        resize_image(input, output, width, height).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("stats") {
        let file = matches.value_of("file").unwrap();
        image_stats(file).unwrap();
    }
}
