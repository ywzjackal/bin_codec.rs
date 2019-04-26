use syn::*;
use quote::*;
use proc_macro2::TokenStream;
use super::field;

pub(crate) fn encode(field: &FieldsNamed, ed: &crate::Endin) -> TokenStream {
    let mut tokens = quote!();
    for (i, field) in field.named.iter().enumerate() {
        let t = field::encode(field, i, ed);
        t.to_tokens(&mut tokens);
    }
    tokens
}

pub(crate) fn decode(field: &FieldsNamed, ed: &crate::Endin) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut values = Vec::new();
    let mut fields = Vec::new();
    for (i, field) in field.named.iter().enumerate() {
        let t = field::decode(field, i, ed);
        values.push(t);
        let name = &field.ident;
        let f = quote! { #name };
        fields.push(f);
    }
    (values, fields)
}
