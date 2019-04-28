use crate::attribute::*;
use crate::Endin;
use proc_macro2::*;
use quote::*;
use syn::*;

pub(crate) fn encode_be(field: &Field, sequence: usize, ed: &Endin) -> TokenStream {
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
    let trait_name = match ed {
        Endin::Big => quote!(bin_codec::EncodeBe),
        Endin::Little => quote!(bin_codec::EncodeLe),
    };
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
            let bits = attr
                .bits
                .map(|l| quote!(#l))
                .unwrap_or(quote!(#trait_name::bits(&#name)));
            quote! {
                let bits = #bits;
                let value = &(#value);
                #trait_name::encode_offset(value, target, ctx, offset, bits);
            }
        }
    };
    quote! {
        #skip
        #bit_copy
    }
}

pub(crate) fn size(field: &Field, sequence: usize, ed: &Endin) -> TokenStream {
    let name = field
        .ident
        .as_ref()
        .map(|id| quote!( self.#id ))
        .unwrap_or(quote!( self.#sequence ));
    let attr = FieldAttr::from_attrs(&field.attrs);
    let skip = attr.skip.map(|l| quote!(+ #l));
    let trait_name = match ed {
        Endin::Big => quote!(bin_codec::EncodeBe),
        Endin::Little => quote!(bin_codec::EncodeLe),
    };
    let bits = attr
        .bits
        .map(|l| quote!(#trait_name::bits_with_user_define(&#name, Some(#l))))
        .unwrap_or(quote!(#trait_name::bits_with_user_define(&#name, None)));
    quote!(#bits #skip)
}

pub(crate) fn decode(field: &Field, ed: &Endin) -> TokenStream {
    let attr = FieldAttr::from_attrs(&field.attrs);
    let ty = &field.ty;
    // let before = before_de(&attr);
    // let after = after(&attr);
    let skip = attr.skip.map(|l| quote!(*data_start += #l;));
    let bits = attr
        .bits
        .map(|l| quote!(#l))
        .unwrap_or(quote!(<#ty as bin_codec::DecodeBe>::default_bits()));
    let count = attr.count.map(|l| quote!(*sd = ShouldDecode::Count(#l);));
    let has_next = attr
        .has_next
        .map(|l| quote!(*sd = ShouldDecode::HasNext(#l);));
    let is_some = attr
        .is_some
        .map(|l| quote!(*sd = ShouldDecode::IsSome(#l);));
    let trait_name = match ed {
        Endin::Big => quote!(bin_codec::DecodeBe),
        Endin::Little => quote!(bin_codec::DecodeLe),
    };
    let before_de = attr.before_de.map(|l| quote!(#l));
    let after_de = attr.after_de.map(|l| quote!(#l));
    let ctx = attr.context.map(|l| quote!(let ctx = #l;)).unwrap_or(quote!(let ctx = &mut ();));
    quote! {
        {
            // #before
            let bits = #bits;
            #count
            #before_de
            #has_next
            #is_some
            #skip
            #ctx
            #trait_name::decode_offset(data, data_start, sd, ctx, bits)?
        };
        #after_de
    }
}
