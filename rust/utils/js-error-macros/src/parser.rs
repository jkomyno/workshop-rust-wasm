use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_derive_internals::ast::{Data, Field};

use crate::container::Container;

pub struct Parser<'a> {
    pub container: &'a Container<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(container: &'a Container<'a>) -> Self {
        Self { container }
    }

    pub fn parse(&self) -> TokenStream {
        let id = self.container.ident().to_string();
        match self.container.serde_data() {
            Data::Struct(_, ref fields) => self.parse_struct(id, fields),
            _ => {
                panic!("Only structs and supported");
            }
        }
    }

    fn parse_struct(&self, id: String, fields: &[Field]) -> TokenStream {
        let parsed_fields = self.parse_fields(fields);

        let set_statements: Vec<_> = parsed_fields
            .iter()
            .map(|(rust_name, js_name)| {
                let rust_name = format_ident!("{}", rust_name);
                quote! {
                    error_object.set(#js_name.into(), self.#rust_name.into());
                }
            })
            .collect();

        let message_field = format_ident!("{}", self.container.message_field());
        let id = format_ident!("{}", id);

        quote! {
            const _: () = {
                use js_sys::JsString;
                use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

                #[wasm_bindgen]
                extern "C" {
                    type ObjectExt;

                    #[wasm_bindgen(method, structural, indexing_setter)]
                    fn set(this: &ObjectExt, key: JsString, value: JsValue);
                }

                impl Into<JsValue> for #id {
                    fn into(self) -> JsValue {
                        let error_object = wasm_bindgen::JsError::new(&self.#message_field);
                        let error_object_as_value = JsValue::from(error_object);
                        let error_object = error_object_as_value.unchecked_into::<ObjectExt>();

                        // set all properties of `self` on `error_object`, one by one, except `message`
                        #(#set_statements)*

                        error_object.into()
                    }
                }
            };
        }
    }

    fn parse_fields(&self, fields: &[Field]) -> Vec<(String, String)> {
        let fields = fields
            .iter()
            .filter(|field| {
                !field.attrs.skip_serializing()
                    && !field.attrs.skip_deserializing()
                    && !is_phantom(field.ty)
            })
            .map(|field| self.parse_field(field))
            .fold(vec![], |mut acc, val| {
                if let Some((rust_name, js_name)) = val {
                    if rust_name != self.container.message_field() {
                        acc.push((rust_name, js_name));
                    }
                }

                acc
            });

        fields
    }

    fn parse_field(&self, field: &Field) -> Option<(String, String)> {
        match &field.member {
            syn::Member::Named(ref ident) => {
                let rust_name = ident.to_string();
                let js_name = field.attrs.name().serialize_name().to_owned();

                Some((rust_name, js_name))
            }
            syn::Member::Unnamed(_) => None,
        }
    }
}

fn is_phantom(ty: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path, .. }) = ty {
        path.segments
            .last()
            .map_or(false, |path| path.ident == "PhantomData")
    } else {
        false
    }
}
