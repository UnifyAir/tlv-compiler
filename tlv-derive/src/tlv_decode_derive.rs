use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Type};
use crate::tlv_config::{get_get_bytes, TlvConfig};
use attribute_derive::Attribute;
use crate::utils::get_struct_name;



// todo add support for LV and TV type, if added re-verify

fn tag_decode(tlv_config: &TlvConfig) -> TokenStream{
	if tlv_config.tag_bytes_format == 0 {
		return quote! {
			let __actual_tag: usize = 0usize;
		};
	}
	match tlv_config.tag {
		Some(tag) => {
			let tag_bytes= tlv_config.tag_bytes_format as usize;
			quote!{
				let __actual_tag: usize = #tag;
				__bytes.advance(#tag_bytes);
			}
		}
		None => {
			let get_bytes = get_get_bytes(tlv_config.tag_bytes_format);
			quote!{
				let __actual_tag = __bytes.#get_bytes() as usize;
			}
		}
	}
}



fn length_decode(tlv_config: &TlvConfig) -> TokenStream{
	if tlv_config.length_bytes_format == 0 {
		return quote! {
			let __actual_length: usize = 0usize;
		};
	}
	match tlv_config.length {
		Some(length) => {
			let length_bytes= tlv_config.length_bytes_format as usize;
			quote!{
				let __actual_length: usize = #length;
				__bytes.advance(#length_bytes);
			}
		}
		None => {
			let get_bytes = get_get_bytes(tlv_config.length_bytes_format);
			quote!{
				let __actual_length = __bytes.#get_bytes() as usize;
			}
		}
	}
}

fn impl_tlv_decode(tlv_config: TlvConfig, struct_name: Ident) -> Result<TokenStream, Error> {

	let tag_stream = tag_decode(&tlv_config);

	let length_stream = length_decode(&tlv_config);

	let encoded_inner_stream = quote! {
		let __output = Self::decode_inner(Bytes::copy_from_slice(__bytes.chunk()), __actual_length)?;
	};

	Ok(quote! {
		impl TlvDecode for #struct_name {
			fn decode(mut __bytes: Bytes) -> Result<Self, tlv::prelude::TlvError> {
				#tag_stream
				#length_stream
				#encoded_inner_stream
				Ok(__output)
			}
		}
	})
}


fn impl_tlv_decode_inner(struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {

	let mut output_stream = Vec::<TokenStream>::new();
	let mut field_names = Vec::<Ident>::new();

	//Todo apply a check for inorder required, array, option
	//Todo apply a check for inorder first half and second half
	//Todo option is not working, check the appropriate tag and length

	for field in data_struct.fields {
		let field_name = field.ident.unwrap();
		field_names.push(field_name.clone());
		let field_type = match field.ty {
			Type::Path(type_path) => {
				type_path.path
			}
			_ => {
				abort_call_site!("Unsupported type in generic");
			}
		};
		let tlv_config= TlvConfig::from_attributes(field.attrs)?;
		let length_bytes_format = tlv_config.length_bytes_format;
		let tag_stream = tag_decode(&tlv_config);
		let length_stream = length_decode(&tlv_config);
		let header_size_bytes = (tlv_config.tag_bytes_format + tlv_config.length_bytes_format) as usize;

		output_stream.push(quote! {
			#tag_stream
			#length_stream
			Right now the splitting happens over here, try to do the spliting in the underlying function call
			as the length is already being forward, this will help in u4 parsing as we can move the cursor one byte
			backward and upon next call with length = 0, we can determine its the u4 second half.
			let #field_name = #field_type::decode_inner(__bytes.split_to(__actual_length), __actual_length)?;
		});

	}

	Ok(
		quote! {
			impl TlvDecodeInner for #struct_name {
				fn decode_inner(mut __bytes: Bytes, length: usize) -> Result<Self, tlv::prelude::TlvError> {
					#(#output_stream)*
					Ok(#struct_name{
						#(#field_names),*
					})
				}
			}
		}
	)

}


pub(crate) fn tlv_decode(token_stream: TokenStream) -> Result<TokenStream, Error> {

	let DeriveInput { attrs, data, .. } = syn::parse2(token_stream.clone())?;
	let tlv_config: Option<TlvConfig> = TlvConfig::from_attributes(attrs).ok();
	let struct_name = get_struct_name(token_stream.clone());
	let mut output_stream = Vec::<TokenStream>::new();
	match tlv_config {
		Some(tlv_config) => {
			match data {
				syn::Data::Struct(data_struct) => {
					output_stream.push(impl_tlv_decode(tlv_config, struct_name.clone())?);
					output_stream.push(impl_tlv_decode_inner(struct_name, data_struct)?);
				}
				_ => {
					abort_call_site!("Currently only struct support");
				},
			}
		}
		None => {
			match data {
				syn::Data::Struct(data_struct) => {
					output_stream.push(impl_tlv_decode_inner(struct_name, data_struct)?);
					todo!()
				}
				_ => {
					abort_call_site!("Currently only struct support");
				},
			}
		}
	};

	Ok(quote!{
		#(#output_stream)*
	})
}


