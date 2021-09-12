use std::path::{is_separator, MAIN_SEPARATOR};

pub fn extract_filename(path: &str) -> (String, String) {
    let mut filename = String::new();

    let mut dirs: Vec<String> = Vec::new();

    for c in path.chars() {
        if is_separator(c) {
            dirs.push(filename);
            filename = String::new();
        } else {
            filename.push(c);
        }
    }

    let ext_idx = filename.find(".").unwrap_or(filename.len());

    filename.replace_range(ext_idx.., ".html");

    (dirs.join(&MAIN_SEPARATOR.to_string()), filename)
}
