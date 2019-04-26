use crate::attribute::*;
use proc_macro2::*;
use quote::*;
use syn::*;

pub(crate) fn encode(field: &Field, sequence: usize, ed: &crate::Endin) -> TokenStream {
    let name = field
        .ident
        .as_ref()
        .map(|id| quote!( self.#id ))
        .unwrap_or(quote!( self.#sequence ));
    let attr = FieldAttr::from_attrs(&field.attrs);
    let skip = attr
        .skip
        .map(|l| quote!(*offset += #l;))
        .unwrap_or(quote!());
    // let has_next = attr
    //     .has_next
    //     .map(|l| quote!(ctx.has_next = Some(#l);))
    //     .unwrap_or(quote!());
    let bit_copy = match &field.ty {
        Type::Path(TypePath { path, .. }) if path.is_ident("u8") || path.is_ident("i8") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 8));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = #value;
                let v = value.to_be() as u8;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u8, 8 - ctx.bits, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u16") || path.is_ident("i16") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 16));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = #value;
                let v = value.to_be() as u16;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u16 as *const u8, 16 - ctx.bits, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u32") || path.is_ident("i32") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 32));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = #value;
                let v = value.to_be() as u32;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u32 as *const u8, 32 - ctx.bits, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u64") || path.is_ident("i64") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 64));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = #value;
                let v = value.to_be() as u64;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u64 as *const u8, 64 - ctx.bits, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("u128") || path.is_ident("i128") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 128));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = #value;
                let v = value.to_be() as u128;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u128 as *const u8, 128 - ctx.bits, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        Type::Path(TypePath { path, .. }) if path.is_ident("bool") => {
            let bits = attr
                .bits
                .map(|l| quote!(ctx.bits = #l))
                .unwrap_or(quote!(ctx.bits = 1));
            let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
            quote! {
                #bits;
                let value = if #value {-1} else {0};
                let v = value as u32;
                unsafe {
                    bin_codec::bit_copy(target.as_mut_ptr(), *offset, &v as *const u32 as * const u8, 0, ctx.bits);
                }
                *offset += ctx.bits;
            }
        }
        _ => {
            let bits = attr
                .bits
                .map(|l| quote!(#l))
                .unwrap_or(quote!(std::mem::size_of::<#field.ty>()));
            quote! {
                ctx.bits = #bits;
                bin_codec::EncodeBe::encode_offset(&#name, target, ctx, offset)?;
            }
        }
    };
    quote! {
        #skip
        #bit_copy
        // #has_next
    }
}

// pub(crate) fn decode(field: &Field, ed: &crate::Endin) -> TokenStream {
//     let attr = FieldAttr::from_attrs(&field.attrs);
//     let before = before_de(&attr);
//     let after = after(&attr);
//     let endin = match ed {
//         crate::Endin::Big => quote! { decode_be },
//         crate::Endin::Little => quote! { decode_le },
//     };
//     let skip = attr.skip.unwrap_or(Lit::Int(LitInt::new(
//         0,
//         IntSuffix::Usize,
//         Span::call_site(),
//     )));
//     quote! {
//         {
//             #before
//             data_start += #skip;
//             let (value, size) = Decode::#endin(data, data_start, ctx)?;
//             data_start += size;
//             #after
//             value
//         }
//     }
// }
