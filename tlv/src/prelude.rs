pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use std::io::Write;
use std::usize;
use thiserror::Error;

//Todo: do some better error handling.
#[derive(Error, Debug)]
pub enum TlvError {
    #[error("unknown error")]
    Unknown,
    #[error("Payload is not as per specification")]
    MalformedPayload,
}

pub trait TlvEncode {
    fn encode(&self, bytes: &mut BytesMut) -> Result<usize, TlvError>;
}


pub trait TlvDecode: Sized {
    fn decode(length: usize, bytes: &mut Bytes) -> Result<Self, TlvError>;
}


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

impl TlvDecode for u8 {
    fn decode(_lenght: usize, bytes: &mut Bytes) -> Result<Self, TlvError> {
        Ok(bytes.get_u8())
    }
}


// Conversion to Vec is essential because if we had used Bytes or &Bytes, it will never be dropped
// until the whole or subsection of that byte is dropped.
//
// For example:
// If we have a big byte buffer, and we cut multiple small byte buffers from it, until all
// the cuts are dropped, then the big byte buffer will never be dropped. Let's say:
//
// ```
// bytes1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// bytes2 = bytes1.split_off(2)
// bytes3 = bytes1.split_off(7)
// ```
//
// Now, if we drop bytes2 or bytes3, either of them will not be dropped until all of bytes1,
// bytes2 and bytes3 are dropped.
//
// This is because malloc is contiguous and free will free everything - when we allocate an array,
// it's not possible to free only a part of it.
//
// See: https://stackoverflow.com/questions/2479766/how-allocate-or-free-only-parts-of-an-array

impl TlvDecode for Vec<u8> {
	fn decode(length: usize, bytes: &mut Bytes) -> Result<Self, TlvError> {
		let mut output = vec![0; length];
		bytes.copy_to_slice(&mut output);
		Ok(output)
	}
}

