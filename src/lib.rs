use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed,
};

#[proc_macro_derive(StructIterTools)]
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

    let field_types = fields.iter().map(|field| &field.ty);
    let types = quote!(#(From<#field_types>)+*);

    let fields_vec: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| match &field.ident {
            Some(id) => Some(format!("{}", id)),
            None => None,
        })
        .collect::<Vec<String>>();

    let result = quote! {
        impl #ident {
            pub fn fields() -> ::std::vec::Vec<::std::string::String>{
                vec![#(#fields_vec.to_string()),*]
            }
        }
        impl #ident{
            pub fn values<E>(&self) -> ::std::vec::Vec<E>
            where
            E: #types
            {
                vec![#(E::from(self.#field_ids.clone())),*]
            }
        }
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
    };
    result.into()
}
