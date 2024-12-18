use proc_macro2::{Ident, TokenStream};
use attribute_derive::__private::proc_macro2;
use attribute_derive::Attribute;
use quote::quote;
use syn::{DataStruct, Error };
use syn::{ DeriveInput };
use crate::tlv_config::{TlvConfig, get_bytes_format, get_put_bytes} ;

use crate::tlv_field:: { get_struct_name };
// todo add support for LV and TV type, if added re-verify
fn tag_encode(tlv_config: &TlvConfig) -> TokenStream{
	match tlv_config.tag {
		Some(tag) => {
			let tag_bytes_format = get_bytes_format(tlv_config.tag_bytes_format);
			let put_bytes = get_put_bytes(tlv_config.tag_bytes_format);
			quote!{
				let tag = #tag as #tag_bytes_format;
				bytes.#put_bytes(tag);
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
			let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
			quote!{
				let length = #length as #length_bytes_format;
				bytes.#put_bytes(length);
			}
		}
		None => {
			let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
			let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
			quote!{
				let length_buf = 0u8 as #length_bytes_format;
				bytes.#put_bytes(length_buf);
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
			let length_bytes_format = tlv_config.length_bytes_format;
			quote! {
				bytes[fix_length_index..fix_length_index + #length_bytes_format as usize].copy_from_slice(&actual_length.to_be_bytes());
			}
		}
	}
}


fn impl_tlv_encode(tlv_config: TlvConfig, struct_name: Ident) -> Result<TokenStream, Error> {

	let estimated_size = tlv_config.estimated_size;
	let initialize_stream = quote! {
		let mut bytes = BytesMut::with_capacity(#estimated_size);
	};
	let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);

	let tag_stream = tag_encode(&tlv_config);

	let fix_length_parameter_stream = fix_length_parameter(&tlv_config);

	let length_stream = length_encode(&tlv_config);

	let encoded_inner_stream = quote! {
		let actual_length = self.encode_inner(&mut bytes)? as #length_bytes_format;
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
				Ok(bytes.freeze())
			}
		}
	})
}


fn impl_tlv_encode_inner (struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {

	let mut output_stream = Vec::<TokenStream>::new();

	let initialize_stream = quote! {
		let mut total_length:usize = 0;
	};

	//Todo apply a check for required, array, option
	for field in data_struct.fields {
		let field_name = field.ident.unwrap();
		let tlv_config= TlvConfig::from_attributes(field.attrs)?;
		let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
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
			let actual_length = self.#field_name.encode_inner(bytes)? as #length_bytes_format;
			total_length += actual_length as usize;
			#fix_length_stream
		});

	}

	Ok(
		quote! {
			impl TlvEncodeInner for #struct_name {
				fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, tlv::prelude::TlvError> {
					#initialize_stream
					#(#output_stream)*
					Ok(total_length)
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






