use std::collections::VecDeque;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error};
use crate::tlv_config::{get_bytes_format, TlvConfig};
use crate::tlv_field::{
	field_type_extractor_from_tag,
	get_struct_name,
	get_struct_tag,
	TlvFieldType,
};


fn impl_tlv_decode(tlv_config: TlvConfig, struct_name: Ident) -> Result<TokenStream, Error> {

	let estimated_size = tlv_config.estimated_size;
	let initialize_stream = quote! {
		let mut bytes = BytesMut::with_capacity(#estimated_size);
	};
	let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);

	let tag_stream = crate::tlv_encode_derive::tag_encode(&tlv_config);

	let fix_length_parameter_stream = crate::tlv_encode_derive::fix_length_parameter(&tlv_config);

	let length_stream = crate::tlv_encode_derive::length_encode(&tlv_config);

	let encoded_inner_stream = quote! {
		let actual_length = self.encode_inner(&mut bytes)? as #length_bytes_format;
	};

	let fix_length_stream = crate::tlv_encode_derive::fix_length_encode(&tlv_config);

	Ok(quote! {
		impl TlvEncode for #struct_name {
			fn encode(&self) -> Result<Bytes, tlv::prelude::TlvError> {
				#initialize_stream
				#tag_stream
				#fix_length_parameter_stream
				#length_stream
				#encoded_inner_stream
				#fix_length_stream
				Ok(bytes.freeze())
			}
		}
	})
}
pub(crate) fn tlv_encode(token_stream: TokenStream) -> Result<TokenStream, Error> {

	let DeriveInput { attrs, data, .. } = syn::parse2(token_stream.clone())?;
	let tlv_config: Option<TlvConfig> = TlvConfig::from_attributes(attrs).ok();
	let struct_name = get_struct_name(token_stream.clone())?;
	let mut output_stream = Vec::<TokenStream>::new();
	match tlv_config {
		Some(tlv_config) => {
			match data {
				syn::Data::Struct(data_struct) => {
					output_stream.push(crate::tlv_encode_derive::impl_tlv_decode(tlv_config, struct_name.clone())?);
					// output_stream.push(crate::tlv_encode_derive::impl_tlv_decode_inner(struct_name, data_struct)?);
				}
				_ => {
					panic!()
				},
			}
		}
		None => {
			match data {
				syn::Data::Struct(data_struct) => {
					// output_stream.push(crate::tlv_encode_derive::impl_tlv_decode_inner(struct_name, data_struct)?);
					todo!()
				}
				_ => {
					panic!()
				},
			}
		}
	};

	Ok(quote!{
		#(#output_stream)*
	})
}

