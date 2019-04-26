#![recursion_limit="128"]
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
extern crate quote;
extern crate bin_codec;

use proc_macro::TokenStream;

pub(crate) enum Endin {
    Big, Little
}

mod error;
pub(crate) mod utils;
mod attribute;
mod codec;

#[proc_macro_derive(BinEncodeBe, attributes(bin))]
pub fn derive_bin_encode_be(input: TokenStream) -> TokenStream {
    codec::encode_be(input)
}

#[proc_macro_derive(BinEncodeLe, attributes(bin))]
pub fn derive_bin_encode_le(input: TokenStream) -> TokenStream {
    codec::encode_le(input)
}

#[proc_macro_derive(BinDecodeBe, attributes(bin))]
pub fn derive_bin_decode_be(input: TokenStream) -> TokenStream {
    codec::decode_be(input)
}

// #[proc_macro_derive(BinDecodeLe, attributes(bin_encode, bin))]
// pub fn derive_bin_decode_le(input: TokenStream) -> TokenStream {
//     codec::decode_le(input)
// }