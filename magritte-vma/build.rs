use std::env;

use cc::Build;

extern crate cc;

// This code is heavily based off of the vk-mem-rs one.
fn main() {
    let mut build = Build::new();

    build.include("../vendors/VulkanMemoryAllocator/src");
    build.include("../vendors/VulkanMemoryAllocator/include");
    build.include("../vendors/Vulkan-Headers/include/");

    #[cfg(not(debug_assertions))]
    build.define("NDEBUG", "");

    // Enable corruption detection in debug mode
    #[cfg(debug_assertions)]
    build.define("VMA_DEBUG_MARGIN", "16");
    #[cfg(debug_assertions)]
    build.define("VMA_DEBUG_DETECT_CORRUPTION", "1");

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
            .flag("-std=c++14")
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
            .flag("-std=c++14")
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
            .flag("-std=c++14")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++14")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else if target.contains("windows") && target.contains("gnu") {
        build
            .flag("-std=c++14")
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

    println!("cargo:rerun-if-changed=wrapper.cpp");
}
