#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_generated_commands");
///[VkIndirectCommandsTokenTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) - Enum specifying token commands
///# C Specifications
///Possible values of those elements of the
///[`IndirectCommandsLayoutCreateInfoNV::p_tokens`] array specifying
///command tokens (other elements of the array specify command parameters) are:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectCommandsTokenTypeNV {
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV = 0,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV = 1,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV = 2,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV = 3,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV = 4,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV = 5,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV = 6,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV = 7,
///} VkIndirectCommandsTokenTypeNV;
///```
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutTokenNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IndirectCommandsTokenTypeNV(i32);
impl const Default for IndirectCommandsTokenTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("IndirectCommandsTokenTypeNV")
            .field(match *self {
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP => &"INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS => &"INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER => &"INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER => &"INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT => &"INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS",
                other => unreachable!("invalid value for `IndirectCommandsTokenTypeNV`: {:?}", other),
            })
            .finish()
    }
}
impl IndirectCommandsTokenTypeNV {
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP: Self = Self(0);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS: Self = Self(1);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER: Self = Self(2);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER: Self = Self(3);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT: Self = Self(4);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED: Self = Self(5);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW: Self = Self(6);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS: Self = Self(7);
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
