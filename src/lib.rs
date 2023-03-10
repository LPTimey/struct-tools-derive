use itertools::Itertools;
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, Attribute, Data::Struct, DataStruct, DeriveInput, Fields::Named,
    FieldsNamed, Ident, Type,
};

#[doc = r#"
Lets you iterate over structs

# Examples

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructIterTools;

#[derive(StructIterTools)]
pub struct Foo{
    field1: i32,
    field2: String,
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
assert_eq!(fields,vec![String::from("field1"), String::from("field2")])
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

#[derive(Debug, Eq, PartialEq)]
enum FooEnum {
  I32(i32),
  String(String),
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

let instance = Foo::default();

let values = instance.values::<FooEnum>();

assert_eq!(values,vec![FooEnum::I32(0), FooEnum::String(String::new())])
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
}

#[derive(Debug, Eq, PartialEq)]
enum FooEnum {
  I32(i32),
  String(String),
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

let instance = Foo::default();

let f_v = instance.fields_and_values::<FooEnum>();

assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new()))])
```"#]
#[proc_macro_derive(StructIterTools, attributes(StructFields, StructValues))]
pub fn derive_struct_iter_tools(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let attrs: Vec<String> = attrs
        .iter()
        .map(|attr| attr.path.get_ident())
        .flatten()
        .map(|attr| attr.to_string())
        .collect();

    let derive_fields: bool = attrs.contains(&"StructFields".to_string());
    let derive_values: bool = attrs.contains(&"StructValues".to_string());

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

    let fields_quote = match derive_fields {
        true => Some(quote! {
            impl #ident {
                pub fn fields() -> ::std::vec::Vec<::std::string::String>{
                    vec![#(#fields_vec.to_string()),*]
                }
            }
        }),
        false => None,
    };
    let values_quote = match derive_values {
        true => Some(quote! {
            impl #ident{
                pub fn values<E>(&self) -> ::std::vec::Vec<E>
                where
                E: #types
                {
                    vec![#(E::from(self.#field_ids.clone())),*]
                }
            }
        }),
        false => None,
    };
    let fields_and_values_quote = match derive_fields == true && derive_values == true {
        true => Some(quote! {
            impl #ident{
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
        }),
        false => None,
    };
    let result = quote! {
        #fields_quote

        #values_quote

        #fields_and_values_quote
    };
    result.into()
}

/**

*/
#[proc_macro_derive(StructBuilder, attributes(StructFields, BuilderDerive, Default))]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input as DeriveInput);

    // TODO: Make atributes for adding default values to specific fields

    let new_ident = Ident::new(&(ident.to_string() + "Builder"), ident.span());
    let error = Ident::new(&(new_ident.to_string() + "Error"), new_ident.span());

    let attr_strings: Vec<String> = attrs
        .iter()
        .map(|attr| attr.path.get_ident())
        .flatten()
        .map(|attr| attr.to_string())
        .collect();
    let get_fields = attr_strings.contains(&"StructFields".to_owned());
    let get_derives = attr_strings.contains(&"BuilderDerive".to_owned());

    let derives = if get_derives {
        let iter = attrs
            .iter()
            .filter(|attr| attr.path.get_ident().unwrap().to_string() == "BuilderDerive")
            .collect_vec();
        Some(quote!(#[derive #(#iter),*]))
    } else {
        None
    };

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };

    let field_types = fields.iter().cloned().map(|field| field.ty).collect_vec();
    let field_names = fields
        .iter()
        .cloned()
        .flat_map(|field| field.ident)
        .collect_vec();

    let set = field_names
        .iter()
        .cloned()
        .map(|ident| Ident::new(&("set_".to_owned() + &ident.to_string()), ident.span()))
        .collect_vec();

    let result = match get_fields {
        true => quote! {

            #[allow(non_camel_case_types)]
            #[derive(Debug)]
            pub enum #error {

                #(#field_names),*

            }
            impl ::std::fmt::Display for #error {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl ::std::error::Error for #error {}

            #derives
            #[derive(Default)]
            pub struct #new_ident{
                #(#field_names : Option< #field_types >),*
            }
            impl #new_ident{
                pub fn build(self) -> Result< #ident , ::std::vec::Vec< #error > > {

                    let mut errors: ::std::vec::Vec< #error > = ::std::vec::Vec::new();

                    #(match self. #field_names{
                        Some(_) => (),
                        None => errors.push( #error :: #field_names),
                    };)*

                    match errors.is_empty(){
                        true => Ok(
                                    #ident {
                                        #(#field_names: self. #field_names .unwrap()),*
                                    }
                                ),
                        false => Err(errors)
                    }
                }
                #(pub fn #set (mut self, #field_names: #field_types) -> #new_ident {
                    self. #field_names = Some( #field_names );
                    self
                })*
            }
        },
        false => panic!("Attribute: \"StructFields\" is not set"),
    };
    //println!("{result}");
    result.into()
}

#[doc = r#"
Will create an Enum which is capable of containing all possible contents of the struct

# Example

```rust
use struct_tools_derive::StructEnum;

