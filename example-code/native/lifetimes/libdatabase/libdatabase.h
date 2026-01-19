/*
 * libdatabase - a sample library written in C
 *
 * Copyright (c) Ferrous Systems, 2026
 *
 * In libdatabase we have three kinds of object:
 *
 * - Database (`database_t`)
 * - Table (`table_t`)
 * - Row (`row_t`)
 *
 * Rows belong to Tables, and Tables belong to Databases. You must open a
 * Database before you can open one of its Tables, and you must open a Table
 * before you can open one of its Rows.
 */

/******************************************************************************
 * Data Types
 *****************************************************************************/

 /// An opaque type representing a Database object
typedef struct database_t database_t;

/// An opaque type representing a Table object
typedef struct table_t table_t;

/// An opaque type representing a Row object
typedef struct row_t row_t;

/******************************************************************************
 * Database Methods
 *****************************************************************************/

/**
 * Create a new Database.
 *
 * - `database_name` - a NUL-terminated string which is the name of the database
 *   to create
 *
 * The function returns either a pointer to a new heap-allocated database, or
 * NULL on error.
 */
database_t*
libdatabase_database_create(const char* database_name);

/**
 * Get a Table from a Database.
 *
 * - `database` - the database we are getting the table from
 * - `table_name` - a NUL-terminated string holding the name of the table to get
 *
 * The function returns either a pointer to a new heap-allocated table, or NULL
 * on error.
 */
table_t*
libdatabase_database_get_table(database_t* p_database, const char* table_name);

/**
 * Drop a database handle.
 *
 * NOTE:
 *
 * - Ensure all Tables from this Database are closed before calling this
 *   function.
 * - Do not use this handle after calling this function.
 */
void
libdatabase_database_close(database_t* p_database);

/******************************************************************************
 * Table Methods
 *****************************************************************************/

/**
 * Get a row from a table.
 *
 * - `table` - the table we are getting the row from
 * - `row` - the zero-based numeric index of the row
 *
 * The function returns either a pointer to a row, or NULL on error.
 */
row_t*
libdatabase_table_get_row(table_t* p_table, unsigned int row_index);

/**
 * Drop a table handle.
 *
 * NOTE:
 *
 * - Ensure all Rows from this Table are closed before calling this
 *   function.
 * - Do not use this handle after calling this function.
 */
void
libdatabase_table_close(table_t* p_table);

/******************************************************************************
 * Row Methods
 *****************************************************************************/

/**
 * Print a row to stdout.
 */
void
libdatabase_row_print(row_t* p_row);

/**
 * Drop a row handle.
 *
 * NOTE:
 *
 * - Do not use this handle after calling this function.
 */
void libdatabase_row_close(row_t* p_row);

/******************************************************************************
 * End of file
 *****************************************************************************/
