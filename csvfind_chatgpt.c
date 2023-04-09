#include <stdio.h>
#include <string.h>

#define MAX_LINE_LENGTH 1024
#define MAX_FIELD_LENGTH 256
#define MAX_FIELDS 100

int main(int argc, char *argv[]) {
    if (argc < 3) {
        printf("Usage: csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]\n");
        return 1;
    }
    
    char *input_file = argv[1];
    char *search_term = argv[2];
    char *header_name = NULL;
    if (argc == 4) {
        header_name = argv[3];
    }
    
    FILE *fp = fopen(input_file, "r");
    if (fp == NULL) {
        printf("Failed to open file %s\n", input_file);
        return 1;
    }
    
    char line[MAX_LINE_LENGTH];
    char field[MAX_FIELD_LENGTH];
    char *fields[MAX_FIELDS];
    int num_fields = 0;
    int header_index = -1;
    int line_number = 0;
    while (fgets(line, MAX_LINE_LENGTH, fp) != NULL) {
        line_number++;
        if (line_number == 1) { // header line
            char *ptr = line;
            while (*ptr != '\0') {
                // extract fields
                if (num_fields == MAX_FIELDS) {
                    printf("Too many fields in header\n");
                    return 1;
                }
                sscanf(ptr, "%[^,\n]", field);
                fields[num_fields] = strdup(field);
                num_fields++;
                // move ptr to next field
                ptr += strlen(field);
                if (*ptr == ',') {
                    ptr++; // skip comma
                } else {
                    break; // end of line
                }
            }
            // find header index
            if (header_name != NULL) {
                int i;
                for (i = 0; i < num_fields; i++) {
                    if (strcmp(fields[i], header_name) == 0) {
                        header_index = i;
                        break;
                    }
                }
                if (header_index == -1) {
                    printf("Header %s not found\n", header_name);
                    return 1;
                }
            }
            // print header
            int i;
            for (i = 0; i < num_fields; i++) {
                printf("%s", fields[i]);
                if (i < num_fields - 1) {
                    printf(",");
                } else {
                    printf("\n");
                }
            }
        } else { // data line
            char *ptr = line;
            int field_index = 0;
            int match = 0;
            while (*ptr != '\0') {
                // extract field
                sscanf(ptr, "%[^,\n]", field);
                // check if field matches search term
                if (header_index == -1 || field_index == header_index) {
                    if (strstr(field, search_term) != NULL) {
                        match = 1;
                        break;
                    }
                }
                // move ptr to next field
                ptr += strlen(field);
                if (*ptr == ',') {
                    ptr++; // skip comma
                } else {
                    break; // end of line
                }
                field_index++;
            }
            // print line if it matches search term
            if (match) {
                printf("%s", line);
            }
        }
    }
    
    fclose(fp);
    return 0;
}
