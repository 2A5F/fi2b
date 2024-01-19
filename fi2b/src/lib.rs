use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, token::Comma, Lit};

struct LitArr {
    pub elems: Punctuated<Lit, Comma>,
}

impl Parse for LitArr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut elems = Punctuated::new();
        while !input.is_empty() {
            let first: Lit = input.parse()?;
            if !matches!(first, Lit::Int(_) | Lit::Float(_)) {
                let span = first.span();
                let err = syn::Error::new(span, "Can only be an int or float literal");
                return Err(err);
            }
            elems.push_value(first);
            if input.is_empty() {
                break;
            }
            let punct = input.parse()?;
            elems.push_punct(punct);
        }
        Ok(Self { elems })
    }
}

/// Convert floating point and integer numbers into bytes and put them into an array
///
/// ```rust
/// # use fi2b::fi2b;
/// #
/// let a: &[u8] = &fi2b![1, 2, 1.0, -1.0];
/// assert_eq!(a, [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 63u8, 0u8, 0u8, 128u8, 191u8])
/// ```
#[proc_macro]
pub fn fi2b(tks: TokenStream) -> TokenStream {
    let arr = parse_macro_input!(tks as LitArr);
    let mut bytes: Vec<u8> = vec![];
    for lit in arr.elems {
        match lit {
            Lit::Int(v) => {
                if let Ok(v) = v.base10_parse::<i32>() {
                    bytes.extend_from_slice(&v.to_le_bytes());
                    continue;
                }
                if let Ok(v) = v.base10_parse::<u32>() {
                    bytes.extend_from_slice(&v.to_le_bytes());
                    continue;
                }
                return TokenStream::from(
                    syn::Error::new(v.span(), "Unable to parse literal").to_compile_error(),
                );
            }
            Lit::Float(v) => {
                if let Ok(v) = v.base10_parse::<f32>() {
                    bytes.extend_from_slice(&v.to_le_bytes());
                    continue;
                }
                return TokenStream::from(
                    syn::Error::new(v.span(), "Unable to parse literal").to_compile_error(),
                );
            }
            _ => {}
        }
    }
    quote! {
        [#(#bytes),*]
    }
    .into()
}
