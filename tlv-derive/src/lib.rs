// #![feature(trace_macros)]

// trace_macros!(true);

mod tlv_decode_derive;
mod tlv_encode_derive;
mod tlv_field;
mod tlv_length_derive;
mod tlv_tag_derive;
mod tlv_config;

use proc_macro::TokenStream;
use syn::Error;
use tlv_decode_derive::tlv_decode;
use tlv_encode_derive::tlv_encode;
use tlv_length_derive::tlv_length;
use quote::quote;

use std::fmt::Debug;

use attribute_derive::{AttributeIdent, FromAttr};
use attribute_derive::__private::proc_macro2;
use syn::{Block, DeriveInput, Result};


// #[proc_macro_derive(TlvLength)]
// pub fn tlv_length_derive(input: TokenStream) -> TokenStream {
// 	let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
// 	let output_stream = tlv_length(parsed_input).unwrap_or_else(Error::into_compile_error);
// 	output_stream.into()
// }

#[proc_macro_derive(TlvEncode, attributes(tlv_config))]
pub fn tlv_encode_derive(input: TokenStream) -> TokenStream {
	let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
	let output_stream: proc_macro2::TokenStream = tlv_encode(parsed_input).unwrap_or_else(Error::into_compile_error);
	output_stream.into()
}

#[proc_macro_derive(TlvDecode)]
pub fn tlv_decode_derive(input: TokenStream) -> TokenStream {
	let parsed_input: proc_macro2::TokenStream = syn::parse_macro_input!(input);
	let output_stream = tlv_decode(parsed_input).unwrap_or_else(Error::into_compile_error);
	output_stream.into()
}

// #[proc_macro_attribute]
// pub fn tlv_tag(
// 	attribute: TokenStream,
// 	implementation: TokenStream,
// ) -> TokenStream {
// 	let attribute = syn::parse_macro_input!(attribute);
// 	let implementation = syn::parse_macro_input!(implementation);
// 	let output_stream = tlv_tag_derive::tlv_tag(attribute, implementation)
// 		.unwrap_or_else(Error::into_compile_error);
// 	output_stream.into()
// }

// #[derive(FromAttr, Debug)]
// #[attribute(ident = loda)]
// #[attribute(error(
//     unknown_field = "expected one of {expected_fields:i(`{}`)(, )}",
//     duplicate_field = "duplicate `{field}`",
//     missing_field = "missing field `{field}`",
//     field_help = "try {attribute}: {field}={example}",
//     conflict = "{first} !!! {second}"
// ))]
// struct Loda  {
//     #[attribute(default = 33)]
//     default: u8,
// }
// #[derive(FromAttr)]
// #[attribute(ident = empty, error(unknown_field_empty = "expected nothing"))]
// struct EmptyCustom {}
//
// #[derive(FromAttr)]
// #[attribute(ident = single, error(unknown_field_single = "expected {expected_field}"))]
// struct SingleCustom {
//     field: bool,
// }
//
// #[proc_macro_derive(Custom, attributes(loda, empty, single))]
// pub fn custom_derive(input: TokenStream) -> TokenStream {
//     let mut tokens =
//         all_attrs::<Loda>(input.clone()).unwrap_or_else(|e| e.to_compile_error().into());
//     tokens.extend(
//         all_attrs::<EmptyCustom>(input.clone()).unwrap_or_else(|e| e.to_compile_error().into()),
//     );
//     tokens.extend(all_attrs::<SingleCustom>(input).unwrap_or_else(|e| e.to_compile_error().into()));
//     tokens
// }
//
// fn all_attrs<T: FromAttr + AttributeIdent>(input: TokenStream) -> Result<TokenStream> {
//     let DeriveInput { attrs, data, .. } = syn::parse(input)?;
//     let attrubyte = T::from_attributes(&attrs).unwrap();
//     match data {
//         syn::Data::Struct(data) => {
//             for field in data.fields {
//                 T::from_attributes(&field.attrs)?;
//             }
//         }
//         syn::Data::Enum(data) => {
//             for variant in data.variants {
//                 T::from_attributes(&variant.attrs)?;
//                 for field in variant.fields {
//                     T::from_attributes(&field.attrs)?;
//                 }
//             }
//         }
//         syn::Data::Union(data) => {
//             for field in data.fields.named {
//                 T::from_attributes(&field.attrs)?;
//             }
//         }
//     }
//     Ok(TokenStream::new())
// }