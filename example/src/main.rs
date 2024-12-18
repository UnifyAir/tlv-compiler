use tlv_derive::{ TlvEncode };
use tlv::prelude::*;
use tlv::{Bytes, BytesMut, BufMut};
fn main() {
    let lester = Lester{
        mohan: Some(5)
    };
    let tester = Tester{
        lester,
        sohan: 6
    };
    println!("{:?}", tester.encode().unwrap().as_ref());
}

#[derive(TlvEncode)]
#[tlv_config(tag=1, length_bytes_format=1, estimated_size=2048)]
pub struct Tester{
    #[tlv_config(tag=2, length_bytes_format=1)]
    lester: Lester,
    #[tlv_config(tag=3, length_bytes_format=1)]
    sohan: u8

}

#[derive(TlvEncode)]
#[tlv_config(tag=4, length_bytes_format=1, estimated_size=2048)]
pub struct Lester{
    #[tlv_config(tag=5, length_bytes_format=1)]
    mohan: Option<u8>,

}


//
// use tlv_derive::TlvEncode;
// use tlv::prelude::*;
// use tlv::{Bytes, BytesMut, BufMut};
// fn main() {
//     let lester = Lester { mohan: Some(5) };
//     let tester = Tester { lester, sohan: 6 };
//     {
//         println!("{:?}", tester.encode());
//     };
// }
// pub struct Tester {
//     lester: Lester,
//     sohan: u8,
// }
// impl TlvEncode for Tester {
//     fn encode(&self) -> Result<Bytes, tlv::prelude::TlvError> {
//         let mut bytes = BytesMut::with_capacity(2048usize);
//         let tag = 1usize as u8;
//         bytes.put_u8(tag);
//         let fix_length_index = bytes.len();
//         let length_buf = 0u8 as u8;
//         bytes.put_u8(length_buf);
//         let actual_length = self.encode_inner(&mut bytes)?;
//         bytes[fix_length_index..fix_length_index + 1u8 as usize]
//             .copy_from_slice(&actual_length.to_be_bytes());
//         Ok(bytes.freeze())
//     }
// }
// impl TlvEncodeInner for Tester {
//     fn encode_inner(
//         &self,
//         bytes: &mut BytesMut,
//     ) -> Result<usize, tlv::prelude::TlvError> {
//         let mut total_length: usize = 0;
//         let tag = 2usize as u8;
//         bytes.put_u8(tag);
//         let fix_length_index = bytes.len();
//         let length_buf = 0u8 as u8;
//         bytes.put_u8(length_buf);
//         total_length += 2u8 as usize;
//         let actual_length = self.lester.encode_inner(bytes)?;
//         total_length += actual_length as usize;
//         bytes[fix_length_index..fix_length_index + 1u8 as usize]
//             .copy_from_slice(&actual_length.to_be_bytes());
//         let tag = 3usize as u8;
//         bytes.put_u8(tag);
//         let fix_length_index = bytes.len();
//         let length_buf = 0u8 as u8;
//         bytes.put_u8(length_buf);
//         total_length += 2u8 as usize;
//         let actual_length = self.sohan.encode_inner(bytes)?;
//         total_length += actual_length as usize;
//         bytes[fix_length_index..fix_length_index + 1u8 as usize]
//             .copy_from_slice(&actual_length.to_be_bytes());
//         Ok(total_length)
//     }
// }
// pub struct Lester {
//     mohan: Option<u8>,
// }
// impl TlvEncode for Lester {
//     fn encode(&self) -> Result<Bytes, tlv::prelude::TlvError> {
//         let mut bytes = BytesMut::with_capacity(2048usize);
//         let tag = 4usize as u8;
//         bytes.put_u8(tag);
//         let fix_length_index = bytes.len();
//         let length_buf = 0u8 as u8;
//         bytes.put_u8(length_buf);
//         let actual_length = self.encode_inner(&mut bytes)?;
//         bytes[fix_length_index..fix_length_index + 1u8 as usize]
//             .copy_from_slice(&actual_length.to_be_bytes());
//         Ok(bytes.freeze())
//     }
// }
// impl TlvEncodeInner for Lester {
//     fn encode_inner(
//         &self,
//         bytes: &mut BytesMut,
//     ) -> Result<usize, tlv::prelude::TlvError> {
//         let mut total_length: usize = 0;
//         let tag = 5usize as u8;
//         bytes.put_u8(tag);
//         let fix_length_index = bytes.len();
//         let length_buf = 0u8 as u8;
//         bytes.put_u8(length_buf);
//         total_length += 2u8 as usize;
//         let actual_length = self.mohan.encode_inner(bytes)? as u8;
//         total_length += actual_length as usize;
//         bytes[fix_length_index..fix_length_index + 1u8 as usize]
//             .copy_from_slice(&actual_length.to_be_bytes());
//         Ok(total_length)
//     }
// }
//




