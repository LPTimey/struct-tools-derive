use itertools::Itertools;
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, Attribute, Data::Struct, DataStruct, DeriveInput, Fields::Named,
    FieldsNamed, Ident, Type,
};

/**
Lets you iterate over structs

# Examples

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructIterTools;

#[derive(StructIterTools)]
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
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
    //{...}
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
    //{...}
}

#[derive(Debug, Eq, PartialEq)]
enum FooEnum {
  I32(i32),
  String(String),
    //{...}
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
//{...}

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
    //{...}
}

#[derive(Debug, Eq, PartialEq)]
enum FooEnum {
    I32(i32),
    String(String),
    //{...}
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
//{...}

let instance = Foo::default();

let f_v = instance.fields_and_values::<FooEnum>();

assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new()))])
*/
#[proc_macro_derive(StructIterTools, attributes(StructFields, StructValues))]
pub fn derive_struct_iter_tools(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let attrs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| attr.path().get_ident())
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

    let field_ids = fields.iter().filter_map(|field| field.ident.clone());

    let field_types = fields.iter().map(|field| &field.ty).unique();
    let types = quote!(#(From<#field_types>)+*);

    let fields_vec: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
        .collect::<Vec<String>>();

    let fields_quote = match derive_fields {
        true => Some(quote! {
            impl #ident {
                /**

                returns the names of the Structs fields

                ```rust
                use struct_tools_derive::StructIterTools;

                #[derive(StructIterTools)]
                #[StructFields]
                pub struct Foo{
                    field1: i32,
                    field2: String,
                    //{...}
                }

                let fields = Foo::fields();
                assert_eq!(fields,vec![String::from("field1"), String::from("field2")/*,{...}*/])
                ```
                 */
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
                /**

                returns the values of this Instance

                ```rust
                use struct_tools_derive::StructIterTools;

                // Default just for demonstration
                #[derive(StructIterTools, Default)]
                #[StructValues]
                pub struct Foo{
                    field1: i32,
                    field2: String,
                    //{...}
                }

                #[derive(Debug, PartialEq)] // only needed for example
                enum FooEnum {
                I32(i32),
                String(String),
                //{...}
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
                //{...}

                let instance = Foo::default();

                let values = instance.values::<FooEnum>();

                assert_eq!(values,vec![FooEnum::I32(0), FooEnum::String(String::new())/*,{...}*/])
                ```
                 */
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
    let fields_and_values_quote = match derive_fields && derive_values {
        true => Some(quote! {
            impl #ident{
                /**

                returns a Vector of Tuples of the field and the values of this Instance

                ```rust
                use struct_tools_derive::StructIterTools;

                // Default just for demonstration
                #[derive(StructIterTools, Default)]
                #[StructFields]
                #[StructValues]
                pub struct Foo{
                    field1: i32,
                    field2: String,
                    //{...}
                }

                #[derive(Debug, PartialEq)] // only needed for example
                enum FooEnum {
                    I32(i32),
                    String(String),
                    //{...}
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
                //{...}

                let instance = Foo::default();

                let f_v = instance.fields_and_values::<FooEnum>();

                assert_eq!(f_v,vec![(String::from("field1"), FooEnum::I32(0)), (String::from("field2"), FooEnum::String(String::new()))/*{,...}*/])
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
        }),
        false => None,
    };
    let result = quote! {
        #fields_quote

        #values_quote

        #fields_and_values_quote
    };
    //println!("{}",result);
    result.into()
}

/**
Will create a BuilderPattern Struct.

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructBuilder;

#[derive(StructBuilder)]
#[StructFields]
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

This Grants you access to an automatically generated struct with the name `{structname}Builder`.

```rust

# pub struct Foo{
#     field1: i32,
#     field2: String,
#     //{...}
# }

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum FooBuilderError {
    field1,
    field2,
    //{...}
}
impl std::fmt::Display for FooBuilderError {
#     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
#         write!(f, "{:?}", self)
#     }
    //{...}
}
impl std::error::Error for FooBuilderError {}

pub struct FooBuilder{
    field1: Option<i32>,
    field2: Option<String>,
    //{...}
}
impl Default for FooBuilder{
    fn default() -> Self {
        /*{...}*/
