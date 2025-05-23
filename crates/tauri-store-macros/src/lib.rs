#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://tb.dev.br/tauri-store/favicon.ico")]

mod collection;
mod collection_builder;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Collection)]
pub fn derive_collection(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  collection::impl_collection(&ast)
}

#[proc_macro_derive(CollectionBuilder)]
pub fn derive_collection_builder(input: TokenStream) -> TokenStream {
  let ast = syn::parse::<DeriveInput>(input).unwrap();
  collection_builder::impl_collection_builder(&ast)
}
