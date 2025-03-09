extern crate tlv;

use tlv::prelude::*;
use tlv::{BufMut, BytesMut};
use tlv_derive::{TlvDecode, TlvEncode};

// Basic TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct BasicTlv {
    #[tlv_config(tag = 42, length_bytes_format = 1, format = "TLV")]
    value: u8,
}

// TLV-E struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct TlvEStruct {
    #[tlv_config(tag = 56, length_bytes_format = 2, format = "TLV-E")]
    value: u8,
}

// LV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct LvStruct {
    #[tlv_config(length_bytes_format = 1, format = "LV")]
    value: u8,
}

// LV-E struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct LvEStruct {
    #[tlv_config(length_bytes_format = 2, format = "LV-E")]
    value: u8,
}

// TV struct with 1-byte tag
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct TvStruct {
    #[tlv_config(tag = 73, tag_bytes_format = 1, length = 1, format = "TV")]
    value: u8,
}

// TV struct with 4-bit tag and value
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct Tv4BitStruct {
    #[tlv_config(tag = 9, tag_bytes_format = 0, format = "TV")]
    value: u8,
}

// 4-bit value pair struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct FourBitPairStruct {
    #[tlv_config(format = "V", value_bytes_format = 0)]
    first: u8,
    #[tlv_config(format = "V", value_bytes_format = 0)]
    second: u8,
}

// Complex mixed format struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct ComplexMixedStruct {
    #[tlv_config(tag = 65, length_bytes_format = 1, format = "TLV")]
    tlv_field: u8,
    #[tlv_config(tag = 78, tag_bytes_format = 1, length = 1, format = "TV")]
    tv_field: u8,
    #[tlv_config(length_bytes_format = 1, format = "LV")]
    lv_field: u8,
    #[tlv_config(tag = 91, length_bytes_format = 2, format = "TLV-E")]
    tlv_e_field: u8,
    #[tlv_config(format = "V", value_bytes_format = 0)]
    four_bit_1: u8,
    #[tlv_config(format = "V", value_bytes_format = 0)]
    four_bit_2: u8,
}

// Optional fields mixed struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalMixedStruct {
    #[tlv_config(tag = 80, length_bytes_format = 1, format = "TLV")]
    required: u8,
    #[tlv_config(tag = 90, tag_bytes_format = 1, length = 1, format = "TV")]
    optional_tv: Option<u8>,
    #[tlv_config(tag = 30, length_bytes_format = 1, format = "TLV")]
    optional_tlv: Option<u8>,
    #[tlv_config(tag = 100, length_bytes_format = 2, format = "TLV-E")]
    optional_tlv_e: Option<u8>,
}

