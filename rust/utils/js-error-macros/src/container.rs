use serde_derive_internals::{ast, ast::Container as SerdeContainer};

use crate::{attrs::ContainerAttrs, ctx::Ctx};

pub struct Container<'a> {
    pub ctx: Ctx,
    pub attrs: ContainerAttrs,
    pub serde_container: SerdeContainer<'a>,
}

impl<'a> Container<'a> {
    pub fn new(serde_container: SerdeContainer<'a>) -> Self {
        let input = &serde_container.original;
        let attrs = ContainerAttrs::from_derive_input(input);
        let ctx = Ctx::new();

        let attrs = match attrs {
            Ok(attrs) => attrs,
            Err(err) => {
                ctx.syn_error(err);
                Default::default()
            }
        };

        Self {
            ctx,
            attrs,
            serde_container,
        }
    }

    pub fn from_derive_input(input: &'a syn::DeriveInput) -> syn::Result<Self> {
        let cx = serde_derive_internals::Ctxt::new();
        let serde_cont =
            SerdeContainer::from_ast(&cx, input, serde_derive_internals::Derive::Serialize);

        match serde_cont {
            Some(serde_container) => {
                cx.check()?;
                Ok(Self::new(serde_container))
            }
            None => Err(cx.check().expect_err("serde_cont is None")),
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.serde_container.ident
    }

    pub fn serde_data(&self) -> &ast::Data {
        &self.serde_container.data
    }

    pub fn check(self) -> syn::Result<()> {
        self.ctx.check()
    }

    pub fn message_field(&self) -> String {
        self.attrs.message_field()
    }
}
