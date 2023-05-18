#include <stdio.h>
#include <string.h>

#define MAX_LINE_LENGTH 1024

void copy_file_contents( FILE *input_file, FILE *output_file, char *line ) {
    while( fgets( line, sizeof(line), input_file ) ) {
        fwrite( line, sizeof(char), strlen(line), output_file );
    }
}

void write_x_times( FILE *input_file, int times, FILE *output_file, char *line ) {
    for( int i = 0; i < times; i++ ) {
        copy_file_contents( input_file, output_file, line );
        rewind( input_file );
    }
}

int main() {
    FILE *file = fopen( "products.csv", "r" );
    FILE *output_file = fopen( "output.csv", "w" );

    char line[MAX_LINE_LENGTH];
    write_x_times( file, 10000, output_file, line );

    fclose( file );
    fclose( output_file );
    return 0;
}
