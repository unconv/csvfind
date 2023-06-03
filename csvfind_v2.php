<?php
if( $argc < 3 ) {
    printf( "Usage: %s INPUT_FILE SEARCH_TERM [HEADER_NAME]\n",$argv[0] );
    exit;
}

$input_file  = $argv[1];
$search_term = ( $argv[2] );
$header_name = $argv[3] ?? null;
$headers = [];
$file = fopen( $input_file, "r" );
$firstLine = fgets( $file );

echo $firstLine;

$headerIndex = null;

if ($header_name) {
    $headers = array_map(static fn (string $value) => trim($value), explode(",", $firstLine));
    $headerIndex = array_search($header_name, $headers);
}

while (($line = fgets($file)) !== false) {
    if( stripos( $line, $search_term )  ) {

        if ($headerIndex === null) {
            echo $line;
            continue;
        }

        $cols = explode(",", $line);
        if (array_key_exists($headerIndex, $cols) && stripos($cols[$headerIndex], $search_term) !== false) {
            echo $line;
        }
    }
}
fclose($file);