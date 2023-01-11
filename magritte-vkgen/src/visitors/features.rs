use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};

use cargo_toml::Manifest;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    codegen::{OriginVisitor, Visitor},
    origin::Origin,
    rustmft::run_rustfmt,
    source::{
        Alias, CommandAlias, ConstAlias, DeprecationStatus, Extension, Function, FunctionPointer, Source, Struct, Union,
    },
};

pub struct FeatureVisitor {
    extensions: TokenStream,
    features: HashMap<String, Vec<String>>,
}

impl FeatureVisitor {
    pub fn new() -> Self {
        let mut features = HashMap::with_capacity(100);

        features.insert("VULKAN_1_2".to_owned(), Vec::new());

        features.insert("VULKAN_1_3".to_owned(), Vec::from(["VULKAN_1_2".to_owned()]));

        Self {
            extensions: TokenStream::new(),
            features,
        }
    }

    pub fn features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }

    pub fn features_mut(&mut self) -> &mut HashMap<String, Vec<String>> {
        &mut self.features
    }

    pub fn write_extensions_mod<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref().join("extensions.rs");
        let out = run_rustfmt(self.extensions.to_string()).expect("failed to run rustfmt");

        std::fs::write(path, out.as_bytes()).expect("failed to write extensions.toml");
    }

    pub fn write<P: AsRef<Path>>(mut self, path: P) {
        let contents = std::fs::read(path.as_ref()).unwrap();
        let mut manifest = Manifest::from_slice(&contents).unwrap();

        let mut features: BTreeMap<String, Vec<String>> = BTreeMap::default();

        features.insert(
            "window".to_string(),
            vec![
                "raw-window-handle".to_string(),
                "VK_KHR_surface".to_string(),
                "VK_KHR_win32_surface".to_string(),
                "VK_KHR_wayland_surface".to_string(),
                "VK_KHR_xlib_surface".to_string(),
                "VK_KHR_xcb_surface".to_string(),
                "VK_KHR_android_surface".to_string(),
                "VK_MVK_macos_surface".to_string(),
                "VK_EXT_metal_surface".to_string(),
            ],
        );

        features.insert(
            "validation".to_string(),
            vec!["log".to_string(), "VK_EXT_debug_utils".to_string()],
        );

        features.insert(
            "default".to_string(),
            vec!["libloading".to_string(), "smallvec".to_string()],
        );

        features.extend(
            self.features_mut()
                .drain()
                .map(|(k, v)| (k, Vec::from_iter(v.into_iter()))),
        );

        manifest.features = features;

        let out = toml::ser::to_string_pretty(&manifest).unwrap();

        std::fs::write(path, out.as_bytes()).expect("failed to write Cargo.toml");
    }
}

impl Visitor for FeatureVisitor {
    type OriginVisitor<'parent> = () where Self: 'parent;

    type VersionVisitor<'parent> = () where Self: 'parent;

    type ExtensionVisitor<'parent> = ExtensionVisitor<'parent> where Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Self::OriginVisitor<'_> {
        ()
    }

    fn visit_version<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Self::VersionVisitor<'_> {
        ()
    }

    fn visit_extension<'a>(&mut self, source: &Source<'a>, extension: &Extension<'a>) -> Self::ExtensionVisitor<'_> {
        ExtensionVisitor::new(self, source, extension)
    }
}

pub struct ExtensionVisitor<'parent> {
    parent: &'parent mut FeatureVisitor,
    origin: Origin<'static>,
    name: String,
    dependencies: HashSet<String>,
}

impl<'parent> ExtensionVisitor<'parent> {
    pub fn new<'a>(parent: &'parent mut FeatureVisitor, source: &Source<'a>, extension: &Extension) -> Self {
        let mut dependencies = HashSet::with_capacity(extension.required().len() + 2);

        match extension.min_core() {
            Origin::Core | Origin::Vulkan1_0 | Origin::Vulkan1_1 => (),
            core @ Origin::Vulkan1_2 | core @ Origin::Vulkan1_3 => dependencies.extend(
                core.feature_flag(source)
                    .expect("Vulkan 1.2/1.3 must have a feature flag"),
            ),
            core => unreachable!("unknown minimum core version: {core:?}"),
        }

        match extension.deprecation_status() {
            DeprecationStatus::PromotedVersion(version) => {
                if let Some(feature_flag) = version.feature_flag(source) {
                    dependencies.extend(feature_flag);
                }
            },
            DeprecationStatus::Promoted(ext) => {
                let ext = source.extensions.get_by_name(ext).expect("unknown extension");
                if let Some(feature_flag) = ext.origin().feature_flag(source) {
                    dependencies.extend(feature_flag);
                }
            },
            _ => {},
        }

        dependencies.extend(extension.required().iter().map(ToString::to_string));

        let cond = extension.origin().condition(source);
        let ident = extension.as_ident();
        parent.extensions.extend(quote! {
            #cond
            pub mod #ident;
        });

        Self {
            parent,
            origin: extension.origin().as_static(),
            name: extension.name().to_string(),
            dependencies,
        }
    }

    fn process_dependency<'a>(&mut self, source: &Source<'a>, owner: &Origin<'a>) {
        match owner {
            Origin::Extension(name, _, false) => {
                self.dependencies.insert(name.to_string());
            },
            core @ Origin::Vulkan1_2 => self.dependencies.extend(
                core.feature_flag(source)
                    .expect("Vulkan 1.2/1.3 must have a feature flag"),
            ),
            core @ Origin::Vulkan1_3 => self.dependencies.extend(
                core.feature_flag(source)
                    .expect("Vulkan 1.2/1.3 must have a feature flag"),
            ),
            _ => {},
        }
    }
}

impl<'parent> Drop for ExtensionVisitor<'parent> {
    fn drop(&mut self) {
        self.parent
            .features
            .insert(self.name.clone(), Vec::from_iter(self.dependencies.drain()));
    }
}

impl<'parent> OriginVisitor<'parent> for ExtensionVisitor<'parent> {
    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {
        let owner = source.resolve(const_alias.of()).expect("unknown alias").origin();
        if owner == &self.origin {
            return;
        }

        self.process_dependency(source, owner);
    }

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {
        let owner = source.resolve(alias.of()).expect("unknown alias").origin();
        if owner == &self.origin {
            return;
        }

        self.process_dependency(source, owner);
    }

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        for arg in struct_.fields() {
            if let Some(name) = arg.ty().base_named_type() {
                let owner = source.resolve_type(name).expect("unknown type").origin();
                if owner == &self.origin {
                    return;
                }

                self.process_dependency(source, owner);
            }
        }
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        for arg in union_.fields() {
            if let Some(name) = arg.ty().base_named_type() {
                let owner = source.resolve_type(name).expect("unknown type").origin();
                if owner == &self.origin {
                    return;
                }

                self.process_dependency(source, owner);
            }
        }
    }

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {
        for arg in function_pointer.arguments() {
            if let Some(name) = arg.ty().base_named_type() {
                let owner = source.resolve_type(name).expect("unknown type").origin();
                if owner == &self.origin {
                    return;
                }

                self.process_dependency(source, owner);
            }
        }
    }

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {
        let owner = source.resolve(command_alias.of()).expect("unknown alias").origin();
        if owner == &self.origin {
            return;
        }

        self.process_dependency(source, owner);
    }

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {
        self.visit_function_pointer(source, &function.as_function_pointer())
    }

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {
        self.visit_function(source, command);
    }
}
