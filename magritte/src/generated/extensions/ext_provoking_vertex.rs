#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_provoking_vertex");
///[VkProvokingVertexModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html) - Specify which vertex in a primitive is the provoking vertex
///# C Specifications
///Possible values of
///[`PipelineRasterizationProvokingVertexStateCreateInfoEXT::provoking_vertex_mode`]
///are:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef enum VkProvokingVertexModeEXT {
///    VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT = 0,
///    VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT = 1,
///} VkProvokingVertexModeEXT;
///```
///# Description
/// - [`PROVOKING_VERTEX_MODE_FIRST_VERTEX`] specifies that the
///provoking vertex is the first non-adjacency vertex in the list of
///vertices used by a primitive.
/// - [`PROVOKING_VERTEX_MODE_LAST_VERTEX`] specifies that the
///provoking vertex is the last non-adjacency vertex in the list of
///vertices used by a primitive.These modes are described more precisely in
///[Primitive Topologies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topologies).
///# Related
/// - [`VK_EXT_provoking_vertex`]
/// - [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkProvokingVertexModeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ProvokingVertexModeEXT(i32);
impl const Default for ProvokingVertexModeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ProvokingVertexModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ProvokingVertexModeEXT")
            .field(match *self {
                Self::PROVOKING_VERTEX_MODE_FIRST_VERTEX => &"PROVOKING_VERTEX_MODE_FIRST_VERTEX",
                Self::PROVOKING_VERTEX_MODE_LAST_VERTEX => &"PROVOKING_VERTEX_MODE_LAST_VERTEX",
                other => unreachable!("invalid value for `ProvokingVertexModeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl ProvokingVertexModeEXT {
    ///[`PROVOKING_VERTEX_MODE_FIRST_VERTEX`] specifies that the
    ///provoking vertex is the first non-adjacency vertex in the list of
    ///vertices used by a primitive.
    pub const PROVOKING_VERTEX_MODE_FIRST_VERTEX: Self = Self(0);
    ///[`PROVOKING_VERTEX_MODE_LAST_VERTEX`] specifies that the
    ///provoking vertex is the last non-adjacency vertex in the list of
    ///vertices used by a primitive.
    pub const PROVOKING_VERTEX_MODE_LAST_VERTEX: Self = Self(1);
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
