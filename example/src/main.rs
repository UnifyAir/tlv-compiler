use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{BytesMut, BufMut};

fn main() {
    let lester = Lester{
        mohan: 11
    };
    let tester = Tester{
        lester,
        sohan: 11,
        pohan: Some(11)
    };
    let mut final_bytes = BytesMut::with_capacity(1024);
    tester.encode(&mut final_bytes).unwrap();
    println!("{:?}", final_bytes.as_ref());
				

    // let tester_new = Tester::decode(final_bytes.clone().into(), final_bytes.len());
    // println!("{}", tester_new.unwrap().lester.mohan.unwrap());
}

#[derive(TlvEncode, TlvDecode)]
pub struct Tester {
    #[tlv_config(tag=2, length_bytes_format=1, format="TLV")]
    lester: Lester,
    #[tlv_config(tag=50, value_bytes_format = 0, format="TLV")]
    sohan: u8,
    #[tlv_config(tag = 3, value_bytes_format = 0, format="TLV")]
    pohan: Option<u8>,
}

#[derive(TlvEncode, TlvDecode)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
    mohan: u8,

}




