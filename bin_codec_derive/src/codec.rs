use crate::Endin;
use crate::attribute::FieldAttr;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

mod field;
mod named;
mod unnamed;
mod variant;

pub(crate) fn encode(input: TokenStream, ed: Endin) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = input.ident;
    let decode: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => encode_struct(&data_struct, &ed),
        _ => unimplemented!()
    };
    let size: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => size_struct(&data_struct, &ed),
        _ => unimplemented!()
    };
    let trait_name = match ed {
        Endin::Big => quote!(bin_codec::EncodeBe),
        Endin::Little => quote!(bin_codec::EncodeLe),
    };
    let expanded = quote! {
        impl #generics #trait_name for #name #generics #where_clause {
            fn encode_offset<__CTX>(&self, target: &mut [u8], ctx: &mut __CTX, offset: &mut usize, bits: usize) {
                use bin_codec::*;
                let target_ptr = target.as_mut_ptr();
                #decode
            }

            fn bits_with_user_define(&self, bits: Option<usize>) -> usize {
                #size
            }
        }
    };
    TokenStream::from(expanded)
}

pub(crate) fn decode(input: TokenStream, ed: Endin) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let attr = FieldAttr::from_attrs(&input.attrs);
    let generics = input.generics;
    let where_clause = &generics.where_clause;
    let name = &input.ident;
    let decode_be: TokenStream2 = match &input.data {
        Data::Struct(data_struct) => decode_struct(&data_struct, name, &ed),
        _ => unimplemented!(),
    };
    let trait_name = match ed {
        Endin::Big => quote!(bin_codec::DecodeBe),
        Endin::Little => quote!(bin_codec::DecodeLe),
    };
    let context_type = attr.context.map(|l| quote!(#l)).unwrap_or(quote!(()));
    let has_next = attr.has_next.map(|l| quote!(*sd = ShouldDecode::HasNext(#l);));
    let expanded = quote! {
        impl #generics #trait_name for #name #generics #where_clause {
            type Context = #context_type;
            fn decode_offset(data: &[u8], data_start: &mut usize, sd: &mut ShouldDecode, ctx: &mut Self::Context, bits: usize) -> bin_codec::Result<Self> {
                #decode_be
                #has_next
                Ok(rt)
            }

            fn default_bits() -> usize {
                0
            }
        }
    };
    TokenStream::from(expanded)
}

fn encode_struct(data: &DataStruct, ed: &Endin) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => self::named::encode(named, ed),
        Fields::Unnamed(unnamed) => self::unnamed::encode(unnamed, ed),
        Fields::Unit => unimplemented!(),
    }
}

fn size_struct(data: &DataStruct, ed: &Endin) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => self::named::size(named, ed),
        Fields::Unnamed(unnamed) => self::unnamed::size(unnamed, ed),
        Fields::Unit => unimplemented!(),
    }
}

fn decode_struct(data: &DataStruct, name: &Ident, ed: &Endin) -> TokenStream2 {
    match &data.fields {
        Fields::Named(named) => {
            let (ref values, ref fields) = self::named::decode(named, ed);
            quote! {
                #(let #fields = #values;)*
                let rt = #name {
                    #(#fields,)*
                };
            }
        }
        Fields::Unnamed(unnamed) => {
            let values = self::unnamed::decode(unnamed, ed);
            quote! {
                let rt = #name (
                    #(#values,)*
                );
            }
        }
        Fields::Unit => unimplemented!("`unit struct` not implemented"),
    }
}