#[derive(Debug, StructEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
}
```

This Grants you access to an automatically generated Enum with the name `{structname}Enum`.

its Variants are named by Capitalizing the first letter of the respective Type

```rust
pub enum FooEnum{
    I32(i32),
    String(String),
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

```

This means, that it can be used with both values and fields and values from StructIterTools

you can also have it derive traits by adding them to the `EnumDerives` attribute like this:

```rust
# use struct_tools_derive::{StructEnum, StructIterTools};
#[derive(StructEnum)]
#[EnumDerive(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
}
```"#]
#[proc_macro_derive(StructEnum, attributes(EnumDerive))]
pub fn derive_struct_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    //println!("{attrs:?}\n");

    let attr: Vec<Attribute> = attrs
        .clone()
        .into_iter()
        .filter(|attr| attr.path.get_ident().unwrap().to_string() == "EnumDerive")
        .collect();
    //println!("{attr:?}\n");

    let ident = Ident::new(&(ident.to_string() + "Enum"), ident.span());

    let derives = match attr.is_empty() {
        false => Some(attr.into_iter().map(|attr| attr.tokens)),
        true => None,
    };
    let derives = match derives {
        Some(iter) => Some(quote! {#[derive #(#iter),*]}),
        None => None,
    };

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
                    //} else if vec!['<','>','(',')','[',']'].contains(&chr){
                    //    String::new()
                    } else if !chr.is_ascii_alphabetic() && !chr.is_ascii_alphanumeric() {
                        String::new()
                    } else {
                        chr.to_string()
                    }
                })
                .collect::<String>();
            let string = string.replace(" ", "");
            Ident::new(&string, Span::call_site().into())
        })
        .collect::<Vec<Ident>>();
    let from_fields = enum_fields.clone();
    let from_types = field_types.clone();

    let result = quote! {
        #derives
        pub enum #ident {
            #(#enum_fields (#field_types)),*
        }

        #(impl From<#from_types> for #ident{
            fn from(value: #field_types) -> Self {
                #ident :: #from_fields (value.into())
            }
        })*
    };
    //println!("{result}");
    result.into()
}

#[doc = r#"
Will create an Enum which is capable of containing all possible contents of the struct

# Example

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructFieldEnum;

#[derive(StructFieldEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
}
```

This Grants you access to an automatically generated Enum with the name `{structname}FieldEnum`.

its Variants are named by CapitalCamelCase-ing the fields of the struct

```rust
pub enum FooFieldEnum{
    Field1(i32),
    Field2(String),
}
```

you can also have it derive traits by adding them to the `EnumDerives` attribute like this:

```rust
# use struct_tools_derive::{StructFieldEnum, StructIterTools};
#[derive(StructFieldEnum)]
#[EnumDerive(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
}
```
"#]
#[proc_macro_derive(StructFieldEnum, attributes(EnumDerive))]
pub fn derive_struct_field_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    let ident = Ident::new(&(ident.to_string() + "FieldEnum"), ident.span());

    let attr: Vec<Attribute> = attrs
        .clone()
        .into_iter()
        .filter(|attr| attr.path.get_ident().unwrap().to_string() == "EnumDerive")
        .collect();
    let derives = match attr.is_empty() {
        false => Some(attr.into_iter().map(|attr| attr.tokens)),
        true => None,
    };
    let derives = match derives {
        Some(iter) => Some(quote! {#[derive #(#iter),*]}),
        None => None,
    };

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };

    let fields_vec: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
        .collect::<Vec<String>>();

    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<&Type>>();

    let variants: Vec<String> = fields_vec
        .iter()
        .cloned()
        .map(|field| {
            field
                .chars()
                .enumerate()
                .map(|(i, chr)| match i {
                    0 => chr.to_uppercase().to_string(),
                    _ => chr.to_string(),
                })
                .collect::<String>()
        })
        .map(|mut field| {
            let i = field.find("_");
            let mut field = match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    match field.get(i) {
                        Some(_) => {
                            field[i] = field[i].to_uppercase().to_string();
                        }
                        None => (),
                    }
                    field.join("")
                }
                None => field,
            };
            let i = field.find("_");
            match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    match field.get(i) {
                        Some(_) => {
                            field[i] = field[i].to_uppercase().to_string();
                        }
                        None => (),
                    }
                    field.join("")
                }
                None => field,
            }
        })
        .collect_vec();
    let variants = variants.into_iter().map(|variant| {
        let variant = Ident::new(&variant, Span::call_site().into());
        quote! {#variant}
    });
    let result = quote! {
        #derives
        pub enum #ident{
            #(#variants (#field_types)),*
        }
    };
    //println!("{result}");
    result.into()
}
