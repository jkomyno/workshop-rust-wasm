#[derive(Debug, Default)]
pub struct ContainerAttrs {
    message_field: Option<String>,
}

impl ContainerAttrs {
    pub fn from_derive_input(input: &syn::DeriveInput) -> syn::Result<Self> {
        let mut attrs = Self {
            message_field: None,
        };

        for attr in &input.attrs {
            if !attr.path().is_ident("js_error") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("message_field") {
                    if attrs.message_field.is_some() {
                        return Err(meta.error("duplicate attribute"));
                    }
                    let lit = meta.value()?.parse::<syn::LitStr>()?;
                    attrs.message_field = Some(lit.value());
                    return Ok(());
                }

                Err(meta.error("unsupported `js_error` attribute, expected one of `message_field`"))
            })?;
        }

        Ok(attrs)
    }

    pub fn message_field(&self) -> String {
        if let Some(ref field) = self.message_field.as_deref() {
            field.to_string()
        } else {
            "message".to_string()
        }
    }
}
