use crate::attribute::*;
use proc_macro2::*;
use quote::*;
use syn::*;

pub(crate) fn encode_be(field: &Field, sequence: usize) -> TokenStream {
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
    let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
    let bit_copy = match &field.ty {
        // Type::Path(TypePath { path, .. }) if path.is_ident("u8") || path.is_ident("i8") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(8));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_be() as u8) as *const u8;
        //         let mut v_start = 8 - bits;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u16") || path.is_ident("i16") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(16));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_be() as u16) as *const u16 as *const u8;
        //         let mut v_start = 16 - bits;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u32") || path.is_ident("i32") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(32));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_be() as u32) as *const u32 as *const u8;
        //         let mut v_start = 32 - bits;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u64") || path.is_ident("i64") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(64));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_be() as u64) as *const u64 as *const u8;
        //         let mut v_start = 64 - bits;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u64, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u128") || path.is_ident("i128") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(128));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_be() as u128) as *const u128 as *const u8;
        //         let mut v_start = 128 - bits;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u128, u64, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("bool") => {
        //     quote! {
        //         let bits = 1;
        //         let value = #value;
        //         let v = &value as *const bool as *const u8;
        //         let mut v_start = 7;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("f32") => {
        //     quote! {
        //         let bits = 32;
        //         let v = #value;
        //         let v = &v as *const f32 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("f64") => {
        //     quote! {
        //         let bits = 64;
        //         let v = #value;
        //         let v = &v as *const 64 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits);
        //         }
        //     }
        // }
        _ => {
            let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(bin_codec::EncodeBe::bits(&#name)));
            quote! {
                let bits = #bits;
                let value = &#value;
                bin_codec::EncodeBe::encode_offset(value, target, ctx, offset, bits);
            }
        }
    };
    quote! {
        use bin_codec::*;
        let target_ptr = target.as_mut_ptr();
        #skip
        #bit_copy
    }
}

pub(crate) fn encode_le(field: &Field, sequence: usize) -> TokenStream {
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
    let value = attr.value.map(|l| quote!(#l)).unwrap_or(quote!(#name));
    let bit_copy = match &field.ty {
        // Type::Path(TypePath { path, .. }) if path.is_ident("u8") || path.is_ident("i8") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(8));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_le() as u8) as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u16") || path.is_ident("i16") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(16));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_le() as u16) as *const u16 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u32") || path.is_ident("i32") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(32));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_le() as u32) as *const u32 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u64") || path.is_ident("i64") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(64));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_le() as u64) as *const u64 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u64, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("u128") || path.is_ident("i128") => {
        //     let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(128));
        //     quote! {
        //         let bits = #bits;
        //         let value = #value;
        //         let v = &(value.to_le() as u128) as *const u128 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u128, u64, u32, u16, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("bool") => {
        //     quote! {
        //         let bits = 1;
        //         let value = #value;
        //         let v = &value as *const bool as *const u8;
        //         let mut v_start = 7;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits, u8);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("f32") => {
        //     quote! {
        //         let bits = 32;
        //         let v = #value;
        //         let v = &v as *const f32 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits);
        //         }
        //     }
        // }
        // Type::Path(TypePath { path, .. }) if path.is_ident("f64") => {
        //     quote! {
        //         let bits = 64;
        //         let v = #value;
        //         let v = &v as *const 64 as *const u8;
        //         let mut v_start = 0;
        //         unsafe {
        //             bin_codec::bit_copy!(target_ptr, offset, v, v_start, bits);
        //         }
        //     }
        // }
        _ => {
            let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(bin_codec::EncodeLe::bits(&#name)));
            quote! {
                let bits = #bits;
                let value = &#value;
                bin_codec::EncodeLe::encode_offset(value, target, ctx, offset, bits);
            }
        }
    };
    quote! {
        use bin_codec::*;
        let target_ptr = target.as_mut_ptr();
        #skip
        #bit_copy
    }
}

pub(crate) fn size(field: &Field, sequence: usize) -> TokenStream {
    let name = field
        .ident
        .as_ref()
        .map(|id| quote!( self.#id ))
        .unwrap_or(quote!( self.#sequence ));
    let attr = FieldAttr::from_attrs(&field.attrs);
    let skip = attr
        .skip
        .map(|l| quote!(+ #l));
    let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(bin_codec::EncodeBe::bits(&#name)));
    quote!(#bits #skip)
}

pub(crate) fn decode_be(field: &Field) -> TokenStream {
    let attr = FieldAttr::from_attrs(&field.attrs);
    let ty = &field.ty;
    // let before = before_de(&attr);
    // let after = after(&attr);
    let skip = attr.skip.map(|l| quote!(*data_start += #l;));
    let bits = attr.bits.map(|l| quote!(#l)).unwrap_or(quote!(std::mem::size_of::<#ty>() * 8));
    quote! {
        {
            // #before
            let bits = #bits;
            #skip
            DecodeBe::decode_offset(data, data_start, sd, ctx, bits)?
        }
    }
}
