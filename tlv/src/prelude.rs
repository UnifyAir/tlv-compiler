pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use std::io::Write;
use std::usize;
use thiserror::Error;

//Todo do some better error handling.
#[derive(Error, Debug)]
pub enum TlvError {
    #[error("unknown error")]
    Unknown,
    #[error("Incomplete byte exist probabily because last insertion was u4 without any spare or remaining u4")]
    InCompleteByteInsertion,
}

pub trait TlvEncode {
    fn encode(&self, bytes: &mut BytesMut) -> Result<usize, TlvError>;
}

// pub trait TlvEncodeInner {
// 	fn encode_inner(
// 		&self,
// 		bytes: &mut BytesMut,
// 	) -> Result<usize, TlvError>;
// }

pub trait TlvDecode: Sized {
    fn decode(length: usize, bytes: &mut Bytes) -> Result<Self, TlvError>;
}

// pub trait TlvDecodeInner: Sized {
// 	fn decode_inner(bytes: Bytes, length: usize) -> Result<Self, TlvError>;
// }

impl TlvEncode for u8 {
    fn encode(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
        bytes.put_u8(self.to_be());
        Ok(1usize)
    }
}

impl TlvEncode for Vec<u8> {
    fn encode(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
		bytes.put(self.as_ref());
		Ok(self.len())
	}
}
// impl<T> TlvEncode for Option<T>
// where T: TlvEncode {
// 	fn encode(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
// 		match &self {
// 			Some(inner) => inner.encode(bytes),
// 			None => Ok(0usize)
// 		}
// 	}
// }

// impl<T> TlvEncodeInner for Vec<T>
// where T: TlvEncodeInner {
// 	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
// 		let mut total_encoded = 0usize;
// 		for item in self {
// 			total_encoded += item.encode_inner(bytes)?;
// 		}
// 		Ok(total_encoded)
// 	}
// }

impl TlvDecode for u8 {
    fn decode(_lenght: usize, bytes: &mut Bytes) -> Result<Self, TlvError> {
        Ok(bytes.get_u8())
    }
}

impl TlvDecode for Vec<u8> {
	fn decode(length: usize, bytes: &mut Bytes) -> Result<Self, TlvError> {
		let mut output = vec![0; length];
		bytes.copy_to_slice(&mut output);
		Ok(output)
	}
}
// impl<T> TlvDecodeInner for Option<T>
// where T: TlvDecodeInner {
// 	fn decode_inner(mut bytes: Bytes, length: usize) -> Result<Self, TlvError> {
// 		if length > 0 {
// 			return Ok(Some(T::decode_inner(bytes.split_to(length), length)?));
// 		}
// 		Ok(None)
// 	}
// }

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

// //Fix this
// impl TlvEncodeInner for u4{
// 	fn encode_inner(&self, bytes: &mut BytesMut) -> Result<usize, TlvError> {
// 		match &self {
// 			u4::FirstHalf(ref byte) => {
// 				bytes.put_u8(byte << 4);
// 				Ok(1usize)
// 			}
// 			u4::SecondHalf(ref byte) => {
// 				let index = bytes.len();
// 				let output: u8 = bytes[index-1] | byte;
// 				bytes[index-1..index].copy_from_slice(&output.to_be_bytes());
// 				Ok(0usize)
// 			}
// 		}
// 	}
// }

// //Fix this
// impl TlvDecodeInner for u4{
// 	fn decode_inner(mut bytes: Bytes, length: usize) -> Result<Self, TlvError> {
// 		if length == 1 {
// 			Ok(u4::FirstHalf(bytes.get_u8() >> 4))
// 		} else {
// 			Ok(u4::SecondHalf(bytes.get_u8() & 15u8))
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
