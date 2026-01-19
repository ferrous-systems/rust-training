//! Rust wrapper around libdatabase
//! 
//! > Copyright (c) Ferrous Systems, 2026

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum Error {
    UnknownFailure,
}

/// The Rust representation of a libdatabase Database
pub struct Database<'name> {
    inner: *mut libdatabase_sys::database_t,
    ph: PhantomData<&'name std::ffi::CStr>,
}

impl<'name> Database<'name> {
    /// Create a new Database
    pub fn new(name: &'name std::ffi::CStr) -> Result<Database<'name>, Error> {
        let p = unsafe { libdatabase_sys::libdatabase_database_create(name.as_ptr()) };
        if p.is_null() {
            Err(Error::UnknownFailure)
        } else {
            Ok(Database {
                inner: p,
                ph: PhantomData,
            })
        }
    }

    /// Get a Table from a Database
    pub fn get_table<'db, 'table_name>(
        &'db mut self,
        table_name: &'table_name std::ffi::CStr,
    ) -> Result<Table<'db, 'table_name>, Error> {
        let p = unsafe {
            libdatabase_sys::libdatabase_database_get_table(self.inner, table_name.as_ptr())
        };
        if p.is_null() {
            return Err(Error::UnknownFailure);
        }
        Ok(Table {
            inner: p,
            db_ph: PhantomData,
            name_ph: PhantomData,
        })
    }
}

impl<'name> Drop for Database<'name> {
    fn drop(&mut self) {
        unsafe {
            libdatabase_sys::libdatabase_database_close(self.inner);
        }
    }
}

/// The Rust representation of a libdatabase Table
pub struct Table<'db, 'name> {
    inner: *mut libdatabase_sys::table_t,
    db_ph: PhantomData<&'db mut Database<'db>>,
    name_ph: PhantomData<&'name std::ffi::CStr>,
}

impl<'db, 'name> Table<'db, 'name> {
    /// Get a row from a table
    pub fn get_row<'table>(
        &'table mut self,
        row_index: std::ffi::c_uint,
    ) -> Result<Row<'table>, Error> {
        let p = unsafe { libdatabase_sys::libdatabase_table_get_row(self.inner, row_index) };
        if p.is_null() {
            return Err(Error::UnknownFailure);
        }
        Ok(Row {
            inner: p,
            ph: PhantomData,
        })
    }
}

impl<'db, 'name> Drop for Table<'db, 'name> {
    fn drop(&mut self) {
        unsafe {
            libdatabase_sys::libdatabase_table_close(self.inner);
        }
    }
}

/// The Rust representation of a libdatabase Row
pub struct Row<'table> {
    inner: *mut libdatabase_sys::row_t,
    ph: PhantomData<&'table Table<'table, 'table>>,
}

impl<'table> Row<'table> {
    /// Print a row to stdout.
    ///
    /// Useful to check all the pointers are still valid.
    pub fn print(&self) {
        unsafe {
            libdatabase_sys::libdatabase_row_print(self.inner);
        }
    }
}

impl<'table> Drop for Row<'table> {
    fn drop(&mut self) {
        unsafe {
            libdatabase_sys::libdatabase_row_close(self.inner);
        }
    }
}

// End of file
