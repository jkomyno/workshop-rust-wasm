use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::{container::Container, parser::Parser};

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    let cont = Container::from_derive_input(&input)?;

    let parser = Parser::new(&cont);
    let tokens = parser.parse();

    cont.check()?;

    Ok(tokens)
}
