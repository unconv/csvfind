use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const USAGE: &str = "Usage: csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]";

trait ContainsExt {
    fn contains_ignore_ascii_case(&self, other: &str) -> bool;
}

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next(); // Skip the program name
    let input_file_path = args.next().expect(USAGE);
    let search_term = args.next().expect(USAGE);
    let header_name = args.next();

    let input_file = File::open(input_file_path)?;

    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    let headers = lines
        .next()
        .expect("Headers should be presented in the CSV file")?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    for line in lines {
        let line = line?;

        // Assume that the number of columns are the same in the header and the line
        for (col, current_header) in line.split(',').zip(&headers) {
            if let Some(header_name) = &header_name {
                if !header_name.eq_ignore_ascii_case(current_header) {
                    continue;
                }
            }

            if col.contains_ignore_ascii_case(&search_term) {
                println!("{}", line);
                break;
            }
        }
    }

    Ok(())
}

impl ContainsExt for str {
    fn contains_ignore_ascii_case(&self, needle: &str) -> bool {
        let mut needle_iter = needle.chars();

        for haystack_char in self.chars() {
            let Some(needle_char) = needle_iter.next() else {
                return true;
            };

            if !needle_char.eq_ignore_ascii_case(&haystack_char) {
                needle_iter = needle.chars();
            }
        }
        false
    }
}
