#![allow(unused)]
use itertools::Itertools;
use std::fmt::Display;
use struct_tools_derive::{
    StructBuilder, StructEnum, StructEnumMut, StructFieldEnum, StructIterTools,
};
pub type Id = u64;

#[derive(
    Debug,
    Default,
    PartialEq,
    StructIterTools,
    StructEnum,
    StructEnumMut,
    StructFieldEnum,
    StructBuilder,
)]
#[StructFields]
#[StructValues]
#[EnumDerive(Debug, Clone)]
#[MutEnumDerive(Debug)]
pub struct Book {
    id: Id,
    title: String,
    #[builder_default(0)]
    pages: u64,
    author: String,
    #[builder_default(None)]
    inspirations: Option<Vec<String>>,
    date_time_: u64,
    tuple: (u8, u8),
}

#[derive(Debug)]
pub enum BookEnumTest {
    U64(u64),
    String(String),
    OptionVecString(Option<Vec<String>>),
    Tuple((u8, u8)),
}

impl From<(u8, u8)> for BookEnumTest {
    fn from(v: (u8, u8)) -> Self {
        Self::Tuple(v)
    }
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

pub struct BookWithLifetime<'a> {
    test: &'a mut String,
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
            "date_time_",
            "tuple"
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
        tuple: (0, 0),
    };
    let expected = vec![
        BookEnumTest::U64(1),
        BookEnumTest::String("Title".to_string()),
        BookEnumTest::U64(100),
        BookEnumTest::String("me".to_string()),
        BookEnumTest::OptionVecString(None),
        BookEnumTest::U64(0),
        BookEnumTest::Tuple((0, 0)),
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
fn enum_try_into_test() {
    let book = Book::default();
    let test = book
        .values::<BookEnum>()
        .into_iter()
        .map(TryInto::<String>::try_into)
        .collect_vec();
    let assert = vec![
        Err(()),
        Ok("".to_owned()),
        Err(()),
        Ok("".to_owned()),
        Err(()),
        Err(()),
        Err(()),
    ];
    assert_eq!(assert, test)
}

#[test]
fn field_enum_test() {
    let test = BookFieldEnum::Id(1);
    let fields_vec = Book::default().get_fields_enums();
    println!("{test:?}");
    println!("{fields_vec:?}");
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
        tuple: (0, 0),
    };
    let expected = vec![
        BookEnum::U64(1),
        BookEnum::String("Title".to_string()),
        BookEnum::U64(100),
        BookEnum::String("me".to_string()),
        BookEnum::OptionVecString(None),
        BookEnum::U64(0),
        BookEnum::u8u8((0, 0)),
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
        tuple: (0, 0),
    };
    let mut builder = BookBuilder::default()
        .set_author("me".to_string())
        .set_date_time_(0)
        .set_id(1)
        .set_inspirations(None)
        .set_pages(100)
        .set_title("Title".to_string())
        .set_tuple((0, 0))
        .build()
        .unwrap();
    assert_eq!(book, builder)
}