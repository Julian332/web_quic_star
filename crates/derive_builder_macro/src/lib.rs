//! Derive a builder for a struct

#![crate_type = "proc-macro"]
#![deny(warnings)]

extern crate derive_builder_core;
extern crate proc_macro;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

/// derive basic web api.
///
#[proc_macro_derive(
    WebApiGen,
    attributes(
        builder,
        builder_field_attr,
        builder_impl_attr,
        builder_setter_attr,
        builder_struct_attr
    )
)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    derive_builder_core::web_api_builder_for_struct(ast).into()
}
/// derive basic web query api.
#[proc_macro_derive(
    ViewApiGen,
    attributes(
        builder,
        builder_field_attr,
        builder_impl_attr,
        builder_setter_attr,
        builder_struct_attr
    )
)]
pub fn derive2(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    derive_builder_core::query_api_builder_for_struct(ast).into()
}
