
pub use std::io::Write;
use bytes::{Buf, BufMut, BytesMut, Bytes};
use thiserror::Error;
use crate::prelude::TlvError::InCompleteByteInsertion;
// // #[derive(Error, Debug)]
// pub enum EnDecError<'a> {
// 	#[error("Io Error: {0}")]
// 	IoError(
// 		#[from]
// 		std::io::Error,
// 	),
//
// 	#[error("Encode error for {0}: {1}")]
// 	EncodeError(u16, String),
//
// 	#[error("Decode error for {0}: {1}")]
// 	DecodeError(u16, String),
//
// 	#[error("New Object Creation Error for {0}: {1}")]
// 	NewError(u16, String),
//
// 	#[error("Decode error for grouped tlv")]
// 	GroupedTlvMultipleFields(Vec<String>),
//
// 	#[error("Required field not specified: {1}")]
// 	RequiredFieldAbsent(Vec<String>, String),
//
// 	#[error("Required field not specified")]
// 	UnknownTlvPresent(Vec<String>, u16, &'a [u8]),
//
// 	#[error("Expected Boundary exceeded data: boundary - {1}")]
// 	IndexOutOfRange(Vec<String>, usize, &'a [u8]),
// }

// impl EnDecError<'_> {
// 	pub fn push_current_function_name(
// 		&mut self,
// 		name: String,
// 	) {
// 		match self {
// 			EnDecError::GroupedTlvMultipleFields(inner, ..)
// 			| EnDecError::RequiredFieldAbsent(inner, ..)
// 			| EnDecError::UnknownTlvPresent(inner, ..) => {
// 				inner.push(name);
// 			}
// 			_ => (),
// 		};
// 	}
// }

// pub(crate) fn tlv_encode_field<T, W>(
// 	field: &T,
// 	writer: &mut W,
// ) -> Result<(), EnDecError<'static>>
// where
// 	T: TlvEncode + TlvLength + TlvTag,
// 	W: io::Write,
// {
// 	let tag = <T as TlvTag>::tag_type();
// 	let length = TlvLength::length(field);
// 	writer.write_all(tag.to_be_bytes().as_ref())?;
// 	writer.write_all(length.to_be_bytes().as_ref())?;
// 	TlvEncode::encode(field, writer)?;
// 	Ok(())
// }


#[derive(Error, Debug)]
pub enum TlvError {
	#[error("unknown error")]
	Unknown,
	#[error("Incomplete byte exist probabily because last insertion was u4 without any spare or remaining u4")]
	InCompleteByteInsertion
}

pub trait TlvEncode {
	fn encode(
		&self
	) -> Result<Bytes, TlvError>;
}

pub trait TlvEncodeInner {
	fn encode_inner(
		&self,
		bytes: &mut BytesMut,
	) -> Result<usize, TlvError>;
}

pub trait TlvDecode: Sized {
	fn decode(bytes: Bytes) -> Result<Self, TlvError>;
}


pub trait TlvDecodeInner: Sized {
	fn decode_inner(bytes: Bytes, length: usize) -> Result<Self, TlvError>;
}

impl TlvEncodeInner for u8{
	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
		bytes.put_u8(self.to_be());
		Ok(1usize)
	}
}

impl<T> TlvEncodeInner for Option<T>
where T: TlvEncodeInner {
	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
		match &self {
			Some(inner) => inner.encode_inner(bytes),
			None => Ok(0usize)
		}
	}
}

impl<T> TlvEncodeInner for Vec<T>
where T: TlvEncodeInner {
	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
		let mut total_encoded = 0usize;
		for item in self {
			total_encoded += item.encode_inner(bytes)?;
		}
		Ok(total_encoded)
	}
}


impl TlvDecodeInner for u8{
	fn decode_inner(mut bytes: Bytes, length: usize) -> Result<Self, TlvError> {
		Ok(bytes.get_u8())
	}
}

impl<T> TlvDecodeInner for Option<T>
where T: TlvDecodeInner {
	fn decode_inner(mut bytes: Bytes, length: usize) -> Result<Self, TlvError> {
		if length > 0 {
			return Ok(Some(T::decode_inner(bytes.split_to(length), length)?));
		}
		Ok(None)
	}
}

// impl<T> TlvDecodeInner for Vec<T>
// where T: TlvDecodeInner {
// 	fn decode_inner(bytes: &[u8], length: usize) -> Result<Self, TlvError> {
// 		let mut output = Vec::<T>::with_capacity(length);
// 		output.push(T::decode_inner())
// 		Ok(Vec::from(bytes))
// 	}
// }


// pub enum u4{
// 	FirstHalf(u8),
// 	SecondHalf(u8)
// }
// impl TlvEncodeInner for u4{
// 	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
// 		match &self {
// 			u4::FirstHalf(ref byte) => {
// 				bytes.put_u8(byte << 4);
// 				Err(InCompleteByteInsertion)
// 			}
// 			u4::SecondHalf(ref byte) => {
// 				let index = bytes.len();
// 				bytes[index-1..index].copy_from_slice((&byte>>4).to_be_bytes());
// 				Ok(1usize)
// 			}
// 		}
// 	}
// }










// ====================================================================================

// #[derive(TlvEncode)]
// #[tlv_config(tag=132, type=tluuv, t=8, l=8)]
// pub struct MyIE{
// 	#[tlv_config(length_type=2_byte, length=7, tag_lenth=2_byte, tag=123, value_type=4_bit)]
//     pub b: MyIE3,
// 	#[tlv_config(length_type=2_byte, length=7, tag_type=2_byte, tag=123)]
//     pub c: Vec<u8>,
// 	#[tlv_config(length_type=2_byte, length=7, tag_type=2_byte, tag=123)]
//     pub a: Option<MyIE2>,
// }



// #[derive(FromAttr, Debug)]
// #[attribute(ident = ident, aliases = [a, b])]
// #[attribute(error(
//     unknown_field = "expected one of {expected_fields:i(`{}`)(, )}",
//     duplicate_field = "duplicate `{field}`",
//     missing_field = "missing field `{field}`",
//     field_help = "try {attribute}: {field}={example}",
//     conflict = "{first} !!! {second}"
// ))]
// struct Custom {
//     optional_implicit: Option<Block>,
//     #[attribute(optional)]
//     optional_explicit: u8,
//     #[attribute(optional, default = 2 * 5)]
//     optional_default: u8,
//     #[attribute(default = 33)]
//     default: u8,
//     #[attribute(conflicts = [conflict_b])]
//     conflict_a: Option<String>,
//     conflict_b: Option<String>,
//     #[attribute(example = "2.5")]
//     example: f32,
//     flag: bool,
// }
// pub struct MyIE2{
//     pub a: u8,
//     pub b: u16,
//     pub c: Vec<u8>,
// }

// #[tlv_type(t=8, l=8)]
// pub struct MyIE3{
//     #[value_type(v=v8)]
//     pub a: [u8],
// }
// #[tag_type = u8]
// pub struct myu8(u8);


// #[derive(Custom)]
// pub struct Tester{
// 	#[loda(default=123)]
// 	pub a: u8,
// }