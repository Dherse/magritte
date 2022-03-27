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
        Empty
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
        ComponentTypeFloat16Nv
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
///[`PhysicalDeviceCooperativeMatrixFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`cooperative_matrix_supported_stages`] is a bitfield of
    ///[`ShaderStageFlagBits`] describing the shader stages that
    ///cooperative matrix instructions are supported in.
    ///[`cooperative_matrix_supported_stages`] will have the
    ///`VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical
    ///device’s queues support `VK_QUEUE_COMPUTE_BIT`.
    cooperative_matrix_supported_stages: ShaderStageFlags,
}
///[VkCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) - Structure specifying cooperative matrix properties
///# C Specifications
///Each [`CooperativeMatrixPropertiesNV`] structure describes a single
///supported combination of types for a matrix multiply/add operation
///(`OpCooperativeMatrixMulAddNV`).
///The multiply **can** be described in terms of the following variables and types
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
///**should** appear earlier in the list enumerated by
///[`GetPhysicalDeviceCooperativeMatrixPropertiesNV`].At least one entry in the list **must** have
/// power of two values for all of
///[`m_size`], [`k_size`], and [`n_size`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`a_type`]**must** be a valid [`ComponentTypeNV`] value
/// - [`b_type`]**must** be a valid [`ComponentTypeNV`] value
/// - [`c_type`]**must** be a valid [`ComponentTypeNV`] value
/// - [`d_type`]**must** be a valid [`ComponentTypeNV`] value
/// - [`scope`]**must** be a valid [`ScopeNV`] value
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
