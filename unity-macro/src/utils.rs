use quote::quote;
use syn::Ident;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};

pub fn import_my_crate() -> proc_macro2::TokenStream {
    let found_crate = crate_name("unity").expect("my-crate is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}