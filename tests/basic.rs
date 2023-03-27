#![allow(unused)]
use std::fmt::Display;
use struct_tools_derive::{StructBuilder, StructEnum, StructFieldEnum, StructIterTools};

#[derive(Debug, PartialEq, StructIterTools, StructEnum, StructFieldEnum, StructBuilder)]
#[StructFields]
#[StructValues]
#[EnumDerive(Debug, Clone)]
pub struct Book {
    id: u64,
    title: String,
    #[default(0)]
    pages: u64,
    author: String,
    #[default(None)]
    inspirations: Option<Vec<String>>,
    date_time_: u64,
}
impl Default for Book {
    fn default() -> Self {
        Self {
            id: Default::default(),
            title: Default::default(),
            pages: Default::default(),
            author: Default::default(),
            inspirations: Default::default(),
            date_time_: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum BookEnumTest {
    U64(u64),
    String(String),
    OptionVecString(Option<Vec<String>>),
}
impl BookEnumTest {
    pub fn gen_id(value: u64) -> Self {
        BookEnumTest::U64(value)
    }
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
impl From<Option<Vec<String>>> for BookEnumTest {
    fn from(value: Option<Vec<String>>) -> Self {
        BookEnumTest::OptionVecString(value)
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
    assert_eq!(
        vec![
            "id",
            "title",
            "pages",
            "author",
            "inspirations",
            "date_time_"
        ],
        fields
    )
}

#[test]
fn values() {
    let book = Book {
        id: 1,
        title: "Title".to_string(),
        pages: 100,
        author: "me".to_string(),
        inspirations: None,
        date_time_: 0,
    };
    let expected = vec![
        BookEnumTest::U64(1),
        BookEnumTest::String("Title".to_string()),
        BookEnumTest::U64(100),
        BookEnumTest::String("me".to_string()),
        BookEnumTest::OptionVecString(None),
        BookEnumTest::U64(0),
    ];

    let book_values: Vec<BookEnumTest> = book.values::<BookEnumTest>();
    println!("fields: {book_values:?}");
    let success = book_values
        .into_iter()
        .enumerate()
        .filter_map(|(i, value)| {
            let expect = expected.get(i).unwrap();
            match matches!(&value, expect) {
                true => None,
                false => Some(value),
            }
        })
        .collect::<Vec<BookEnumTest>>()
        .is_empty();

    assert!(success)
}

#[test]
fn fields_and_values_test() {
    let instance = Book::default();

    let f_v = instance.fields_and_values::<BookEnum>();
}

#[test]
fn enum_test() {
    let test = BookEnum::String("()".to_string());
    println!("{test:?}");
}

#[test]
fn field_enum_test() {
    let test = BookFieldEnum::Id(1);
    println!("{test:?}")
}
#[test]
fn enum_values_test() {
    let book = Book {
        id: 1,
        title: "Title".to_string(),
        pages: 100,
        author: "me".to_string(),
        inspirations: None,
        date_time_: 0,
    };
    let expected = vec![
        BookEnum::U64(1),
        BookEnum::String("Title".to_string()),
        BookEnum::U64(100),
        BookEnum::String("me".to_string()),
        BookEnum::OptionVecString(None),
        BookEnum::U64(0),
    ];
    let book_values: Vec<BookEnum> = book.values::<BookEnum>();
    //println!("fields: {book_values:?}");
    let success = book_values
        .into_iter()
        .enumerate()
        .filter_map(|(i, value)| {
            let expect = expected.get(i).unwrap();
            match matches!(&value, expect) {
                true => None,
                false => Some(value),
            }
        })
        .collect::<Vec<BookEnum>>()
        .is_empty();

    assert!(success)
}

#[test]
fn builder_test() {
    let book = Book {
        id: 1,
        title: "Title".to_string(),
        pages: 100,
        author: "me".to_string(),
        inspirations: None,
        date_time_: 0,
    };
    let mut builder = BookBuilder::default()
        .set_author("me".to_string())
        .set_date_time_(0)
        .set_id(1)
        .set_inspirations(None)
        .set_pages(100)
        .set_title("Title".to_string())
        .build()
        .unwrap();
    assert_eq!(book, builder)
}
