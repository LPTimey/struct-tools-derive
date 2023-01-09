use struct_tools_derive::{StructFieldsIter};

#[derive(StructFieldsIter)]
pub struct Book {
    id: u64,
    title: String,
    pages: u64,
    author: String,
}

#[test]
fn fields() {
    let fields = Book::fields();
    println!("fields: {:?}", fields);
    assert_eq!(vec!["id", "title", "pages", "author"], fields)
}
