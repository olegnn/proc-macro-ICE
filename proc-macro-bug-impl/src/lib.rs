extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

thread_local! {
    static DEFAULT_CRATE_PATH: Path = parse_quote! { ::std };
}

#[proc_macro_attribute]
pub fn some_macro(_: TokenStream, _: TokenStream) -> TokenStream {
    TokenStream::from(DEFAULT_CRATE_PATH.with(|default_crate_path| {
        quote! {
            use #default_crate_path::boxed::Box;
        }
    }))
}
