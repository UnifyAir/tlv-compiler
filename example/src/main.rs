use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{Bytes, BytesMut, BufMut};


fn main() {
    let lester = Lester{
        mohan: 11
    };
    let tester = Tester{
        lester,
        // sohan: u4::FirstHalf(4),
        // pohan: u4::SecondHalf(1)
        sohan: 3,
        pohan: 1
    };
    let mut final_bytes = BytesMut::with_capacity(1024);
    tester.encode(&mut final_bytes).unwrap();
    println!("{:?}", final_bytes.as_ref());

    let tester_new = Tester::decode(final_bytes.clone().into(), final_bytes.len());
    println!("{}", tester_new.unwrap().pohan);
}

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




