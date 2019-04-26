use super::field;
use proc_macro2::TokenStream;
use quote::*;
use syn::*;

pub(crate) fn encode_be(field: &FieldsNamed) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.named.iter().enumerate() {
        let t = field::encode_be(field, i);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn encode_le(field: &FieldsNamed) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.named.iter().enumerate() {
        let t = field::encode_le(field, i);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn size(field: &FieldsNamed) -> TokenStream {
    let tokens = field
        .named
        .iter()
        .enumerate()
        .map(|(i, f)| field::size(f, i))
        .collect::<Vec<_>>();
    quote!(#(#tokens)+*)
}

pub(crate) fn decode_be(field: &FieldsNamed) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut values = Vec::new();
    let mut fields = Vec::new();
    for field in field.named.iter() {
        let t = field::decode_be(field);
        values.push(t);
        let name = &field.ident;
        let f = quote! { #name };
        fields.push(f);
    }
    (values, fields)
}
