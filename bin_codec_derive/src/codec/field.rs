use syn::*;
use quote::*;
use crate::attribute::*;
use proc_macro2::*;

pub(crate) fn encode(field: &Field, sequence: usize, ed: &crate::Endin) -> TokenStream {
    let name = field.ident.as_ref().map(|id| {
        quote!( self.#id )
    }).unwrap_or(quote!( self.#sequence ));
    let attr = FieldAttr::ser_attrs(&field.attrs);
    let before = before(&attr);
    let after = after(&attr);
    let endin = match ed {
        crate::Endin::Big => quote! { encode_be },
        crate::Endin::Little => quote! { encode_le },
    };
    match attr.value {
        Some(Lit::Int(v)) => {
            quote! {
                #before
                let _value = #v;
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
                let size = Encode::#endin(&_value, target, target_start, ctx)?;
                target_start += size;
                #after
            }
        }
        None => {
            quote! {
                #before
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

pub(crate) fn decode(field: &Field, ed: &crate::Endin) -> TokenStream {
    let attr = FieldAttr::des_attrs(&field.attrs);
    let before = before(&attr);
    let after = after(&attr);
    let endin = match ed {
        crate::Endin::Big => quote! { decode_be },
        crate::Endin::Little => quote! { decode_le },
    };
    quote! {
        {
            #before
            let (value, size) = Decode::#endin(data, data_start, ctx)?;
            data_start += size;
            #after
            value
        }
    }
}
