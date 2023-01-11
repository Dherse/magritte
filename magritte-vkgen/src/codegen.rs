use crate::{
    origin::Origin,
    source::{
        Alias, Basetype, BitFlag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function, FunctionPointer,
        Handle, OpaqueType, Source, Struct, Union,
    },
};

const VERSIONS: &[Origin<'static>] = &[
    Origin::Vulkan1_0,
    Origin::Vulkan1_1,
    Origin::Vulkan1_2,
    Origin::Vulkan1_3,
];

bitflags::bitflags! {
    pub struct VisitorFlags: u32 {
        const ORIGINS = 1;
        const VERSIONS = 2;
        const EXTENSIONS = 4;

        const ALL_SOURCES = Self::ORIGINS.bits()
            | Self::VERSIONS.bits()
            | Self::EXTENSIONS.bits();

        const OPAQUE_TYPES = 1 << 4;
        const ALIASES = 2 << 4;
        const STRUCTS = 4 << 4;
        const UNIONS = 8 << 4;
        const HANDLES = 16 << 4;
        const BASE_TYPES = 32 << 4;
        const FUNCTION_POINTERS = 64 << 4;
        const BITMASKS = 128 << 4;
        const CONSTANTS = 256 << 4;
        const CONSTANT_ALIASES = 512 << 4;
        const BITFLAGS = 1024 << 4;
        const ENUMS = 2048 << 4;
        const COMMAND_ALIASES = 4096 << 4;
        const FUNCTIONS = 819 << 4;
        const COMMANDS = 16384 << 4;

        const ALL_OBJECTS = Self::OPAQUE_TYPES.bits()
            | Self::ALIASES.bits()
            | Self::STRUCTS.bits()
            | Self::UNIONS.bits()
            | Self::HANDLES.bits()
            | Self::BASE_TYPES.bits()
            | Self::FUNCTION_POINTERS.bits()
            | Self::BITMASKS.bits()
            | Self::CONSTANTS.bits()
            | Self::CONSTANT_ALIASES.bits()
            | Self::BITFLAGS.bits()
            | Self::ENUMS.bits()
            | Self::COMMAND_ALIASES.bits()
            | Self::FUNCTIONS.bits()
            | Self::COMMANDS.bits();
    }
}

impl<'a> Source<'a> {
    pub fn visit_all<V: Visitor>(&self, visitor: &mut V) {
        self.visit(visitor, VisitorFlags::all())
    }

    pub fn visit<V: Visitor>(&self, visitor: &mut V, flags: VisitorFlags) {
        if flags.contains(VisitorFlags::ORIGINS) {
            for origin in self.origins.iter().filter(|o| !o.is_disabled()) {
                let visitor = visitor.visit_origin(self, origin);
                self.visit_origin(origin, visitor, flags).finish();
            }
        }

        if flags.contains(VisitorFlags::VERSIONS) {
            for origin in VERSIONS {
                let visitor = visitor.visit_version(self, origin);
                self.visit_origin(origin, visitor, flags).finish();
            }
        }

        if flags.contains(VisitorFlags::EXTENSIONS) {
            for extension in self.extensions.iter().filter(|e| !e.disabled()) {
                let visitor = visitor.visit_extension(self, extension);
                self.visit_origin(extension.origin(), visitor, flags).finish();
            }
        }
    }

    fn visit_origin<'b, V: OriginVisitor<'b>>(&self, origin: &Origin<'a>, mut visitor: V, flags: VisitorFlags) -> V {
        if flags.contains(VisitorFlags::OPAQUE_TYPES) {
            self.opaque_types
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_opaque_type(self, v));
        }

        if flags.contains(VisitorFlags::ALIASES) {
            self.aliases
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_type_alias(self, v));
        }

        if flags.contains(VisitorFlags::STRUCTS) {
            self.structs
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_struct(self, v));
        }

        if flags.contains(VisitorFlags::UNIONS) {
            self.unions
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_union(self, v));
        }

        if flags.contains(VisitorFlags::HANDLES) {
            self.handles
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_handle(self, v));
        }

        if flags.contains(VisitorFlags::FUNCTION_POINTERS) {
            self.funcpointers
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_function_pointer(self, v));
        }

        if flags.contains(VisitorFlags::BASE_TYPES) {
            self.basetypes
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_base_type(self, v));
        }

        if flags.contains(VisitorFlags::BITMASKS) {
            self.bitmasks
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_bitmask(self, v));
        }

        if flags.contains(VisitorFlags::CONSTANTS) {
            self.constants
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_const(self, v));
        }

        if flags.contains(VisitorFlags::CONSTANT_ALIASES) {
            self.constant_aliases
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_const_alias(self, v));
        }

        if flags.contains(VisitorFlags::BITFLAGS) {
            self.bitflags
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_bitflag(self, v));
        }

        if flags.contains(VisitorFlags::ENUMS) {
            self.enums
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_enum(self, v));
        }

        if flags.contains(VisitorFlags::COMMAND_ALIASES) {
            self.command_aliases
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_command_alias(self, v));
        }

        if flags.contains(VisitorFlags::FUNCTIONS) {
            self.functions
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_function(self, v));
        }

        if flags.contains(VisitorFlags::COMMANDS) {
            self.commands
                .iter()
                .filter(|v| v.origin() == origin)
                .for_each(|v| visitor.visit_command(self, v));
        }

        visitor
    }
}

