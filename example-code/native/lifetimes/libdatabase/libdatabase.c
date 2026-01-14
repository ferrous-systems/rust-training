/*
 * libdatabase - a sample library written in C
 *
 * Copyright (c) Ferrous Systems, 2026
 *
 * This is an implementation of the API given in libdatabase.h
 */

#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include "libdatabase.h"

#define DATABASE_NAME_MAX_LEN 32

struct database_t {
    const char* name;
};

struct table_t {
    database_t* p_parent;
    const char* name;
};

struct row_t {
    table_t* p_parent;
    unsigned int index;
};

/**
 * Heap allocate a new database object.
 *
 * I hope that string sticks around - we don't copy it.
 */
database_t* libdatabase_database_create(const char* database_name) {
    printf("> %s\n", __FUNCTION__);
    database_t* p_database = (database_t*) calloc(1, sizeof(database_t));
    if (!p_database) {
        return NULL;
    }
    p_database->name = database_name;
    return p_database;
}

/**
 * Heap allocate a new table, pointing at the database.
 *
 * I hope that string sticks around - we don't copy it.
 */
table_t*
libdatabase_database_add_table(database_t* p_database, const char* table_name) {
    printf("> %s\n", __FUNCTION__);
    if (!p_database) {
        return NULL;
    }
    table_t* p_table = (table_t*) calloc(1, sizeof(table_t));
    if (!p_table) {
        return NULL;
    }
    p_table->name = table_name;
    p_table->p_parent = p_database;
    return p_table;
}

/**
 * Destroy the database object.
 *
 * I hope there aren't any tables left lying around.
 */
void libdatabase_database_close(database_t* p_database) {
    printf("< %s\n", __FUNCTION__);
    if (!p_database) {
        return;
    }
    free(p_database);
}

/**
 * Heap allocate a new row that points to the table
 */
row_t*
libdatabase_table_get_row(table_t* p_table, unsigned int row_index) {
    printf("> %s\n", __FUNCTION__);
    if (!p_table) {
        return NULL;
    }
    row_t* p_row = (row_t*) calloc(1, sizeof(row_t));
    if (!p_row) {
        return NULL;
    }
    p_row->p_parent = p_table;
    p_row->index = row_index;
    return p_row;
}

/**
 * Destroy the table object.
 *
 * I hope there aren't any rows left lying around.
 */
void libdatabase_table_close(table_t* p_table) {
    printf("< %s\n", __FUNCTION__);
    if (!p_table) {
        return;
    }
    free(p_table);
}

/**
 * Print a row to stdout.
 *
 * Useful to check the pointers are all still valid.
 */
void libdatabase_row_print(row_t* p_row) {
    if (!p_row) {
        return;
    }
    table_t* p_table = p_row->p_parent;
    database_t* p_database = p_table->p_parent; 
    printf("DB: %s, Table: %s, Row: %u\n", p_database->name, p_table->name, p_row->index);
}

/**
 * Destroy the row object
 */
void libdatabase_row_close(row_t* p_row) {
    printf("< %s\n", __FUNCTION__);
    if (!p_row) {
        return;
    }
    free(p_row);
}

/*
 * End of file
 */
