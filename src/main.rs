mod args;
mod utils;

use std::fs;

use comrak::{self, ComrakOptions};

use args::Args;

fn main() {

    let args = Args::parse();

    let input_content = fs::read_to_string(args.input.to_string()).unwrap_or_else(|err| {
        eprintln!("An error occurred when reading the input file: {}", err);
        std::process::exit(1);
    });

    let html = comrak::markdown_to_html(&input_content, &ComrakOptions::default());

    let outdir = std::path::Path::new(&args.output_dir);

    if !outdir.is_dir() {
        std::fs::create_dir_all(outdir).unwrap_or_else(|err| {
            if args.opt_verbose() {
                eprintln!(
                    "{}\n{:?}\n{}",
                    "There was an error creating the output directory",
                    args,
                    err
                )
            } else {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        });
    }

    fs::write(args.output(), html).unwrap_or_else(|err| {
        if args.opt_verbose() {
            eprintln!(
                "{}\n{:?}\n{}",
                "There was an error writing the output to a file",
                args,
                err
            )
        } else {
            eprintln!("{}", err);
        }
        std::process::exit(1)
    });

}
