//! Procedural macro implementation for the `sized-data` crate.
//!
//! This crate provides the derive macro for automatically implementing
//! the `SizedData` trait on structs. Used primarily with Anchor framework
//! for Solana programs.
//!
//! # Example
//!
//! ```rust
//! use sized_data_derive::SizedData;
//! use anchor_lang::prelude::*;
//!
//! #[derive(SizedData)]
//! pub struct UserAccount {
//!     pub authority: Pubkey,
//!     pub counter: u64,
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Derives the `SizedData` trait for structs.
///
/// # Implementation Details
///
/// The macro generates a `size()` implementation that:
/// 1. Sums the sizes of all fields using their `SizedData` implementations
/// 2. Works with named fields, tuple structs, and unit structs
///
/// # Example Generated Code
///
/// ```rust
/// # use anchor_lang::prelude::*;
/// struct UserAccount {
///     authority: Pubkey,
///     counter: u64,
/// }
///
/// impl SizedData for UserAccount {
///     fn size() -> usize {
///         <Pubkey as SizedData>::size() +
///         <u64 as SizedData>::size()
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if used on enums or unions - only structs are supported.
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