use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{Buf, Bytes, BytesMut, BufMut};
fn main() {
    let lester = Lester{
        mohan: 11
    };
    let tester = Tester{
        lester,
        sohan: u4::FirstHalf(4),
        pohan: u4::SecondHalf(1)
    };
    let encoded = tester.encode().unwrap();
    println!("{:?}", encoded.as_ref());

    let tester_new = Tester::decode(encoded);
    println!("{}", tester_new.unwrap().lester.mohan);
}

#[derive(TlvEncode, TlvDecode)]
#[tlv_config(tag=1, length_bytes_format=1, estimated_size=2048)]
pub struct Tester{
    #[tlv_config(tag=2, length_bytes_format=1)]
    lester: Lester,
    #[tlv_config(tag=3, length_bytes_format=1)]
    sohan: u4,
    #[tlv_config(tag=3, tag_bytes_format=0, length_bytes_format=0)]
    pohan: u4

}

#[derive(TlvEncode, TlvDecode)]
#[tlv_config(tag=4, length_bytes_format=1, estimated_size=2048)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1)]
    mohan: u8,

}

