use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{BytesMut, BufMut};

fn main() {
    // let lester = Lester{
    //     mohan: 11
    // };
    // let tester = Tester{
    //     // lester,
    //     sohan: 1,
    //     pohan: 1,
    // };
    // let mut final_bytes = BytesMut::with_capacity(1024);
    // tester.encode(&mut final_bytes).unwrap();
    // println!("{:?}", final_bytes.as_ref());
				

    // let tester_new = Tester::decode(final_bytes.clone().into(), final_bytes.len());
    // println!("{}", tester_new.unwrap().pohan);


    // let optional_none = OptionalTlv {
    //     required: 10,
    //     optional: None
    // };
    // let mut bytes = BytesMut::with_capacity(32);
    // let len = optional_none.encode(&mut bytes).unwrap();
    // println!("{:?}", bytes.as_ref());
    // let decoded = OptionalTlv::decode(bytes.clone().into(), len).unwrap();
    // assert_eq!(optional_none, decoded);
}



#[derive(TlvEncode, TlvDecode)]
pub struct Tester {
    // #[tlv_config(tag=2, length_bytes_format=1, format="TLV")]
    // lester: Lester,
    #[tlv_config(tag=15, tag_bytes_format= 0, value_bytes_format = 0, format="TV")]
    sohan: u8,
    #[tlv_config(tag = 3, tag_bytes_format = 0, length = 1, format="TV")]
    pohan: u8,
}
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalTlv {
    #[tlv_config(tag=4, length_bytes_format=1, format="TLV")]
    required: u8,
    #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
    optional: Option<u8>
}




// #[derive(TlvEncode, TlvDecode)]
// pub struct Lester{
//     #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
//     mohan: u8,

// }




