use std::{
    collections::HashMap,
    iter::once,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use crate::{
    codegen::{OriginVisitor, Visitor},
    doc::Documentation,
    origin::Origin,
    source::{
        Alias, Basetype, BitFlag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function, FunctionPointer,
        Handle, OpaqueType, Source, Struct, Union,
    },
    symbols::SymbolName,
};

#[derive(Debug, Clone)]
pub struct BuiltDoc {
    pub path: PathBuf,
    pub members: Option<HashMap<String, PathBuf>>,
}

impl BuiltDoc {
    #[inline]
    pub fn new(path: PathBuf) -> Self {
        Self { path, members: None }
    }

    #[inline]
    pub fn with_members(path: PathBuf, members: HashMap<String, PathBuf>) -> Self {
        Self {
            path,
            members: Some(members),
        }
    }
}

pub struct DocVisitor {
    doc: Documentation,
    path_stub: PathBuf,
    doc_map: HashMap<String, BuiltDoc>,
}

impl DocVisitor {
    pub fn new<P: AsRef<Path>>(doc: Documentation, path_stub: P) -> Self {
        Self {
            doc,
            path_stub: path_stub.as_ref().into(),
            doc_map: HashMap::with_capacity(1000),
        }
    }

    pub fn find(&self, name: &str) -> Option<&BuiltDoc> {
        self.doc_map.get(name)
    }
}

impl Visitor for DocVisitor {
    type OriginVisitor<'parent> = DocOriginVisitor<'parent> where Self: 'parent;

    type VersionVisitor<'parent> = () where Self: 'parent;

    type ExtensionVisitor<'parent> = () where Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Self::OriginVisitor<'_> {
        DocOriginVisitor {
            parent: self,
            buffer: String::with_capacity(1 << 20),
        }
    }

    fn visit_version<'a>(&mut self, source: &Source<'a>, origin: &Origin<'a>) -> Self::VersionVisitor<'_> {
        let doc_name = match origin {
            Origin::Vulkan1_0 => "VK_VERSION_1_0",
            Origin::Vulkan1_1 => "VK_VERSION_1_1",
            Origin::Vulkan1_2 => "VK_VERSION_1_2",
            Origin::Vulkan1_3 => "VK_VERSION_1_3",
            _ => unreachable!("not a Vulkan version"),
        };

        if let Some(mut doc) = self.doc.find(doc_name) {
            let name = doc.name(source, origin);
            let desc = doc.description(source, origin, None);
            let related = doc.related(source);
            let copyright = doc.copyright();

            let mut buffer = String::with_capacity(1 << 20);

            buffer.extend(name);
            buffer.extend(desc);
            buffer.extend(related);
            buffer.extend(once(copyright));

            let root = origin.as_dir_path(&self.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", doc_name));
            std::fs::write(&path, &buffer).expect("Failed to write documentation");

            self.doc_map.insert(doc_name.to_string(), BuiltDoc::new(path));
        }

        ()
    }

    fn visit_extension<'a>(&mut self, source: &Source<'a>, extension: &Extension<'a>) -> Self::ExtensionVisitor<'_> {
        if let Some(mut doc) = self.doc.find(extension.name()) {
            let name = doc.name(source, extension);
            let desc = doc.description(source, extension, None);
            let ext = doc.extension(source, extension);

            let related = doc.related(source);
            let copyright = doc.copyright();

            let mut buffer = String::with_capacity(1 << 20);

            buffer.extend(name);
            buffer.extend(desc);
            buffer.extend(once(ext));
            buffer.extend(related);
            buffer.extend(once(copyright));

            let root = extension.origin().as_dir_path(&self.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", extension.name()));
            std::fs::write(&path, &buffer).expect("Failed to write documentation");

            self.doc_map.insert(extension.name().to_string(), BuiltDoc::new(path));
        }
        ()
    }
}

pub struct DocOriginVisitor<'parent> {
    parent: &'parent mut DocVisitor,
    buffer: String,
}

impl<'parent> Deref for DocOriginVisitor<'parent> {
    type Target = DocVisitor;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'parent> DerefMut for DocOriginVisitor<'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'parent> Drop for DocOriginVisitor<'parent> {
    fn drop(&mut self) {}
}

macro_rules! doc {
    (simple $this:ident, $source:ident, $var:ident) => {
        $this.buffer.clear();

        if let Some(mut doc) = $this.doc.find($var.original_name()) {
            let name = doc.name($source, $var);
            let specs = doc.specification($source, $var);
            let related = doc.related($source);
            let copyright = doc.copyright();

            $this.buffer.extend(name);
            $this.buffer.extend(specs);
            $this.buffer.extend(related);
            $this.buffer.extend(once(copyright));

            let root = $var.origin().as_dir_path(&$this.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", $var.original_name()));
            std::fs::write(&path, &$this.buffer).expect("Failed to write documentation");

            $this.doc_map.insert($var.name().to_string(), BuiltDoc::new(path));
        }
    };

    (with_members $this:ident, $source:ident, $var:ident) => {
        $this.buffer.clear();

        if let Some(mut doc) = $this.doc.find(&$var.original_name()) {
            let mut fields = HashMap::with_capacity($var.fields().len());

            let name = doc.name($source, $var);
            let specs = doc.specification($source, $var);
            let members = doc.members($source, $var, Some(&mut fields));
            let desc = doc.description($source, $var, fields.is_empty().then(|| &mut fields));
            let related = doc.related($source);
            let copyright = doc.copyright();

            $this.buffer.extend(name);
            $this.buffer.extend(specs);
            $this.buffer.extend(members);
            $this.buffer.extend(desc);
            $this.buffer.extend(related);
            $this.buffer.extend(once(copyright));

            let root = $var.origin().as_dir_path(&$this.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", $var.original_name()));
            std::fs::write(&path, &$this.buffer).expect("Failed to write documentation");

            let members = fields
                .into_iter()
                .map(|(name, doc)| {
                    let path = root.join(format!("{}${}.md", $var.original_name(), name));
                    std::fs::write(&path, &doc).expect("Failed to write field documentation");

                    (name, path)
                })
                .collect();

            $this
                .doc_map
                .insert($var.name().to_string(), BuiltDoc::with_members(path, members));
        }
    };

    (with_arguments $this:ident, $source:ident, $var:ident) => {
        $this.buffer.clear();

        if let Some(mut doc) = $this.doc.find(&$var.original_name()) {
            let mut members = HashMap::with_capacity($var.arguments().len());

            let name = doc.name($source, $var);
            let specs = doc.specification($source, $var);
            let arguments = doc.parameters($source, $var, Some(&mut members));
            let desc = doc.description($source, $var, members.is_empty().then(|| &mut members));
            let related = doc.related($source);
            let copyright = doc.copyright();

            $this.buffer.extend(name);
            $this.buffer.extend(specs);
            $this.buffer.extend(arguments);
            $this.buffer.extend(desc);
            $this.buffer.extend(related);
            $this.buffer.extend(once(copyright));

            let root = $var.origin().as_dir_path(&$this.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", $var.original_name()));
            std::fs::write(&path, &$this.buffer).expect("Failed to write documentation");

            let members = members
                .into_iter()
                .map(|(name, doc)| {
                    let path = root.join(format!("{}${}.md", $var.original_name(), name));
                    std::fs::write(&path, &doc).expect("Failed to write field documentation");

                    (name, path)
                })
                .collect();

            $this
                .doc_map
                .insert($var.name().to_string(), BuiltDoc::with_members(path, members));
        }
    };

    (with_desc $this:ident, $source:ident, $var:ident, $members:ident) => {
        $this.buffer.clear();

        if let Some(mut doc) = $this.doc.find(&$var.original_name()) {
            let mut members = HashMap::with_capacity($var.$members().len());

            let name = doc.name($source, $var);
            let specs = doc.specification($source, $var);
            let desc = doc.description($source, $var, Some(&mut members));
            let related = doc.related($source);
            let copyright = doc.copyright();

            $this.buffer.extend(name);
            $this.buffer.extend(specs);
            $this.buffer.extend(desc);
            $this.buffer.extend(related);
            $this.buffer.extend(once(copyright));

            let root = $var.origin().as_dir_path(&$this.path_stub).join("doc/");
            std::fs::create_dir_all(&root).expect("failed to create doc dir");

            let path = root.join(format!("{}.md", $var.original_name()));
            std::fs::write(&path, &$this.buffer).expect("Failed to write documentation");

            let members = members
                .into_iter()
                .map(|(name, doc)| {
                    let path = root.join(format!("{}${}.md", $var.original_name(), name));
                    std::fs::write(&path, &doc).expect("Failed to write field documentation");

                    (name, path)
                })
                .collect();

            $this
                .doc_map
                .insert($var.name().to_string(), BuiltDoc::with_members(path, members));
        }
    };
}

impl<'parent> OriginVisitor<'parent> for DocOriginVisitor<'parent> {
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {
        doc!(simple self, source, const_);
    }

    fn visit_const_alias<'a>(&mut self, _source: &Source<'a>, _const_alias: &ConstAlias<'a>) {}

    fn visit_opaque_type<'a>(&mut self, source: &Source<'a>, opaque_type: &OpaqueType<'a>) {
        doc!(simple self, source, opaque_type);
    }

    fn visit_type_alias<'a>(&mut self, _source: &Source<'a>, _alias: &Alias<'a>) {}

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        doc!(with_members self, source, struct_);
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        doc!(with_members self, source, union_);
    }

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {
        doc!(simple self, source, handle);
    }

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {
        doc!(with_arguments self, source, function_pointer);
    }

    fn visit_base_type<'a>(&mut self, source: &Source<'a>, base_type: &Basetype<'a>) {
        doc!(simple self, source, base_type);
    }

    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {
        doc!(simple self, source, bitmask);
    }

    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &BitFlag<'a>) {
        doc!(with_desc self, source, bitflag, bits);
    }

    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {
        doc!(with_desc self, source, enum_, variants);
    }

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {
        let command = source.find(command_alias.of()).expect("unknown command alias");

        let text = format!("See [`{}`] for more information", command.as_string_path());

        let root = command_alias.origin().as_dir_path(&self.path_stub).join("doc/");
        std::fs::create_dir_all(&root).expect("failed to create doc dir");

        let path = root.join(format!("{}.md", command_alias.original_name()));
        std::fs::write(&path, &text).expect("Failed to write documentation");
    }

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {
        let ptrs = function.as_function_pointer();
        self.visit_function_pointer(source, &ptrs);
    }

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {
        let ptrs = command.as_function_pointer();
        self.visit_function_pointer(source, &ptrs);
    }
}
