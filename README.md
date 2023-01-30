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

```rust
use struct_tools_derive::StructIterTools;

enum FooEnum {
  I32(i32),
  String(String),
  {...}
}
impl From<i32> for FooEnum {
    fn from(value: 32) -> Self {
        FooEnum::I32(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}

let instance = Foo::default();

let values = instance.values::<FooEnum>();

assert_eq!(values,vec![FooEnum::I32(0), FooEnum::String(String::new()),...])
```

```rust
use struct_tools_derive::StructIterTools;

enum FooEnum {
  I32(i32),
  String(String),
  {...}
}
impl From<i32> for FooEnum {
    fn from(value: 32) -> Self {
        FooEnum::I32(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}

let instance = Foo::default();

let f_v = instance.fields_and_values::<FooEnum>();

assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new())),...])
```
