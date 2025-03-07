extern crate tlv;

use tlv_derive::{ TlvEncode, TlvDecode};
use tlv::prelude::*;
use tlv::{BytesMut, BufMut};

// Basic TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct BasicTlv {
    #[tlv_config(tag=1, length_bytes_format=1, format="TLV")]
    value: u8
}

// TLV-E struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct TlvEStruct {
    #[tlv_config(tag=2, length_bytes_format=2, format="TLV-E")]
    value: u8
}

// LV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct LvStruct {
    #[tlv_config(length_bytes_format=1, format="LV")]
    value: u8
}

// LV-E struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct LvEStruct {
    #[tlv_config(length_bytes_format=2, format="LV-E")]
    value: u8
}

// TV struct with 1-byte tag
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct TvStruct {
    #[tlv_config(tag=3, tag_bytes_format=1, length=1, format="TV")]
    value: u8
}

// TV struct with 4-bit tag and value
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct Tv4BitStruct {
    #[tlv_config(tag=4, tag_bytes_format=0, format="TV")]
    value: u8
}

// 4-bit value pair struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct FourBitPairStruct {
    #[tlv_config(format="V", value_bytes_format=0)]
    first: u8,
    #[tlv_config(format="V", value_bytes_format=0)]
    second: u8
}

// Complex mixed format struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct ComplexMixedStruct {
    #[tlv_config(tag=5, length_bytes_format=1, format="TLV")]
    tlv_field: u8,
    #[tlv_config(tag=6, tag_bytes_format=1, length=1, format="TV")]
    tv_field: u8,
    #[tlv_config(length_bytes_format=1, format="LV")]
    lv_field: u8,
    #[tlv_config(tag=7, length_bytes_format=2, format="TLV-E")]
    tlv_e_field: u8,
    #[tlv_config(format="V", value_bytes_format=0)]
    four_bit_1: u8,
    #[tlv_config(format="V", value_bytes_format=0)]
    four_bit_2: u8
}

// Optional fields mixed struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalMixedStruct {
    #[tlv_config(tag=8, length_bytes_format=1, format="TLV")]
    required: u8,
    #[tlv_config(tag=9, tag_bytes_format=1, length=1, format="TV")]
    optional_tv: Option<u8>,
    #[tlv_config(tag=10, length_bytes_format=1, format="TLV")]
    optional_tlv: Option<u8>,
    #[tlv_config(tag=11, length_bytes_format=2, format="TLV-E")]
    optional_tlv_e: Option<u8>
}

#[test]
fn test_tlv_e() {
    let tlv_e = TlvEStruct { value: 10 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tlv_e.encode(&mut bytes).unwrap();
    
    let decoded = TlvEStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(tlv_e, decoded);
}

#[test]
fn test_lv() {
    let lv = LvStruct { value: 42 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = lv.encode(&mut bytes).unwrap();
    
    let decoded = LvStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(lv, decoded);
}

#[test]
fn test_lv_e() {
    let lv_e = LvEStruct { value: 100 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = lv_e.encode(&mut bytes).unwrap();
    
    let decoded = LvEStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(lv_e, decoded);
}

#[test]
fn test_tv() {
    let tv = TvStruct { value: 42 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv.encode(&mut bytes).unwrap();
    
    let decoded = TvStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(tv, decoded);
}

#[test]
fn test_tv_4bit() {
    let tv_4bit = Tv4BitStruct { value: 7 }; // Testing with value < 16
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv_4bit.encode(&mut bytes).unwrap();
    
    let decoded = Tv4BitStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(tv_4bit, decoded);
}

#[test]
fn test_4bit_pair() {
    let pair = FourBitPairStruct { first: 7, second: 15 }; // Testing with values < 16
    let mut bytes = BytesMut::with_capacity(32);
    let len = pair.encode(&mut bytes).unwrap();
    
    let decoded = FourBitPairStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(pair, decoded);
}

#[test]
fn test_complex_mixed() {
    let complex = ComplexMixedStruct {
        tlv_field: 42,
        tv_field: 43,
        lv_field: 44,
        tlv_e_field: 35,
        four_bit_1: 7,
        four_bit_2: 15
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = complex.encode(&mut bytes).unwrap();
    
    let decoded = ComplexMixedStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(complex, decoded);
}

#[test]
fn test_optional_mixed() {
    // Test with all fields present
    let optional_all = OptionalMixedStruct {
        required: 42,
        optional_tv: Some(43),
        optional_tlv: Some(44),
        optional_tlv_e: Some(10)
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_all.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(optional_all, decoded);

    // Test with some fields None
    let optional_some = OptionalMixedStruct {
        required: 42,
        optional_tv: None,
        optional_tlv: Some(44),
        optional_tlv_e: None
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_some.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(optional_some, decoded);

    // Test with all optional fields None
    let optional_none = OptionalMixedStruct {
        required: 42,
        optional_tv: None,
        optional_tlv: None,
        optional_tlv_e: None
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_none.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(optional_none, decoded);
}

// Vector TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct VectorTlvStruct {
    #[tlv_config(tag=12, length_bytes_format=1, format="TLV")]
    bytes: Vec<u8>
}

// Vector LV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct VectorLvStruct {
    #[tlv_config(length_bytes_format=1, format="LV")]
    bytes: Vec<u8>
}

// Optional Vector TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalVectorStruct {
    #[tlv_config(tag=13, length_bytes_format=1, format="TLV")]
    required_bytes: Vec<u8>,
    #[tlv_config(tag=14, length_bytes_format=1, format="TLV")]
    optional_bytes: Option<Vec<u8>>
}

#[test]
fn test_vector_tlv() {
    let vector_tlv = VectorTlvStruct { 
        bytes: vec![1, 2, 3, 4, 5] 
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = vector_tlv.encode(&mut bytes).unwrap();
    
    let decoded = VectorTlvStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(vector_tlv, decoded);
    
    // Test with empty vector
    let empty_vector_tlv = VectorTlvStruct { 
        bytes: vec![] 
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = empty_vector_tlv.encode(&mut bytes).unwrap();
    
    let decoded = VectorTlvStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(empty_vector_tlv, decoded);
}

#[test]
fn test_vector_lv() {
    let vector_lv = VectorLvStruct { 
        bytes: vec![10, 20, 30, 40, 50] 
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = vector_lv.encode(&mut bytes).unwrap();
    
    let decoded = VectorLvStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(vector_lv, decoded);
}

#[test]
fn test_optional_vector() {
    // Test with optional vector present
    let optional_present = OptionalVectorStruct {
        required_bytes: vec![1, 2, 3],
        optional_bytes: Some(vec![4, 5, 6])
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_present.encode(&mut bytes).unwrap();
    
    let decoded = OptionalVectorStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(optional_present, decoded);
    
    // Test with optional vector absent
    let optional_absent = OptionalVectorStruct {
        required_bytes: vec![1, 2, 3],
        optional_bytes: None
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_absent.encode(&mut bytes).unwrap();
    
    let decoded = OptionalVectorStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(optional_absent, decoded);
    
    // Test with empty vectors
    let empty_vectors = OptionalVectorStruct {
        required_bytes: vec![],
        optional_bytes: Some(vec![])
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = empty_vectors.encode(&mut bytes).unwrap();
    
    let decoded = OptionalVectorStruct::decode(bytes.clone().into(), len).unwrap();
    assert_eq!(empty_vectors, decoded);
}