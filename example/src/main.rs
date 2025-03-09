use tlv_derive::{TlvDecode, TlvEncode};
use tlv::prelude::*;
use tlv::{BytesMut, BufMut};
use derive_more::{Into, From};

fn main() {



  let optional_present = OptionalVectorStruct {
    required_bytes: Some(23),
    optional_bytes: Some(vec![4, 5, 6])
  };
  let mut bytes = BytesMut::with_capacity(32);
  let len = optional_present.encode(&mut bytes).unwrap();
  println!("{:?}", bytes.as_ref());
  let mut new_bytes = bytes.freeze();
  let decoded = OptionalVectorStruct::decode(len, &mut new_bytes).unwrap();

  println!("{:?}", decoded);
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

    // let a: VectorTlvStruct = VectorTlvStruct{
    //     lohan: vec![1, 2, 4],
    //     mohan: 7
    // };

    // let mut final_bytes = BytesMut::with_capacity(1024);
    // a.encode(&mut final_bytes).unwrap();
    // println!("{:?}", final_bytes.as_ref());
    // let reverse = VectorTlvStruct::decode(final_bytes.clone().into(), final_bytes.len());
    // println!("{:?}", reverse.unwrap().lohan);
}

#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalVectorStruct {
    #[tlv_config(tag=120, tag_bytes_format = 1, length_bytes_format=1, length = 1, format="TV")]
    required_bytes: Option<u8>,
    #[tlv_config(tag=198, length_bytes_format=1, format="TLV")]
    optional_bytes: Option<Vec<u8>>
}


// #[derive(Debug, TlvEncode, TlvDecode)]
// pub struct AtsssContainer(Vec<u8>);


// #[derive(Debug, TlvEncode, TlvDecode)]
// pub struct ExtendedProtocolDiscriminator(u8);


// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct SecurityHeaderType(u8);


// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct SpareHalfOctet(u8);


// #[derive(Debug, TlvEncode, TlvDecode)]
// pub struct MessageType(u8);


// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct FivegsRegistrationType(u8);


// #[derive(Debug, TlvEncode, TlvDecode, Into, From, Clone)]
// pub struct KeySetIdentifier(u8);


// #[derive(Debug, TlvEncode, TlvDecode)]
// pub struct FivegsMobileIdentity(Vec<u8>);

// #[derive(Debug, TlvEncode, TlvDecode)]
// pub struct NasRegistrationRequest {
//     /* Mandatory fields */
//     #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
//     nas_extended_protocol_discriminator: ExtendedProtocolDiscriminator,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_security_header_type: SecurityHeaderType,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_spare_half_octet: SpareHalfOctet,

//     #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
//     nas_registration_request_message_identity: MessageType,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_5gs_registration_type: FivegsRegistrationType,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_ngksi: KeySetIdentifier,

//     #[tlv_config(tag_bytes_format = 0, length_bytes_format = 2, format = "LV-E")]
//     nas_5gs_mobile_identity: FivegsMobileIdentity,

//       /* Optional fields */
//     #[tlv_config(tag = 0xC, tag_bytes_format = 0, length = 0, length_bytes_format = 0, format = "TV")]
//     nas_non_current_native_nas_key_set_identifier: Option<KeySetIdentifier>,
// }

// #[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
// pub struct VectorTlvStruct {
//     #[tlv_config(tag=13, length_bytes_format=1, format="TLV")]
//     mohan: u8,
//     #[tlv_config(tag=12, length_bytes_format=1, format="TLV")]
//     lohan: Vec<u8>,
// }

// fn test_vector_tlv() {
//     let vector_tlv = VectorTlvStruct { 
//         bytes: vec![1, 2, 3, 4, 5] 
//     };
//     let mut bytes = BytesMut::with_capacity(32);
//     let len = vector_tlv.encode(&mut bytes).unwrap();
    
//     let decoded = VectorTlvStruct::decode(bytes.clone().into(), len).unwrap();
//     assert_eq!(vector_tlv, decoded);
    
//     // Test with empty vector
//     let empty_vector_tlv = VectorTlvStruct { 
//         bytes: vec![] 
//     };
//     let mut bytes = BytesMut::with_capacity(32);
//     let len = empty_vector_tlv.encode(&mut bytes).unwrap();
    
//     let decoded = VectorTlvStruct::decode(bytes.clone().into(), len).unwrap();
//     assert_eq!(empty_vector_tlv, decoded);
// }



// #[derive(TlvEncode, TlvDecode)]
// pub struct Tester {
//     // #[tlv_config(tag=2, length_bytes_format=1, format="TLV")]
//     // lester: Lester,
//     #[tlv_config(tag=15, tag_bytes_format= 0, value_bytes_format = 0, format="TV")]
//     sohan: u8,
//     #[tlv_config(tag = 3, tag_bytes_format = 0, length = 1, format="TV")]
//     pohan: u8,
// }
// #[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
// pub struct OptionalTlv {
//     #[tlv_config(tag=4, length_bytes_format=1, format="TLV")]
//     required: u8,
//     #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
//     optional: Option<u8>
// }




// #[derive(TlvEncode, TlvDecode)]
// pub struct Lester{
//     #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
//     mohan: u8,

// }



// #[derive(TlvEncode, TlvDecode)]
// pub struct NasRegistrationRequesta {
//     /* Mandatory fields */
//     #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
//     nas_extended_protocol_discriminator: u8,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_security_header_type: u8,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_spare_half_octet: u8,

//     #[tlv_config(tag_bytes_format = 0, length = 1, length_bytes_format = 0, format = "V")]
//     nas_registration_request_message_identity: u8,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_5gs_registration_type: u8,

//     #[tlv_config(tag_bytes_format = 0, length = 0, length_bytes_format = 0, value_bytes_format = 0, format = "V")]
//     nas_ngksi: u8,

//     #[tlv_config(tag_bytes_format = 0, length_bytes_format = 2, format = "LV-E")]
//     nas_5gs_mobile_identity: u8,

// }