#[test]
fn test_tlv_e() {
    let tlv_e = TlvEStruct { value: 10 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tlv_e.encode(&mut bytes).unwrap();

    let decoded = TlvEStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(tlv_e, decoded);
}

#[test]
fn test_lv() {
    let lv = LvStruct { value: 42 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = lv.encode(&mut bytes).unwrap();

    let decoded = LvStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(lv, decoded);
}

#[test]
fn test_lv_e() {
    let lv_e = LvEStruct { value: 100 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = lv_e.encode(&mut bytes).unwrap();

    let decoded = LvEStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(lv_e, decoded);
}

#[test]
fn test_tv() {
    let tv = TvStruct { value: 42 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv.encode(&mut bytes).unwrap();

    let decoded = TvStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(tv, decoded);
}

#[test]
fn test_tv_4bit() {
    let tv_4bit = Tv4BitStruct { value: 7 }; // Testing with value < 16
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv_4bit.encode(&mut bytes).unwrap();

    let decoded = Tv4BitStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(tv_4bit, decoded);
}

#[test]
fn test_4bit_pair() {
    let pair = FourBitPairStruct {
        first: 7,
        second: 15,
    }; // Testing with values < 16
    let mut bytes = BytesMut::with_capacity(32);
    let len = pair.encode(&mut bytes).unwrap();

    let decoded = FourBitPairStruct::decode(len, &mut bytes.freeze()).unwrap();
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
        four_bit_2: 15,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = complex.encode(&mut bytes).unwrap();

    let decoded = ComplexMixedStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(complex, decoded);
}

#[test]
fn test_optional_mixed() {
    // Test with all fields present
    let optional_all = OptionalMixedStruct {
        required: 42,
        optional_tv: Some(43),
        optional_tlv: Some(44),
        optional_tlv_e: Some(10),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_all.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_all, decoded);

    // Test with some fields None
    let optional_some = OptionalMixedStruct {
        required: 42,
        optional_tv: None,
        optional_tlv: Some(44),
        optional_tlv_e: None,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_some.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_some, decoded);

    // Test with all optional fields None
    let optional_none = OptionalMixedStruct {
        required: 42,
        optional_tv: None,
        optional_tlv: None,
        optional_tlv_e: None,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_none.encode(&mut bytes).unwrap();
    let decoded = OptionalMixedStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_none, decoded);
}

// Vector TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct VectorTlvStruct {
    #[tlv_config(tag = 68, length_bytes_format = 1, format = "TLV")]
    bytes: Vec<u8>,
}

// Vector LV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct VectorLvStruct {
    #[tlv_config(length_bytes_format = 1, format = "LV")]
    bytes: Vec<u8>,
}

// Optional Vector TLV struct
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalVectorStruct {
    #[tlv_config(tag = 71, length_bytes_format = 1, format = "TLV")]
    required_bytes: Vec<u8>,
    #[tlv_config(tag = 82, length_bytes_format = 1, format = "TLV")]
    optional_bytes: Option<Vec<u8>>,
}

#[test]
fn test_vector_tlv() {
    let vector_tlv = VectorTlvStruct {
        bytes: vec![1, 2, 3, 4, 5],
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = vector_tlv.encode(&mut bytes).unwrap();

    let decoded = VectorTlvStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(vector_tlv, decoded);

    // Test with empty vector
    let empty_vector_tlv = VectorTlvStruct { bytes: vec![] };
    let mut bytes = BytesMut::with_capacity(32);
    let len = empty_vector_tlv.encode(&mut bytes).unwrap();

    let decoded = VectorTlvStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(empty_vector_tlv, decoded);
}

#[test]
fn test_vector_lv() {
    let vector_lv = VectorLvStruct {
        bytes: vec![10, 20, 30, 40, 50],
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = vector_lv.encode(&mut bytes).unwrap();

    let decoded = VectorLvStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(vector_lv, decoded);
}

#[test]
fn test_optional_vector() {
    // Test with optional vector present
    let optional_present = OptionalVectorStruct {
        required_bytes: vec![1, 2, 3],
        optional_bytes: Some(vec![4, 5, 6]),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_present.encode(&mut bytes).unwrap();

    let decoded = OptionalVectorStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_present, decoded);

    // Test with optional vector absent
    let optional_absent = OptionalVectorStruct {
        required_bytes: vec![1, 2, 3],
        optional_bytes: None,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_absent.encode(&mut bytes).unwrap();

    let decoded = OptionalVectorStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_absent, decoded);

    // Test with empty vectors
    let empty_vectors = OptionalVectorStruct {
        required_bytes: vec![],
        optional_bytes: Some(vec![]),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = empty_vectors.encode(&mut bytes).unwrap();

    let decoded = OptionalVectorStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(empty_vectors, decoded);
}

// Newtype struct for u8
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct NewTypeU8(u8);

// Newtype struct for Vec<u8>
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct NewTypeVec(Vec<u8>);

// Struct containing newtypes
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct ContainsNewtypes {
    #[tlv_config(tag = 53, length_bytes_format = 1, format = "TLV")]
    newtype_u8: NewTypeU8,
    #[tlv_config(tag = 77, length_bytes_format = 1, format = "TLV")]
    newtype_vec: NewTypeVec,
}

// Struct with optional newtypes
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct OptionalNewtypes {
    #[tlv_config(tag = 49, length_bytes_format = 1, format = "TLV")]
    required_newtype: NewTypeU8,
    #[tlv_config(tag = 95, length_bytes_format = 1, format = "TLV")]
    optional_newtype: Option<NewTypeU8>,
    #[tlv_config(tag = 103, length_bytes_format = 1, format = "TLV")]
    optional_newtype_vec: Option<NewTypeVec>,
}

// Mixed struct with newtypes and regular types
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct MixedNewtypeStruct {
    #[tlv_config(tag = 62, length_bytes_format = 1, format = "TLV")]
    regular_u8: u8,
    #[tlv_config(tag = 87, length_bytes_format = 1, format = "TLV")]
    newtype_u8: NewTypeU8,
    #[tlv_config(tag = 108, length_bytes_format = 1, format = "TLV")]
    optional_regular: Option<u8>,
    #[tlv_config(tag = 119, length_bytes_format = 1, format = "TLV")]
    optional_newtype: Option<NewTypeU8>,
}

#[test]
fn test_newtype_u8() {
    let newtype = NewTypeU8(42);
    let mut bytes = BytesMut::with_capacity(32);
    let len = newtype.encode(&mut bytes).unwrap();

    let decoded = NewTypeU8::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(newtype, decoded);
}

#[test]
fn test_newtype_vec() {
    let newtype = NewTypeVec(vec![1, 2, 3, 4, 5]);
    let mut bytes = BytesMut::with_capacity(32);
    let len = newtype.encode(&mut bytes).unwrap();

    let decoded = NewTypeVec::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(newtype, decoded);

    // Test with empty vector
    let empty_newtype = NewTypeVec(vec![]);
    let mut bytes = BytesMut::with_capacity(32);
    let len = empty_newtype.encode(&mut bytes).unwrap();

    let decoded = NewTypeVec::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(empty_newtype, decoded);
}

#[test]
fn test_contains_newtypes() {
    let contains_newtypes = ContainsNewtypes {
        newtype_u8: NewTypeU8(42),
        newtype_vec: NewTypeVec(vec![1, 2, 3, 4, 5]),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = contains_newtypes.encode(&mut bytes).unwrap();

    let decoded = ContainsNewtypes::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(contains_newtypes, decoded);
}

#[test]
fn test_optional_newtypes() {
    // Test with all fields present
    let optional_all = OptionalNewtypes {
        required_newtype: NewTypeU8(42),
        optional_newtype: Some(NewTypeU8(43)),
        optional_newtype_vec: Some(NewTypeVec(vec![1, 2, 3])),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_all.encode(&mut bytes).unwrap();

    let decoded = OptionalNewtypes::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_all, decoded);

    // Test with some fields None
    let optional_some = OptionalNewtypes {
        required_newtype: NewTypeU8(42),
        optional_newtype: None,
        optional_newtype_vec: Some(NewTypeVec(vec![1, 2, 3])),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_some.encode(&mut bytes).unwrap();

    let decoded = OptionalNewtypes::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_some, decoded);

    // Test with all optional fields None
    let optional_none = OptionalNewtypes {
        required_newtype: NewTypeU8(42),
        optional_newtype: None,
        optional_newtype_vec: None,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = optional_none.encode(&mut bytes).unwrap();

    let decoded = OptionalNewtypes::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(optional_none, decoded);
}

#[test]
fn test_mixed_newtype_struct() {
    // Test with all fields present
    let mixed_all = MixedNewtypeStruct {
        regular_u8: 42,
        newtype_u8: NewTypeU8(43),
        optional_regular: Some(44),
        optional_newtype: Some(NewTypeU8(45)),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = mixed_all.encode(&mut bytes).unwrap();

    let decoded = MixedNewtypeStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(mixed_all, decoded);

    // Test with some fields None
    let mixed_some = MixedNewtypeStruct {
        regular_u8: 42,
        newtype_u8: NewTypeU8(43),
        optional_regular: None,
        optional_newtype: Some(NewTypeU8(45)),
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = mixed_some.encode(&mut bytes).unwrap();

    let decoded = MixedNewtypeStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(mixed_some, decoded);

    // Test with all optional fields None
    let mixed_none = MixedNewtypeStruct {
        regular_u8: 42,
        newtype_u8: NewTypeU8(43),
        optional_regular: None,
        optional_newtype: None,
    };
    let mut bytes = BytesMut::with_capacity(32);
    let len = mixed_none.encode(&mut bytes).unwrap();

    let decoded = MixedNewtypeStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(mixed_none, decoded);
}
