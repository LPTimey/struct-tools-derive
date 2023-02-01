use std::fmt::Display;

use struct_tools_derive::{StructEnum, StructIterTools};

#[derive(StructIterTools, StructEnum)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}
impl Book {
    pub fn test<E>(&self) -> E
    where
        E: From<u64>,
    {
        E::from(self.id)
    }
}
#[derive(Debug)]
pub enum BookEnumTest {
    U64(u64),
    String(String),
}
impl From<u64> for BookEnumTest {
    fn from(value: u64) -> Self {
        BookEnumTest::U64(value)
    }
}
impl From<String> for BookEnumTest {
    fn from(value: String) -> Self {
        BookEnumTest::String(value)
    }
}
impl Display for BookEnumTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[test]
fn fields() {
    let fields = Book::fields();
    println!("fields: {fields:?}");
    assert_eq!(vec!["id", "title", "pages", "author"], fields)
}

#[test]
fn values() {
    let book = Book {
        id: 1,
        title: "Title".to_string(),
        pages: 100,
        author: "me".to_string(),
    };
    let book_values: Vec<BookEnumTest> = book.values::<BookEnumTest>();
    println!("fields: {book_values:?}");
}

#[test]
fn enum_test() {
    let test = String::from("value1").to_uppercase();
    let test = BookEnum::String("()".to_string());
    println!("{test:?}");
}
