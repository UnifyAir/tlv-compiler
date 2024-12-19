use syn::{Type, TypePath};

pub fn is_u4_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last) = tp.path.segments.last() {
            return last.ident == "u4" && last.arguments.is_empty();
        }
    }
    false
}

fn extract_full_path_string(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { qself: None, path }) = ty {
        let full_path = path.segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        Some(full_path)
    } else {
        None
    }
}

