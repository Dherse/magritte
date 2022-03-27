use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorSetLayout, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_VALVE_descriptor_set_host_mapping");
///[VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html) - Stub description of VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           descriptorSetHostMapping;
///} VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
///```
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
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
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    descriptor_set_host_mapping: Bool32,
}
///[VkDescriptorSetBindingReferenceVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html) - Stub description of VkDescriptorSetBindingReferenceVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkDescriptorSetBindingReferenceVALVE {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkDescriptorSetLayout    descriptorSetLayout;
///    uint32_t                 binding;
///} VkDescriptorSetBindingReferenceVALVE;
///```
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
/// - [`p_next`]**must** be `NULL`
/// - [`descriptor_set_layout`]**must** be a valid [`DescriptorSetLayout`] handle
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
/// - [`DescriptorSetLayout`]
/// - [`StructureType`]
/// - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
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
pub struct DescriptorSetBindingReferenceVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *mut BaseInStructure<'lt>,
    ///[`descriptor_set_layout`]**must** be a valid [`DescriptorSetLayout`] handle
    descriptor_set_layout: DescriptorSetLayout,
    ///No documentation found
    binding: u32,
}
///[VkDescriptorSetLayoutHostMappingInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html) - Stub description of VkDescriptorSetLayoutHostMappingInfoVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkDescriptorSetLayoutHostMappingInfoVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    size_t             descriptorOffset;
///    uint32_t           descriptorSize;
///} VkDescriptorSetLayoutHostMappingInfoVALVE;
///```
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
/// - [`StructureType`]
/// - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
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
pub struct DescriptorSetLayoutHostMappingInfoVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
    s_type: StructureType,
    ///[`p_next`]**must** be `NULL`
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    descriptor_offset: usize,
    ///No documentation found
    descriptor_size: u32,
}
