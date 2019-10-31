extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Path;

struct MacroGenerator<'a> {
    some_crate_path: Option<&'a Path>,
}

impl<'a> MacroGenerator<'a> {
    pub fn new(some_crate_path: Option<&'a Path>) -> Self
    where
        Self: Sized,
    {
        MacroGenerator { some_crate_path }
    }
}

impl<'a> ToTokens for MacroGenerator<'a> {
    fn to_tokens(&self, output: &mut TokenStream) {
        let some_crate_path = self.some_crate_path;
        output.extend(quote! {
            use #some_crate_path::boxed::Box;
        });
    }
}

thread_local! {
    static DEFAULT_PATH: Path = syn::parse_quote! { ::std };
}

#[proc_macro_attribute]
pub fn some_macro(_: TokenStream1, _: TokenStream1) -> TokenStream1 {
    TokenStream1::from(
        DEFAULT_PATH.with(|default_crate_path| {
            MacroGenerator::new(Some(default_crate_path)).into_token_stream()
        })
    )
}
