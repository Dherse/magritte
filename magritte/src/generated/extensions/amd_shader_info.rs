#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_info");
///[VkShaderInfoTypeAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderInfoTypeAMD.html) - Enum specifying which type of shader information to query
///# C Specifications
///Possible values of [`GetShaderInfoAMD`]`::infoType`, specifying the
///information being queried from a shader, are:
///```c
///// Provided by VK_AMD_shader_info
///typedef enum VkShaderInfoTypeAMD {
///    VK_SHADER_INFO_TYPE_STATISTICS_AMD = 0,
///    VK_SHADER_INFO_TYPE_BINARY_AMD = 1,
///    VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD = 2,
///} VkShaderInfoTypeAMD;
///```
///# Description
/// - [`SHADER_INFO_TYPE_STATISTICS`] specifies that device resources
///used by a shader will be queried.
/// - [`SHADER_INFO_TYPE_BINARY`] specifies that
///implementation-specific information will be queried.
/// - [`SHADER_INFO_TYPE_DISASSEMBLY`] specifies that human-readable
///dissassembly of a shader.
///# Related
/// - [`VK_AMD_shader_info`]
/// - [`GetShaderInfoAMD`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderInfoTypeAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderInfoTypeAMD(i32);
impl const Default for ShaderInfoTypeAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ShaderInfoTypeAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ShaderInfoTypeAMD")
            .field(match *self {
                Self::SHADER_INFO_TYPE_STATISTICS => &"SHADER_INFO_TYPE_STATISTICS",
                Self::SHADER_INFO_TYPE_BINARY => &"SHADER_INFO_TYPE_BINARY",
                Self::SHADER_INFO_TYPE_DISASSEMBLY => &"SHADER_INFO_TYPE_DISASSEMBLY",
                other => unreachable!("invalid value for `ShaderInfoTypeAMD`: {:?}", other),
            })
            .finish()
    }
}
impl ShaderInfoTypeAMD {
    ///[`SHADER_INFO_TYPE_STATISTICS`] specifies that device resources
    ///used by a shader will be queried.
    pub const SHADER_INFO_TYPE_STATISTICS: Self = Self(0);
    ///[`SHADER_INFO_TYPE_BINARY`] specifies that
    ///implementation-specific information will be queried.
    pub const SHADER_INFO_TYPE_BINARY: Self = Self(1);
    ///[`SHADER_INFO_TYPE_DISASSEMBLY`] specifies that human-readable
    ///dissassembly of a shader.
    pub const SHADER_INFO_TYPE_DISASSEMBLY: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
