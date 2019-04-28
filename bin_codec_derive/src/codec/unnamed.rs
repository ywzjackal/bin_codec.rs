use super::field;
use crate::Endin;
use proc_macro2::TokenStream;
use quote::*;
use syn::*;

pub(crate) fn encode(field: &FieldsUnnamed, ed: &Endin) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.unnamed.iter().enumerate() {
        let t = field::encode_be(field, i, ed);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn size(field: &FieldsUnnamed, ed: &Endin) -> TokenStream {
    let tokens = field
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| field::size(f, i, ed))
        .collect::<Vec<_>>();
    quote!(#(#tokens)+*)
}

pub(crate) fn decode(field: &FieldsUnnamed, ed: &Endin) -> Vec<TokenStream> {
    field.unnamed.iter().map(|i| field::decode(i, ed)).collect()
}
