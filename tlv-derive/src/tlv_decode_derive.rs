use crate::tlv_config::{get_get_bytes, TlvConfig};
use crate::utils::{get_struct_name, is_newtype};
use attribute_derive::Attribute;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{DataStruct, DeriveInput, Error, Field, GenericArgument, Type};

fn tag_decode(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.tag_bytes_format == 0 {
        return quote! {
            let __actual_tag: usize = 0usize;
        };
    }
    match tlv_config.tag {
        Some(tag) => {
            let tag_bytes = tlv_config.tag_bytes_format as usize;
            quote! {
                let __actual_tag: usize = #tag;
                __bytes.advance(#tag_bytes);
            }
        }
        None => {
            let get_bytes = get_get_bytes(tlv_config.tag_bytes_format);
            quote! {
                let __actual_tag = __bytes.#get_bytes() as usize;
            }
        }
    }
}

fn length_decode(tlv_config: &TlvConfig) -> TokenStream {
    if tlv_config.length_bytes_format == 0 {
        return quote! {
            let __actual_length: usize = 0usize;
        };
    }
    match tlv_config.length {
        Some(length) => {
            let length_bytes = tlv_config.length_bytes_format as usize;
            quote! {
                let __actual_length: usize = #length;
                __bytes.advance(#length_bytes);
            }
        }
        None => {
            let get_bytes = get_get_bytes(tlv_config.length_bytes_format);
            quote! {
                let __actual_length = __bytes.#get_bytes() as usize;
            }
        }
    }
}

fn format_tlv_decode(field: Field, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let field_name = field.ident.unwrap();
    let field_type = match field.ty {
        Type::Path(type_path) => type_path.path,
        _ => {
            abort_call_site!("Unsupported type in generic");
        }
    };
    let tag_stream = tag_decode(&tlv_config);
    let length_stream = length_decode(&tlv_config);

    Ok(quote! {
        #tag_stream
        #length_stream
        let #field_name = <#field_type>::decode(__bytes.split_to(__actual_length), __actual_length)?;
    })
}

fn format_lv_decode(field: Field, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let field_name = field.ident.unwrap();
    let field_type = match field.ty {
        Type::Path(type_path) => type_path.path,
        _ => {
            abort_call_site!("Unsupported type in generic");
        }
    };
    let length_stream = length_decode(&tlv_config);

    Ok(quote! {
        #length_stream
        let #field_name = <#field_type>::decode(__bytes.split_to(__actual_length), __actual_length)?;
    })
}

fn format_tv_decode(field: Field, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let field_name = field.ident.unwrap();
    let field_type = match field.ty {
        Type::Path(type_path) => type_path.path,
        _ => {
            abort_call_site!("Unsupported type in generic");
        }
    };

    if tlv_config.tag_bytes_format == 0 {
        // Its a 4bit tag 4bit valie case
        let _tag = tlv_config.tag.expect("TAG is required to type Tv") as u8;
        return Ok(quote! {
            let #field_name = __bytes.get_u8() & 0b00001111;
        });
    } else {
        // Its a 1 or more byte tag and 1 or mote byte value case
        let tag_stream = tag_decode(&tlv_config);
        let length = tlv_config.length.expect("LENGTH is required to type Tv") as usize;
        return Ok(quote! {
            #tag_stream
            let #field_name = <#field_type>::decode(__bytes.split_to(#length), #length)?;
        });
    }
}

fn format_t_decode(field: Field, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let field_name = field.ident.unwrap();
    let field_type = match field.ty {
        Type::Path(type_path) => type_path.path,
        _ => {
            abort_call_site!("Unsupported type in generic");
        }
    };
    let tag_stream = tag_decode(&tlv_config);

    Ok(quote! {
        #tag_stream
		let __actual_length = 1usize;
        let #field_name = <#field_type>::decode(__bytes.split_to(__actual_length), __actual_length)?;
    })
}

fn format_v_decode(field: Field, tlv_config: TlvConfig) -> Result<TokenStream, Error> {
    let field_name = field.ident.unwrap();
    let field_type = match field.ty {
        Type::Path(type_path) => type_path.path,
        _ => {
            abort_call_site!("Unsupported type in generic");
        }
    };

    let length = tlv_config.length.expect("LENGTH is required to type Tv") as usize;
    Ok(quote! {
        let #field_name = <#field_type>::decode(__bytes.split_to(#length), #length)?;
    })
}

fn format_4bit_v_decode(
    field_1: Field,
    field_2: Field,
    _: TlvConfig,
) -> Result<TokenStream, Error> {
    // Its a 4bit & 4bit value case
    let field_name_1 = field_1.ident.unwrap();
    let value_stream_1: TokenStream = quote! {
        let #field_name_1: u8 = __chunk >> 4;
    };
    let field_name_2 = field_2.ident.unwrap();
    let value_stream_2: TokenStream = quote! {
        let #field_name_2: u8 = __chunk & 0b00001111;
    };
    return Ok(quote! {
        let __chunk = __bytes.get_u8();
        #value_stream_1
        #value_stream_2
    });
}

fn format_option_decode(
    generic: GenericArgument,
    field: Field,
    tlv_config: TlvConfig,
) -> Result<TokenStream, Error> {
    // Option with TLV, TV, TLV-E are supported

    match tlv_config.format.clone().as_str() {
        "TLV" | "TLV-E" => {
            let field_name = field.ident.unwrap();
            let _field_type = match field.ty {
                Type::Path(type_path) => type_path.path,
                _ => {
                    abort_call_site!("Unsupported type in generic");
                }
            };

            let tag_stream = tag_decode(&tlv_config);
            let length_stream = length_decode(&tlv_config);

            return Ok(quote! {
                #tag_stream
                #length_stream
                #field_name = Some(<#generic>::decode(__bytes.split_to(__actual_length), __actual_length)?);
            });
        }
        "TV" => {
            let field_name = field.ident.unwrap();
            let _field_type = match field.ty {
                Type::Path(type_path) => type_path.path,
                _ => {
                    abort_call_site!("Unsupported type in generic");
                }
            };

            if tlv_config.tag_bytes_format == 0 {
                // Its a 4bit tag 4bit valie case
                let _tag = tlv_config.tag.expect("TAG is required to type Tv") as u8;
                return Ok(quote! {
                    #field_name = Some(__bytes.get_u8() & 0b00001111);
                });
            } else {
                // Its a 1 or more byte tag and 1 or mote byte value case
                let tag_stream = tag_decode(&tlv_config);
                let length = tlv_config.length.expect("LENGTH is required to type Tv") as usize;
                return Ok(quote! {
                    #tag_stream
                    #field_name = Some(<#generic>::decode(__bytes.split_to(#length), #length)?);
                });
            }
        }
        _ => {
            abort_call_site!("Option with TLV, TV, TLV-E are supported")
        }
    }
}

fn init_option_decoder(
    optional_tlvs: Vec<(GenericArgument, Field, TlvConfig)>,
) -> Result<TokenStream, Error> {
    if optional_tlvs.is_empty() {
        return Ok(quote! {});
    }

    let mut output_stream: Vec<TokenStream> = Vec::<TokenStream>::new();

    for (opt_tlv_generic, opt_tlv_field, opt_tlv_tlv_config) in optional_tlvs {
        let opt_tag = opt_tlv_tlv_config
            .tag
            .expect("TAG is required for optional tlvs");
        let format_option_decode_stream =
            format_option_decode(opt_tlv_generic, opt_tlv_field, opt_tlv_tlv_config).unwrap();
        output_stream.push(quote! {
            #opt_tag => {
                #format_option_decode_stream
            }
        });
    }

    Ok(quote! {
        /************************************************************************
         * This Section is super error prone
		 * Nested match should be more efficient keeping the 4 bit version outside
		 * Because TV of 1 bytes could match with full 1 byte tag in worst case
		 * scenerio, if 4bit kept outside this would not be the case.
		 * 4 bit version should only match with 4 bit tlv_config.tag, currently
		 * all tags are matched with 4 bit tag. 
         **************************************************************************/

        while __bytes.remaining() != 0 {
            let __tag: u8 = *__bytes.chunk().first().ok_or(TlvError::Unknown)?;
            let __4bitTag: u8 = __tag >> 4;

            if (__4bitTag < 0x8) {
                match __tag as usize {
                    #(#output_stream)*
                    _ => {
                        // Currently panicing for unknown tag, a better impl is required
                        ::std::panic!("Unknown tag in Optional TLV parsing")
                    }
                }
            } else {
                match __4bitTag as usize {
                    #(#output_stream)*
                    _ => {
                        // Currently panicing for unknown tag, a better impl is required
                        ::std::panic!("Unknown tag in Optional TLV parsing")
                    }
                }
            }
            // match __4bitTag as usize {
            //     #(#output_stream)*
            //     _ => {
            //         match __tag as usize {
            //             #(#output_stream)*
            //             _ => {
            //                 // Currently panicing for unknown tag, a better impl is required
            //                 ::std::panic!("Unknown tag in Optional TLV parsing")
            //             }
            //         }
            //     }
            // }
        }
    })
}

fn impl_tlv_decode(struct_name: Ident, data_struct: DataStruct) -> Result<TokenStream, Error> {
    let mut output_stream = Vec::<TokenStream>::new();
    let mut field_names = Vec::<Ident>::new();

    let mut optional_tlvs: Vec<(GenericArgument, Field, TlvConfig)> =
        Vec::<(GenericArgument, Field, TlvConfig)>::new();

    let mut temp_first_value_of_4bit_value: Option<Field> = None;
    let mut is_4bit_value_packed = true;

    let mut has_optional_fields_started = false;

    for field in data_struct.fields {
        let field_name = field.clone().ident.unwrap();
        field_names.push(field_name.clone());
        let tlv_config = TlvConfig::from_attributes(field.attrs.clone())?;

        match field.clone().ty {
            Type::Path(type_path) => {
                if type_path.path.segments.len() == 1
                    && type_path.path.segments[0].ident == "Option"
                {
                    if let syn::PathArguments::AngleBracketed(args) =
                        &type_path.path.segments[0].arguments
                    {
                        if args.args.len() == 1 {
                            has_optional_fields_started = true;
                            let inner_type = &args.args[0];
                            optional_tlvs.push((inner_type.clone(), field.clone(), tlv_config));
                            output_stream.push(quote! {
                                let mut #field_name: Option<#inner_type> = None;
                            });
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
                        temp_first_value_of_4bit_value = Some(field);
                        is_4bit_value_packed = false;
                        continue;
                    }
                    output_stream.push(
                        format_4bit_v_decode(
                            temp_first_value_of_4bit_value.clone().unwrap(),
                            field,
                            tlv_config,
                        )
                        .unwrap(),
                    );
                    is_4bit_value_packed = true;
                } else {
                    output_stream.push(format_v_decode(field, tlv_config).unwrap());
                }
            }
            "TLV" | "TLV-E" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_tlv_decode(field, tlv_config).unwrap());
            }
            "LV" | "LV-E" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_lv_decode(field, tlv_config).unwrap());
            }
            "TV" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_tv_decode(field, tlv_config).unwrap());
            }
            "T" => {
                if !is_4bit_value_packed {
                    abort_call_site!("Two 4bit value should be consecutive")
                }
                output_stream.push(format_t_decode(field, tlv_config).unwrap());
            }
            _ => {
                abort_call_site!("Unkown TLV format")
            }
        }
    }

    output_stream.push(init_option_decoder(optional_tlvs).unwrap());

    Ok(quote! {
        impl TlvDecode for #struct_name {
            fn decode(mut __bytes: Bytes, length: usize) -> Result<Self, tlv::prelude::TlvError> {
                #(#output_stream)*
                Ok(#struct_name{
                    #(#field_names),*
                })
            }
        }
    })
}

fn impl_newtype_decode(struct_name: Ident) -> Result<TokenStream, Error> {
    Ok(quote! {
        impl TlvDecode for #struct_name {
            fn decode(mut bytes: Bytes, _length: usize) -> Result<Self, tlv::prelude::TlvError> {
                let inner = <_>::decode(bytes, _length)?;
                Ok(#struct_name(inner))
            }
        }
    })
}


pub(crate) fn tlv_decode(token_stream: TokenStream) -> Result<TokenStream, Error> {
    let DeriveInput { data, .. } = syn::parse2(token_stream.clone())?;
    let struct_name = get_struct_name(token_stream.clone());

    match data {
        syn::Data::Struct(data_struct) => {
            if is_newtype(&data_struct) {
                impl_newtype_decode(struct_name)
            } else {
                impl_tlv_decode(struct_name, data_struct)
            }
        },
        _ => {
            abort_call_site!("Currently only structs are supported");
        }
    }
}