use syn::*;
use quote::*;
use proc_macro2::TokenStream;

#[derive(Default)]
pub struct FieldAttr {
    pub bit_size: Option<usize>,
    pub is_some: Option<Lit>,
    pub has_next: Option<Lit>,
    pub count: Option<Lit>,
    pub value: Option<Lit>,
}

impl FieldAttr {
    pub fn des_attrs(attrs: &[Attribute]) -> Self {
        let mut rt = FieldAttr::default();
        if let Some(lit) = find_in_attrs(attrs, "bits") {
            if let Lit::Int(int) = lit {
                rt.bit_size = Some(int.value() as usize);
            } else {
                panic!("attribute bits only support integer value")
            }
        }
        rt.is_some = find_in_attrs(attrs, "is_some");
        rt.count = find_in_attrs(attrs, "count");
        if let Some(has_next) = find_in_attrs(attrs, "has_next") {
            rt.has_next = Some(has_next);
        }
        rt
    }

    pub fn ser_attrs(attrs: &[Attribute]) -> Self {
        let mut rt = FieldAttr::default();
        if let Some(lit) = find_in_attrs(attrs, "bits") {
            if let Lit::Int(int) = lit {
                rt.bit_size = Some(int.value() as usize);
            } else {
                panic!("attribute bits only support integer value")
            }
        }
        rt.value = find_in_attrs(attrs, "value");
        rt
    }
}

fn find_in_attrs(attrs: &[Attribute], ident: &str) -> Option<Lit> {
    for attr in attrs {
        if let Ok(meta) = attr.parse_meta() {
            if let Some(lit) = find_in_meta(&meta, ident) {
                return Some(lit);
            }
        }
    }
    None
}

fn find_in_meta(meta: &Meta, ident: &str) -> Option<Lit> {
    match meta {
        Meta::Word(ref word) if word == ident => {
            None
        }
        Meta::NameValue(ref m) if m.ident == ident => {
            Some(m.lit.clone())
        }
        Meta::List(ref list) => {
            find_in_meta_list(list, ident)
        }
        _ => None
    }
}

fn find_in_meta_list(list: &MetaList, ident: &str) -> Option<Lit> {
    for meta in list.nested.iter() {
        match meta {
            NestedMeta::Meta(ref meta) => {
                if let Some(lit) = find_in_meta(meta, ident) {
                    return Some(lit)
                }
            }
            NestedMeta::Literal(_) => {
                // not support yet
            }
        }
    }
    None
}

pub fn before(attr: &FieldAttr) -> Box<dyn ToTokens> {
    let mut tokens = Box::new(quote!{
//        ctx.set_has_next(None);
    });
    if let Some(bit_size) = attr.bit_size {
        Box::new(quote! {
            ctx.set_bit_size(Some(#bit_size));
        }).to_tokens(&mut tokens);
    } else {
        Box::new(quote! {
            ctx.set_bit_size(None);
        }).to_tokens(&mut tokens);
    }
    if let Some(ref lif) = attr.is_some {
        match lif {
            Lit::Str(v) => {
                let token: TokenStream = v.value().parse().unwrap_or_else(|e| {
                    panic!("parse `{}` to expr fail:{:?}", v.value(), e);
                });
                Box::new(quote! {
                    ctx.set_is_some(Some(#token));
                }).to_tokens(&mut tokens);
            }
            _ => panic!("expect string")
        }
    }
    if let Some(ref lif) = attr.count {
        match lif {
            Lit::Str(v) => {
                let token: TokenStream = v.value().parse().unwrap_or_else(|e| {
                    panic!("parse `{}` to expr fail:{:?}", v.value(), e);
                });
                Box::new(quote! {
                    ctx.set_count(Some(#token));
                }).to_tokens(&mut tokens);
            }
            _ => panic!("expect string")
        }
    }
    tokens
}

pub fn after(attr: &FieldAttr) -> Box<dyn ToTokens> {
    let mut tokens = Box::new(quote!{});
    if let Some(ref lif) = attr.has_next {
        match lif {
            Lit::Str(v) => {
                let token: TokenStream = v.value().parse().unwrap_or_else(|e| {
                    panic!("parse `{}` to expr fail:{:?}", v.value(), e);
                });
                Box::new(quote! {
                    ctx.set_has_next(Some(#token));
                }).to_tokens(&mut tokens);
            }
            _ => panic!("expect string")
        }
    }
    tokens
}