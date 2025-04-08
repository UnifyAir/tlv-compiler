use tlv::prelude::*;
use tlv::{BufMut, BytesMut};
use tlv::tlv_derive::*;

/*
 * SCRATCH PAD - Quick Testing Only
 * 
 * NOTE: This main.rs is just for rapid prototyping and quick tests.
 * For proper test cases and comprehensive testing, please refer to
 * the test directory.
 * 
 * This is NOT production code!
 */

fn main() {
    let tv_4bit = Tv4BitStruct { value: None }; // Testing with value < 16
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv_4bit.encode(&mut bytes).unwrap();

    let bb = bytes.clone().freeze();
    println!("{:?}", bb.as_ref());
    let decoded = Tv4BitStruct::decode(len, &mut bytes.freeze()).unwrap();
    println!("{:?}", decoded);

    assert_eq!(tv_4bit, decoded);
}

#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct Tv4BitStruct {
    #[tlv_config(tag = 0x9, tag_bytes_format = 0, format = "TV")]
    value: Option<u8>,
}
