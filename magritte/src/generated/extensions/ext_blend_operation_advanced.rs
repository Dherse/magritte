use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_blend_operation_advanced");
///[VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html) - Enumerant specifying the blend overlap parameter
///# C Specifications
///The weighting functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> are defined
///in table [Advanced Blend Overlap
///Modes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced-overlap-modes).
///In these functions, the A components of the source and destination colors
///are taken to indicate the portion of the pixel covered by the fragment
///(source) and the fragments previously accumulated in the pixel
///(destination).
///The functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> approximate the
///relative portion of the pixel covered by the intersection of the source and
///destination, covered only by the source, and covered only by the
///destination, respectively.Possible values of
///[`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`],
///specifying the blend overlap functions, are:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef enum VkBlendOverlapEXT {
///    VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,
///    VK_BLEND_OVERLAP_DISJOINT_EXT = 1,
///    VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
///} VkBlendOverlapEXT;
///```
///# Description
/// - [`BlendOverlapUncorrelatedExt`] specifies that there is no correlation between the source and
///   destination coverage.
/// - [`BlendOverlapConjointExt`] specifies that the source and destination coverage are considered
///   to have maximal overlap.
/// - [`BlendOverlapDisjointExt`] specifies that the source and destination coverage are considered
///   to have minimal overlap.
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
/// - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBlendOverlapEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum BlendOverlapEXT {
    ///[`BlendOverlapUncorrelatedExt`] specifies that there is no
    ///correlation between the source and destination coverage.
    BlendOverlapUncorrelatedExt = 0,
    ///[`BlendOverlapDisjointExt`] specifies that the source and
    ///destination coverage are considered to have minimal overlap.
    BlendOverlapDisjointExt = 1,
    ///[`BlendOverlapConjointExt`] specifies that the source and
    ///destination coverage are considered to have maximal overlap.
    BlendOverlapConjointExt = 2,
}
impl const Default for BlendOverlapEXT {
    fn default() -> Self {
        BlendOverlapUncorrelatedExt
    }
}
impl BlendOverlapEXT {
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
///[VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) - Structure describing advanced blending features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           advancedBlendCoherentOperations;
///} VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_coherent_operations`] specifies whether blending using [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced)
///   is guaranteed to execute atomically and in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
///   If this is [`TRUE`], `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the same
///   as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending needs no additional
///   synchronization over basic blending. If this is [`FALSE`], then memory dependencies are
///   required to guarantee order between two advanced blending operations that occur on the same
///   sample.
///If the [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
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
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`advanced_blend_coherent_operations`] specifies whether blending using
    ///[advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced) is guaranteed
    ///to execute atomically and in [primitive
    ///order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    ///If this is [`TRUE`],
    ///`VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the
    ///same as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending
    ///needs no additional synchronization over basic blending.
    ///If this is [`FALSE`], then memory dependencies are required to
    ///guarantee order between two advanced blending operations that occur on
    ///the same sample.
    advanced_blend_coherent_operations: Bool32,
}
///[VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) - Structure describing advanced blending limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           advancedBlendMaxColorAttachments;
///    VkBool32           advancedBlendIndependentBlend;
///    VkBool32           advancedBlendNonPremultipliedSrcColor;
///    VkBool32           advancedBlendNonPremultipliedDstColor;
///    VkBool32           advancedBlendCorrelatedOverlap;
///    VkBool32           advancedBlendAllOperations;
///} VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_max_color_attachments`] is one greater than the highest color attachment index that **can** be used in a subpass, for a pipeline that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
/// - [`advanced_blend_independent_blend`] specifies whether advanced blend operations **can** vary
///   per-attachment.
/// - [`advanced_blend_non_premultiplied_src_color`] specifies whether the source color **can** be
///   treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::src_premultiplied`]**must** be [`TRUE`].
/// - [`advanced_blend_non_premultiplied_dst_color`] specifies whether the destination color **can**
///   be treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::dst_premultiplied`]**must** be [`TRUE`].
/// - [`advanced_blend_correlated_overlap`] specifies whether the overlap mode **can** be treated as
///   correlated. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`]**must** be
///   `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
/// - [`advanced_blend_all_operations`] specifies whether all advanced blend operation enums are
///   supported. See the valid usage of [`PipelineColorBlendAttachmentState`].
///# Description
///If the [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
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
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`advanced_blend_max_color_attachments`] is one greater than the highest
    ///color attachment index that **can** be used in a subpass, for a pipeline
    ///that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
    advanced_blend_max_color_attachments: u32,
    ///[`advanced_blend_independent_blend`] specifies whether advanced blend
    ///operations **can** vary per-attachment.
    advanced_blend_independent_blend: Bool32,
    ///[`advanced_blend_non_premultiplied_src_color`] specifies whether the source
    ///color **can** be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`srcPremultiplied`**must** be [`TRUE`].
    advanced_blend_non_premultiplied_src_color: Bool32,
    ///[`advanced_blend_non_premultiplied_dst_color`] specifies whether the
    ///destination color **can** be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`dstPremultiplied`**must** be [`TRUE`].
    advanced_blend_non_premultiplied_dst_color: Bool32,
    ///[`advanced_blend_correlated_overlap`] specifies whether the overlap mode
    ///**can** be treated as correlated.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`blendOverlap`**must** be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
    advanced_blend_correlated_overlap: Bool32,
    ///[`advanced_blend_all_operations`]
    ///specifies whether all advanced blend operation enums are supported.
    ///See the valid usage of [`PipelineColorBlendAttachmentState`].
    advanced_blend_all_operations: Bool32,
}
///[VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) - Structure specifying parameters that affect advanced blend operations
///# C Specifications
///If the [`p_next`] chain of [`PipelineColorBlendStateCreateInfo`]
///includes a [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure,
///then that structure includes parameters that affect advanced blend
///operations.The [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkBool32             srcPremultiplied;
///    VkBool32             dstPremultiplied;
///    VkBlendOverlapEXT    blendOverlap;
///} VkPipelineColorBlendAdvancedStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_premultiplied`] specifies whether the source color of the blend operation is treated as
///   premultiplied.
/// - [`dst_premultiplied`] specifies whether the destination color of the blend operation is
///   treated as premultiplied.
/// - [`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the source and destination
///   sample’s coverage is correlated.
///# Description
///If this structure is not present, [`src_premultiplied`] and
///[`dst_premultiplied`] are both considered to be [`TRUE`], and
///[`blend_overlap`] is considered to be
///`VK_BLEND_OVERLAP_UNCORRELATED_EXT`.Valid Usage
/// - If the [non-premultiplied source color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedSrcColor)
///   property is not supported, [`src_premultiplied`]**must** be [`TRUE`]
/// - If the [non-premultiplied destination color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedDstColor)
///   property is not supported, [`dst_premultiplied`]**must** be [`TRUE`]
/// - If the [correlated overlap](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendCorrelatedOverlap)
///   property is not supported, [`blend_overlap`]**must** be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
/// - [`blend_overlap`]**must** be a valid [`BlendOverlapEXT`] value
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
/// - [`BlendOverlapEXT`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src_premultiplied`] specifies whether the source color of the blend
    ///operation is treated as premultiplied.
    src_premultiplied: Bool32,
    ///[`dst_premultiplied`] specifies whether the destination color of the
    ///blend operation is treated as premultiplied.
    dst_premultiplied: Bool32,
    ///[`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the
    ///source and destination sample’s coverage is correlated.
    blend_overlap: BlendOverlapEXT,
}
