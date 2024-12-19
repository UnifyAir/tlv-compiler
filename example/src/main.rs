use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{Buf, Bytes, BytesMut, BufMut};
fn main() {
    let lester = Lester{
        mohan: 11
    };
    let tester = Tester{
        lester,
        sohan: 6
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
    sohan: u8

}

#[derive(TlvEncode, TlvDecode)]
#[tlv_config(tag=4, length_bytes_format=1, estimated_size=2048)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1)]
    mohan: u8,

}


// #[derive(TlvDecode)]
// #[tlv_config(tag=12, length_bytes_format=1, length=6, estimated_size=2048)]
// pub struct Tester{
//     // #[tlv_config(tag=2, length_bytes_format=1)]
//     // lester: crate::Lester,
//     #[tlv_config(tag=3, length_bytes_format=1)]
//     sohan: u8
//
// }
//
// #[derive(TlvDecode)]
// #[tlv_config(tag=4, length_bytes_format=1, estimated_size=2048)]
// pub struct Lester{
//     #[tlv_config(tag=5, length_bytes_format=1)]
//     mohan: u8,
//
// }
//




// use tlv_derive::{TlvDecode, TlvEncode};
// use tlv::prelude::*;
// use tlv::{Buf, Bytes, BytesMut, BufMut};
// fn main() {}
// pub struct Tester {
//     sohan: u8,
// }
// impl TlvDecode for Tester {
//     fn decode(mut __bytes: Bytes) -> Result<Self, tlv::prelude::TlvError> {
//         let __actual_tag: usize = 12usize;
//         __bytes.advance(1usize);
//         let __actual_length: usize = 6usize;
//         __bytes.advance(1usize);
//         let __output = Self::decode_inner(
//             Bytes::from_owner(__bytes.chunk()),
//             __actual_length,
//         )?;
//         Ok(__output)
//     }
// }
// impl TlvDecodeInner for Tester {
//     fn decode_inner(
//         mut __bytes: Bytes,
//         length: usize,
//     ) -> Result<Self, tlv::prelude::TlvError> {
//         let __actual_tag: usize = 3usize;
//         __bytes.advance(1usize);
//         let __actual_length = __bytes.get_u8() as usize;
//         __bytes.advance(2usize);
//         let sohan = u8::decode_inner(
//             Bytes::from_owner(__bytes.chunk()),
//             __actual_length,
//         )?;
//         Ok(Tester { sohan })
//     }
// }




//
// use tlv_derive::{TlvDecode, TlvEncode};
// use tlv::prelude::*;
// use tlv::{Buf, Bytes, BytesMut, BufMut};
// fn main() {}
// pub struct Tester {
//     sohan: u8,
// }
// impl TlvDecode for Tester {
//     fn decode(mut __bytes: Bytes) -> Result<Self, tlv::prelude::TlvError> {
//         let __actual_tag: usize = 12usize;
//         __bytes.advance(1usize);
//         let __actual_length: usize = 6usize;
//         __bytes.advance(1usize);
//         let __output = Self::decode_inner(__bytes.chunk(), __actual_length)?;
//         Ok(__output)
//     }
// }
// impl TlvDecodeInner for Tester {
//     fn decode_inner(
//         __data: &[u8],
//         length: usize,
//     ) -> Result<Self, tlv::prelude::TlvError> {
//         let mut __bytes: Bytes = Bytes::copy_from_slice(__data.clone());
//         let __actual_tag: usize = 3usize;
//         __bytes.advance(1usize);
//         let __actual_length = __bytes.get_u8() as usize;
//         __bytes.advance(2usize);
//         let sohan = u8::decode_inner(__bytes.chunk(), __actual_length)?;
//         Ok(Tester { sohan })
//     }
// }
//


