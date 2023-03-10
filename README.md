# struct-tools-derive

Table of Contents

- [struct-tools-derive](#struct-tools-derive)
  - [Contains](#contains)
  - [How to use](#how-to-use)
    - [StructIterTools](#structitertools)
      - [fields](#fields)
      - [values](#values)
      - [fields and values](#fields-and-values)
    - [StructEnum](#structenum)
    - [StructFieldEnum](#structfieldenum)
    - [StructBuilder](#structbuilder)

## Contains

Tools for working with structs.

currently contains:

- StructIterTools
  - `fields` ( ) -> Vec<`String`>: A Function that returns the names of its fields
  - `values`<`E`> (`&self`) -> Vec<`E`>: A Method that returns the values of its Instance
  - `fields_and_values`<`E`> (`&self`) -> Vec<`(String, E)`>: A Method that returns a Vector of Touples of the field and the values of its Instance
- StructEnum
- StructFieldEnum
- StructBuilder

## How to use

### StructIterTools

Lets you iterate over structs

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

#### fields

```rust
use struct_tools_derive::StructIterTools;

#[derive(StructIterTools)]
#[StructFields]
pub struct Foo{
    field1: i32,
    field2: String,
}

let fields = Foo::fields();
assert_eq!(fields,vec![String::from("field1"), String::from("field2"),...])
```

#### values

```rust
use struct_tools_derive::StructIterTools;

// Default just for demonstration
#[derive(StructIterTools, Default)]
#[StructValues]
pub struct Foo{
    field1: i32,
    field2: String,
}

enum FooEnum {
  I32(i32),
  String(String),
  {...}
}
impl From<i32> for FooEnum {
    fn from(value: i32) -> Self {
        FooEnum::I32(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}
{...}

let instance = Foo::default();

let values = instance.values::<FooEnum>();

assert_eq!(values,vec![FooEnum::I32(0), FooEnum::String(String::new()),...])
```

#### fields and values

```rust
use struct_tools_derive::StructIterTools;

// Default just for demonstration
#[derive(StructIterTools, Default)]
#[StructFields]
#[StructValues]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}

enum FooEnum {
  I32(i32),
  String(String),
  {...}
}
impl From<i32> for FooEnum {
    fn from(value: i32) -> Self {
        FooEnum::I32(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}
{...}

let instance = Foo::default();

let f_v = instance.fields_and_values::<FooEnum>();

assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new())),...])
```

---

### StructEnum

Will create an Enum which is capable of containing all possible contents of the struct

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
use struct_tools_derive::StructEnum;

#[derive(StructEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```

This Grants you access to an automatically generated Enum with the name `{structname}Enum`.

its Variants are named by Capitalizing the first letter of the respective Type

```rust
pub enum FooEnum{
    I32(i32),
    String(String),
    {...}
}
impl From<i32> for FooEnum {
    fn from(value: i32) -> Self {
        FooEnum::I32(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}
{...}
```

This means, that it can be used with both [values](#values) and [fields and values](#fields-and-values)

you can also have it derive traits by adding them to the `EnumDerives` attribute like this:

```rust
#[derive(StructEnum)]
#[EnumDerive(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```

---

### StructFieldEnum

Will create an Enum which is capable of containing all possible contents of the struct

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
use struct_tools_derive::StructFieldEnum;

#[derive(StructFieldEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```

This Grants you access to an automatically generated Enum with the name `{structname}FieldEnum`.

its Variants are named by CapitalCamelCase-ing the fields of the struct

```rust
pub enum FooFieldEnum{
    Field1(i32),
    Field2(String),
    {...}
}
```

you can also have it derive traits by adding them to the `EnumDerives` attribute like this:

```rust
#[derive(StructFieldEnum)]
#[EnumDerive(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
}
```

---

### StructBuilder

TODO!
