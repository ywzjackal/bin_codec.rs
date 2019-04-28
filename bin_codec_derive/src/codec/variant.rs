use crate::Endin;
use syn::*;
use proc_macro2::TokenStream;

#[allow(dead_code)]
pub(crate) fn encode_be(variant: &Variant, ed: &Endin) -> TokenStream {
    match &variant.fields {
        Fields::Named(named) => {
            super::named::encode(named, ed)
        }
        Fields::Unnamed(unnamed) => {
            super::unnamed::encode(unnamed, ed)
        }
        Fields::Unit => {
            unimplemented!();
        }
    }
}
#[allow(dead_code)]
pub(crate) fn encode_le(variant: &Variant, ed: &Endin) -> TokenStream {
    match &variant.fields {
        Fields::Named(named) => {
            super::named::encode(named, ed)
        }
        Fields::Unnamed(unnamed) => {
            super::unnamed::encode(unnamed, ed)
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
