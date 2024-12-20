use attribute_derive::{Attribute, FromAttr};
use proc_macro::Span;
use syn::Ident;

#[derive(FromAttr, Debug)]
#[attribute(ident = tlv_config)]
#[attribute(error(
    unknown_field = "expected one of {expected_fields:i(`{}`)(, )}",
    duplicate_field = "duplicate `{field}`",
    missing_field = "missing field `{field}`",
    field_help = "try {attribute}: {field}={example}",
    conflict = "{first} !!! {second}"
))]
pub struct TlvConfig{
    #[attribute(optional, default = 1024)]
    pub(crate) estimated_size: usize,
	pub(crate) tag: Option<usize>,
    #[attribute(optional, default = 1)]
	pub(crate) tag_bytes_format: u8,
    #[attribute(optional, default = Some(1))]
	pub(crate) length: Option<usize>,
    #[attribute(optional, default = 1)]
	pub(crate) length_bytes_format: u8,
	pub(crate) value: Option<usize>,
    #[attribute(optional, default = 8)]
	pub(crate) value_bits_format: i8,
}


pub(crate) fn get_bytes_format(bytes_format: u8) -> Ident {
    match bytes_format {
        1 => {
            Ident::new("u8", Span::call_site().into())
        }
        2 => {
            Ident::new("u16", Span::call_site().into())
        }
        4 => {
            Ident::new("u32", Span::call_site().into())
        }
        8 => {
            Ident::new("u64", Span::call_site().into())
        }
        16 => {
            Ident::new("u128", Span::call_site().into())
        }
        _ => {
            panic!("Invalid tag_bytes_format")
        }
    }
}


pub(crate) fn get_put_bytes(bytes_format: u8) -> Ident {
    match bytes_format {
        1 => {
            Ident::new("put_u8", Span::call_site().into())
        }
        2 => {
            Ident::new("put_u16", Span::call_site().into())
        }
        4 => {
            Ident::new("put_u32", Span::call_site().into())
        }
        8 => {
            Ident::new("put_u64", Span::call_site().into())
        }
        16 => {
            Ident::new("put_u128", Span::call_site().into())
        }
        _ => {
            panic!("Invalid tag_bytes_format")
        }
    }
}

pub(crate) fn get_get_bytes(bytes_format: u8) -> Ident {
    match bytes_format {
        1 => {
            Ident::new("get_u8", Span::call_site().into())
        }
        2 => {
            Ident::new("get_u16", Span::call_site().into())
        }
        4 => {
            Ident::new("get_u32", Span::call_site().into())
        }
        8 => {
            Ident::new("get_u64", Span::call_site().into())
        }
        16 => {
            Ident::new("get_u128", Span::call_site().into())
        }
        _ => {
            panic!("Invalid tag_bytes_format")
        }
    }
}