<?php
if( $argc < 3 ) {
    printf( "Usage: %s INPUT_FILE SEARCH_TERM [HEADER_NAME]\n",$argv[0] );
    exit;
}

$input_file  = $argv[1];
$search_term = ( $argv[2] );
$header_name = $argv[3] ?? null;

if( $header_name ) {
    $header_name = ( $header_name );
}

$headers = [];
$line_number = 0;
$file = fopen( $input_file, "r" );

while( ($line = fgets( $file )) !== false ) {
    $cols = explode( ",", $line );
    $print_line = false;

    foreach( $cols as $col_index => $col ) {
        if( $line_number === 0 ) {
            $headers[$col_index] = trim( $col );
            $print_line = true;
            continue;
        }

        if( $header_name !== null && $header_name !== ( $headers[$col_index] ?? null ) ) {
            continue;
        }

        if( stripos( $col, $search_term ) !== false ) {
            $print_line = true;
        }
    }

    if( $print_line ) {
        echo $line;
    }

    $line_number++;
}

fclose( $file );
