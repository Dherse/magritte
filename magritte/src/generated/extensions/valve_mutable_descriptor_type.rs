use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorType, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_VALVE_mutable_descriptor_type");
///[VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html) - Structure describing whether the mutable descriptor type is supported
///# C Specifications
///The [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is
///defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           mutableDescriptorType;
///} VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mutable_descriptor_type`] indicates that the implementation **must** support using the
///   [`DescriptorType`] of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` with at least the following
///   descriptor types, where any combination of the types **must** be supported:  -
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`  - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`  -
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`  -
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`
/// - Additionally, [`mutable_descriptor_type`] indicates that:  - Non-uniform descriptor indexing
///   **must** be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding non-uniform indexing features
///   enabled in [`PhysicalDeviceDescriptorIndexingFeatures`].  -
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` with `descriptorType` of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` relaxes the list of required descriptor types to the
///   descriptor types which have the corresponding update-after-bind feature enabled in
///   [`PhysicalDeviceDescriptorIndexingFeatures`].  - Dynamically uniform descriptor indexing
///   **must** be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding dynamic indexing features enabled.
///   - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`**must** be supported.  -
///   `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE`**must** be supported.
///If the [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
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
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`mutable_descriptor_type`] indicates
    ///that the implementation **must** support using the [`DescriptorType`]
    ///of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` with at least the following
    ///descriptor types, where any combination of the types **must** be supported:
    /// - `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`
    /// - `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`
    mutable_descriptor_type: Bool32,
}
///[VkMutableDescriptorTypeListVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListVALVE.html) - Structure describing descriptor types that a given descriptor may mutate to
///# C Specifications
///The list of potential descriptor types a given mutable descriptor **can**
///mutate to is passed in a [`MutableDescriptorTypeListVALVE`] structure.The
/// [`MutableDescriptorTypeListVALVE`] structure is defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkMutableDescriptorTypeListVALVE {
///    uint32_t                   descriptorTypeCount;
///    const VkDescriptorType*    pDescriptorTypes;
///} VkMutableDescriptorTypeListVALVE;
///```
///# Members
/// - [`descriptor_type_count`] is the number of elements in [`p_descriptor_types`].
/// - [`p_descriptor_types`] is `NULL` or a pointer to an array of
///   [`descriptor_type_count`][`DescriptorType`] values defining which descriptor types a given
///   binding may mutate to.
///# Description
///Valid Usage
/// - [`descriptor_type_count`]**must** not be `0` if the corresponding binding is of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`p_descriptor_types`]**must** be a valid pointer to an array of [`descriptor_type_count`]
///   valid, unique [`DescriptorType`] values if the given binding is of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` type
/// - [`descriptor_type_count`]**must** be `0` if the corresponding binding is not of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`p_descriptor_types`]**must** not contain `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`p_descriptor_types`]**must** not contain `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
/// - [`p_descriptor_types`]**must** not contain `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
/// - [`p_descriptor_types`]**must** not contain `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
///Valid Usage (Implicit)
/// - If [`descriptor_type_count`] is not `0`, [`p_descriptor_types`]**must** be a valid pointer to
///   an array of [`descriptor_type_count`] valid [`DescriptorType`] values
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
/// - [`DescriptorType`]
/// - [`MutableDescriptorTypeCreateInfoVALVE`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MutableDescriptorTypeListVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`descriptor_type_count`] is the number of elements in
    ///[`p_descriptor_types`].
    descriptor_type_count: u32,
    ///[`p_descriptor_types`] is `NULL` or a pointer to an array of
    ///[`descriptor_type_count`][`DescriptorType`] values defining which
    ///descriptor types a given binding may mutate to.
    p_descriptor_types: *mut DescriptorType,
}
///[VkMutableDescriptorTypeCreateInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html) - Structure describing the list of possible active descriptor types for mutable type descriptors
///# C Specifications
///Information about the possible descriptor types for mutable descriptor types
///is passed in a [`MutableDescriptorTypeCreateInfoVALVE`] structure as a
///[`p_next`] to a [`DescriptorSetLayoutCreateInfo`] structure or a
///[`DescriptorPoolCreateInfo`] structure.The [`MutableDescriptorTypeCreateInfoVALVE`] structure is
/// defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkMutableDescriptorTypeCreateInfoVALVE {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   mutableDescriptorTypeListCount;
///    const VkMutableDescriptorTypeListVALVE*    pMutableDescriptorTypeLists;
///} VkMutableDescriptorTypeCreateInfoVALVE;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mutable_descriptor_type_list_count`] is the number of elements in
///   [`p_mutable_descriptor_type_lists`].
/// - [`p_mutable_descriptor_type_lists`] is a pointer to an array of
///   [`MutableDescriptorTypeListVALVE`] structures.
///# Description
///If [`mutable_descriptor_type_list_count`] is zero or if this structure is not
///included in the [`p_next`] chain, the
///[`MutableDescriptorTypeListVALVE`] for each element is considered to be
///zero or `NULL` for each member.
///Otherwise, the descriptor set layout binding at
///[`DescriptorSetLayoutCreateInfo::p_bindings`][i] uses the
///descriptor type lists in
///[`MutableDescriptorTypeCreateInfoVALVE`]::[`p_mutable_descriptor_type_lists`][i].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`
/// - If [`mutable_descriptor_type_list_count`] is not `0`,
///   [`p_mutable_descriptor_type_lists`]**must** be a valid pointer to an array of
///   [`mutable_descriptor_type_list_count`] valid [`MutableDescriptorTypeListVALVE`] structures
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
/// - [`MutableDescriptorTypeListVALVE`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`mutable_descriptor_type_list_count`] is the number of elements in
    ///[`p_mutable_descriptor_type_lists`].
    mutable_descriptor_type_list_count: u32,
    ///[`p_mutable_descriptor_type_lists`] is a pointer to an array of
    ///[`MutableDescriptorTypeListVALVE`] structures.
    p_mutable_descriptor_type_lists: *mut MutableDescriptorTypeListVALVE<'lt>,
}
