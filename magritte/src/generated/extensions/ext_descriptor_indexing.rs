//![VK_EXT_descriptor_indexing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_descriptor_indexing.html) - device extension
//!# Description
//!This extension adds several small features which together enable
//!applications to create large descriptor sets containing substantially all of
//!their resources, and selecting amongst those resources with dynamic
//!(non-uniform) indexes in the shader.
//!There are feature enables and SPIR-V capabilities for non-uniform descriptor
//!indexing in the shader, and non-uniform indexing in the shader requires use
//!of a new `NonUniformEXT` decoration defined in the
//!`SPV_EXT_descriptor_indexing` SPIR-V extension.
//!There are descriptor set layout binding creation flags enabling several
//!features:
//! - Descriptors can be updated after they are bound to a command buffer, such that the execution
//!   of the command buffer reflects the most recent update to the descriptors.
//! - Descriptors that are not used by any pending command buffers can be updated, which enables
//!   writing new descriptors for frame N+1 while frame N is executing.
//! - Relax the requirement that all descriptors in a binding that is “statically used” must be
//!   valid, such that descriptors that are not accessed by a submission need not be valid and can
//!   be updated while that submission is executing.
//! - The final binding in a descriptor set layout can have a variable size (and unsized arrays of
//!   resources are allowed in the `GL_EXT_nonuniform_qualifier` and `SPV_EXT_descriptor_indexing`
//!   extensions).
//!Note that it is valid for multiple descriptor arrays in a shader to use the
//!same set and binding number, as long as they are all compatible with the
//!descriptor type in the pipeline layout.
//!This means a single array binding in the descriptor set can serve multiple
//!texture dimensionalities, or an array of buffer descriptors can be used with
//!multiple different block layouts.There are new descriptor set layout and descriptor pool
//! creation flags that
//!are required to opt in to the update-after-bind functionality, and there are
//!separate `maxPerStage`* and `maxDescriptorSet`* limits that apply to
//!these descriptor set layouts which  **may**  be much higher than the pre-existing
//!limits.
//!The old limits only count descriptors in non-updateAfterBind descriptor set
//!layouts, and the new limits count descriptors in all descriptor set layouts
//!in the pipeline layout.
# ! [doc = concat ! ("# " , "Revision")]
//!2
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`khr_maintenance3`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_descriptor_indexing]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_descriptor_indexing extension>>)
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`DescriptorSetAllocateInfo`]:  -
//!   [`DescriptorSetVariableDescriptorCountAllocateInfoEXT`]
//! - Extending [`DescriptorSetLayoutCreateInfo`]:  -
//!   [`DescriptorSetLayoutBindingFlagsCreateInfoEXT`]
//! - Extending [`DescriptorSetLayoutSupport`]:  -
//!   [`DescriptorSetVariableDescriptorCountLayoutSupportEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDescriptorIndexingFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceDescriptorIndexingPropertiesEXT`]
# ! [doc = concat ! ("# " , "New enums")]
//! - [`DescriptorBindingFlagBitsEXT`]
# ! [doc = concat ! ("# " , "New bitmasks")]
//! - [`DescriptorBindingFlagsEXT`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME`]
//! - [`EXT_DESCRIPTOR_INDEXING_SPEC_VERSION`]
//! - Extending [`DescriptorBindingFlagBits`]:  - `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT`  -
//!   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT`  -
//!   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT`  -
//!   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT`
//! - Extending [`DescriptorPoolCreateFlagBits`]:  -
//!   `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT`
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:  -
//!   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_FRAGMENTATION_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2017-07-26 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-10-02 (Jeff Bolz)  - ???
//!# Other info
//! * 2017-10-02
//! * - Promoted to Vulkan 1.2 Core  - This extension requires [`SPV_EXT_descriptor_indexing`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_descriptor_indexing.html)
//!   - This extension provides API support for [`GL_EXT_nonuniform_qualifier`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_nonuniform_qualifier.txt)
//! * - Jeff Bolz, NVIDIA  - Daniel Rakos, AMD  - Slawomir Grajewski, Intel  - Tobias Hector,
//!   Imagination Technologies
//!# Related
//! - [`DescriptorBindingFlagBitsEXT`]
//! - [`DescriptorBindingFlagsEXT`]
//! - [`DescriptorSetLayoutBindingFlagsCreateInfoEXT`]
//! - [`DescriptorSetVariableDescriptorCountAllocateInfoEXT`]
//! - [`DescriptorSetVariableDescriptorCountLayoutSupportEXT`]
//! - [`PhysicalDeviceDescriptorIndexingFeaturesEXT`]
//! - [`PhysicalDeviceDescriptorIndexingPropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_descriptor_indexing");
