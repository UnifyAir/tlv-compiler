use syn::{DataStruct, DeriveInput};
use proc_macro2::{TokenStream, Ident};
use proc_macro_error::abort_call_site;

pub fn get_struct_name(struct_stream: TokenStream) -> Ident {
	let input = syn::parse2::<DeriveInput>(struct_stream.clone()).unwrap();
	match input.data {
		syn::Data::Struct(_) => input.ident,
		_ => {
            abort_call_site!(
                "It's not a struct, check back !!!");
        },
	}
}

pub fn is_newtype(data_struct: &DataStruct) -> bool {
    match &data_struct.fields {
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => true,
        _ => false,
    }
}

// pub fn is_u4_type(ty: &Type) -> bool {
//     if let Type::Path(tp) = ty {
//         if let Some(last) = tp.path.segments.last() {
//             return last.ident == "u4" && last.arguments.is_empty();
//         }
//     }
//     false
// }

// fn extract_full_path_string(ty: &Type) -> Option<String> {
//     if let Type::Path(TypePath { qself: None, path }) = ty {
//         let full_path = path.segments
//             .iter()
//             .map(|segment| segment.ident.to_string())
//             .collect::<Vec<_>>()
//             .join("::");
//         Some(full_path)
//     } else {
//         None
//     }
// }

