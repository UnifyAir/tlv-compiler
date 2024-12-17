use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, LitInt};

use crate::tlv_field::get_struct_name;

pub(crate) fn tlv_tag(
	tlv_tag_stream: TokenStream,
	struct_stream: TokenStream,
) -> Result<TokenStream, Error> {
	// Parse the attribute input to extract the tag value.
	let tag_value = syn::parse2::<LitInt>(tlv_tag_stream)?;
	let tag_value = tag_value.base10_parse::<u16>()?;

	let struct_name = get_struct_name(struct_stream.clone())?;

	// Generate the implementation of the TlvTag trait
	let generated = quote! {
		impl TlvTag for #struct_name {
			const TLV_TAG: u16 = #tag_value;
		}
	};

	// Combine the original struct definition with the generated implementation
	let result = quote! {
		#struct_stream
		#generated
	};

	Ok(result)
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_tlv_tag() {
		let tag_attribute: TokenStream = quote! { 123 };
		let struct_definition: TokenStream = quote! { struct TestStruct; };
		let result = tlv_tag(tag_attribute, struct_definition);
		assert!(result.is_ok());
		let expected_output: TokenStream = quote! {
			struct TestStruct;
			impl TlvTag for TestStruct {
				const TLV_TAG: u16 = 123u16;
			}
		};
		assert_eq!(result.unwrap().to_string(), expected_output.to_string());
	}
}
