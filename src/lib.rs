use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named,
    FieldsNamed,
};

#[proc_macro_derive(StructFieldsIter)]
pub fn derive_struct_fields_iter(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let fields = if let Struct(DataStruct {
        fields: Named(FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!();
    };

    let fields_vec: std::vec::Vec<std::string::String> = fields.iter().filter_map(|field| {match &field.ident {
        Some(id) => Some(format!("{}",id)),
        None => None,
    }}).collect::<Vec<String>>();


    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<::std::string::String>{
                vec![#(#fields_vec.to_string()),*]
            }
        }
    };
    result.into()
}

