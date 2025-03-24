pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use std::io::Write;
use std::usize;
use thiserror::Error;

//Todo do some better error handling.
#[derive(Error, Debug)]
pub enum TlvError {
    #[error("unknown error")]
    Unknown,
    #[error("Payload is not as per specification")]
    MarformedPayload,
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

impl TlvDecode for Vec<u8> {
	fn decode(length: usize, bytes: &mut Bytes) -> Result<Self, TlvError> {
		let mut output = vec![0; length];
		bytes.copy_to_slice(&mut output);
		Ok(output)
	}
}

