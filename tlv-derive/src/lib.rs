mod tlv_decode_derive;
mod tlv_encode_derive;
mod tlv_config;
mod utils;

use proc_macro::TokenStream;
use syn::Error;
use tlv_decode_derive::tlv_decode;
use tlv_encode_derive::tlv_encode;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(TlvEncode, attributes(tlv_config))]
pub fn tlv_encode_derive(input: TokenStream) -> TokenStream {
	let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
	let output_stream: proc_macro2::TokenStream = tlv_encode(parsed_input).unwrap_or_else(Error::into_compile_error);
	output_stream.into()
}

#[proc_macro_error]
#[proc_macro_derive(TlvDecode, attributes(tlv_config))]
pub fn tlv_decode_derive(input: TokenStream) -> TokenStream {
	let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
	let output_stream = tlv_decode(parsed_input).unwrap_or_else(Error::into_compile_error);
	output_stream.into()
}