#         todo!()
    }
}
impl FooBuilder {
    pub fn build(self) -> Result<Foo, Vec<FooBuilderError>>{
        //{...}
        # todo!()
    }
    pub fn set_field1(mut self, val:i32) -> Self{
        self.field1 = Some(val);
        self
    }
    pub fn set_field2(mut self, val:String) -> Self{
        self.field2 = Some(val);
        self
    }
    //{...}
}
```

If you want specific fields to have specific Default-values you can add the default-Attribute to it like this:

```rust
use struct_tools_derive::{StructFieldEnum, StructBuilder};

#[derive(StructFieldEnum, StructBuilder)]
#[StructFields]
pub struct Foo{
    #[builder_default(1)]
    field1: i32,
    #[builder_default("Hello".to_owned())]
    field2: String,
    //{...}
}
```

TODO!
*/
#[proc_macro_derive(
    StructBuilder,
    attributes(StructFields, BuilderDerive, builder_default)
)]
pub fn derive_struct_builder(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let new_ident = Ident::new(&(ident.to_string() + "Builder"), ident.span());
    let error = Ident::new(&(new_ident.to_string() + "Error"), new_ident.span());

    let attr_strings: Vec<String> = attrs
        .iter()
        .filter_map(|attr| attr.path().get_ident())
        .map(|attr| attr.to_string())
        .collect();
    //println!("{:#?}; {:#?}",attr_strings, attrs);

    let get_fields = attr_strings.contains(&"StructFields".to_owned());
    let get_derives = attr_strings.contains(&"BuilderDerive".to_owned());

    let derives = match get_derives {
        true => {
            let iter = attrs
                .iter()
                .filter(|attr| attr.path().is_ident("BuilderDerive"))
                .collect_vec();
            Some(quote!(#[derive #(#iter),*]))
        }
        false => None,
    };

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };
    //println!("{:#?}",fields);

    let field_defaults = fields
        .iter()
        .cloned()
        .map(|field| {
            let defaults = field
                .attrs
                .iter()
                .cloned()
                .filter(|attr| attr.path().is_ident("builder_default"))
                .flat_map(|attr| attr.parse_args::<proc_macro2::TokenStream>())
                .collect_vec();
            (field.ident.unwrap(), defaults)
        })
        .map(|(field, attrs)| match attrs.is_empty() {
            true => quote!(#field: None),
            false => {
                let default = attrs[0].clone();
                quote!(#field: Some(#default))
            }
        })
        .collect_vec();
    //println!("{:?}", field_defaults);

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
            pub struct #new_ident{
                #(#field_names : Option< #field_types >),*
            }
            impl Default for #new_ident{
                fn default() -> Self {
                    Self {
                        #( #field_defaults ),*
                    }
                }
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

/**
Will create an Enum which is capable of containing all possible contents of the struct

# Example

```rust
use struct_tools_derive::StructEnum;

#[derive(Debug, StructEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

This Grants you access to an automatically generated Enum with the name `{structname}Enum`.

its Variants are named by Capitalizing the first letter of the respective Type

