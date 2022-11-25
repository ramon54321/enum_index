use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Error, Meta, NestedMeta};

#[proc_macro_derive(EnumIndex)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident.clone();

    match validate_repr(&input) {
        Err(error) => {
            let message = match error {
                ReprError::NoRepr => {
                    "Missing repr on enum. EnumIndex requires enum to be repr(u16)"
                }
                ReprError::WrongRepr => {
                    "Incorrect repr on enum. EnumIndex requires enum to be repr(u16)"
                }
                ReprError::MultipleRepr => {
                    "Multiple repr on enum. EnumIndex requires enum to be repr(u16)"
                }
            };
            return Error::new(input.span(), message).to_compile_error().into();
        }
        _ => (),
    }

    let expanded = quote! {
        impl EnumIndexGet for #enum_name {
            fn index(&self) -> u16 {
                //
                // SAFETY: This is implemented through a macro which ensures the enum is repr(u16)
                //
                unsafe { *((self as *const _) as *const u16) }
            }
        }
    };

    TokenStream::from(expanded)
}

fn validate_repr(input: &DeriveInput) -> Result<(), ReprError> {
    for attribute in &input.attrs {
        let Some(Meta::List(meta)) = attribute.parse_meta().ok() else {
            continue;
        };

        if !meta.path.is_ident("repr") {
            continue;
        }

        if meta.nested.len() > 1 {
            return Err(ReprError::MultipleRepr);
        }

        let repr = match meta.nested.first() {
            Some(repr) => repr,
            None => continue,
        };

        let repr_name = match repr {
            NestedMeta::Meta(Meta::Path(path)) => path
                .get_ident()
                .and_then(|ident| Some(ident.clone().to_string())),
            _ => None,
        };

        let Some(repr_name) = repr_name else {
            return Err(ReprError::WrongRepr);
        };

        if repr_name != String::from("u16") {
            return Err(ReprError::WrongRepr);
        }

        return Ok(());
    }
    Err(ReprError::NoRepr)
}

#[derive(Debug)]
enum ReprError {
    NoRepr,
    WrongRepr,
    MultipleRepr,
}
