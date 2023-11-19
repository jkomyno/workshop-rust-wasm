mod attrs;
mod container;
mod ctx;
mod derive;
mod parser;

use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoJsError, attributes(js_error, serde))]
pub fn derive_js_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: DeriveInput = parse_macro_input!(input);

    derive::expand(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
