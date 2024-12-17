use std::collections::VecDeque;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Error;

use crate::tlv_field::{field_type_extractor_from_tag, get_struct_name, TlvFieldType};

pub(crate) fn tlv_length(struct_stream: TokenStream) -> Result<TokenStream, Error> {
	let fields = field_type_extractor_from_tag(struct_stream.clone())?;
	let struct_name = get_struct_name(struct_stream.clone())?;
	let mut field_length = VecDeque::new();
	for field in fields {
		let field_name = Ident::new(field.field_name.as_str(), Span::call_site());
		let length_expr = match field.field_type {
			TlvFieldType::Optional => quote! {
				self.#field_name.as_ref().map_or(0, |data| TlvLength::length(data) as u16 + 4u16)
			},
			TlvFieldType::Required => quote! {
				TlvLength::length(&self.#field_name) + 4u16
			},
			TlvFieldType::Array => {
				quote! {self.#field_name.iter().map(|field|TlvLength::length(field) as u16 + 4u16).reduce(|len, field_len| len + field_len).unwrap_or(0u16)}
			}
		};
		field_length.push_back(length_expr);
	}
	let last = field_length.pop_back().unwrap_or(quote! {0u16});
	let fields = field_length.iter();
	let field_length_expr = quote! {#(#fields + )* #last};
	Ok(quote! {
		impl TlvLength for #struct_name {
			#[inline]
			fn length(&self) -> u16 {
				#field_length_expr
			}
		}
	})
}

#[cfg(test)]
mod tests {
	use syn::parse_quote;

	use super::*;
	#[test]
	fn test_tlv_length() {
		let input: TokenStream = parse_quote! {
			#[derive(TlvLengthDerive, TlvEncodeDerive, TlvDecodeDerive)]
			pub struct PfcpPfdManagementRequest {
				pub application_id_s_pfds: Vec<ApplicationIdSPfds>,
				pub node_id: Option<NodeId>,
				pub recovery_time_stamp: RecoveryTimeStamp,
			}
		};
		let expected_output = quote! {
			impl TlvLength for PfcpPfdManagementRequest {
				#[inline]
				fn length(&self) -> u16 {
					self
						.application_id_s_pfds
						.iter()
						.map(|field| TlvLength::length(field) as u16 + 4u16)
						.reduce(|len, field_len| len + field_len)
						.unwrap_or(0u16)
						+ self.node_id.as_ref().map_or(0, |data| TlvLength::length(data) as u16 + 4u16)
					+ TlvLength::length(&self.recovery_time_stamp) + 4u16
				}
			}
		};
		assert_eq!(
			tlv_length(input).unwrap().to_string(),
			expected_output.to_string()
		);
	}

	#[test]
	fn test_length_on_empty_struct() {
		let input: TokenStream = parse_quote! {
			#[derive(TlvLengthDerive, TlvEncodeDerive, TlvDecodeDerive)]
			pub struct PfcpPfdManagementRequest;
		};
		let expected_output = quote! {
			impl TlvLength for PfcpPfdManagementRequest {
				#[inline]
				fn length(&self) -> u16 {
					0u16
				}
			}
		};
		assert_eq!(
			tlv_length(input).unwrap().to_string(),
			expected_output.to_string()
		);
	}
}