```rust
pub enum FooEnum{
    I32(i32),
    String(String),
    //{...}
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
//{...}

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
    //{...}
}
*/
#[proc_macro_derive(StructEnum, attributes(EnumDerive))]
pub fn derive_struct_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    //println!("{attrs:?}\n");

    let attr: Vec<Attribute> = attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("EnumDerive"))
        .collect();
    //println!("{attr:?}\n");

    let old_ident = ident.clone();

    let ident = Ident::new(&(ident.to_string() + "Enum"), ident.span());

    let derives = match attr.is_empty() {
        false => Some(
            attr.into_iter()
                .flat_map(|attr| attr.parse_args::<proc_macro2::TokenStream>()),
        ),
        true => None,
    };
    let derives = derives.map(|iter| quote! {#[derive (#(#iter),*)]});

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

    let struct_fields = fields.iter().flat_map(|field| &field.ident).collect_vec();
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
            let string = string.replace(' ', "");
            let string = string.replace('(', "");
            let string = string.replace(')', "");
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

        impl #old_ident{
            pub fn gets_enums(&self) -> Vec<#ident> {
                let mut result = Vec::new();
                #(result.push(#ident::from(self.#struct_fields.clone()));)*
                result
            }
        }

        #(impl From<#from_types> for #ident{
            fn from(value: #field_types) -> Self {
                #ident :: #from_fields (value)
            }
        })*

        #(impl TryInto<#from_types> for #ident{
            type Error=();

            fn try_into(self) -> Result<#from_types, Self::Error> {
                if let Self::#enum_fields (val) = self{
                    return Ok(val);
                }
                else{return Err(());}
            }
        })*
    };
    //println!("{result}");
    result.into()
}

/**
you can also have it derive traits by adding them to the `MutEnumDerives` attribute like this:

```rust
# use struct_tools_derive::{StructEnumMut};
#[derive(StructEnumMut)]
#[MutEnumDerive(Debug)]
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

TODO!

*/
#[proc_macro_derive(StructEnumMut, attributes(MutEnumDerive))]
pub fn derive_struct_enum_mut(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    //println!("{attrs:?}\n");

    let attr: Vec<Attribute> = attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("MutEnumDerive"))
        .collect();
    //println!("{attr:?}\n");

    let old_ident = ident.clone();

    let ident = Ident::new(&(ident.to_string() + "EnumMut"), ident.span());

    let derives = match attr.is_empty() {
        false => Some(
            attr.into_iter()
                .flat_map(|attr| attr.parse_args::<proc_macro2::TokenStream>()),
        ),
        true => None,
    };
    let derives = derives.map(|iter| quote! {#[derive (#(#iter),*)]});

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
    let struct_fields = fields.iter().flat_map(|field| &field.ident).collect_vec();
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
            let string = string.replace(' ', "");
            let string = string.replace('(', "");
            let string = string.replace(')', "");
            Ident::new(&string, Span::call_site().into())
        })
        .collect::<Vec<Ident>>();
    let from_fields = enum_fields.clone();

    let result = quote! {
        #derives
        pub enum #ident<'a> {
            #(#enum_fields (&'a mut #field_types)),*
        }

        impl #old_ident{
            pub fn gets_enums_mut(&mut self) -> Vec<#ident> {
                let mut result = Vec::new();
                #(result.push(#ident::from(&mut self.#struct_fields));)*
                result
            }
        }

        #(impl<'a> From<&'a mut #field_types> for #ident<'a>{
            fn from(value: &'a mut #field_types) -> Self {
                #ident :: #from_fields (value)
            }
        })*

        #(impl<'a> TryInto<&'a mut #field_types> for #ident<'a>{
            type Error=();

            fn try_into(self) -> Result<&'a mut #field_types, Self::Error> {
                if let Self::#enum_fields (val) = self{
                    return Ok(val);
                } else {
                return Err(());
            }
            }
        })*

        #(impl<'a> TryInto<&'a mut #field_types> for &'a mut #ident<'a>{
            type Error=();

            fn try_into(self) -> Result<&'a mut #field_types, Self::Error> {
                match self {
                    #ident ::#enum_fields(val) => Ok(*val),
                    _ => Err(()),
                }
            }
        })*

    };
    println!("{result}");
    result.into()
}

