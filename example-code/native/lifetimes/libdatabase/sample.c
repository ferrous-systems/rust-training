/*
 * Sample program showing how to use libdatabase
 */

#include <stdio.h>
#include <stdint.h>
#include <memory.h>


#include "libdatabase.h"

int main(int argc, char** argv) {
    printf("Starting database example...\n");

    printf("1. Opening Database\n");
    database_t* p_database = libdatabase_database_create("example_db");
    if (!p_database) {
        fprintf(stderr, "Failed to open database\n");
        return -1;
    }

    printf("2. Opening Table in Database\n");
    table_t* p_table = libdatabase_database_add_table(p_database, "example_table");
    if (!p_table) {
        fprintf(stderr, "Failed to open table\n");
        return -1;
    }

    printf("3. Opening Row in Table\n");
    row_t* p_row = libdatabase_table_get_row(p_table, 10);
    if (!p_row) {
        fprintf(stderr, "Failed to open row\n");
        return -1;
    }

    printf("4. Printing Row\n");
    libdatabase_row_print(p_row);

    printf("5. Closing Row\n");
    libdatabase_row_close(p_row);

    printf("6. Closing Table\n");
    libdatabase_table_close(p_table);

    printf("7. Closing Database\n");
    libdatabase_database_close(p_database);

    return 0;
}

/*
 * End of file
 */
