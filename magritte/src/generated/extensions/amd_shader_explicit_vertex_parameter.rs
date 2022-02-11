use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_AMD_shader_explicit_vertex_parameter");