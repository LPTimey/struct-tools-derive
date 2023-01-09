use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named,
    FieldsNamed,
};

#[proc_macro_derive(StructIterTools)]
pub fn derive_struct_iter_tools(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    println!("{:?}", ast);
    let DeriveInput { ident, data, .. } = ast.clone();

    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => todo!(),
    };

    println!("{:?}", fields);

    let field_ids = fields
        .clone()
        .into_iter()
        .filter_map(|field| match field.ident {
            Some(id) => Some(id),
            None => None,
        })
        .collect::<Vec<syn::Ident>>();

    let mut field_types = fields.clone().into_iter().filter_map(|fields|{
        match fields.ty{
            syn::Type::Path(type_path) => Some(type_path.path.segments[0].clone().ident.to_string()),
            _ => None,
        }
    }).collect::<Vec<String>>();
    field_types.sort();
    field_types.dedup();

    println!("{:?}",field_types);

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
            E: Display + From<u64> + From<String>// + #(From<#field_types>)+*
            {
                vec![#(E::from(self.#field_ids.clone())),*]
            }

        }
    };
    result.into()
}
