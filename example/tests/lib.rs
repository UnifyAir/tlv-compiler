extern crate tlv;

use tlv_derive::{ TlvEncode, TlvDecode};
use tlv::prelude::*;

// Encode TLV 
#[derive(TlvEncode, TlvDecode)]
pub struct Tester {
    #[tlv_config(tag=2, length_bytes_format=1, format="TLV")]
    lester: Lester,
    #[tlv_config(tag=5, format="TLV")]
    sohan: u8,
    #[tlv_config(tag = 3, tag_bytes_format = 1, length = 1, format="TV")]
    pohan: u8
}

#[derive(TlvEncode, TlvDecode)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
    mohan: u8,

}

#[test]
fn encode_tlv() {
    assert_eq!(1,1)
}