pub(crate) fn tlv_decode(struct_stream: TokenStream) -> Result<TokenStream, Error> {
	let fields = field_type_extractor_from_tag(struct_stream.clone())?;
	let struct_name = get_struct_name(struct_stream.clone())?;
	let struct_name_string = struct_name.to_string();
	let mut initializer_expr = VecDeque::new();
	let mut decode_exprs = VecDeque::new();
	let mut obj_creation = VecDeque::new();
	let is_struct_tlv = get_struct_tag(struct_stream.clone())?.map_or(false, |_| true);

	for field in fields.iter() {
		let field_name = Ident::new(field.field_name.as_str(), Span::call_site());
		let field_type = Ident::new(field.inner_type.as_str(), Span::call_site());
		let init_field_name = Ident::new(
			(field.field_name.clone() + "_init").as_str(),
			Span::call_site(),
		);
		let init_expr = match field.field_type {
			TlvFieldType::Optional | TlvFieldType::Required => quote! {
				let mut #init_field_name = None;
			},
			TlvFieldType::Array => {
				quote! {
					let mut #init_field_name = Vec::new();
				}
			}
		};

		let mut decode_expr = match field.field_type {
			TlvFieldType::Optional | TlvFieldType::Required => quote! {
				if #init_field_name.is_none() {
					#init_field_name = Some(decoded_field);
				} else {
					return Err(EnDecError::GroupedTlvMultipleFields(vec![#struct_name_string.to_string()], Backtrace::force_capture()));
				}
			},
			TlvFieldType::Array => {
				quote! {
					#init_field_name.push(decoded_field);
				}
			}
		};
		let string_field_name = field.field_name.clone();
		let obj_create_expr = match field.field_type {
			TlvFieldType::Optional | TlvFieldType::Array => quote! {
				#field_name : #init_field_name
			},
			TlvFieldType::Required => quote! {
				#field_name : #init_field_name.ok_or(
					EnDecError::RequiredFieldAbsent(vec![#struct_name_string.to_string()],#string_field_name.to_string(),Backtrace::force_capture())
				)?
			},
		};
		decode_expr = quote! {
			<#field_type as TlvTag>::TLV_TAG  => {
				let decoded_field = match <#field_type as TlvDecode>::decode(tlv_data) {
					Ok(decoded_field) => decoded_field,
					Err(mut error) => {
						error.push_current_function_name(#struct_name_string.to_string());
						return Err(error);
					}
				};
				#decode_expr
			}
		};

		initializer_expr.push_back(init_expr);
		decode_exprs.push_back(decode_expr);
		obj_creation.push_back(obj_create_expr);
	}
	if fields.len() == 0 {
		return Ok(quote! {
			impl TlvDecode for #struct_name {
				fn decode(data: &[u8]) -> Result<Self> {
					Ok(#struct_name{})
				}
			}
		});
	}
	let initializer_expr = initializer_expr.into_iter();
	let decode_exprs = decode_exprs.into_iter();
	let obj_creation = obj_creation.into_iter();
	let boundary_value: usize = if is_struct_tlv { 4 } else { 0 };
	Ok(quote! {
	 		impl TlvDecode for #struct_name {
	 			fn decode(data: &[u8]) -> Result<Self> {
					#(#initializer_expr)*
	 				let mut boundary = #boundary_value;
	 				while boundary + 4 < data.len() {
	 					let tag = u16::from_be_bytes([data[boundary], data[boundary+1]]);
	 					let length = u16::from_be_bytes([data[boundary + 2], data[boundary+3]]);
						if boundary + 4 + length as usize> data.len() {
			                return Err(EnDecError::IndexOutOfRange(
			                    vec![#struct_name_string.to_string()],
    			                boundary + 4 + length as usize , data, Backtrace::force_capture())
							);
            			}
        				let tlv_data = &data[boundary..boundary + 4 + length as usize];

						match tag {
	 						#(#decode_exprs)*
	 						_ => {
	 							return	Err(EnDecError::UnknownTlvPresent(vec![#struct_name_string.to_string()], tag, tlv_data,Backtrace::force_capture()));
							}
	 					};
	 					boundary += (4 + length) as usize;
	 				}
	 				Ok(#struct_name {
	 					#(#obj_creation,)*
	 				})
	 			}
	 		}
	 	})
}

#[cfg(test)]
mod tests {
	use syn::parse_quote;

	use super::*;
	use crate::tlv_field::get_struct_tag;

	#[test]
	fn test_tlv_decode() {
		let input: TokenStream = parse_quote! {
			#[tlv_tag(247)]
			pub struct QosMonitoringReport {
				pub qfi: Option<Qfi>,
				pub qos_monitoring_measurement: Option<QosMonitoringMeasurement>,
				pub time_stamp: Vec<TimeStamp>,
			}
		};

		#[rustfmt::skip]
        let expected = quote! {
        	impl TlvDecode for QosMonitoringReport {
        		fn decode(data: &[u8]) -> Result<Self> {
        			let mut qfi_init = None;
        			let mut qos_monitoring_measurement_init = None;
        			let mut time_stamp_init = Vec::new();
        			let mut boundary = 4usize;

        			while boundary + 4 < data.len() {
        				let tag = u16::from_be_bytes([data[boundary], data[boundary + 1]]);
        				let length = u16::from_be_bytes([data[boundary + 2], data[boundary + 3]]);
						if boundary + 4 + length as usize > data . len () {
							return Err(
								EnDecError::IndexOutOfRange(
									vec!["QosMonitoringReport".to_string()],
									boundary + 4 + length as usize ,
									data,
									Backtrace::force_capture()
								)
							);
						}
        				let tlv_data = &data[boundary..boundary + 4 + length as usize];

        				match tag {
        					<Qfi as TlvTag>::TLV_TAG => {
								let decoded_field = match <Qfi as TlvDecode>::decode(tlv_data){
									Ok(decoded_field) => decoded_field,
									Err(mut error) => {
										error.push_current_function_name("QosMonitoringReport".to_string());
										return Err(error);
									}
								};
        						if qfi_init.is_none() {
        							qfi_init = Some(decoded_field);
        						} else {
        							return Err(EnDecError::GroupedTlvMultipleFields(
										vec!["QosMonitoringReport".to_string()],
        								Backtrace::force_capture()
        							));
        						}
        					}
        					<QosMonitoringMeasurement as TlvTag>::TLV_TAG => {
								let decoded_field = match <QosMonitoringMeasurement as TlvDecode>::decode(tlv_data) {
									Ok(decoded_field) => decoded_field,
									Err(mut error) => {
										error.push_current_function_name("QosMonitoringReport".to_string());
										return Err(error);
									}
								};
        						if qos_monitoring_measurement_init.is_none() {
        							qos_monitoring_measurement_init = Some(decoded_field);
        						} else {
        							return Err(EnDecError::GroupedTlvMultipleFields(
										vec!["QosMonitoringReport".to_string()],
        								Backtrace::force_capture()
        							));
        						}
        					}
        					<TimeStamp as TlvTag>::TLV_TAG => {
								let decoded_field = match <TimeStamp as TlvDecode>::decode(tlv_data){
									Ok(decoded_field) => decoded_field,
									Err(mut error) => {
										error.push_current_function_name("QosMonitoringReport".to_string());
										return Err(error);
									}
								};
        						time_stamp_init.push(decoded_field);
        					}
        					_ => {
		 						return	Err(EnDecError::UnknownTlvPresent(vec!["QosMonitoringReport".to_string()], tag, tlv_data,Backtrace::force_capture()));
        					}
        				};

        				boundary += (4 + length) as usize;
        			}

        			Ok(QosMonitoringReport {
        				qfi: qfi_init,
        				qos_monitoring_measurement: qos_monitoring_measurement_init,
        				time_stamp: time_stamp_init,
        			})
        		}
        	}
        };
		assert_eq!(
			tlv_decode(TokenStream::from(input)).unwrap().to_string(),
			expected.to_string(),
		);
	}

	#[test]
	fn test_tlv_decode_on_empty_struct() {
		let input: TokenStream = parse_quote! {
			#[derive(TlvLengthDerive, TlvEncodeDerive, TlvDecodeDerive)]
			pub struct QosMonitoringReport;
		};
		let expected_output = quote! {
			impl TlvDecode for QosMonitoringReport {
				fn decode(data: &[u8]) -> Result<Self> {
					Ok(QosMonitoringReport{})
				}
			}
		};
		assert_eq!(
			tlv_decode(TokenStream::from(input)).unwrap().to_string(),
			expected_output.to_string(),
		);
	}
}
