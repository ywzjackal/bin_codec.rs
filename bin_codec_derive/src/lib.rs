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

mod utils;
mod attribute;
mod codec;

#[proc_macro_derive(BinEncode, attributes(bin_encode, bin))]
pub fn derive_bin_encode(input: TokenStream) -> TokenStream {
    codec::encode(input)
}

#[proc_macro_derive(BinDecode, attributes(bin_encode, bin))]
pub fn derive_bin_decode(input: TokenStream) -> TokenStream {
    codec::decode(input)
}