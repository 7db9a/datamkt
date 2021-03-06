use core::borrow::Borrow;
use eosio::{AccountName, ReadError, ScopeName, Table, WriteError};

pub enum Payer {
    Same,
    New(AccountName),
}

/// Table Cursor
pub trait TableCursor<T>: IntoIterator
where
    T: Table,
{
    /// Read and deserialize the current table row
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue reading the stored value.
    fn get(&self) -> Result<T::Row, ReadError>;

    /// Erase the current row
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue reading the stored value. Stored
    /// values must be read in order to erase secondary indexes.
    fn erase(&self) -> Result<T::Row, ReadError>;

    /// Modify the current row
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue serializing the value.
    fn modify<I: Borrow<T::Row>>(
        &self,
        payer: Payer,
        item: I,
    ) -> Result<usize, WriteError>;
}

/// Table index
pub trait TableIndex<'a, K, T>
where
    T: Table + 'a,
{
    /// The kind of cursor this table index uses
    type Cursor: TableCursor<T> + 'a;
    /// Returns the account name of the smart contract
    fn code(&'a self) -> AccountName;
    /// Returns the table scope
    fn scope(&'a self) -> ScopeName;
    /// Returns a cursor pointing to the first row that matches a key
    fn lower_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;
    /// Returns a cursor pointing to the last row that matches a key
    fn upper_bound<N: Into<K>>(&'a self, key: N) -> Option<Self::Cursor>;

    /// Inserts a new row into the table
    ///
    /// # Errors
    ///
    /// Will return `Err` if there was an issue serializing the value.
    fn emplace<I: Borrow<T::Row>>(
        &'a self,
        payer: AccountName,
        item: I,
    ) -> Result<(), WriteError>;
}

/// Table iterator
pub trait TableIterator: DoubleEndedIterator {}
