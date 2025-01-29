use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(SizedData)]
pub fn derive_sized_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let size_calculation = match input.data {
        Data::Struct(data) => {
            let field_sizes = match data.fields {
                Fields::Named(fields) => fields.named.iter().map(|f| {
                    let ty = &f.ty;
                    quote! {
                        <#ty as SizedData>::size()
                    }
                }).collect(),
                Fields::Unnamed(fields) => fields.unnamed.iter().map(|f| {
                    let ty = &f.ty;
                    quote! {
                        <#ty as SizedData>::size()
                    }
                }).collect(),
                Fields::Unit => Vec::new(),
            };

            quote! {
                impl SizedData for #name {
                    fn size() -> usize {
                        0 #(+ #field_sizes)*
                    }
                }
            }
        }
        _ => panic!("SizedData can only be derived for structs"),
    };

    size_calculation.into()
}