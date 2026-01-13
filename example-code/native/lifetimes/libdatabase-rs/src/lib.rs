//! Rust wrapper around libdatabase
//! 
//! > Copyright (c) Ferrous Systems, 2026

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub enum Error {
    UnknownFailure,
}

pub struct Database<'name> {
    inner: *mut libdatabase_sys::database_t,
    ph: PhantomData<&'name std::ffi::CStr>,
}

impl<'name> Database<'name> {
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

    pub fn add_table<'table_name>(
        &mut self,
        table_name: &'table_name std::ffi::CStr,
    ) -> Result<Table<'_, 'table_name>, Error> {
        let p = unsafe {
            libdatabase_sys::libdatabase_database_add_table(self.inner, table_name.as_ptr())
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

pub struct Table<'db, 'name> {
    inner: *mut libdatabase_sys::table_t,
    db_ph: PhantomData<&'db mut Database<'db>>,
    name_ph: PhantomData<&'name std::ffi::CStr>,
}

impl<'db, 'name> Table<'db, 'name> {
    pub fn get_row<'table>(
        &'table mut self,
        row_index: std::ffi::c_uint,
    ) -> Result<Row<'table, 'db>, Error> {
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

pub struct Row<'table, 'db> {
    inner: *mut libdatabase_sys::row_t,
    ph: PhantomData<&'table Table<'db, 'table>>,
}

impl<'table, 'db> Row<'table, 'db> {
    pub fn print(&self) {
        unsafe {
            libdatabase_sys::libdatabase_row_print(self.inner);
        }
    }
}

impl<'table, 'db> Drop for Row<'table, 'db> {
    fn drop(&mut self) {
        unsafe {
            libdatabase_sys::libdatabase_row_close(self.inner);
        }
    }
}
