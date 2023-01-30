# struct-tools-derive

Tools for working with structs.

currently contains:

- StructIterTools
  - `fields` ( ) -> Vec<`String`>: A Function that returns the names of its fields
  - `values`<`E`> (`&self`) -> Vec<`E`>: A Method that returns the values of its Instance
  - `fields_and_values`<`E`> (`&self`) -> Vec<`(String, E)`>: A Method that returns a Touple of the field and the values of its Instance

## How to use

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructIterTools;

#[derive(StructIterTools)]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```

This now let's you use it like this:

```rust
use struct_tools_derive::StructIterTools;

let fields = Foo::fields();
assert_eq!(fields,vec![String::from("field1"), String::from("field2"),...])
```
