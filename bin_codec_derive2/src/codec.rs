use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

mod named;
mod unnamed;
mod variant;
mod field;

pub fn encode_be(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = input.ident;
    let decode: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => encode_struct_be(&data_struct),
        Data::Enum(data_enum) => encode_enum(&data_enum, &crate::Endin::Big),
        Data::Union(data_union) => encode_union(&data_union, &crate::Endin::Big),
    };
    let size: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => size_struct(&data_struct),
        Data::Enum(data_enum) => size_enum(&data_enum),
        Data::Union(data_union) => size_union(&data_union),
    };
    let expanded = quote! {
        impl #generics bin_codec::EncodeBe for #name #generics #where_clause {
            fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
                #decode
            }

            fn bits(&self) -> usize {
                #size
            }
        }
    };
    TokenStream::from(expanded)
}

pub fn encode_le(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = input.ident;
    let inner: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => encode_struct_le(&data_struct),
        Data::Enum(data_enum) => encode_enum(&data_enum, &crate::Endin::Little),
        Data::Union(data_union) => encode_union(&data_union, &crate::Endin::Little),
    };
    let size: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => size_struct(&data_struct),
        Data::Enum(data_enum) => size_enum(&data_enum),
        Data::Union(data_union) => size_union(&data_union),
    };
    let expanded = quote! {
        impl #generics bin_codec::EncodeLe for #name #generics #where_clause {
            fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
                #inner
            }

            fn bits(&self) -> usize {
                #size
            }
        }
    };
    TokenStream::from(expanded)
}

pub fn decode_be(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = &input.ident;
    let decode_be: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => decode_struct_be(&data_struct, name),
        _ => unimplemented!(),
    };
    let expanded = quote! {
        impl #generics bin_codec::DecodeBe for #name #generics #where_clause {
            fn decode_offset<T>(data: &[u8], data_start: &mut usize, sd: &mut ShouldDecode, ctx: &mut T, bits: usize) -> bin_codec::Result<Self> {
                #decode_be
                Ok(rt)
            }
        }
    };
    TokenStream::from(expanded)
}

fn encode_struct_be(data: &DataStruct) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            self::named::encode_be(named)
        }
        Fields::Unnamed(unnamed) => {
            self::unnamed::encode_be(unnamed)
        }
        Fields::Unit => {
            unimplemented!()
        }
    }
}

fn encode_struct_le(data: &DataStruct) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            self::named::encode_le(named)
        }
        Fields::Unnamed(unnamed) => {
            self::unnamed::encode_le(unnamed)
        }
        Fields::Unit => {
            unimplemented!()
        }
    }
}

fn size_struct(data: &DataStruct) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            self::named::size(named)
        }
        Fields::Unnamed(unnamed) => {
            self::unnamed::size(unnamed)
        }
        Fields::Unit => {
            unimplemented!()
        }
    }
}

fn decode_struct_be(data: &DataStruct, name: &Ident) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            let (ref values, ref fields) = self::named::decode_be(named);
            quote! {
                #(let #fields = #values;)*
                let rt = #name {
                    #(#fields),*
                };
            }
        }
        Fields::Unnamed(unnamed) => {
            let values = self::unnamed::decode_be(unnamed);
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

fn size_enum(_: &DataEnum) -> TokenStream2 {
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

fn size_union(_data: &DataUnion) -> TokenStream2 {
    unimplemented!()
}