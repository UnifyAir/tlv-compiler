use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
	DeriveInput,
	Error,
	Fields::{Named, Unit},
	FieldsNamed,
	LitInt,
	Type,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TlvFieldType {
	Optional,
	Array,
	Required,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TlvFieldData {
	pub(crate) field_type: TlvFieldType,
	pub(crate) field_name: String,
	pub(crate) inner_type: String,
}

impl TlvFieldData {
	#[allow(dead_code)]
	pub(crate) fn get_full_type(&self) -> String {
		match self.field_type {
			TlvFieldType::Optional => format!("Option<Tlv<{}>>", self.inner_type),
			TlvFieldType::Required => format!("Tlv<{}>", self.inner_type),
			TlvFieldType::Array => format!("Vec<Tlv<{}>>", self.inner_type),
		}
	}
}

pub(crate) fn get_struct_name(struct_stream: TokenStream) -> Result<Ident, Error> {
	let input = syn::parse2::<DeriveInput>(struct_stream.clone())?;
	match input.data {
		syn::Data::Struct(_) => Ok(input.ident),
		_ => Err(Error::new_spanned(
			struct_stream.clone(),
			"tlv_tag can only be applied to structs",
		)),
	}
}

pub(crate) fn get_struct_tag(struct_stream: TokenStream) -> Result<Option<u16>, Error> {
	let input = syn::parse2::<DeriveInput>(struct_stream)?;
	let mut tlv_tag = None;
	for attr in input.attrs {
		if attr.path().to_token_stream().to_string() == "tlv_tag" {
			tlv_tag = Some(attr.parse_args::<LitInt>()?.base10_parse::<u16>()?);
		}
	}
	Ok(tlv_tag)
}

pub fn field_type_extractor_from_tag(
	input_stream: TokenStream
) -> Result<Vec<TlvFieldData>, Error> {
	// Parse the input token stream as a derive input
	let input = syn::parse2::<DeriveInput>(input_stream)?;

	// Check that the input is a struct
	let fields = if let syn::Data::Struct(data) = &input.data {
		match &data.fields {
			Named(FieldsNamed { named, .. }) => named,
			Unit => return Ok(Vec::new()),
			_ => {
				return Err(Error::new_spanned(
					input,
					"Expected a struct with named fields.",
				));
			}
		}
	} else {
		return Err(Error::new_spanned(input, "Expected a struct."));
	};

	// Collect field data
	let mut tlv_fields = Vec::new();

	for field in fields {
		let field_name = field.ident.as_ref().unwrap().to_string();
		let field_type = extract_field_type(&field.ty)?;
		let inner_type = extract_inner_type(&field.ty)?;

		tlv_fields.push(TlvFieldData {
			field_type,
			field_name,
			inner_type,
		});
	}

	Ok(tlv_fields)
}

fn extract_field_type(ty: &Type) -> Result<TlvFieldType, Error> {
	if let Type::Path(type_path) = ty {
		let last_segment = &type_path.path.segments.last().unwrap().ident;
		if last_segment == "Option" {
			Ok(TlvFieldType::Optional)
		} else if last_segment == "Vec" {
			Ok(TlvFieldType::Array)
		} else {
			Ok(TlvFieldType::Required)
		}
	} else {
		Err(Error::new_spanned(ty, "Unsupported field type"))
	}
}

fn extract_inner_type(ty: &Type) -> Result<String, Error> {
	if let Type::Path(type_path) = ty {
		let last_segment = type_path.path.segments.last().unwrap();
		if let syn::PathArguments::AngleBracketed(angle_brackets) = &last_segment.arguments {
			if let Some(syn::GenericArgument::Type(inner_type)) = angle_brackets.args.first() {
				return Ok(inner_type.to_token_stream().to_string());
			}
		}
		Ok(last_segment.ident.to_string())
	} else {
		Err(Error::new_spanned(ty, "Unsupported field type"))
	}
}

#[cfg(test)]
mod tests {
	use syn::parse_quote;

	use super::*;

	#[test]
	fn test_field_type_extractor_from_tag() {
		let input: TokenStream = parse_quote! {
			#[derive(TlvLengthDerive, TlvEncodeDerive, TlvDecodeDerive)]
			pub struct PfcpPfdManagementRequest {
				pub application_id_s_pfds: Vec<ApplicationIdSPfds>,
				pub node_id: Option<NodeId>,
				pub recovery_time_stamp: RecoveryTimeStamp,
			}
		};

		// Expected result
		let expected = vec![
			TlvFieldData {
				field_type: TlvFieldType::Array,
				field_name: "application_id_s_pfds".to_string(),
				inner_type: "ApplicationIdSPfds".to_string(),
			},
			TlvFieldData {
				field_type: TlvFieldType::Optional,
				field_name: "node_id".to_string(),
				inner_type: "NodeId".to_string(),
			},
			TlvFieldData {
				field_type: TlvFieldType::Required,
				field_name: "recovery_time_stamp".to_string(),
				inner_type: "RecoveryTimeStamp".to_string(),
			},
		];

		let result = field_type_extractor_from_tag(input);

		assert!(result.is_ok());
		let actual = result.unwrap();
		assert_eq!(
			actual.len(),
			expected.len(),
			"Number of fields do not match"
		);

		// Compare each field data
		for (actual_field, expected_field) in actual.iter().zip(expected.iter()) {
			assert_eq!(
				actual_field.field_type, expected_field.field_type,
				"Field type mismatch"
			);
			assert_eq!(
				actual_field.field_name, expected_field.field_name,
				"Field name mismatch"
			);
			assert_eq!(
				actual_field.inner_type, expected_field.inner_type,
				"Inner type mismatch"
			);
		}
	}
}
