use crate::tlv_config::{get_bytes_format, get_put_bytes, TlvConfig};
use crate::utils::{get_struct_name, is_newtype};
use attribute_derive::Attribute;
use attribute_derive::__private::proc_macro2;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Type};

fn tag_encode(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.tag_bytes_format == 0 {
        return quote! {};
    }
    match tlv_config.tag {
        Some(tag) => {
            let tag_bytes_format = get_bytes_format(tlv_config.tag_bytes_format);
            let put_bytes = get_put_bytes(tlv_config.tag_bytes_format);
            quote! {
                let __tag: #tag_bytes_format = #tag as #tag_bytes_format;
                __bytes.#put_bytes(__tag);
            }
        }
        None => {
            quote! {}
        }
    }
}

fn length_encode(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.length_bytes_format == 0 {
        return quote! {};
    }
    match tlv_config.length {
        Some(length) => {
            let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
            let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
            quote! {
                const __length: #length_bytes_format = #length as #length_bytes_format;
                __bytes.#put_bytes(__length);
            }
        }
        None => {
            let length_bytes_format = get_bytes_format(tlv_config.length_bytes_format);
            let put_bytes = get_put_bytes(tlv_config.length_bytes_format);
            quote! {
                let __length_buf: #length_bytes_format = 0u8 as #length_bytes_format;
                __bytes.#put_bytes(__length_buf);
            }
        }
    }
}

fn fix_length_parameter(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.length_bytes_format == 0 {
        return quote! {};
    }
    match tlv_config.length {
        Some(_) => {
            quote! {}
        }
        None => {
            quote! {
                let __fix_length_index = __bytes.len();
            }
        }
    }
}

