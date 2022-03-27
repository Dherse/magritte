use crate::vulkan1_0::{AttachmentReference, BaseInStructure, BaseOutStructure, Bool32, Extent2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map");
///[VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) - Structure describing fragment density map features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMap;
///    VkBool32           fragmentDensityMapDynamic;
///    VkBool32           fragmentDensityMapNonSubsampledImages;
///} VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map`] specifies whether the implementation supports render passes with a
///   fragment density map attachment. If this feature is not enabled and the [`p_next`] chain of
///   [`RenderPassCreateInfo`] includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure,
///   `fragmentDensityMapAttachment`**must** be [`ATTACHMENT_UNUSED`].
/// - [`fragment_density_map_dynamic`] specifies whether the implementation supports dynamic
///   fragment density map image views. If this feature is not enabled,
///   `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`**must** not be included in
///   [`ImageViewCreateInfo::flags`].
/// - [`fragment_density_map_non_subsampled_images`] specifies whether the implementation supports regular non-subsampled image attachments with fragment density map render passes. If this feature is not enabled, render passes with a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment)**must** only have [subsampled attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.
///If the [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMapFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map`]
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
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`fragment_density_map`] specifies
    ///whether the implementation supports render passes with a fragment
    ///density map attachment.
    ///If this feature is not enabled and the [`p_next`] chain of
    ///[`RenderPassCreateInfo`] includes a
    ///[`RenderPassFragmentDensityMapCreateInfoEXT`] structure,
    ///`fragmentDensityMapAttachment`**must** be [`ATTACHMENT_UNUSED`].
    fragment_density_map: Bool32,
    ///[`fragment_density_map_dynamic`]
    ///specifies whether the implementation supports dynamic fragment density
    ///map image views.
    ///If this feature is not enabled,
    ///`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`**must**
    ///not be included in [`ImageViewCreateInfo`]::`flags`.
    fragment_density_map_dynamic: Bool32,
    ///[`fragment_density_map_non_subsampled_images`] specifies whether the
    ///implementation supports regular non-subsampled image attachments with
    ///fragment density map render passes.
    ///If this feature is not enabled, render passes with a
    ///[fragment density map
    ///attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment)**must** only have [subsampled
    ///attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.
    fragment_density_map_non_subsampled_images: Bool32,
}
///[VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) - Structure describing fragment density map properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         minFragmentDensityTexelSize;
///    VkExtent2D         maxFragmentDensityTexelSize;
///    VkBool32           fragmentDensityInvocations;
///} VkPhysicalDeviceFragmentDensityMapPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_fragment_density_texel_size`] is the minimum [fragment density texel size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-fragment-density-texel-size).
/// - [`max_fragment_density_texel_size`] is the maximum fragment density texel size.
/// - [`fragment_density_invocations`] specifies whether the implementation **may** invoke
///   additional fragment shader invocations for each covered sample.
///# Description
///If the [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map`]
/// - [`Bool32`]
/// - [`Extent2D`]
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
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`min_fragment_density_texel_size`]
    ///is the minimum [fragment density
    ///texel size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-fragment-density-texel-size).
    min_fragment_density_texel_size: Extent2D,
    ///[`max_fragment_density_texel_size`]
    ///is the maximum fragment density texel size.
    max_fragment_density_texel_size: Extent2D,
    ///[`fragment_density_invocations`]
    ///specifies whether the implementation **may** invoke additional fragment
    ///shader invocations for each covered sample.
    fragment_density_invocations: Bool32,
}
///[VkRenderPassFragmentDensityMapCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) - Structure containing fragment density map attachment for render pass
///# C Specifications
///If the [`RenderPassCreateInfo`]::[`p_next`] chain includes a
///[`RenderPassFragmentDensityMapCreateInfoEXT`] structure, then that
///structure includes a fragment density map attachment for the render pass.The
/// [`RenderPassFragmentDensityMapCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkRenderPassFragmentDensityMapCreateInfoEXT {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkAttachmentReference    fragmentDensityMapAttachment;
///} VkRenderPassFragmentDensityMapCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map_attachment`] is the fragment density map to use for the render pass.
///# Description
///The fragment density map is read at an implementation-dependent time with
///the following constraints determined by the attachment’s image view
///`flags`:
/// - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` specifies that the fragment
///   density map will be read by the device during
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` specifies that the fragment
///   density map will be read by the host during [`EndCommandBuffer`] of the primary command buffer
///   that the render pass is recorded into
/// - Otherwise the fragment density map will be read by the host during [`CmdBeginRenderPass`]
///The fragment density map **may** additionally be read by the device during
///`VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT` for any mode.If this structure is not
/// present, it is as if
///[`fragment_density_map_attachment`] was given as [`ATTACHMENT_UNUSED`].Valid Usage
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`]**must** not be an element of
///   [`SubpassDescription::p_input_attachments`], [`SubpassDescription::p_color_attachments`],
///   [`SubpassDescription::p_resolve_attachments`],
///   [`SubpassDescription::p_depth_stencil_attachment`], or
///   [`SubpassDescription::p_preserve_attachments`] for any subpass
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], `layout`**must** be equal
///   to `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`, or `VK_IMAGE_LAYOUT_GENERAL`
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`]**must** reference an attachment with a `loadOp` equal to
///   `VK_ATTACHMENT_LOAD_OP_LOAD` or `VK_ATTACHMENT_LOAD_OP_DONT_CARE`
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`]**must** reference an attachment with a `storeOp` equal to
///   `VK_ATTACHMENT_STORE_OP_DONT_CARE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
/// - [`fragment_density_map_attachment`]**must** be a valid [`AttachmentReference`] structure
///# Related
/// - [`VK_EXT_fragment_density_map`]
/// - [`AttachmentReference`]
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
pub struct RenderPassFragmentDensityMapCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`fragment_density_map_attachment`] is the fragment density map to use
    ///for the render pass.
    fragment_density_map_attachment: AttachmentReference,
}
