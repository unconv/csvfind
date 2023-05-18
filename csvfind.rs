use std::{fs::File, io::{BufReader, BufRead}, process};

fn show_usage() {
    println!( "Usage: csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]" );
}
fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    let input_file_path = args.get( 1 ).unwrap_or_else( || {
        show_usage();
        process::exit( 1 );
    });

    let search_term = args.get( 2 ).unwrap_or_else(|| {
        show_usage();
        process::exit( 1 );
    }).to_lowercase();

    let input_file = File::open( input_file_path ).unwrap_or_else( |_| {
        println!( "Unable read input file" );
        process::exit( 1 );
    } );

    let header_name = args.get( 3 ).unwrap_or(&String::new()).to_lowercase();

    let mut headers: Vec<String> = Vec::new();

    let reader = BufReader::new( input_file );
    reader.lines().enumerate().for_each(|(line_number, line)| {
        let line = line.unwrap_or_else(|e| {
            println!("Error while reading line: {}", e);
            process::exit( 1 );
        });

        let mut print_line = false;

        for (col_number, col) in line.split(",").enumerate() {
            if line_number == 0 {
                headers.push( col.trim().to_string() );
            }

            let current_header = headers.get( col_number );

            if ! header_name.is_empty() && ( current_header.is_none() || header_name != current_header.unwrap().to_lowercase() ) {
                continue;
            }

            if line_number == 0 || col.to_lowercase().contains( search_term.as_str() )  {
                print_line = true;
                break;
            }
        };

        if print_line {
            println!( "{}", line );
        }
    });
}
