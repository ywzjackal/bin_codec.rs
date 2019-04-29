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

pub(crate) fn decode(field: &FieldsUnnamed, ed: &Endin) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut values = Vec::new();
    let mut fields = Vec::new();
    for (i, field) in field.unnamed.iter().enumerate() {
        let t = field::decode(field, ed);
        values.push(t);
        let f: TokenStream = format!("_{}", i).parse().unwrap();
        fields.push(f);
    }
    (values, fields)
}
