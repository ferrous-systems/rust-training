//! A program showing how to use libdatabase-rs
//! 
//! > Copyright (c) Ferrous Systems, 2026

use std::str::FromStr;

fn main() {
    let db_name = std::ffi::CString::from_str("my_database").expect("valid db name");
    let table_name = std::ffi::CString::from_str("my_table").expect("valid db name");
    let mut db = libdatabase_rs::Database::new(&db_name).expect("Opening database");

    // You cannot drop the database name then go on to use the database:
    // drop(db_name);

    let mut table = db.get_table(&table_name).expect("adding table");

    // You cannot drop the DB then go on to use it, or its tables, or its rows:
    // drop(db);

    // You cannot drop the table name then go on to use the table or its rows:
    // drop(table_name);

    let row = table.get_row(100).expect("getting row");

    // You cannot drop the table then go on to use the table or its rows
    // drop(table);

    row.print();

    // still cannot drop the table name - we need it around to drop the table
    // drop(table_name);

    // we can drop things in this order (which is the default order):
    drop(row);
    drop(table);
    drop(table_name);
    drop(db);
    drop(db_name);
}