fn fix_length_encode(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.length_bytes_format == 0 {
        return quote! {};
    }
    match tlv_config.length {
        Some(_) => {
            quote! {}
        }
        None => {
            let length_bytes_format = tlv_config.length_bytes_format;
            let bytes_format = get_bytes_format(length_bytes_format);
            quote! {
                let __fix_length = __actual_length as #bytes_format;
                __bytes[__fix_length_index..__fix_length_index + #length_bytes_format as usize].copy_from_slice(&__fix_length.to_be_bytes());
            }
        }
    }
}

fn format_tlv_encode(field_name: Ident, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let tag_stream = tag_encode(&tlv_config);
    let fix_length_parameter_stream = fix_length_parameter(&tlv_config);
    let length_stream = length_encode(&tlv_config);
    let header_size_bytes = tlv_config.tag_bytes_format + tlv_config.length_bytes_format;
    let fix_length_stream = fix_length_encode(&tlv_config);

    Ok(quote! {
        #tag_stream
        #fix_length_parameter_stream
        #length_stream
        __total_length += #header_size_bytes as usize;
        let __actual_length = self.#field_name.encode(__bytes)?;
        __total_length += __actual_length as usize;
        #fix_length_stream
    })
}

fn format_lv_encode(field_name: Ident, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let fix_length_parameter_stream = fix_length_parameter(&tlv_config);
    let length_stream = length_encode(&tlv_config);
    let header_size_bytes = tlv_config.length_bytes_format;
    let fix_length_stream = fix_length_encode(&tlv_config);

    Ok(quote! {
        #fix_length_parameter_stream
        #length_stream
        __total_length += #header_size_bytes as usize;
        let __actual_length = self.#field_name.encode(__bytes)?;
        __total_length += __actual_length as usize;
        #fix_length_stream
    })
}

fn format_tv_encode(field_name: Ident, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let header_size_bytes = tlv_config.tag_bytes_format;

    if tlv_config.tag_bytes_format == 0 {
        // Its a 4bit tag 4bit value case
        let tag = tlv_config.tag.expect("TAG is required to type TV") as u8;
        let tag_stream = quote! {
            let __tag: u8 = #tag;
        };

        let value_stream: TokenStream = quote! {
            let __value: u8 = self.#field_name.to_be() << 4;
        };
        return Ok(quote! {
            #tag_stream
            __total_length += #header_size_bytes as usize;
            #value_stream
            __bytes.put_u8(__tag | __value);
            let __actual_length = 1usize;
            __total_length += __actual_length as usize;
        });
    } else {
        // Its a 1 or more byte tag and 1 or mote byte value case
        let tag_stream = tag_encode(&tlv_config);
        let header_size_bytes = tlv_config.tag_bytes_format;

        return Ok(quote! {
            #tag_stream
            __total_length += #header_size_bytes as usize;
            let __actual_length = self.#field_name.encode(__bytes)?;
            __total_length += __actual_length as usize;
        });
    };
}

fn format_t_encode(_: Ident, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let tag_stream = tag_encode(&tlv_config);
    let header_size_bytes = 1u8;

    Ok(quote! {
        #tag_stream
        __total_length += #header_size_bytes as usize;
    })
}

fn format_v_encode(field_name: Ident, _: TlvConfig) -> Result<TokenStream, Error> {
    // Its a 1 or mote byte value case
    return Ok(quote! {
        let __actual_length = self.#field_name.encode(__bytes)?;
        __total_length += __actual_length as usize;
    });
}

fn format_4bit_v_encode(
    field_name_1: Ident,
    field_name_2: Ident,
    _: TlvConfig,
) -> Result<TokenStream, Error> {
    // Its a 4bit & 4bit value case
    let value_stream_1: TokenStream = quote! {
        let __value_1: u8 = u8::from(self.#field_name_1.clone()).to_be();
    };
    let value_stream_2: TokenStream = quote! {
        let __value_2: u8 = u8::from(self.#field_name_2.clone()).to_be()<<4;
    };
    return Ok(quote! {
        #value_stream_1
        #value_stream_2
        __bytes.put_u8(__value_1 | __value_2);
        let __actual_length = 1usize;
        __total_length += __actual_length as usize;
    });
}

fn format_option_encode(field_name: Ident, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    // Option with TLV, TV, TLV-E are supported
    let tag_stream = tag_encode(&tlv_config);
    let fix_length_parameter_stream = fix_length_parameter(&tlv_config);
    let length_stream = length_encode(&tlv_config);
    let header_size_bytes = tlv_config.tag_bytes_format + tlv_config.length_bytes_format;
    let fix_length_stream = fix_length_encode(&tlv_config);

    match tlv_config.format.clone().as_str() {
        "TLV" | "TLV-E" => {
            return Ok(quote! {
                match &self.#field_name {
                    Some(__inner) => {
                        #tag_stream
                           #fix_length_parameter_stream
                        #length_stream
                        __total_length += #header_size_bytes as usize;
                        let __actual_length = __inner.encode(__bytes)?;
                        __total_length += __actual_length as usize;
                        #fix_length_stream
                    }
                    None => {}
                }
            });
        }
        "TV" => {
            return Ok(quote! {
                match &self.#field_name {
                    Some(__inner) => {
                        #tag_stream
                        __total_length += #header_size_bytes as usize;
                        let __actual_length = __inner.encode(__bytes)?;
                        __total_length += __actual_length as usize;
                    }
                    None => {}
                }
            });
        }
        _ => {
            abort_call_site!("Option with TLV, TV, TLV-E are supported")
        }
    }
}

fn impl_tlv_encode(struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {
    let mut output_stream = Vec::<TokenStream>::new();

    let initialize_stream = quote! {
        let mut __total_length:usize = 0;
    };

    let mut temp_first_value_of_4bit_value: Option<Ident> = None;
    let mut is_4bit_value_packed = true;

    let mut has_optional_fields_started = false;

    for field in data_struct.fields {
        let field_name = field.ident.unwrap();
        let tlv_config = TlvConfig::from_attributes(field.attrs)?;

        match field.ty {
            Type::Path(type_path) => {
                if type_path.path.segments.len() == 1
                    && type_path.path.segments[0].ident == "Option"
                {
                    if let syn::PathArguments::AngleBracketed(args) =
                        &type_path.path.segments[0].arguments
                    {
                        if args.args.len() == 1 {
                            has_optional_fields_started = true;
                            output_stream
                                .push(format_option_encode(field_name, tlv_config).unwrap());
                            continue;
                        } else {
                            abort_call_site!("Option must have exactly one type parameter");
                        }
                    } else {
                        abort_call_site!("Invalid Option type format");
                    }
                }
            }
            _ => {
                abort_call_site!("Unsupported type in generic");
            }
        };

        if has_optional_fields_started {
            abort_call_site!("Optional Fields should be the at the last")
        }

        match tlv_config.format.clone().as_str() {
            "V" => {
                if tlv_config.value_bytes_format == 0 {
                    if is_4bit_value_packed {
                        temp_first_value_of_4bit_value = Some(field_name);
                        is_4bit_value_packed = false;
                        continue;
                    }
                    output_stream.push(
                        format_4bit_v_encode(
                            temp_first_value_of_4bit_value.clone().unwrap(),
                            field_name,
                            tlv_config,
                        )
                        .unwrap(),
                    );
                    is_4bit_value_packed = true;
                } else {
                    output_stream.push(format_v_encode(field_name, tlv_config).unwrap());
                }
            }
            "TLV" | "TLV-E" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_tlv_encode(field_name, tlv_config).unwrap());
            }
            "LV" | "LV-E" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_lv_encode(field_name, tlv_config).unwrap());
            }
            "TV" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_tv_encode(field_name, tlv_config).unwrap());
            }
            "T" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_t_encode(field_name, tlv_config).unwrap());
            }
            _ => {
                abort_call_site!("Unkown TLV format")
            }
        }
    }

    Ok(quote! {
        impl TlvEncode for #struct_name {
            fn encode(&self, __bytes: &mut BytesMut) -> Result<usize, tlv::prelude::TlvError> {
                #initialize_stream
                #(#output_stream)*
                Ok(__total_length)
            }
        }
    })
}

fn impl_newtype_encode(struct_name: Ident) -> Result<TokenStream, Error> {
    Ok(quote! {
        impl TlvEncode for #struct_name {
            fn encode(&self, __bytes: &mut BytesMut) -> Result<usize, tlv::prelude::TlvError> {
                self.0.encode(__bytes)
            }
        }
    })
}

pub(crate) fn tlv_encode(token_stream: TokenStream) -> Result<TokenStream, Error> {
    let DeriveInput { data, .. } = syn::parse2(token_stream.clone())?;
    let struct_name = get_struct_name(token_stream.clone());

    match data {
        syn::Data::Struct(data_struct) => {
            if is_newtype(&data_struct) {
                impl_newtype_encode(struct_name)
            } else {
                impl_tlv_encode(struct_name, data_struct)
            }
        },
        _ => {
            abort_call_site!("Currently only structs are supported");
        }
    }
}


