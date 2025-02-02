use crate::ast::{DatabaseValue, Select};
use std::borrow::Cow;

/// An object that can be aliased.
pub trait Aliasable<'a> {
    /// Alias table for usage elsewhere in the query.
    fn alias<T>(self, alias: T) -> Table<'a>
    where
        T: Into<Cow<'a, str>>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableType<'a> {
    Table(Cow<'a, str>),
    Query(Select<'a>),
}

/// A table definition
#[derive(Clone, Debug, PartialEq)]
pub struct Table<'a> {
    pub typ: TableType<'a>,
    pub alias: Option<Cow<'a, str>>,
    pub database: Option<Cow<'a, str>>,
}

impl<'a> Table<'a> {
    /// Define in which database the table is located
    pub fn database<T>(mut self, database: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.database = Some(database.into());
        self
    }

    /// A qualified asterisk to this table
    #[inline]
    pub fn asterisk(self) -> DatabaseValue<'a> {
        DatabaseValue::Asterisk(Some(Box::new(self)))
    }
}

impl<'a> From<&'a str> for Table<'a> {
    #[inline]
    fn from(s: &'a str) -> Table<'a> {
        Table {
            typ: TableType::Table(s.into()),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<(&'a str, &'a str)> for Table<'a> {
    #[inline]
    fn from(s: (&'a str, &'a str)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<String> for Table<'a> {
    #[inline]
    fn from(s: String) -> Self {
        Table {
            typ: TableType::Table(s.into()),
            alias: None,
            database: None,
        }
    }
}

impl<'a> From<(String, String)> for Table<'a> {
    #[inline]
    fn from(s: (String, String)) -> Table<'a> {
        let table: Table<'a> = s.1.into();
        table.database(s.0)
    }
}

impl<'a> From<Select<'a>> for Table<'a> {
    #[inline]
    fn from(select: Select<'a>) -> Self {
        Table {
            typ: TableType::Query(select),
            alias: None,
            database: None,
        }
    }
}

impl<'a> Aliasable<'a> for Table<'a> {
    fn alias<T>(mut self, alias: T) -> Table<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        self.alias = Some(alias.into());
        self
    }
}

macro_rules! aliasable {
    ($($kind:ty),*) => (
        $(
            impl<'a> Aliasable<'a> for $kind {
                #[inline]
                fn alias<T>(self, alias: T) -> Table<'a>
                where
                    T: Into<Cow<'a, str>>,
                {
                    let table: Table = self.into();
                    table.alias(alias)
                }
            }
        )*
    );
}

aliasable!(String, (String, String));
aliasable!(&'a str, (&'a str, &'a str));
