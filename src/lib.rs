use itertools::Itertools;
use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    self, parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed,
    Ident, Type,
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

    let field_types = fields.iter().map(|field| &field.ty).unique();
    let types = quote!(#(From<#field_types>)+*);

    let fields_vec: std::vec::Vec<std::string::String> = fields
        .iter()
        .filter_map(|field| field.ident.as_ref().map(|id| format!("{id}")))
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

#[proc_macro_derive(StructEnum)]
pub fn derive_struct_enum(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let ident = Ident::new(&(ident.to_string() + "Enum"), ident.span());

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
        //#[derive(Debug)]
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
