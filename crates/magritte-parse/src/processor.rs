use magritte_types::Source;
use vk_parse::Type;

use crate::Visitor;

pub(crate) fn process_type(out: &mut Source<'static>, type_: Type) {
    if type_.requires.is_some() {
        out.visit_opaque(type_);
        return;
    }

    if type_.alias.is_some() {
        out.visit_alias(type_);
        return;
    }

    match type_.category.as_ref().map(|s| s as &str) {
        Some("struct") if type_.name.is_some() => out.visit_struct(type_),
        Some("union") if type_.name.is_some() => out.visit_union(type_),
        Some("handle") if type_.name.is_some() => out.visit_handle(type_),
        Some("handle") => out.visit_handle_no_name(type_),
        Some("funcpointer") => out.function_pointer(type_),
        Some("basetype") => out.visit_base_type(type_),
        Some("bitmask") => out.visit_bitmask(type_),
        _ => (),
    }
}
