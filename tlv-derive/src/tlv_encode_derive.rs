use std::collections::VecDeque;

use proc_macro2::{Ident, Span, TokenStream};
use attribute_derive::__private::proc_macro2;
use attribute_derive::Attribute;
use quote::quote;
use syn::{DataStruct, Error };
use syn::{ DeriveInput };
use crate::tlv_config;
use crate::tlv_config::{TlvConfig, get_bytes_format} ;

use crate::tlv_field::{field_type_extractor_from_tag, get_struct_name, TlvFieldType};

fn tag_encode(tlv_config: &TlvConfig) -> TokenStream{
	match tlv_config.tag {
		Some(tag) => {
			let tag_bytes_format = get_bytes_format(tlv_config.tag_bytes_format);
			quote!{
				let tag = #tag as #tag_bytes_format;
				bytes.put(tag);
			}
		}
		None => {
			quote!{}
		}
	}
}

fn length_encode(tlv_config: &TlvConfig) -> TokenStream{
	match tlv_config.length {
		Some(length) => {
			let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
			quote!{
				let length = #length as #length_bytes_format;
				bytes.put(length);
			}
		}
		None => {
			let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
			quote!{
				let length_buf = 0u8 as #length_bytes_format;
				bytes.put(length_buf);
			}
		}
	}
}

fn fix_length_parameter(tlv_config: &TlvConfig) -> TokenStream {
	match tlv_config.length {
		Some(_) => {
			quote!{}
		}
		None => {
			quote! {
				let fix_length_index = bytes.len();
			}
		}
	}
}
fn fix_length_encode(tlv_config: &TlvConfig) -> TokenStream {
	match tlv_config.length {
		Some(_) => {
			quote!{}
		}
		None => {
			quote! {
				bytes[fix_length_index..fix_length_index + tlv_config.length_bytes_format].copy_from_slice(&length.to_be_bytes());
			}
		}
	}
}


fn impl_tlv_encode(tlv_config: TlvConfig, struct_name: Ident) -> Result<TokenStream, Error> {

	let estimated_size = tlv_config.estimated_size;
	let initialize_stream = quote! {
		let mut bytes = BytesMut::with_capacity(#estimated_size);
	};

	let tag_stream = tag_encode(&tlv_config);

	let fix_length_parameter_stream = fix_length_parameter(&tlv_config);

	let length_stream = length_encode(&tlv_config);

	let encoded_inner_stream = quote! {
		let length = self.encode_inner(&mut bytes)?;
	};

	let fix_length_stream = fix_length_encode(&tlv_config);

	Ok(quote! {
		impl TlvEncode for #struct_name {
			fn encode(&self) -> Result<Bytes, ()> {
				#initialize_stream
				#tag_stream
				#fix_length_parameter_stream
				#length_stream
				#encoded_inner_stream
				#fix_length_stream
				Ok(bytes)
			}
		}
	})
}


fn impl_tlv_encode_inner (struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {

	let mut output_stream = Vec::<TokenStream>::new();

	let initialize_stream = quote! {
		let mut total_length:usize = 0;
	};
	for field in data_struct.fields {
		let field_name = field.ident.unwrap();
		let tlv_config= TlvConfig::from_attributes(field.attrs)?;
		let tag_stream = tag_encode(&tlv_config);
		let fix_length_parameter_stream = fix_length_parameter(&tlv_config);
		let length_stream = length_encode(&tlv_config);
		let header_size_bytes = tlv_config.tag_bytes_format + tlv_config.length_bytes_format;
		let fix_length_stream = fix_length_encode(&tlv_config);
		output_stream.push(quote! {
			#tag_stream
			#fix_length_parameter_stream
			#length_stream
			total_length += #header_size_bytes as usize;
			total_length += &self.#field_name.encode_inner(bytes);
			#fix_length_stream
		});
	}

	Ok(
		quote! {
			impl TlvEncodeInner for #struct_name {
				fn encode_inner(&self, buffer: &mut Bytes) -> Result<usize> {
					#initialize_stream
					#(#output_stream)*
					total_length
				}
			}
		}
	)

}

// for field in fields {
// 		let field_name = Ident::new(field.field_name.as_str(), Span::call_site());
// 		let encode_expr = match field.field_type {
// 			TlvFieldType::Optional => quote! {
// 				self.#field_name.as_ref().map_or(Ok(()), |field| tlv_encode_field(field, writer))?;
// 			},
// 			TlvFieldType::Required => quote! {
// 				tlv_encode_field(&self.#field_name, writer)?;
// 			},
// 			TlvFieldType::Array => {
// 				quote! {
// 					for field in self.#field_name.iter() {
// 						tlv_encode_field(field, writer)?;
// 					}
// 				}
// 			}
// 		};
// 		field_length.push_back(encode_expr);
// 	}
// 	let last = field_length.pop_back().unwrap_or(quote! {});
// 	let fields = field_length.iter();
// 	let final_token_stream = quote! {#(
// 		#fields
// 	)* #last};
//
// 				Ok(())
// 			}
// 		}
// }

