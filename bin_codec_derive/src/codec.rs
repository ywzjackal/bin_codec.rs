use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

mod named;
mod unnamed;
mod variant;
mod field;

pub fn encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = input.ident;
    let inner_be: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => encode_struct(&data_struct, &crate::Endin::Big),
        Data::Enum(data_enum) => encode_enum(&data_enum, &crate::Endin::Big),
        Data::Union(data_union) => encode_union(&data_union, &crate::Endin::Big),
    };
    let inner_le: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => encode_struct(&data_struct, &crate::Endin::Little),
        Data::Enum(data_enum) => encode_enum(&data_enum, &crate::Endin::Little),
        Data::Union(data_union) => encode_union(&data_union, &crate::Endin::Little),
    };
    // let fieldattr = crate::attribute::FieldAttr::from_attrs(&input.attrs);
    // let before = crate::attribute::before_en(&fieldattr, &name);
    // let after = crate::attribute::after(&fieldattr);
    let expanded = quote! {
        impl #generics bin_codec::Encode for #name #generics #where_clause {
            fn encode_be(&self, target: &mut [u8], mut target_start: usize, ctx: &mut bin_codec::Context) -> bin_codec::Result<usize> {
                let old_target_start = target_start;
                // #before
                #inner_be
                // #after
                Ok(target_start - old_target_start)
            }
            fn encode_le(&self, target: &mut [u8], mut target_start: usize, ctx: &mut bin_codec::Context) -> bin_codec::Result<usize> {
                let old_target_start = target_start;
                // #before
                #inner_le
                // #after
                Ok(target_start - old_target_start)
            }
        }
    };
//    println!("{}", expanded);
    TokenStream::from(expanded)
}

pub fn decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = &input.ident;
    let inner_be: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => decode_struct(&data_struct, name,&crate::Endin::Big),
        Data::Enum(data_enum) => decode_enum(&data_enum, &crate::Endin::Big),
        Data::Union(data_union) => decode_union(&data_union, &crate::Endin::Big),
    };
    let inner_le: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => decode_struct(&data_struct, name, &crate::Endin::Little),
        Data::Enum(data_enum) => decode_enum(&data_enum, &crate::Endin::Little),
        Data::Union(data_union) => decode_union(&data_union, &crate::Endin::Little),
    };
    // let fieldattr = crate::attribute::FieldAttr::from_attrs(&input.attrs);
    // let before = crate::attribute::before_de(&fieldattr, name);
    // let after = crate::attribute::after(&fieldattr);
    let expanded = quote! {
        impl #generics bin_codec::Decode for #name #generics #where_clause {
            fn decode_be(data: &[u8], mut data_start: usize, ctx: &mut bin_codec::Context) -> bin_codec::Result<(Self, usize)> {
                let data_start_old = data_start;
                // #before
                #inner_be
                // #after
                Ok((rt, data_start - data_start_old))
            }
            fn decode_le(data: &[u8], mut data_start: usize, ctx: &mut bin_codec::Context) -> bin_codec::Result<(Self, usize)> {
                let data_start_old = data_start;
                // #before
                #inner_le
                // #after
                Ok((rt, data_start - data_start_old))
            }
        }
    };
//    println!("{}", expanded);
    TokenStream::from(expanded)
}

fn encode_struct(data: &DataStruct, ed: &crate::Endin) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            self::named::encode(named, ed)
        }
        Fields::Unnamed(unnamed) => {
            self::unnamed::encode(unnamed, ed)
        }
        Fields::Unit => {
            unimplemented!()
        }
    }
}

fn decode_struct(data: &DataStruct, name: &Ident, ed: &crate::Endin) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            let (ref values, ref fields) = self::named::decode(named, ed);
            quote! {
                #(let #fields = #values;)*
                let rt = #name {
                    #(#fields),*
                };
            }
        }
        Fields::Unnamed(unnamed) => {
            let values = self::unnamed::decode(unnamed, ed);
            quote! {
                let rt = #name (
                    #(#values),*
                );
            }
        }
        Fields::Unit => {
            unimplemented!("`unit struct` not implemented")
        }
    }
}

fn encode_enum(_data: &DataEnum, _ed: &crate::Endin) -> TokenStream2 {
//    let mut tokens = Box::new(quote!());
//    for variant in data.variants.iter() {
//        let t = self::variant::encode(variant, ed);
//        t.to_tokens(&mut tokens);
//    }
//    tokens
    unimplemented!()
}

fn decode_enum(_data: &DataEnum, _ed: &crate::Endin) -> TokenStream2 {
//    let tokens = Box::new(quote!());
//    for variant in data.variants.iter() {
//        let (_f, _t) = self::variant::decode(variant, ed);
//        unimplemented!();
//    }
//    tokens
    unimplemented!()
}

fn encode_union(_data: &DataUnion, _ed: &crate::Endin) -> TokenStream2 {
    unimplemented!()
}

fn decode_union(_data: &DataUnion, _ed: &crate::Endin) -> TokenStream2 {
    unimplemented!()
}