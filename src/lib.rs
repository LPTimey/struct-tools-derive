use itertools::Itertools;
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed,
    Ident, Type,
};

#[proc_macro_derive(StructIterTools)]
#[doc = r#"If you have a struct

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

let fields = Foo::fields();
assert_eq!(fields,vec![String::from("field1"), String::from("field2"),...])
```

#### values

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

#### fields and values

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
```"#]
pub fn derive_struct_iter_tools(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };

    let field_ids = fields.iter().filter_map(|field| match &field.ident {
        Some(id) => Some(id),
        None => None,
    });

    let field_types = fields.iter().map(|field| &field.ty).unique();
    let types = quote!(#(From<#field_types>)+*);

    let fields_vec: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
        .collect::<Vec<String>>();

    let result = quote! {
        impl #ident {
            /**
```rust
use struct_tools_derive::StructIterTools;
!#[derive(StructIterTools, Default)]
!pub struct Foo{
!    field1: i32,
!    field2: String,
!}
let fields = Foo::fields();
assert_eq!(fields,vec![String::from("field1"), String::from("field2")])
```
            */
            pub fn fields() -> ::std::vec::Vec<::std::string::String>{
                vec![#(#fields_vec.to_string()),*]
            }
        }
        impl #ident{
            /**

```rust
use struct_tools_derive::StructIterTools;

!#[derive(StructIterTools, Default)]
!pub struct Foo{
!    field1: i32,
!    field2: String,
!}

enum FooEnum {
  I32(i32),
  String(String),
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

assert_eq!(values,vec![FooEnum::I32(0), FooEnum::String(String::new())])
```

            */
            pub fn values<E>(&self) -> ::std::vec::Vec<E>
            where
            E: #types
            {
                vec![#(E::from(self.#field_ids.clone())),*]
            }
        }
        impl #ident{
            /**
```rust
use struct_tools_derive::StructIterTools;

!#[derive(StructIterTools, Default)]
!pub struct Foo{
!    field1: i32,
!    field2: String,
!}

enum FooEnum {
  I32(i32),
  String(String)
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

assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new()))])
```
            */
            pub fn fields_and_values<E>(&self) -> ::std::vec::Vec<(::std::string::String, E)>
            where
            E: #types
            {
                let fields = Self::fields();
                let values = self.values();
                let erg = fields.into_iter().zip(values).collect();
                erg
            }
        }
    };
    result.into()
}

#[proc_macro_derive(StructEnum, attributes(EnumDerives))]
#[doc = r"
Will create an Enum which is capable of containing all possible contents of the struct

# Example

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
impl From<u64> for FooEnum {
    fn from(value: u64) -> Self {
        FooEnum::U64(value)
    }
}
impl From<String> for FooEnum {
    fn from(value: String) -> Self {
        FooEnum::String(value)
    }
}
{...}
```

This means, that it can be used with both values and fields and values from StructIterTools

you can also have it derive traits by adding them to the `EnumDerives` attribute like this:

```rust
#[derive(StructEnum)]
#[EnumDerives(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
    {...}
}
```"]
pub fn derive_struct_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    let ident = Ident::new(&(ident.to_string() + "Enum"), ident.span());

    let derives = attrs
        .iter()
        .filter(|attr| {
            match attr.style {
                syn::AttrStyle::Outer => (),
                syn::AttrStyle::Inner(_) => {
                    return false;
                }
            };
            match attr.path.get_ident() {
                Some(ident) => {
                    if ident != &Ident::new("EnumDerives", ident.span()) {
                        return false;
                    }
                    true
                },
                _ => false,
            }
        })
        .map(|attr| attr.tokens.clone());

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };

    let field_types = fields
        .iter()
        .map(|field| &field.ty)
        .unique()
        .collect::<Vec<&Type>>();
    let enum_fields = field_types
        .iter()
        .cloned()
        .map(|typ| {
            let string = typ
                .to_token_stream()
                .to_string()
                .char_indices()
                .map(|(i, chr)| {
                    if i == 0 {
                        chr.to_uppercase().to_string()
                    } else {
                        chr.to_string()
                    }
                })
                .collect::<String>();
            Ident::new(&string, Span::call_site().into())
        })
        .collect::<Vec<Ident>>();
    let from_fields = enum_fields.clone();
    let from_types = field_types.clone();

    let result = quote! {
        #[derive #(#derives),*]
        pub enum #ident {
            #(#enum_fields (#field_types)),*
        }

        #(impl From<#from_types> for #ident{
            fn from(value: #field_types) -> Self {
                #ident :: #from_fields (value.into())
            }
        })*
    };
    result.into()
}
