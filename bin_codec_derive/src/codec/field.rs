use syn::*;
use quote::*;
use crate::attribute::*;
use proc_macro2::*;

pub(crate) fn encode(field: &Field, sequence: usize, ed: &crate::Endin) -> TokenStream {
    let name = field.ident.as_ref().map(|id| {
        quote!( self.#id )
    }).unwrap_or(quote!( self.#sequence ));
    let attr = FieldAttr::from_attrs(&field.attrs);
    let before = before_en(&attr, &field.ty);
    let after = after(&attr);
    let endin = match ed {
        crate::Endin::Big => quote! { encode_be },
        crate::Endin::Little => quote! { encode_le },
    };
    let skip = attr.skip.unwrap_or(Lit::Int(LitInt::new(0, IntSuffix::Usize, Span::call_site())));
    match attr.value {
        Some(Lit::Int(v)) => {
            quote! {
                #before
                let _value = #v;
                target_start += #skip;
                let size = Encode::#endin(&_value, target, target_start, ctx)?;
                target_start += size;
                #after
            }
        }
        Some(Lit::Str(v)) => {
            let token: TokenStream = v.value().parse().unwrap_or_else(|e| {
                panic!("parse `{}` to expr fail:{:?}", v.value(), e);
            });
            quote! {
                #before
                let _value = #token;
                target_start += #skip;
                let size = Encode::#endin(&_value, target, target_start, ctx)?;
                target_start += size;
                #after
            }
        }
        None => {
            quote! {
                #before
                target_start += #skip;
                let size = Encode::#endin(&#name, target, target_start, ctx)?;
                target_start += size;
                #after
            }
        }
        Some(lit) => {
            unimplemented!("attribute `value` with value `{:?}` not supported", lit)
        }
    }
}

pub(crate) fn decode(field: &Field, sequence: usize, ed: &crate::Endin) -> TokenStream {
    let name = field.ident.as_ref().map(|id| {
        quote!( #id )
    }).unwrap_or(quote!( _#sequence ));
    let attr = FieldAttr::from_attrs(&field.attrs);
    let before = before_de(&attr, &field.ty);
    let after = after(&attr);
    let endin = match ed {
        crate::Endin::Big => quote! { decode_be },
        crate::Endin::Little => quote! { decode_le },
    };
    let skip = attr.skip.unwrap_or(Lit::Int(LitInt::new(0, IntSuffix::Usize, Span::call_site())));
    quote! {
        {
            #before
            data_start += #skip;
            let (value, size) = Decode::#endin(data, data_start, ctx)?;
            data_start += size;
            #after
            value
        }
    }
}
