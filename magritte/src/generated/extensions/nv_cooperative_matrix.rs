#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_cooperative_matrix");
///[VkScopeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkScopeNV.html) - Specify SPIR-V scope
///# C Specifications
///Possible values for [`ScopeNV`] include:
///```c
///// Provided by VK_NV_cooperative_matrix
///typedef enum VkScopeNV {
///    VK_SCOPE_DEVICE_NV = 1,
///    VK_SCOPE_WORKGROUP_NV = 2,
///    VK_SCOPE_SUBGROUP_NV = 3,
///    VK_SCOPE_QUEUE_FAMILY_NV = 5,
///} VkScopeNV;
///```
///# Description
/// - [`SCOPE_DEVICE`] corresponds to SPIR-V [`Device`] scope.
/// - [`SCOPE_WORKGROUP`] corresponds to SPIR-V `Workgroup` scope.
/// - [`SCOPE_SUBGROUP`] corresponds to SPIR-V `Subgroup` scope.
/// - [`SCOPE_QUEUE_FAMILY`] corresponds to SPIR-V `QueueFamily`
///scope.All enum values match the corresponding SPIR-V value.
///# Related
/// - [`VK_NV_cooperative_matrix`]
/// - [`CooperativeMatrixPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkScopeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ScopeNV(i32);
impl const Default for ScopeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ScopeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ScopeNV")
            .field(match *self {
                Self::SCOPE_DEVICE => &"SCOPE_DEVICE",
                Self::SCOPE_WORKGROUP => &"SCOPE_WORKGROUP",
                Self::SCOPE_SUBGROUP => &"SCOPE_SUBGROUP",
                Self::SCOPE_QUEUE_FAMILY => &"SCOPE_QUEUE_FAMILY",
                other => unreachable!("invalid value for `ScopeNV`: {:?}", other),
            })
            .finish()
    }
}
impl ScopeNV {
    ///[`SCOPE_DEVICE`] corresponds to SPIR-V [`Device`] scope.
    pub const SCOPE_DEVICE: Self = Self(1);
    ///[`SCOPE_WORKGROUP`] corresponds to SPIR-V `Workgroup` scope.
    pub const SCOPE_WORKGROUP: Self = Self(2);
    ///[`SCOPE_SUBGROUP`] corresponds to SPIR-V `Subgroup` scope.
    pub const SCOPE_SUBGROUP: Self = Self(3);
    ///[`SCOPE_QUEUE_FAMILY`] corresponds to SPIR-V `QueueFamily`
    ///scope.
    pub const SCOPE_QUEUE_FAMILY: Self = Self(5);
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
///[VkComponentTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkComponentTypeNV.html) - Specify SPIR-V cooperative matrix component type
///# C Specifications
///Possible values for [`ComponentTypeNV`] include:
///```c
///// Provided by VK_NV_cooperative_matrix
///typedef enum VkComponentTypeNV {
///    VK_COMPONENT_TYPE_FLOAT16_NV = 0,
///    VK_COMPONENT_TYPE_FLOAT32_NV = 1,
///    VK_COMPONENT_TYPE_FLOAT64_NV = 2,
///    VK_COMPONENT_TYPE_SINT8_NV = 3,
///    VK_COMPONENT_TYPE_SINT16_NV = 4,
///    VK_COMPONENT_TYPE_SINT32_NV = 5,
///    VK_COMPONENT_TYPE_SINT64_NV = 6,
///    VK_COMPONENT_TYPE_UINT8_NV = 7,
///    VK_COMPONENT_TYPE_UINT16_NV = 8,
///    VK_COMPONENT_TYPE_UINT32_NV = 9,
///    VK_COMPONENT_TYPE_UINT64_NV = 10,
///} VkComponentTypeNV;
///```
///# Description
/// - [`COMPONENT_TYPE_FLOAT16`] corresponds to SPIR-V
///`OpTypeFloat` 16.
/// - [`COMPONENT_TYPE_FLOAT32`] corresponds to SPIR-V
///`OpTypeFloat` 32.
/// - [`COMPONENT_TYPE_FLOAT64`] corresponds to SPIR-V
///`OpTypeFloat` 64.
/// - [`COMPONENT_TYPE_SINT8`] corresponds to SPIR-V `OpTypeInt` 8 1.
/// - [`COMPONENT_TYPE_SINT16`] corresponds to SPIR-V `OpTypeInt`
///16 1.
/// - [`COMPONENT_TYPE_SINT32`] corresponds to SPIR-V `OpTypeInt`
///32 1.
/// - [`COMPONENT_TYPE_SINT64`] corresponds to SPIR-V `OpTypeInt`
///64 1.
/// - [`COMPONENT_TYPE_UINT8`] corresponds to SPIR-V `OpTypeInt` 8 0.
/// - [`COMPONENT_TYPE_UINT16`] corresponds to SPIR-V `OpTypeInt`
///16 0.
/// - [`COMPONENT_TYPE_UINT32`] corresponds to SPIR-V `OpTypeInt`
///32 0.
/// - [`COMPONENT_TYPE_UINT64`] corresponds to SPIR-V `OpTypeInt`
///64 0.
///# Related
/// - [`VK_NV_cooperative_matrix`]
/// - [`CooperativeMatrixPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkComponentTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ComponentTypeNV(i32);
impl const Default for ComponentTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ComponentTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ComponentTypeNV")
            .field(match *self {
                Self::COMPONENT_TYPE_FLOAT16 => &"COMPONENT_TYPE_FLOAT16",
                Self::COMPONENT_TYPE_FLOAT32 => &"COMPONENT_TYPE_FLOAT32",
                Self::COMPONENT_TYPE_FLOAT64 => &"COMPONENT_TYPE_FLOAT64",
                Self::COMPONENT_TYPE_SINT8 => &"COMPONENT_TYPE_SINT8",
                Self::COMPONENT_TYPE_SINT16 => &"COMPONENT_TYPE_SINT16",
                Self::COMPONENT_TYPE_SINT32 => &"COMPONENT_TYPE_SINT32",
                Self::COMPONENT_TYPE_SINT64 => &"COMPONENT_TYPE_SINT64",
                Self::COMPONENT_TYPE_UINT8 => &"COMPONENT_TYPE_UINT8",
                Self::COMPONENT_TYPE_UINT16 => &"COMPONENT_TYPE_UINT16",
                Self::COMPONENT_TYPE_UINT32 => &"COMPONENT_TYPE_UINT32",
                Self::COMPONENT_TYPE_UINT64 => &"COMPONENT_TYPE_UINT64",
                other => unreachable!("invalid value for `ComponentTypeNV`: {:?}", other),
            })
            .finish()
    }
}
impl ComponentTypeNV {
    ///[`COMPONENT_TYPE_FLOAT16`] corresponds to SPIR-V
    ///`OpTypeFloat` 16.
    pub const COMPONENT_TYPE_FLOAT16: Self = Self(0);
    ///[`COMPONENT_TYPE_FLOAT32`] corresponds to SPIR-V
    ///`OpTypeFloat` 32.
    pub const COMPONENT_TYPE_FLOAT32: Self = Self(1);
    ///[`COMPONENT_TYPE_FLOAT64`] corresponds to SPIR-V
    ///`OpTypeFloat` 64.
    pub const COMPONENT_TYPE_FLOAT64: Self = Self(2);
    ///[`COMPONENT_TYPE_SINT8`] corresponds to SPIR-V `OpTypeInt` 8 1.
    pub const COMPONENT_TYPE_SINT8: Self = Self(3);
    ///[`COMPONENT_TYPE_SINT16`] corresponds to SPIR-V `OpTypeInt`
    ///16 1.
    pub const COMPONENT_TYPE_SINT16: Self = Self(4);
    ///[`COMPONENT_TYPE_SINT32`] corresponds to SPIR-V `OpTypeInt`
    ///32 1.
    pub const COMPONENT_TYPE_SINT32: Self = Self(5);
    ///[`COMPONENT_TYPE_SINT64`] corresponds to SPIR-V `OpTypeInt`
    ///64 1.
    pub const COMPONENT_TYPE_SINT64: Self = Self(6);
    ///[`COMPONENT_TYPE_UINT8`] corresponds to SPIR-V `OpTypeInt` 8 0.
    pub const COMPONENT_TYPE_UINT8: Self = Self(7);
    ///[`COMPONENT_TYPE_UINT16`] corresponds to SPIR-V `OpTypeInt`
    ///16 0.
    pub const COMPONENT_TYPE_UINT16: Self = Self(8);
    ///[`COMPONENT_TYPE_UINT32`] corresponds to SPIR-V `OpTypeInt`
    ///32 0.
    pub const COMPONENT_TYPE_UINT32: Self = Self(9);
    ///[`COMPONENT_TYPE_UINT64`] corresponds to SPIR-V `OpTypeInt`
    ///64 0.
    pub const COMPONENT_TYPE_UINT64: Self = Self(10);
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
