# TLV Compiler for 5G NAS Messages

![README AI Generated](https://img.shields.io/badge/README-AI%20Generated-blue.svg)

A Rust library for encoding and decoding Type-Length-Value (TLV) structures, specifically designed for 5G Non-Access Stratum (NAS) messages as described in 3GPP specifications TS 124 007 and TS 124 501.

## Overview

This library provides a flexible and efficient way to work with TLV structures commonly used in telecommunications protocols, particularly in 5G NAS messages. It supports various TLV formats including:

- **TLV**: Tag-Length-Value
- **TLV-E**: Tag-Length-Value with Extended Length
- **LV**: Length-Value
- **LV-E**: Length-Value with Extended Length
- **TV**: Tag-Value
- **TV-4bit**: Tag-Value with 4-bit tag and 4-bit value (packed in a single byte)
- **V**: Value-only (for 4-bit values)

## Features

- Derive macros for easy implementation of `TlvEncode` and `TlvDecode` traits
- Support for multiple TLV formats with configurable tag and length sizes
- Optional field support
- Vector field support
- Newtype pattern support
- Efficient memory handling with zero-copy operations where possible
- Comprehensive test suite

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tlv = { git = "https://github.com/UnifyAir/tlv-compiler.git", package = "tlv", branch = "master" }
```

## Usage

### Basic Example

```rust
use tlv::prelude::*;
use tlv::{BufMut, BytesMut};
use tlv::tlv_derive::*;

#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct BasicTlv {
    #[tlv_config(tag = 42, length_bytes_format = 1, format = "TLV")]
    value: u8,
}

fn main() {
    let tlv = BasicTlv { value: 42 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tlv.encode(&mut bytes).unwrap();
    
    let decoded = BasicTlv::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(tlv, decoded);
}
```

### Advanced Example with Optional Fields

```rust
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
```

### 4-bit Tag and 4-bit Value Example

```rust
#[derive(TlvEncode, TlvDecode, Debug, PartialEq)]
pub struct FourBitTagValueStruct {
    #[tlv_config(tag = 0x9, tag_bytes_format = 0, format = "TV")]
    value: u8,
}

fn main() {
    // Create a struct with a 4-bit tag (0x9) and 4-bit value (0x7)
    let tv_4bit = FourBitTagValueStruct { value: 0x7 };
    let mut bytes = BytesMut::with_capacity(32);
    let len = tv_4bit.encode(&mut bytes).unwrap();
    
    // The encoded byte will be 0x97 (tag 0x9 in upper 4 bits, value 0x7 in lower 4 bits)
    let decoded = FourBitTagValueStruct::decode(len, &mut bytes.freeze()).unwrap();
    assert_eq!(tv_4bit, decoded);
}
```

## 3GPP Specifications

This library is designed to work with 5G NAS messages as specified in:

- **TS 124 007**: "Mobile radio interface signalling layer 3; General aspects"
- **TS 124 501**: "Non-Access-Stratum (NAS) protocol for 5G System (5GS); Stage 3"

These specifications define the TLV structures used in 5G NAS messages for various procedures including registration, authentication, and service requests.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
