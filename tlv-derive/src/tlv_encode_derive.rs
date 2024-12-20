use proc_macro2::{Ident, TokenStream};
use attribute_derive::__private::proc_macro2;
use attribute_derive::Attribute;
use quote::quote;
use syn::{DataStruct, Error };
use syn::{ DeriveInput };
use crate::tlv_config::{TlvConfig, get_bytes_format, get_put_bytes};

use crate::tlv_field:: { get_struct_name };
use crate::utils::is_u4_type;
// todo add support for LV and TV type, if added re-verify

fn tag_encode(tlv_config: &TlvConfig) -> TokenStream{
	if tlv_config.tag_bytes_format == 0 {
		return quote! {};
	}
	match tlv_config.tag {
		Some(tag) => {
			let tag_bytes_format = get_bytes_format(tlv_config.tag_bytes_format);
			let put_bytes = get_put_bytes(tlv_config.tag_bytes_format);
			quote!{
				let __tag: #tag_bytes_format = #tag as #tag_bytes_format;
				__bytes.#put_bytes(__tag);
			}
		}
		None => {
			quote!{}
		}
	}
}

fn length_encode(tlv_config: &TlvConfig) -> TokenStream{
	if tlv_config.length_bytes_format == 0 {
		return quote! {};
	}
	match tlv_config.length {
		Some(length) => {
			let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
			let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
			quote!{
				const __length: #length_bytes_format = #length as #length_bytes_format;
				__bytes.#put_bytes(__length);
			}
		}
		None => {
			let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
			let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
			quote!{
				let __length_buf: #length_bytes_format = 0u8 as #length_bytes_format;
				__bytes.#put_bytes(__length_buf);
			}
		}
	}
}

fn fix_length_parameter(tlv_config: &TlvConfig) -> TokenStream {
	if tlv_config.length_bytes_format == 0 {
		return quote! {};
	}
	match tlv_config.length {
		Some(_) => {
			quote!{}
		}
		None => {
			quote! {
				let __fix_length_index = __bytes.len();
			}
		}
	}
}

fn fix_length_encode(tlv_config: &TlvConfig) -> TokenStream {
	if tlv_config.length_bytes_format == 0 {
		return quote! {};
	}
	match tlv_config.length {
		Some(_) => {
			quote!{}
		}
		None => {
			let length_bytes_format = tlv_config.length_bytes_format;
			let bytes_format = get_bytes_format(length_bytes_format);
			quote! {
				let __fix_length = __actual_length as #bytes_format;
				__bytes[__fix_length_index..__fix_length_index + #length_bytes_format as usize].copy_from_slice(&__fix_length.to_be_bytes());
			}
		}
	}
}

// fn fix_value_parameter(tlv_config: &TlvConfig) -> TokenStream{
// 	match tlv_config.value_bits_format{
// 		8 => quote! {}
// 		_ => {
// 			quote! {
// 				let mut result: Result<usize, tlv::prelude::TlvError>  = Err(TlvError::InCompleteByteInsertion);
// 			}
// 		}
// 	}
// }
//
// fn fix_value_encode(tlv_config: &TlvConfig) -> TokenStream{
// 	match tlv_config.value_bits_format{
// 		-4 => {
// 			quote! {
// 				result
// 			}
// 		}
// 		4 => {
// 			quote! {
// 				result = Ok()
// 			}
// 		}
// 		_ => quote! {},
// 	}
// }


fn impl_tlv_encode(tlv_config: TlvConfig, struct_name: Ident) -> Result<TokenStream, Error> {

	let estimated_size = tlv_config.estimated_size;
	let initialize_stream = quote! {
		let mut __bytes = BytesMut::with_capacity(#estimated_size);
	};

	let tag_stream = tag_encode(&tlv_config);

	let fix_length_parameter_stream = fix_length_parameter(&tlv_config);

	let length_stream = length_encode(&tlv_config);

	let encoded_inner_stream = quote! {
		let __actual_length = self.encode_inner(&mut __bytes)?;
	};

	let fix_length_stream = fix_length_encode(&tlv_config);

	Ok(quote! {
		impl TlvEncode for #struct_name {
			fn encode(&self) -> Result<Bytes, tlv::prelude::TlvError> {
				#initialize_stream
				#tag_stream
				#fix_length_parameter_stream
				#length_stream
				#encoded_inner_stream
				#fix_length_stream
				Ok(__bytes.freeze())
			}
		}
	})
}


fn impl_tlv_encode_inner (struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {

	let mut output_stream = Vec::<TokenStream>::new();

	let initialize_stream = quote! {
		let mut __total_length:usize = 0;
	};

	//Todo apply a check for inorder required, array, option
	//Todo apply a check for inorder first half and second half

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
			__total_length += #header_size_bytes as usize;
			let __actual_length = self.#field_name.encode_inner(__bytes)?;
			__total_length += __actual_length as usize;
			#fix_length_stream
		});

	}

	Ok(
		quote! {
			impl TlvEncodeInner for #struct_name {
				fn encode_inner(&self, __bytes: &mut BytesMut) -> Result<usize, tlv::prelude::TlvError> {
					#initialize_stream
					#(#output_stream)*
					Ok(__total_length)
				}
			}
		}
	)

}

pub(crate) fn tlv_encode(token_stream: TokenStream) -> Result<TokenStream, Error> {

	let DeriveInput { attrs, data, .. } = syn::parse2(token_stream.clone())?;
	let tlv_config: Option<TlvConfig> = TlvConfig::from_attributes(attrs).ok();
	let struct_name = get_struct_name(token_stream.clone())?;
	let mut output_stream = Vec::<TokenStream>::new();
	// todo tlv config is disregarded and this match always goes into some.
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
			match data {
				syn::Data::Struct(data_struct) => {
					output_stream.push(impl_tlv_encode_inner(struct_name, data_struct)?);
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






