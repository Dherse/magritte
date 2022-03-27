use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map2");
///[VkPhysicalDeviceFragmentDensityMap2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html) - Structure describing additional fragment density map features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map2
///typedef struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMapDeferred;
///} VkPhysicalDeviceFragmentDensityMap2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map_deferred`] specifies whether the implementation supports deferred reads
///   of fragment density map image views. If this feature is not enabled,
///   `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT`**must** not be included in
///   [`ImageViewCreateInfo::flags`].
///If the [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMap2FeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map2`]
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
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`fragment_density_map_deferred`]
    ///specifies whether the implementation supports deferred reads of fragment
    ///density map image views.
    ///If this feature is not enabled,
    ///`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT`**must**
    ///not be included in [`ImageViewCreateInfo`]::`flags`.
    fragment_density_map_deferred: Bool32,
}
///[VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) - Structure describing additional fragment density map properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map2
///typedef struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           subsampledLoads;
///    VkBool32           subsampledCoarseReconstructionEarlyAccess;
///    uint32_t           maxSubsampledArrayLayers;
///    uint32_t           maxDescriptorSetSubsampledSamplers;
///} VkPhysicalDeviceFragmentDensityMap2PropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`subsampled_loads`] specifies if performing image data read with load operations on
///   subsampled attachments will be resampled to the fragment density of the render pass
/// - [`subsampled_coarse_reconstruction_early_access`] specifies if performing image data read with
///   samplers created with `flags` containing
///   `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` in fragment shader will trigger
///   additional reads during `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
/// - [`max_subsampled_array_layers`] is the maximum number of [`ImageView`] array layers for usages
///   supporting subsampled samplers
/// - [`max_descriptor_set_subsampled_samplers`] is the maximum number of subsampled samplers that
///   **can** be included in a [`PipelineLayout`]
///# Description
///If the [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map2`]
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
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`subsampled_loads`] specifies if performing
    ///image data read with load operations on subsampled attachments will be
    ///resampled to the fragment density of the render pass
    subsampled_loads: Bool32,
    ///[`subsampled_coarse_reconstruction_early_access`] specifies if performing
    ///image data read with samplers created with `flags` containing
    ///`VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` in
    ///fragment shader will trigger additional reads during
    ///`VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
    subsampled_coarse_reconstruction_early_access: Bool32,
    ///[`max_subsampled_array_layers`] is
    ///the maximum number of [`ImageView`] array layers for usages
    ///supporting subsampled samplers
    max_subsampled_array_layers: u32,
    ///[`max_descriptor_set_subsampled_samplers`] is the maximum number of
    ///subsampled samplers that **can** be included in a [`PipelineLayout`]
    max_descriptor_set_subsampled_samplers: u32,
}
