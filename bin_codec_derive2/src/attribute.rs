use syn::*;

pub struct FieldAttr {
    pub bits: Option<Lit>,
    pub is_some: Option<Lit>,
    pub has_next: Option<Lit>,
    pub count: Option<Lit>,
    pub value: Option<Lit>,
    pub skip: Option<Lit>,
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
        }
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