pub trait Visitor {
    type OriginVisitor<'parent>: OriginVisitor<'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent>: OriginVisitor<'parent>
    where
        Self: 'parent;

    type ExtensionVisitor<'parent>: OriginVisitor<'parent>
    where
        Self: 'parent;

    fn visit_origin<'a>(&mut self, source: &Source<'a>, origin: &Origin<'a>) -> Self::OriginVisitor<'_>;

    fn visit_version<'a>(&mut self, source: &Source<'a>, origin: &Origin<'a>) -> Self::VersionVisitor<'_>;

    fn visit_extension<'a>(&mut self, source: &Source<'a>, extension: &Extension<'a>) -> Self::ExtensionVisitor<'_>;
}

pub trait OriginVisitor<'parent>: Sized + 'parent {
    #[allow(unused_variables)]
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {}

    #[allow(unused_variables)]
    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {}

    #[allow(unused_variables)]
    fn visit_opaque_type<'a>(&mut self, source: &Source<'a>, opaque_type: &OpaqueType<'a>) {}

    #[allow(unused_variables)]
    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {}

    #[allow(unused_variables)]
    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {}

    #[allow(unused_variables)]
    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {}

    #[allow(unused_variables)]
    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {}

    #[allow(unused_variables)]
    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {}

    #[allow(unused_variables)]
    fn visit_base_type<'a>(&mut self, source: &Source<'a>, base_type: &Basetype<'a>) {}

    #[allow(unused_variables)]
    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {}

    #[allow(unused_variables)]
    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &BitFlag<'a>) {}

    #[allow(unused_variables)]
    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {}

    #[allow(unused_variables)]
    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {}

    #[allow(unused_variables)]
    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {}

    #[allow(unused_variables)]
    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {}

    fn finish(self) {}
}

impl<'parent> OriginVisitor<'parent> for () {}

impl<'parent, V: OriginVisitor<'parent>> OriginVisitor<'parent> for Box<V> {
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {
        <V as OriginVisitor<'parent>>::visit_const(&mut **self, source, const_)
    }

    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {
        <V as OriginVisitor<'parent>>::visit_const_alias(&mut **self, source, const_alias)
    }

    fn visit_opaque_type<'a>(&mut self, source: &Source<'a>, opaque_type: &OpaqueType<'a>) {
        <V as OriginVisitor<'parent>>::visit_opaque_type(&mut **self, source, opaque_type)
    }

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {
        <V as OriginVisitor<'parent>>::visit_type_alias(&mut **self, source, alias)
    }

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        <V as OriginVisitor<'parent>>::visit_struct(&mut **self, source, struct_)
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        <V as OriginVisitor<'parent>>::visit_union(&mut **self, source, union_)
    }

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {
        <V as OriginVisitor<'parent>>::visit_handle(&mut **self, source, handle)
    }

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {
        <V as OriginVisitor<'parent>>::visit_function_pointer(&mut **self, source, function_pointer)
    }

    fn visit_base_type<'a>(&mut self, source: &Source<'a>, base_type: &Basetype<'a>) {
        <V as OriginVisitor<'parent>>::visit_base_type(&mut **self, source, base_type)
    }

    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {
        <V as OriginVisitor<'parent>>::visit_bitmask(&mut **self, source, bitmask)
    }

    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &BitFlag<'a>) {
        <V as OriginVisitor<'parent>>::visit_bitflag(&mut **self, source, bitflag)
    }

    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {
        <V as OriginVisitor<'parent>>::visit_enum(&mut **self, source, enum_)
    }

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {
        <V as OriginVisitor<'parent>>::visit_command_alias(&mut **self, source, command_alias)
    }

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {
        <V as OriginVisitor<'parent>>::visit_function(&mut **self, source, function)
    }

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {
        <V as OriginVisitor<'parent>>::visit_command(&mut **self, source, command)
    }
}
