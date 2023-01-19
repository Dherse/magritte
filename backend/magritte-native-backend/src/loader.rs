use std::{ffi::{OsStr, CStr}, ops::Deref};

use libloading::Library;
use magritte_common::cstr;

use crate::entry::EntryVTable;


#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
/// Lib path for Linux
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
/// Lib path for Android
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
/// Lib path for MacOS/iOS
const LIB_PATH: &str = "libvulkan.dylib";

#[cfg(windows)]
/// Lib path for Windows
const LIB_PATH: &str = "vulkan-1.dll";

pub struct Entry {
    pub library: Library,
    pub v_table: EntryVTable,
}

impl Deref for Entry {
    type Target = EntryVTable;

    fn deref(&self) -> &Self::Target {
        &self.v_table
    }
}

impl Entry {
    /// Tries to load the entry from a default location
    pub fn new() -> Result<Self, libloading::Error> {
        Self::from_location(LIB_PATH)
    }

    /// Tries to load the entry from a specified location
    pub fn from_location<P: AsRef<OsStr>>(path: P) -> Result<Self, libloading::Error> {
        // dynamically load the library
        let mut library = unsafe { Library::new(path)? };

        // function to more easily load functions.
        // thanks `erupt` for the code
        let symbol_fn = |library: &mut Library, name: &CStr| unsafe {
            library
                .get::<unsafe extern "system" fn()>(name.to_bytes_with_nul())
                .ok()
                .map(|symbol| *symbol)
        };

        let v_table = unsafe {
            EntryVTable {
                create_instance: std::mem::transmute(symbol_fn(&mut library, cstr!("vkCreateInstance")).expect("missing `vkCreateInstance`")),
                get_instance_proc_addr: std::mem::transmute(symbol_fn(&mut library, cstr!("vkGetInstanceProcAddr")).expect("missing `vkCreateInstance`")),
                enumerate_instance_version: symbol_fn(
                    &mut library,
                    cstr!("vkEnumerateInstanceVersion"),
                ).map(|t| std::mem::transmute(t)),
                enumerate_instance_layer_properties: std::mem::transmute(symbol_fn(
                    &mut library,
                    cstr!("vkEnumerateInstanceLayerProperties"),
                ).expect("missing `vkCreateInstance`")),
                enumerate_instance_extension_properties: std::mem::transmute(symbol_fn(
                    &mut library,
                    cstr!("vkEnumerateInstanceExtensionProperties"),
                ).expect("missing `vkCreateInstance`")),
            }
        };

        Ok(Self {
            library,
            v_table,
        })
    }
}