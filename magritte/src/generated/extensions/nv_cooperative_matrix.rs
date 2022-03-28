//![VK_NV_cooperative_matrix](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_cooperative_matrix.html) - device extension
//!# Description
//!This extension adds support for using cooperative matrix types in SPIR-V.
//!Cooperative matrix types are medium-sized matrices that are primarily
//!supported in compute shaders, where the storage for the matrix is spread
//!across all invocations in some scope (usually a subgroup) and those
//!invocations cooperate to efficiently perform matrix multiplies.Cooperative matrix types are
//! defined by the
//![`SPV_NV_cooperative_matrix`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_cooperative_matrix.html)
//!SPIR-V extension and can be used with the
//![`GL_NV_cooperative_matrix`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_cooperative_matrix.txt)
//!GLSL extension.This extension includes support for enumerating the matrix types and
//!dimensions that are supported by the implementation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_cooperative_matrix]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_cooperative_matrix extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceCooperativeMatrixPropertiesNV`]
//!# New structures
//! - [`CooperativeMatrixPropertiesNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceCooperativeMatrixFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceCooperativeMatrixPropertiesNV`]
//!# New enums
//! - [`ComponentTypeNV`]
//! - [`ScopeNV`]
//!# New constants
//! - [`NV_COOPERATIVE_MATRIX_EXTENSION_NAME`]
//! - [`NV_COOPERATIVE_MATRIX_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!(1) What matrix properties will be supported in practice? **RESOLVED** : In NVIDIA’s initial
//! implementation, we will support:
//! - AType = BType = fp16 CType = DType = fp16 MxNxK = 16x8x16 scope = Subgroup
//! - AType = BType = fp16 CType = DType = fp16 MxNxK = 16x8x8 scope = Subgroup
//! - AType = BType = fp16 CType = DType = fp32 MxNxK = 16x8x16 scope = Subgroup
//! - AType = BType = fp16 CType = DType = fp32 MxNxK = 16x8x8 scope = Subgroup
//!# Version History
//! - Revision 1, 2019-02-05 (Jeff Bolz)  - Internal revisions
//!# Other info
//! * 2019-02-05
//! * - This extension requires [`SPV_NV_cooperative_matrix`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_cooperative_matrix.html)
//!   - This extension provides API support for [`GL_NV_cooperative_matrix`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_cooperative_matrix.txt)
//! * - Jeff Bolz, NVIDIA  - Markus Tavenrath, NVIDIA  - Daniel Koch, NVIDIA
//!# Related
//! - [`ComponentTypeNV`]
//! - [`CooperativeMatrixPropertiesNV`]
//! - [`PhysicalDeviceCooperativeMatrixFeaturesNV`]
//! - [`PhysicalDeviceCooperativeMatrixPropertiesNV`]
//! - [`ScopeNV`]
//! - [`GetPhysicalDeviceCooperativeMatrixPropertiesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, ShaderStageFlags, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
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
/// - [`ScopeDeviceNv`] corresponds to SPIR-V [`Device`] scope.
/// - [`ScopeWorkgroupNv`] corresponds to SPIR-V `Workgroup` scope.
/// - [`ScopeSubgroupNv`] corresponds to SPIR-V `Subgroup` scope.
/// - [`ScopeQueueFamilyNv`] corresponds to SPIR-V `QueueFamily` scope.
///All enum values match the corresponding SPIR-V value.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ScopeNV {
    #[doc(hidden)]
    Empty = 0,
    ///[`ScopeDeviceNv`] corresponds to SPIR-V [`Device`] scope.
    ScopeDeviceNv = 1,
    ///[`ScopeWorkgroupNv`] corresponds to SPIR-V `Workgroup` scope.
    ScopeWorkgroupNv = 2,
    ///[`ScopeSubgroupNv`] corresponds to SPIR-V `Subgroup` scope.
    ScopeSubgroupNv = 3,
    ///[`ScopeQueueFamilyNv`] corresponds to SPIR-V `QueueFamily`
    ///scope.
    ScopeQueueFamilyNv = 5,
}
impl const Default for ScopeNV {
    fn default() -> Self {
        Self::Empty
    }
}
impl ScopeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
/// - [`ComponentTypeFloat16Nv`] corresponds to SPIR-V `OpTypeFloat` 16.
/// - [`ComponentTypeFloat32Nv`] corresponds to SPIR-V `OpTypeFloat` 32.
/// - [`ComponentTypeFloat64Nv`] corresponds to SPIR-V `OpTypeFloat` 64.
/// - [`ComponentTypeSint8Nv`] corresponds to SPIR-V `OpTypeInt` 8 1.
/// - [`ComponentTypeSint16Nv`] corresponds to SPIR-V `OpTypeInt` 16 1.
/// - [`ComponentTypeSint32Nv`] corresponds to SPIR-V `OpTypeInt` 32 1.
/// - [`ComponentTypeSint64Nv`] corresponds to SPIR-V `OpTypeInt` 64 1.
/// - [`ComponentTypeUint8Nv`] corresponds to SPIR-V `OpTypeInt` 8 0.
/// - [`ComponentTypeUint16Nv`] corresponds to SPIR-V `OpTypeInt` 16 0.
/// - [`ComponentTypeUint32Nv`] corresponds to SPIR-V `OpTypeInt` 32 0.
/// - [`ComponentTypeUint64Nv`] corresponds to SPIR-V `OpTypeInt` 64 0.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ComponentTypeNV {
    ///[`ComponentTypeFloat16Nv`] corresponds to SPIR-V
    ///`OpTypeFloat` 16.
    ComponentTypeFloat16Nv = 0,
    ///[`ComponentTypeFloat32Nv`] corresponds to SPIR-V
    ///`OpTypeFloat` 32.
    ComponentTypeFloat32Nv = 1,
    ///[`ComponentTypeFloat64Nv`] corresponds to SPIR-V
    ///`OpTypeFloat` 64.
    ComponentTypeFloat64Nv = 2,
    ///[`ComponentTypeSint8Nv`] corresponds to SPIR-V `OpTypeInt` 8 1.
    ComponentTypeSint8Nv = 3,
    ///[`ComponentTypeSint16Nv`] corresponds to SPIR-V `OpTypeInt`
    ///16 1.
    ComponentTypeSint16Nv = 4,
    ///[`ComponentTypeSint32Nv`] corresponds to SPIR-V `OpTypeInt`
    ///32 1.
    ComponentTypeSint32Nv = 5,
    ///[`ComponentTypeSint64Nv`] corresponds to SPIR-V `OpTypeInt`
    ///64 1.
    ComponentTypeSint64Nv = 6,
    ///[`ComponentTypeUint8Nv`] corresponds to SPIR-V `OpTypeInt` 8 0.
    ComponentTypeUint8Nv = 7,
    ///[`ComponentTypeUint16Nv`] corresponds to SPIR-V `OpTypeInt`
    ///16 0.
    ComponentTypeUint16Nv = 8,
    ///[`ComponentTypeUint32Nv`] corresponds to SPIR-V `OpTypeInt`
    ///32 0.
    ComponentTypeUint32Nv = 9,
    ///[`ComponentTypeUint64Nv`] corresponds to SPIR-V `OpTypeInt`
    ///64 0.
    ComponentTypeUint64Nv = 10,
}
impl const Default for ComponentTypeNV {
    fn default() -> Self {
        Self::ComponentTypeFloat16Nv
    }
}
impl ComponentTypeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceCooperativeMatrixFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixFeaturesNV.html) - Structure describing cooperative matrix features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCooperativeMatrixFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_cooperative_matrix
///typedef struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           cooperativeMatrix;
///    VkBool32           cooperativeMatrixRobustBufferAccess;
///} VkPhysicalDeviceCooperativeMatrixFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`cooperative_matrix`] indicates that the implementation supports the `CooperativeMatrixNV`
///   SPIR-V capability.
/// - [`cooperative_matrix_robust_buffer_access`] indicates that the implementation supports robust
///   buffer access for SPIR-V `OpCooperativeMatrixLoadNV` and `OpCooperativeMatrixStoreNV`
///   instructions.
///If the [`PhysicalDeviceCooperativeMatrixFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCooperativeMatrixFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`
///# Related
/// - [`VK_NV_cooperative_matrix`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`cooperative_matrix`] indicates that
    ///the implementation supports the `CooperativeMatrixNV` SPIR-V
    ///capability.
    cooperative_matrix: Bool32,
    ///[`cooperative_matrix_robust_buffer_access`] indicates that the
    ///implementation supports robust buffer access for SPIR-V
    ///`OpCooperativeMatrixLoadNV` and `OpCooperativeMatrixStoreNV`
    ///instructions.
    cooperative_matrix_robust_buffer_access: Bool32,
}
impl<'lt> Default for PhysicalDeviceCooperativeMatrixFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            cooperative_matrix: 0,
            cooperative_matrix_robust_buffer_access: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCooperativeMatrixFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::cooperative_matrix`]
    pub fn cooperative_matrix_raw(&self) -> Bool32 {
        self.cooperative_matrix
    }
    ///Gets the raw value of [`Self::cooperative_matrix_robust_buffer_access`]
    pub fn cooperative_matrix_robust_buffer_access_raw(&self) -> Bool32 {
        self.cooperative_matrix_robust_buffer_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::cooperative_matrix`]
    pub fn set_cooperative_matrix_raw(&mut self, value: Bool32) -> &mut Self {
        self.cooperative_matrix = value;
        self
    }
    ///Sets the raw value of [`Self::cooperative_matrix_robust_buffer_access`]
    pub fn set_cooperative_matrix_robust_buffer_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.cooperative_matrix_robust_buffer_access = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::cooperative_matrix`]
    pub fn cooperative_matrix(&self) -> bool {
        unsafe { std::mem::transmute(self.cooperative_matrix as u8) }
    }
    ///Gets the value of [`Self::cooperative_matrix_robust_buffer_access`]
    pub fn cooperative_matrix_robust_buffer_access(&self) -> bool {
        unsafe { std::mem::transmute(self.cooperative_matrix_robust_buffer_access as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::cooperative_matrix`]
    pub fn cooperative_matrix_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.cooperative_matrix as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.cooperative_matrix as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::cooperative_matrix_robust_buffer_access`]
    pub fn cooperative_matrix_robust_buffer_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.cooperative_matrix_robust_buffer_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.cooperative_matrix_robust_buffer_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::cooperative_matrix`]
    pub fn set_cooperative_matrix(&mut self, value: bool) -> &mut Self {
        self.cooperative_matrix = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::cooperative_matrix_robust_buffer_access`]
    pub fn set_cooperative_matrix_robust_buffer_access(&mut self, value: bool) -> &mut Self {
        self.cooperative_matrix_robust_buffer_access = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCooperativeMatrixPropertiesNV.html) - Structure describing cooperative matrix properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCooperativeMatrixPropertiesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_cooperative_matrix
///typedef struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkShaderStageFlags    cooperativeMatrixSupportedStages;
///} VkPhysicalDeviceCooperativeMatrixPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`cooperative_matrix_supported_stages`] is a bitfield of [`ShaderStageFlagBits`] describing
///   the shader stages that cooperative matrix instructions are supported in.
///   [`cooperative_matrix_supported_stages`] will have the `VK_SHADER_STAGE_COMPUTE_BIT` bit set if
///   any of the physical device’s queues support `VK_QUEUE_COMPUTE_BIT`.
///# Description
///If the [`PhysicalDeviceCooperativeMatrixPropertiesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`
///# Related
/// - [`VK_NV_cooperative_matrix`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`cooperative_matrix_supported_stages`] is a bitfield of
    ///[`ShaderStageFlagBits`] describing the shader stages that
    ///cooperative matrix instructions are supported in.
    ///[`cooperative_matrix_supported_stages`] will have the
    ///`VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical
    ///device’s queues support `VK_QUEUE_COMPUTE_BIT`.
    cooperative_matrix_supported_stages: ShaderStageFlags,
}
impl<'lt> Default for PhysicalDeviceCooperativeMatrixPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            cooperative_matrix_supported_stages: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceCooperativeMatrixPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::cooperative_matrix_supported_stages`]
    pub fn cooperative_matrix_supported_stages(&self) -> ShaderStageFlags {
        self.cooperative_matrix_supported_stages
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::cooperative_matrix_supported_stages`]
    pub fn cooperative_matrix_supported_stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.cooperative_matrix_supported_stages
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::cooperative_matrix_supported_stages`]
    pub fn set_cooperative_matrix_supported_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.cooperative_matrix_supported_stages = value;
        self
    }
}
///[VkCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) - Structure specifying cooperative matrix properties
///# C Specifications
///Each [`CooperativeMatrixPropertiesNV`] structure describes a single
///supported combination of types for a matrix multiply/add operation
///(`OpCooperativeMatrixMulAddNV`).
///The multiply  **can**  be described in terms of the following variables and types
///(in SPIR-V pseudocode):
///```c
///    %A is of type OpTypeCooperativeMatrixNV %AType %scope %MSize %KSize
///    %B is of type OpTypeCooperativeMatrixNV %BType %scope %KSize %NSize
///    %C is of type OpTypeCooperativeMatrixNV %CType %scope %MSize %NSize
///    %D is of type OpTypeCooperativeMatrixNV %DType %scope %MSize %NSize
///
///    %D = %A * %B + %C // using OpCooperativeMatrixMulAddNV
///```
///A matrix multiply with these dimensions is known as an *MxNxK* matrix
///multiply.The [`CooperativeMatrixPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_cooperative_matrix
///typedef struct VkCooperativeMatrixPropertiesNV {
///    VkStructureType      sType;
///    void*                pNext;
///    uint32_t             MSize;
///    uint32_t             NSize;
///    uint32_t             KSize;
///    VkComponentTypeNV    AType;
///    VkComponentTypeNV    BType;
///    VkComponentTypeNV    CType;
///    VkComponentTypeNV    DType;
///    VkScopeNV            scope;
///} VkCooperativeMatrixPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`m_size`] is the number of rows in matrices A, C, and D.
/// - [`k_size`] is the number of columns in matrix A and rows in matrix B.
/// - [`n_size`] is the number of columns in matrices B, C, D.
/// - [`a_type`] is the component type of matrix A, of type [`ComponentTypeNV`].
/// - [`b_type`] is the component type of matrix B, of type [`ComponentTypeNV`].
/// - [`c_type`] is the component type of matrix C, of type [`ComponentTypeNV`].
/// - [`d_type`] is the component type of matrix D, of type [`ComponentTypeNV`].
/// - [`scope`] is the scope of all the matrix types, of type [`ScopeNV`].
///# Description
///If some types are preferred over other types (e.g. for performance), they
/// **should**  appear earlier in the list enumerated by
///[`GetPhysicalDeviceCooperativeMatrixPropertiesNV`].At least one entry in the list  **must**
/// have power of two values for all of
///[`m_size`], [`k_size`], and [`n_size`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`a_type`] **must**  be a valid [`ComponentTypeNV`] value
/// - [`b_type`] **must**  be a valid [`ComponentTypeNV`] value
/// - [`c_type`] **must**  be a valid [`ComponentTypeNV`] value
/// - [`d_type`] **must**  be a valid [`ComponentTypeNV`] value
/// - [`scope`] **must**  be a valid [`ScopeNV`] value
///# Related
/// - [`VK_NV_cooperative_matrix`]
/// - [`ComponentTypeNV`]
/// - [`ScopeNV`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceCooperativeMatrixPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`m_size`] is the number of rows in matrices A, C, and D.
    m_size: u32,
    ///[`n_size`] is the number of columns in matrices B, C, D.
    n_size: u32,
    ///[`k_size`] is the number of columns in matrix A and rows in matrix B.
    k_size: u32,
    ///[`a_type`] is the component type of matrix A, of type
    ///[`ComponentTypeNV`].
    a_type: ComponentTypeNV,
    ///[`b_type`] is the component type of matrix B, of type
    ///[`ComponentTypeNV`].
    b_type: ComponentTypeNV,
    ///[`c_type`] is the component type of matrix C, of type
    ///[`ComponentTypeNV`].
    c_type: ComponentTypeNV,
    ///[`d_type`] is the component type of matrix D, of type
    ///[`ComponentTypeNV`].
    d_type: ComponentTypeNV,
    ///[`scope`] is the scope of all the matrix types, of type
    ///[`ScopeNV`].
    scope: ScopeNV,
}
impl<'lt> Default for CooperativeMatrixPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            m_size: 0,
            n_size: 0,
            k_size: 0,
            a_type: Default::default(),
            b_type: Default::default(),
            c_type: Default::default(),
            d_type: Default::default(),
            scope: Default::default(),
        }
    }
}
impl<'lt> CooperativeMatrixPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::m_size`]
    pub fn m_size(&self) -> u32 {
        self.m_size
    }
    ///Gets the value of [`Self::n_size`]
    pub fn n_size(&self) -> u32 {
        self.n_size
    }
    ///Gets the value of [`Self::k_size`]
    pub fn k_size(&self) -> u32 {
        self.k_size
    }
    ///Gets the value of [`Self::a_type`]
    pub fn a_type(&self) -> ComponentTypeNV {
        self.a_type
    }
    ///Gets the value of [`Self::b_type`]
    pub fn b_type(&self) -> ComponentTypeNV {
        self.b_type
    }
    ///Gets the value of [`Self::c_type`]
    pub fn c_type(&self) -> ComponentTypeNV {
        self.c_type
    }
    ///Gets the value of [`Self::d_type`]
    pub fn d_type(&self) -> ComponentTypeNV {
        self.d_type
    }
    ///Gets the value of [`Self::scope`]
    pub fn scope(&self) -> ScopeNV {
        self.scope
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::m_size`]
    pub fn m_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::n_size`]
    pub fn n_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::k_size`]
    pub fn k_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::a_type`]
    pub fn a_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.a_type
    }
    ///Gets a mutable reference to the value of [`Self::b_type`]
    pub fn b_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.b_type
    }
    ///Gets a mutable reference to the value of [`Self::c_type`]
    pub fn c_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.c_type
    }
    ///Gets a mutable reference to the value of [`Self::d_type`]
    pub fn d_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.d_type
    }
    ///Gets a mutable reference to the value of [`Self::scope`]
    pub fn scope_mut(&mut self) -> &mut ScopeNV {
        &mut self.scope
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::m_size`]
    pub fn set_m_size(&mut self, value: u32) -> &mut Self {
        self.m_size = value;
        self
    }
    ///Sets the raw value of [`Self::n_size`]
    pub fn set_n_size(&mut self, value: u32) -> &mut Self {
        self.n_size = value;
        self
    }
    ///Sets the raw value of [`Self::k_size`]
    pub fn set_k_size(&mut self, value: u32) -> &mut Self {
        self.k_size = value;
        self
    }
    ///Sets the raw value of [`Self::a_type`]
    pub fn set_a_type(&mut self, value: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> &mut Self {
        self.a_type = value;
        self
    }
    ///Sets the raw value of [`Self::b_type`]
    pub fn set_b_type(&mut self, value: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> &mut Self {
        self.b_type = value;
        self
    }
    ///Sets the raw value of [`Self::c_type`]
    pub fn set_c_type(&mut self, value: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> &mut Self {
        self.c_type = value;
        self
    }
    ///Sets the raw value of [`Self::d_type`]
    pub fn set_d_type(&mut self, value: crate::extensions::nv_cooperative_matrix::ComponentTypeNV) -> &mut Self {
        self.d_type = value;
        self
    }
    ///Sets the raw value of [`Self::scope`]
    pub fn set_scope(&mut self, value: crate::extensions::nv_cooperative_matrix::ScopeNV) -> &mut Self {
        self.scope = value;
        self
    }
}
