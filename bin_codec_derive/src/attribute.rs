use proc_macro2::TokenStream;
use quote::*;
use syn::*;

pub struct FieldAttr {
    pub bits: Option<TokenStream>,
    pub is_some: Option<TokenStream>,
    pub has_next: Option<TokenStream>,
    pub count: Option<TokenStream>,
    pub value: Option<TokenStream>,
    pub skip: Option<TokenStream>,
    pub before_de: Option<TokenStream>,
    pub after_de: Option<TokenStream>,
    pub context: Option<TokenStream>,
}

impl FieldAttr {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        FieldAttr {
            bits: find_in_attrs(attrs, "bits"),
            is_some: find_in_attrs(attrs, "is_some"),
            count: find_in_attrs(attrs, "count"),
            has_next: find_in_attrs(attrs, "has_next"),
            value: find_in_attrs(attrs, "value"),
            skip: find_in_attrs(attrs, "skip"),
            before_de: find_in_attrs(attrs, "before_de"),
            after_de: find_in_attrs(attrs, "after_de"),
            context: find_in_attrs(attrs, "context"),
        }
    }
}

fn find_in_attrs(attrs: &[Attribute], ident: &str) -> Option<TokenStream> {
    for attr in attrs {
        match attr.parse_meta() {
            Ok(meta) => {
                if let Some(lit) = find_in_meta(&meta, ident) {
                    return Some(lit_to_tokenstream(&lit))
                }
            }
            Err(_) => panic!("parse attribute fail: {}", attr.tts.to_string()),
        }
    }
    None
}

fn find_in_meta(meta: &Meta, ident: &str) -> Option<Lit> {
    match meta {
        Meta::Word(ref word) if word == ident => None,
        Meta::NameValue(ref m) if m.ident == ident => Some(m.lit.clone()),
        Meta::List(ref list) => find_in_meta_list(list, ident),
        _ => None,
    }
}

fn find_in_meta_list(list: &MetaList, ident: &str) -> Option<Lit> {
    for meta in list.nested.iter() {
        match meta {
            NestedMeta::Meta(ref meta) => {
                if let Some(lit) = find_in_meta(meta, ident) {
                    return Some(lit);
                }
            }
            NestedMeta::Literal(lit) => {
                if list.ident == ident {
                    return Some(lit.clone());
                }
            }
        }
    }
    None
}

fn lit_to_tokenstream(lit: &Lit) -> TokenStream {
    match lit {
        Lit::Str(litstr) => {
            litstr.value().parse().unwrap_or_else(|_| {
                crate::utils::error(litstr, "parse expr fail")
            })
        }
        _ => quote!(#lit)
    }
}