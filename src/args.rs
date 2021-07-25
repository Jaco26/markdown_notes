
use crate::utils::extract_filename;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output_dir: String,
    pub output_file: String,
    pub options: Vec<String>,
}


impl Args {

    pub fn parse() -> Args {
        let mut args = Args {
            input: String::new(),
            output_dir: String::from("./output"),
            output_file: String::new(),
            options: Vec::new(),
        };

        let mut env_args = std::env::args();

        env_args.next();

        match env_args.next() {
            Some(input) => {
                args.input = input;
                let output = env_args.next();

                match output {
                    Some(output) => {
                        if output.starts_with("-") {
                            let (_, filename) = extract_filename(&args.input);
                            args.output_file = filename;
                            args.options.push(output);
                        } else {
                            let (dirs_path, filename) = extract_filename(&output);
                            args.output_file = filename;
                            args.output_dir = dirs_path;
                        }
                    }
                    None => {
                        let (_, filename) = extract_filename(&args.input);
                        args.output_file = filename;
                    }
                }
            }
            None => {}
        };

        args.options = vec![args.options, env_args.collect()].concat();

        args
    }

    pub fn output(&self) -> String {
        format!(
            "{}{}{}",
            self.output_dir,
            std::path::MAIN_SEPARATOR.to_string(),
            self.output_file
        )
    }

    pub fn opt_verbose(&self) -> bool {
        self.options.contains(&String::from("-v"))
        || self.options.contains(&String::from("--verbose"))
    }

}