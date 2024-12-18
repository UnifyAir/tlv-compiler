use syn::Type;

pub fn is_u4_type(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if let Some(last) = tp.path.segments.last() {
            return last.ident == "u4" && last.arguments.is_empty();
        }
    }
    false
}