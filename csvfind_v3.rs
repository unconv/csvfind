use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const USAGE: &str = "Usage: csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]";

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next(); // Skip the program name
    let input_file_path = args.next().expect(USAGE);
    let search_term = args.next().map(|s| s.to_ascii_lowercase()).expect(USAGE);
    let header_name = args.next().map(|col| col.trim().to_ascii_lowercase());

    let input_file = File::open(input_file_path)?;

    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    let raw_headers = lines
        .next()
        .expect("Headers should be present in the CSV file")?;

    println!("{}", raw_headers);

    let header_idx = header_name.as_ref().and_then(|filter_col| {
        raw_headers
            .split(',')
            .map(|col| col.trim().to_ascii_lowercase())
            .position(|col| &col == filter_col)
    });

    for line in lines.map_while(Result::ok) {
        if !line.to_ascii_lowercase().contains(&search_term) {
            continue;
        }
        let Some(col_idx) = header_idx else {
            println!("{line}");
            continue;
        };
        // get the correct cell from the expected index
        let Some(cell) = line.split(',').nth(col_idx).map(|c| c.trim()) else {
            continue;
        };
        if cell.to_ascii_lowercase().contains(&search_term) {
            println!("{line}");
            continue;
        }
    }

    Ok(())
}
