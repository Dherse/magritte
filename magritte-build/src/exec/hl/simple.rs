use magritte_types::{Native, Source, Struct, Ty, TypeRef};

use crate::edge_case::EdgeCase;

pub fn is_struct_simple(
    edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
    source: &Source<'_>,
    struct_: &Struct<'_>,
) -> bool {
    let fields = struct_
        .fields()
        .iter()
        .filter(|field| edge_cases.field_filter(source, TypeRef::Struct(struct_), field))
        .collect::<Vec<_>>();

    if fields.len() != struct_.fields().len() {
        return false;
    }

    for field in fields {
        let mut ty = field.ty();
        'inner: loop {
            match ty {
                Ty::Native(_) => break 'inner,
                Ty::Named(name) if name == "VkBool32" => return false,
                Ty::Named(name) => match source.resolve_type(name).expect("unknown type") {
                    TypeRef::Union(_) | TypeRef::FunctionPointer(_) | TypeRef::Handle(_) => return false,
                    TypeRef::Struct(s) => {
                        if !is_struct_simple(edge_cases, source, s) {
                            return false;
                        } else {
                            break 'inner;
                        }
                    },
                    TypeRef::OpaqueType(_)
                    | TypeRef::Basetype(_)
                    | TypeRef::Bitmask(_)
                    | TypeRef::Bitflag(_)
                    | TypeRef::Enum(_) => {
                        break 'inner;
                    },
                    other => unreachable!("type reference is incorrect: {other:?}"),
                },
                Ty::StringArray(_) => return false,
                Ty::NullTerminatedString(_) => return false,
                Ty::Array(box Ty::Native(Native::Char), _) => return false,
                Ty::Array(inner, _) => ty = inner,
                Ty::Pointer(_, _) | Ty::Slice(_, _, _) => return false,
            }
        }
    }

    true
}
