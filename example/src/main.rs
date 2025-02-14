use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{Buf, Bytes, BytesMut, BufMut};


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
    let encoded = tester.encode(&mut final_bytes).unwrap();
    println!("{:?}", final_bytes.as_ref());

    // let tester_new = Tester::decode(encoded);
    // println!("{}", tester_new.unwrap().lester.mohan);
}

#[derive(TlvEncode)]
pub struct Tester{
    #[tlv_config(tag=2, length_bytes_format=1, format="TLV")]
    lester: Lester,
    #[tlv_config(value_bytes_format = 0, format="V")]
    sohan: u8,
    #[tlv_config(value_bytes_format = 0, format="V")]
    pohan: u8

}

#[derive(TlvEncode)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
    mohan: u8,

}




