use std::env;

use cc::Build;

extern crate cc;
extern crate bindgen;

// This code is heavily based off of the vk-mem-rs one.
fn main() {
    let mut build = Build::new();

    build.include("../vendors/VulkanMemoryAllocator/src");
    build.include("../vendors/VulkanMemoryAllocator/include");
    build.include("../vendors/Vulkan-Headers/include/");

    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", "");

    // Disable static linking
    build.define("VMA_STATIC_VULKAN_FUNCTIONS", "0");

    // Disable automatic loading
    build.define("VMA_DYNAMIC_VULKAN_FUNCTIONS", "0");

    // Set the maximum Vulkan version to 1.3
    build.define("VMA_VULKAN_VERSION ", "1003000");

    // Enable relevant extensions
    build.define("VMA_DEDICATED_ALLOCATION", "1");
    build.define("VMA_MEMORY_BUDGET", "1");
    build.define("VMA_BUFFER_DEVICE_ADDRESS", "1");
    build.define("VMA_MEMORY_PRIORITY ", "1");
    build.define("VMA_BIND_MEMORY2", "1");
    build.define("VMA_VULKAN_VERSION", "1");
    build.define("VMA_EXTERNAL_MEMORY", "1");

    // Enable recording if the feature is enabled
    #[cfg(feature = "recording")]
    build.define("VMA_RECORDING_ENABLED", "1");

    // Add the source file
    build.file("wrapper.cpp");

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-nullability-completeness")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("ios") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("android") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else if target.contains("windows") && target.contains("gnu") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-type-limits")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    }

    // build and link
    build.compile("vma");

    generate_bindings("bindings.rs")

    
}

fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I../vendors/Vulkan-Headers/include/")
        .header("../vendors/VulkanMemoryAllocator/include/vk_mem_alloc.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .blocklist_type("__darwin_.*")
        .blocklist_type("VmaAllocator")
        .blocklist_type("VmaPool")
        .blocklist_type("VmaAllocation")
        .blocklist_type("VmaDefragmentationContext")
        .blocklist_type("VmaVirtualAllocation")
        .blocklist_type("VmaVirtualBlock")
        .blocklist_type("VmaAllocator_T")
        .blocklist_type("VmaPool_T")
        .blocklist_type("VmaAllocation_T")
        .blocklist_type("VmaDefragmentationContext_T")
        .blocklist_type("VmaVirtualAllocation_T")
        .blocklist_type("VmaVirtualBlock_T")
        .blocklist_type("VmaVulkanFunctions")
        .blocklist_type("VmaAllocatorCreateFlags")
        .blocklist_type("VmaAllocationCreateFlags")
        .blocklist_type("VmaPoolCreateFlags")
        .blocklist_type("VmaDefragmentationFlags")
        .blocklist_type("VmaVirtualBlockCreateFlags")
        .blocklist_type("VmaVirtualAllocationCreateFlags")
        .blocklist_type("Vk.*")
        .blocklist_type("PFN_vk.*")
        .allowlist_function("vma.*")
        .parse_callbacks(Box::new(FixMagritteTypes))
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[derive(Debug)]
struct FixMagritteTypes;

impl bindgen::callbacks::ParseCallbacks for FixMagritteTypes {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("Vk") {
            // Strip `Vk` prefix, will use `ash::vk::*` instead
            Some(original_item_name.trim_start_matches("Vk").to_string())
        } else {
            None
        }
    }

    // When ignoring `Vk` types, bindgen loses derives for some type. Quick workaround.
    fn add_derives(&self, name: &str) -> Vec<String> {
        if name.starts_with("VmaAllocationInfo") || name.starts_with("VmaDefragmentationStats") {
            vec!["Debug".into(), "Copy".into(), "Clone".into()]
        } else {
            vec![]
        }
    }
}