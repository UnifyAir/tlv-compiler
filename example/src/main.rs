use tlv_derive::{ TlvEncode };
use tlv::prelude::*;
use tlv::{Bytes, BytesMut, BufMut};
fn main() {
}

#[derive(TlvEncode)]
#[tlv_config(tag=123, length_bytes_format=4, estimated_size=2048)]
pub struct Tester{
    // #[tlv_config(tag=123, length_bytes_format=4)]
    // lester: Lester,
    #[tlv_config(tag=123, length_bytes_format=4)]
    sohan: u8

}

// #[derive(TlvEncode)]
// #[tlv_config(tag=123, length=3, length_bytes_format=4, estimated_size=2048)]
// pub struct Lester{
//     #[tlv_config(tag=123, length=3, length_bytes_format=4)]
//     mohan: Option<u8>,
//
// }
//
//