pub(crate) fn tlv_encode(token_stream: TokenStream) -> Result<TokenStream, Error> {

	let DeriveInput { attrs, data, .. } = syn::parse2(token_stream.clone())?;
	let tlv_config: Option<TlvConfig> = TlvConfig::from_attributes(attrs).ok();
	let struct_name = get_struct_name(token_stream.clone())?;
	let mut output_stream = Vec::<TokenStream>::new();
	match tlv_config {
		Some(tlv_config) => {
			match data {
				syn::Data::Struct(data_struct) => {
					output_stream.push(impl_tlv_encode(tlv_config, struct_name.clone())?);
					output_stream.push(impl_tlv_encode_inner(struct_name, data_struct)?);
				}
				_ => {
					panic!()
				},
			}
		}
		None => {
			// implement encode inner
			todo!()
		}
	};

	Ok(quote!{
		#(#output_stream)*
	})
	// let fields = field_type_extractor_from_tag(token_stream.clone())?;
	// let struct_name = get_struct_name(struct_stream.clone())?;
	// let mut field_length = VecDeque::new();

	// for field in fields {
	// 	let field_name = Ident::new(field.field_name.as_str(), Span::call_site());
	// 	let encode_expr = match field.field_type {
	// 		TlvFieldType::Optional => quote! {
	// 			self.#field_name.as_ref().map_or(Ok(()), |field| tlv_encode_field(field, writer))?;
	// 		},
	// 		TlvFieldType::Required => quote! {
	// 			tlv_encode_field(&self.#field_name, writer)?;
	// 		},
	// 		TlvFieldType::Array => {
	// 			quote! {
	// 				for field in self.#field_name.iter() {
	// 					tlv_encode_field(field, writer)?;
	// 				}
	// 			}
	// 		}
	// 	};
	// 	field_length.push_back(encode_expr);
	// }
	// let last = field_length.pop_back().unwrap_or(quote! {});
	// let fields = field_length.iter();
	// let field_length_expr = quote! {#(
	// 	#fields
	// )* #last};

}





// #[proc_macro_derive(Custom, attributes(ident, a, b, empty, single))]
// pub fn custom_derive(input: TokenStream) -> proc_macro::TokenStream {
//     let mut tokens =
//         all_attrs::<Custom>(input.clone()).unwrap_or_else(|e| e.to_compile_error().into());
//     tokens.extend(
//         all_attrs::<EmptyCustom>(input.clone()).unwrap_or_else(|e| e.to_compile_error().into()),
//     );
//     tokens.extend(all_attrs::<SingleCustom>(input).unwrap_or_else(|e| e.to_compile_error().into()));
//     tokens
// }

// fn all_attrs<T: FromAttr + AttributeIdent>(input: TokenStream) -> Result<TokenStream> {
//     let DeriveInput { attrs, data, .. } = syn::parse(input)?;
//     T::from_attributes(&attrs)?;
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





// #[cfg(test)]
// mod tests {
// 	use syn::parse_quote;

// 	use super::*;
// 	use crate::tlv_field::get_struct_tag;
// 	#[test]
// 	fn test_tlv_encode() {
// 		let input: TokenStream = parse_quote! {
// 			#[tlv_tag(247)]
// 			pub struct QosMonitoringReport {
// 				pub qfi: Option<Qfi>,
// 				pub qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
// 				pub time_stamp: Vec<TimeStamp>,
// 			}
// 		};

// 		#[rustfmt::skip]
// 		let expected: TokenStream = parse_quote! {
// 			impl TlvEncode for QosMonitoringReport {
//     			fn encode(&self, writer: &mut impl Write) -> Result<'static, ()> {
// 					self.qfi
// 						.as_ref()
// 						.map_or(Ok(()), |field| tlv_encode_field(field, writer))?;

// 					self.qos_monitoring_measurement
// 						.as_ref()
// 						.map_or(Ok(()), |field| tlv_encode_field(field, writer))?;

// 					for field in self.time_stamp.iter() {
// 						tlv_encode_field(field, writer)?;
// 					}
// 					Ok(())
// 				}
//     		}
// 		};

// 		let tag = get_struct_tag(input.clone()).unwrap();
// 		println!("struct tag: {:?}", tag);
// 		assert_eq!(
// 			tlv_encode(TokenStream::from(input)).unwrap().to_string(),
// 			expected.to_string(),
// 		);
// 	}

// 	#[test]
// 	fn test_tlv_encode_on_empty_struct() {
// 		let input: TokenStream = parse_quote! {
// 			#[derive(TlvLengthDerive, TlvEncodeDerive, TlvDecodeDerive)]
// 			pub struct QosMonitoringReport;
// 		};
// 		let expected_output = quote! {
// 			impl TlvEncode for QosMonitoringReport {
// 				fn encode(&self, writer: &mut impl Write) -> Result<'static, ()> {
// 					Ok(())
// 				}
// 			}
// 		};
// 		assert_eq!(
// 			tlv_encode(TokenStream::from(input)).unwrap().to_string(),
// 			expected_output.to_string(),
// 		);
// 	}
// }