/**
Will create an Enum which is capable of containing all possible contents of the struct

# Example

If you have a struct

```rust
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

you can just add the derive to it

```rust
use struct_tools_derive::StructFieldEnum;

#[derive(StructFieldEnum)]
pub struct Foo{
    field1: i32,
    field2: String,
    //{...}
}
```

This Grants you access to an automatically generated Enum with the name `{structname}FieldEnum`.

its Variants are named by CapitalCamelCase-ing the fields of the struct

```rust
pub enum FooFieldEnum{
    Field1(i32),
    Field2(String),
    //{...}
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
    //{...}
}
```
*/
#[proc_macro_derive(StructFieldEnum, attributes(EnumDerive, StructFields, StructValues))]
pub fn derive_struct_field_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    let new_ident = Ident::new(&(ident.to_string() + "FieldEnum"), ident.span());

    let derives = match attrs.is_empty() {
        false => Some(
            attrs
                .into_iter()
                .filter(|attr| attr.path().is_ident("EnumDerive"))
                .flat_map(|attr| attr.parse_args::<proc_macro2::TokenStream>()),
        ),
        true => None,
    };
    let derives = derives.map(|iter| quote! {#[derive (#(#iter),*)]});

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };
    let fields_vec = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect_vec();

    let fields_str: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
        .collect::<Vec<String>>();

    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<&Type>>();

    let variants_str: Vec<String> = fields_str
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
            let i = field.find('_');
            let mut field = match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    if field.get(i).is_some() {
                        field[i] = field[i].to_uppercase();
                    }
                    field.join("")
                }
                None => field,
            };
            let i = field.find('_');
            match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    if field.get(i).is_some() {
                        field[i] = field[i].to_uppercase();
                    }
                    field.join("")
                }
                None => field,
            }
        })
        .collect_vec();
    let variants = variants_str
        .iter()
        .map(|variant| {
            let variant = Ident::new(variant, Span::call_site().into());
            quote! {#variant}
        })
        .collect_vec();
    let get_fields_enums = quote! {
        impl #ident {
            pub fn get_fields_enums(&self) -> Vec< #new_ident > {
                vec![#(#new_ident :: #variants (self. #fields_vec .clone())),*]
            }
        }
    };
    let result = quote! {
        #derives
        pub enum #new_ident{
            #(#variants (#field_types)),*
        }
        #get_fields_enums
        impl #new_ident{
            pub fn get_variants() -> Vec<&'static str> {
                vec![#( #variants_str ),*]
            }
        }
    };
    //println!("{result}");
    result.into()
}

#[proc_macro_derive(StructFieldEnumMut, attributes(EnumDerive, StructFields, StructValues))]
pub fn derive_struct_field_enum_mut(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        attrs, ident, data, ..
    } = ast;

    let new_ident = Ident::new(&(ident.to_string() + "FieldEnumMut"), ident.span());

    let derives = match attrs.is_empty() {
        false => Some(
            attrs
                .into_iter()
                .filter(|attr| attr.path().is_ident("EnumDerive"))
                .flat_map(|attr| attr.parse_args::<proc_macro2::TokenStream>()),
        ),
        true => None,
    };
    let derives = derives.map(|iter| quote! {#[derive (#(#iter),*)]});

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };
    let fields_vec = fields
        .iter()
        .filter_map(|field| field.ident.as_ref())
        .collect_vec();

    let fields_str: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
        .collect::<Vec<String>>();

    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<&Type>>();

    let variants_str: Vec<String> = fields_str
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
            let i = field.find('_');
            let mut field = match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    if field.get(i).is_some() {
                        field[i] = field[i].to_uppercase();
                    }
                    field.join("")
                }
                None => field,
            };
            let i = field.find('_');
            match i {
                Some(i) => {
                    field.remove(i);
                    let mut field = field.chars().map(|chr| chr.to_string()).collect_vec();
                    if field.get(i).is_some() {
                        field[i] = field[i].to_uppercase();
                    }
                    field.join("")
                }
                None => field,
            }
        })
        .collect_vec();
    let variants = variants_str
        .iter()
        .map(|variant| {
            let variant = Ident::new(variant, Span::call_site().into());
            quote! {#variant}
        })
        .collect_vec();
    let get_fields_enums_mut = quote! {
        impl #ident {
            pub fn get_fields_enums_mut(&mut self) -> Vec< #new_ident > {
                vec![#(#new_ident :: #variants (&mut self. #fields_vec)),*]
            }
        }
    };
    let result = quote! {
        #derives
        pub enum #new_ident<'a>{
            #(#variants (&'a mut #field_types)),*
        }
        #get_fields_enums_mut
        impl #new_ident<_>{
            pub fn get_variants() -> Vec<&'static str> {
                vec![#( #variants_str ),*]
            }
        }
    };
    //println!("{result}");
    result.into()
}
