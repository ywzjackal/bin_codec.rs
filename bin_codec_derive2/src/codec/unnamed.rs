use syn::*;
use quote::*;
use proc_macro2::TokenStream;
use super::field;

pub(crate) fn encode_be(field: &FieldsUnnamed) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.unnamed.iter().enumerate() {
        let t = field::encode_be(field, i);
        t.to_tokens(&mut tokens);
    }
    tokens
}
pub(crate) fn encode_le(field: &FieldsUnnamed) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.unnamed.iter().enumerate() {
        let t = field::encode_le(field, i);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn size(field: &FieldsUnnamed) -> TokenStream {
    let tokens = field
        .unnamed
        .iter()
        .enumerate()
        .map(|(i, f)| field::size(f, i))
        .collect::<Vec<_>>();
    quote!(#(#tokens)+*)
}

pub(crate) fn decode_be(field: &FieldsUnnamed) -> Vec<TokenStream> {
    field.unnamed.iter().map(|i| field::decode_be(i)).collect()
}