use syn::*;
use quote::*;
use proc_macro2::TokenStream;
use super::field;

pub(crate) fn encode(field: &FieldsUnnamed, ed: &crate::Endin) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.unnamed.iter().enumerate() {
        let t = field::encode(field, i, ed);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn decode(field: &FieldsUnnamed, ed: &crate::Endin) -> (Vec<TokenStream>) {
    let mut values = Vec::new();
    for field in field.unnamed.iter() {
        let t = field::decode(field, ed);
        values.push(t);
    }
    values
}