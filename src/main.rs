mod args;
mod utils;
mod html;

use std::fs;

use args::Args;

fn main() {

    let args = Args::parse();

    let input_content = fs::read_to_string(args.input.to_string()).unwrap_or_else(|err| {
        eprintln!("An error occurred when reading the input file: {}", err);
        std::process::exit(1);
    });

    let input_filename = args.input_filename().replace(".html", "");

    let html_content = html::md_to_html(&input_filename, &input_content);

    let outdir = std::path::Path::new(&args.output_dir);

    if !outdir.is_dir() {
        std::fs::create_dir_all(outdir).unwrap_or_else(|err| {
            if args.is_verbose {
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
    
    fs::write(args.output(), html_content).unwrap_or_else(|err| {
        if args.is_verbose {
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
