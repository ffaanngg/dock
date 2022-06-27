use syn::{AttributeArgs, LitBool, LitStr, NestedMeta};
pub struct CommandArgs {
    pub name: Option<LitStr>,
    pub description: Option<LitStr>,
    pub disabled: Option<LitBool>,
}

impl CommandArgs {
    pub fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut disabled = None;

        for arg in args {
            match arg {
                NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.is_ident("name") {
                        if let syn::Lit::Str(lit) = nv.lit {
                            name = Some(lit);
                        } else {
                            return Err(syn::Error::new_spanned(
                                nv.lit,
                                "Attribute name expects literal string!",
                            ));
                        }
                    } else if nv.path.is_ident("description") {
                        if let syn::Lit::Str(lit) = nv.lit {
                            description = Some(lit);
                        } else {
                            return Err(syn::Error::new_spanned(
                                nv.lit,
                                "Attribute description expects literal string!",
                            ));
                        }
                    } else if nv.path.is_ident("disabled") {
                        if let syn::Lit::Bool(lit) = nv.lit {
                            disabled = Some(lit)
                        }
                    }
                }
                _ => return Err(syn::Error::new_spanned(arg, "Unknown attribute")),
            };
        }

        Ok(Self {
            name,
            description,
            disabled,
        })
    }
}
