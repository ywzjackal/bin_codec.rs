use syn::*;
use proc_macro2::TokenStream;

#[allow(dead_code)]
pub(crate) fn encode_be(variant: &Variant) -> TokenStream {
    match &variant.fields {
        Fields::Named(named) => {
            super::named::encode_be(named)
        }
        Fields::Unnamed(unnamed) => {
            super::unnamed::encode_be(unnamed)
        }
        Fields::Unit => {
            unimplemented!();
        }
    }
}
#[allow(dead_code)]
pub(crate) fn encode_le(variant: &Variant) -> TokenStream {
    match &variant.fields {
        Fields::Named(named) => {
            super::named::encode_le(named)
        }
        Fields::Unnamed(unnamed) => {
            super::unnamed::encode_le(unnamed)
        }
        Fields::Unit => {
            unimplemented!();
        }
    }
}
// #[allow(dead_code)]
// pub(crate) fn decode(variant: &Variant, ed: &crate::Endin) -> (Vec<TokenStream>, Vec<TokenStream>) {
//     match &variant.fields {
//         Fields::Named(named) => {
//             super::named::decode(named, ed)
//         }
//         Fields::Unnamed(_unnamed) => {
//             unimplemented!();
//         }
//         Fields::Unit => {
//             unimplemented!();
//         }
//     }
// }
