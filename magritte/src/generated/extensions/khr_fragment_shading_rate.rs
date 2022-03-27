use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Extent2D, SampleCountFlagBits, SampleCountFlags, StructureType,
    },
    vulkan1_2::AttachmentReference2,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_fragment_shading_rate");
///[VkFragmentShadingRateCombinerOpKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) - Control how fragment shading rates are combined
///# C Specifications
///The equation used for each combiner operation is defined by
///[`FragmentShadingRateCombinerOpKHR`]:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef enum VkFragmentShadingRateCombinerOpKHR {
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4,
///} VkFragmentShadingRateCombinerOpKHR;
///```
///# Description
/// - [`FragmentShadingRateCombinerOpKeepKhr`] specifies a combiner operation of
///   combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
/// - [`FragmentShadingRateCombinerOpReplaceKhr`] specifies a combiner operation of
///   combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
/// - [`FragmentShadingRateCombinerOpMinKhr`] specifies a combiner operation of
///   combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FragmentShadingRateCombinerOpMaxKhr`] specifies a combiner operation of
///   combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FragmentShadingRateCombinerOpMulKhr`] specifies a combiner operation of
///   combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.
///where combine(A<sub>xy</sub>,B<sub>xy</sub>) is the combine operation, and A<sub>xy</sub>
///and B<sub>xy</sub> are the inputs to the operation.If [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) is [`FALSE`], using
///[`FragmentShadingRateCombinerOpMulKhr`] with values of 1 for both
///A and B in the same dimension results in the value 2 being produced for that
///dimension.
///See the definition of [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) for more information.These operations are performed in a component-wise fashion.
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
/// - [`PipelineFragmentShadingRateStateCreateInfoKHR`]
/// - [`CmdSetFragmentShadingRateEnumNV`]
/// - [`CmdSetFragmentShadingRateKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum FragmentShadingRateCombinerOpKHR {
    ///[`FragmentShadingRateCombinerOpKeepKhr`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
    FragmentShadingRateCombinerOpKeepKhr = 0,
    ///[`FragmentShadingRateCombinerOpReplaceKhr`] specifies a
    ///combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
    FragmentShadingRateCombinerOpReplaceKhr = 1,
    ///[`FragmentShadingRateCombinerOpMinKhr`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
    FragmentShadingRateCombinerOpMinKhr = 2,
    ///[`FragmentShadingRateCombinerOpMaxKhr`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
    FragmentShadingRateCombinerOpMaxKhr = 3,
    ///[`FragmentShadingRateCombinerOpMulKhr`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.
    FragmentShadingRateCombinerOpMulKhr = 4,
}
impl const Default for FragmentShadingRateCombinerOpKHR {
    fn default() -> Self {
        FragmentShadingRateCombinerOpKeepKhr
    }
}
impl FragmentShadingRateCombinerOpKHR {
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
///[VkFragmentShadingRateAttachmentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateAttachmentInfoKHR.html) - Structure specifying a fragment shading rate attachment for a subpass
///# C Specifications
///The [`FragmentShadingRateAttachmentInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef struct VkFragmentShadingRateAttachmentInfoKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    const VkAttachmentReference2*    pFragmentShadingRateAttachment;
///    VkExtent2D                       shadingRateAttachmentTexelSize;
///} VkFragmentShadingRateAttachmentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_fragment_shading_rate_attachment`] is `NULL` or a pointer to a [`AttachmentReference2`]
///   structure defining the fragment shading rate attachment for this subpass.
/// - [`shading_rate_attachment_texel_size`] specifies the size of the portion of the framebuffer
///   corresponding to each texel in [`p_fragment_shading_rate_attachment`].
///# Description
///If no shading rate attachment is specified, or if this structure is not
///specified, the implementation behaves as if a valid shading rate attachment
///was specified with all texels specifying a single pixel per fragment.Valid Usage
/// - If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], its `layout` member **must** be equal to `VK_IMAGE_LAYOUT_GENERAL` or
///   `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
/// - If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width`**must** be a power of two value
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width`**must** be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width`**must** be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// - If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height`**must** be a power of two value
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height`**must** be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height`**must** be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], the quotient of `shadingRateAttachmentTexelSize.width` and `shadingRateAttachmentTexelSize.height`**must** be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
/// -    If [`p_fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], the quotient of `shadingRateAttachmentTexelSize.height` and `shadingRateAttachmentTexelSize.width`**must** be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
/// - If [`p_fragment_shading_rate_attachment`] is not `NULL`,
///   [`p_fragment_shading_rate_attachment`]**must** be a valid pointer to a valid
///   [`AttachmentReference2`] structure
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`AttachmentReference2`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_fragment_shading_rate_attachment`] is `NULL` or a pointer to a
    ///[`AttachmentReference2`] structure defining the fragment shading
    ///rate attachment for this subpass.
    p_fragment_shading_rate_attachment: *mut AttachmentReference2<'lt>,
    ///[`shading_rate_attachment_texel_size`] specifies the size of the portion
    ///of the framebuffer corresponding to each texel in
    ///[`p_fragment_shading_rate_attachment`].
    shading_rate_attachment_texel_size: Extent2D,
}
///[VkPipelineFragmentShadingRateStateCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateStateCreateInfoKHR.html) - Structure specifying parameters controlling the fragment shading rate
///# C Specifications
///The [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExtent2D                            fragmentSize;
///    VkFragmentShadingRateCombinerOpKHR    combinerOps[2];
///} VkPipelineFragmentShadingRateStateCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_size`] specifies a [`Extent2D`] structure containing the fragment size used to
///   define the pipeline fragment shading rate for drawing commands using this pipeline.
/// - [`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`] value determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
///   [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive),
///   and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
///   are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining)
///   for fragments generated by drawing commands using the created pipeline.
///# Description
///If the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] includes a
///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure, then that
///structure includes parameters controlling the pipeline fragment shading
///rate.If this structure is not present, [`fragment_size`] is considered to be
///equal to (1,1), and both elements of [`combiner_ops`] are considered
///to be equal to `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`
/// - Any given element of [`combiner_ops`]**must** be a valid [`FragmentShadingRateCombinerOpKHR`]
///   value
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`Extent2D`]
/// - [`FragmentShadingRateCombinerOpKHR`]
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
pub struct PipelineFragmentShadingRateStateCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`fragment_size`] specifies a [`Extent2D`] structure containing the
    ///fragment size used to define the pipeline fragment shading rate for
    ///drawing commands using this pipeline.
    fragment_size: Extent2D,
    ///[`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`]
    ///value determining how the
    ///[pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
    ///[primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and
    ///[attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
    ///are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining) for fragments
    ///generated by drawing commands using the created pipeline.
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
}
///[VkPhysicalDeviceFragmentShadingRateFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateFeaturesKHR.html) - Structure indicating support for variable rate fragment shading
///# C Specifications
///The [`PhysicalDeviceFragmentShadingRateFeaturesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pipelineFragmentShadingRate;
///    VkBool32           primitiveFragmentShadingRate;
///    VkBool32           attachmentFragmentShadingRate;
///} VkPhysicalDeviceFragmentShadingRateFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_fragment_shading_rate`] indicates that the implementation supports the [pipeline fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline).
/// - [`primitive_fragment_shading_rate`] indicates that the implementation supports the [primitive fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive).
/// - [`attachment_fragment_shading_rate`] indicates that the implementation supports the [attachment fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment).
///If the [`PhysicalDeviceFragmentShadingRateFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentShadingRateFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
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
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`pipeline_fragment_shading_rate`] indicates that the implementation
    ///supports the [pipeline
    ///fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline).
    pipeline_fragment_shading_rate: Bool32,
    ///[`primitive_fragment_shading_rate`] indicates that the implementation
    ///supports the [primitive
    ///fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive).
    primitive_fragment_shading_rate: Bool32,
    ///[`attachment_fragment_shading_rate`] indicates that the implementation
    ///supports the [attachment
    ///fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment).
    attachment_fragment_shading_rate: Bool32,
}
///[VkPhysicalDeviceFragmentShadingRatePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRatePropertiesKHR.html) - Structure describing variable fragment shading rate limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkExtent2D               minFragmentShadingRateAttachmentTexelSize;
///    VkExtent2D               maxFragmentShadingRateAttachmentTexelSize;
///    uint32_t                 maxFragmentShadingRateAttachmentTexelSizeAspectRatio;
///    VkBool32                 primitiveFragmentShadingRateWithMultipleViewports;
///    VkBool32                 layeredShadingRateAttachments;
///    VkBool32                 fragmentShadingRateNonTrivialCombinerOps;
///    VkExtent2D               maxFragmentSize;
///    uint32_t                 maxFragmentSizeAspectRatio;
///    uint32_t                 maxFragmentShadingRateCoverageSamples;
///    VkSampleCountFlagBits    maxFragmentShadingRateRasterizationSamples;
///    VkBool32                 fragmentShadingRateWithShaderDepthStencilWrites;
///    VkBool32                 fragmentShadingRateWithSampleMask;
///    VkBool32                 fragmentShadingRateWithShaderSampleMask;
///    VkBool32                 fragmentShadingRateWithConservativeRasterization;
///    VkBool32                 fragmentShadingRateWithFragmentShaderInterlock;
///    VkBool32                 fragmentShadingRateWithCustomSampleLocations;
///    VkBool32                 fragmentShadingRateStrictMultiplyCombiner;
///} VkPhysicalDeviceFragmentShadingRatePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_fragment_shading_rate_attachment_texel_size`] indicates minimum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value **must** be less than or equal to the values in [`max_fragment_shading_rate_attachment_texel_size`]. Each value **must** be a power-of-two. It **must** be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`max_fragment_shading_rate_attachment_texel_size`] indicates maximum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value **must** be greater than or equal to the values in [`min_fragment_shading_rate_attachment_texel_size`]. Each value **must** be a power-of-two. It **must** be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the maximum ratio between the width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`]**must** be a power-of-two value, and **must** be less than or equal to max(`maxFragmentShadingRateAttachmentTexelSize.width` / `minFragmentShadingRateAttachmentTexelSize.height`, `maxFragmentShadingRateAttachmentTexelSize.height` / `minFragmentShadingRateAttachmentTexelSize.width`). It **must** be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`primitive_fragment_shading_rate_with_multiple_viewports`] specifies     whether the [primitive     fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive)**can** be used when multiple viewports are used.     If this value is [`FALSE`], only a single viewport **must** be used,     and applications **must** not write to the     `ViewportMaskNV` or     `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.     It **must** be [`FALSE`] if     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.
/// - [`layered_shading_rate_attachments`] specifies whether a shading rate     attachment image view **can** be created with multiple layers.     If this value is [`FALSE`], when creating an image view with a     `usage` that includes     `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,     `layerCount`**must** be `1`.     It **must** be [`FALSE`] if     the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature,     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`fragment_shading_rate_non_trivial_combiner_ops`] specifies whether [`FragmentShadingRateCombinerOpKHR`] enums other than `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR`**can** be used. It **must** be [`FALSE`] unless either the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.
/// - [`max_fragment_size`] indicates the maximum supported width and height of a fragment. Its
///   `width` and `height` members **must** both be power-of-two values. This limit is purely
///   informational, and is not validated.
/// - [`max_fragment_size_aspect_ratio`] indicates the maximum ratio between the width and height of
///   a fragment. [`max_fragment_size_aspect_ratio`]**must** be a power-of-two value, and **must**
///   be less than or equal to the maximum of the `width` and `height` members of
///   [`max_fragment_size`]. This limit is purely informational, and is not validated.
/// - [`max_fragment_shading_rate_coverage_samples`] specifies the maximum number of coverage
///   samples supported in a single fragment. [`max_fragment_shading_rate_coverage_samples`]**must**
///   be less than or equal to the product of the `width` and `height` members of
///   [`max_fragment_size`], and the sample count reported by
///   [`max_fragment_shading_rate_rasterization_samples`].
///   [`max_fragment_shading_rate_coverage_samples`]**must** be less than or equal to
///   `maxSampleMaskWords` × 32 if [`fragment_shading_rate_with_shader_sample_mask`] is supported.
///   This limit is purely informational, and is not validated.
/// - [`max_fragment_shading_rate_rasterization_samples`] is a [`SampleCountFlagBits`] value
///   specifying the maximum sample rate supported when a fragment covers multiple pixels. This
///   limit is purely informational, and is not validated.
/// - [`fragment_shading_rate_with_shader_depth_stencil_writes`] specifies whether the
///   implementation supports writing `FragDepth` or `FragStencilRefEXT` from a fragment shader for
///   multi-pixel fragments. If this value is [`FALSE`], writing to those built-ins will clamp the
///   fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_sample_mask`] specifies whether the the implementation supports
///   setting valid bits of [`PipelineMultisampleStateCreateInfo::p_sample_mask`] to `0` for
///   multi-pixel fragments. If this value is [`FALSE`], zeroing valid bits in the sample mask will
///   clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_shader_sample_mask`] specifies whether the implementation
///   supports reading or writing [`SampleMask`] for multi-pixel fragments. If this value is
///   [`FALSE`], using that built-in will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_conservative_rasterization`] specifies whether [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) is supported for multi-pixel fragments. It **must** be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]` is not supported. If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_fragment_shader_interlock`] specifies whether [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments. It **must** be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]` is not supported. If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_custom_sample_locations`] specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) are supported for multi-pixel fragments. It **must** be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not supported. If this value is [`FALSE`], using [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_strict_multiply_combiner`] specifies whether
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a multiplication or not.
///   Implementations where this value is [`FALSE`] will instead combine rates with an addition. If
///   [`fragment_shading_rate_non_trivial_combiner_ops`] is [`FALSE`], implementations **must**
///   report this as [`FALSE`]. If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`],
///   implementations **should** report this as [`TRUE`].
///# Description
///If the [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties are related to [fragment
///shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`Bool32`]
/// - [`Extent2D`]
/// - [`SampleCountFlagBits`]
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
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`min_fragment_shading_rate_attachment_texel_size`] indicates minimum
    ///supported width and height of the portion of the framebuffer
    ///corresponding to each texel in a fragment shading rate attachment.
    ///Each value **must** be less than or equal to the values in
    ///[`max_fragment_shading_rate_attachment_texel_size`].
    ///Each value **must** be a power-of-two.
    ///It **must** be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    min_fragment_shading_rate_attachment_texel_size: Extent2D,
    ///[`max_fragment_shading_rate_attachment_texel_size`] indicates maximum
    ///supported width and height of the portion of the framebuffer
    ///corresponding to each texel in a fragment shading rate attachment.
    ///Each value **must** be greater than or equal to the values in
    ///[`min_fragment_shading_rate_attachment_texel_size`].
    ///Each value **must** be a power-of-two.
    ///It **must** be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    max_fragment_shading_rate_attachment_texel_size: Extent2D,
    ///[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the
    ///maximum ratio between the width and height of the portion of the
    ///framebuffer corresponding to each texel in a fragment shading rate
    ///attachment.
    ///[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`]**must** be a
    ///power-of-two value, and **must** be less than or equal to
    ///max(`maxFragmentShadingRateAttachmentTexelSize.width` /
    ///`minFragmentShadingRateAttachmentTexelSize.height`,
    ///`maxFragmentShadingRateAttachmentTexelSize.height` /
    ///`minFragmentShadingRateAttachmentTexelSize.width`).
    ///It **must** be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    ///[`primitive_fragment_shading_rate_with_multiple_viewports`] specifies
    ///    whether the [primitive
    ///    fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive)**can** be used when multiple viewports are used.
    ///    If this value is [`FALSE`], only a single viewport **must** be used,
    ///    and applications **must** not write to the
    ///    `ViewportMaskNV` or
    ///    `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.
    ///    It **must** be [`FALSE`] if
    ///    the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,
    ///    the `[`VK_EXT_shader_viewport_index_layer`]` extension,
    ///or
    ///    the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not
    ///    supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.
    primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    ///[`layered_shading_rate_attachments`] specifies whether a shading rate
    ///    attachment image view **can** be created with multiple layers.
    ///    If this value is [`FALSE`], when creating an image view with a
    ///    `usage` that includes
    ///    `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,
    ///    `layerCount`**must** be `1`.
    ///    It **must** be [`FALSE`] if
    ///    the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature,
    ///    the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,
    ///    the `[`VK_EXT_shader_viewport_index_layer`]` extension,
    ///or
    ///    the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not
    ///    supported, or if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    layered_shading_rate_attachments: Bool32,
    ///[`fragment_shading_rate_non_trivial_combiner_ops`] specifies whether
    ///[`FragmentShadingRateCombinerOpKHR`] enums other than
    ///`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or
    ///`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR`**can** be used.
    ///It **must** be [`FALSE`] unless either the
    ///[`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or
    ///[`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.
    fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    ///[`max_fragment_size`] indicates the maximum
    ///supported width and height of a fragment.
    ///Its `width` and `height` members **must** both be power-of-two
    ///values.
    ///This limit is purely informational, and is not validated.
    max_fragment_size: Extent2D,
    ///[`max_fragment_size_aspect_ratio`]
    ///indicates the maximum ratio between the width and height of a fragment.
    ///[`max_fragment_size_aspect_ratio`]**must** be a power-of-two value, and
    ///**must** be less than or equal to the maximum of the `width` and
    ///`height` members of [`max_fragment_size`].
    ///This limit is purely informational, and is not validated.
    max_fragment_size_aspect_ratio: u32,
    ///[`max_fragment_shading_rate_coverage_samples`] specifies the maximum number
    ///of coverage samples supported in a single fragment.
    ///[`max_fragment_shading_rate_coverage_samples`]**must** be less than or equal
    ///to the product of the `width` and `height` members of
    ///[`max_fragment_size`], and the sample count reported by
    ///[`max_fragment_shading_rate_rasterization_samples`].
    ///[`max_fragment_shading_rate_coverage_samples`]**must** be less than or equal
    ///to `maxSampleMaskWords` × 32 if
    ///[`fragment_shading_rate_with_shader_sample_mask`] is supported.
    ///This limit is purely informational, and is not validated.
    max_fragment_shading_rate_coverage_samples: u32,
    ///[`max_fragment_shading_rate_rasterization_samples`] is a
    ///[`SampleCountFlagBits`] value specifying the maximum sample rate
    ///supported when a fragment covers multiple pixels.
    ///This limit is purely informational, and is not validated.
    max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
    ///[`fragment_shading_rate_with_shader_depth_stencil_writes`] specifies whether
    ///the implementation supports writing `FragDepth`
    ///or `FragStencilRefEXT`
    ///from a fragment shader for multi-pixel fragments.
    ///If this value is [`FALSE`], writing to those built-ins will clamp
    ///the fragment shading rate to (1,1).
    fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    ///[`fragment_shading_rate_with_sample_mask`] specifies whether the the
    ///implementation supports setting valid bits of
    ///[`PipelineMultisampleStateCreateInfo`]::`pSampleMask` to `0` for
    ///multi-pixel fragments.
    ///If this value is [`FALSE`], zeroing valid bits in the sample mask
    ///will clamp the fragment shading rate to (1,1).
    fragment_shading_rate_with_sample_mask: Bool32,
    ///[`fragment_shading_rate_with_shader_sample_mask`] specifies whether the
    ///implementation supports reading or writing [`SampleMask`] for
    ///multi-pixel fragments.
    ///If this value is [`FALSE`], using that built-in will clamp the
    ///fragment shading rate to (1,1).
    fragment_shading_rate_with_shader_sample_mask: Bool32,
    ///[`fragment_shading_rate_with_conservative_rasterization`]
    ///specifies whether [conservative
    ///rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) is supported for multi-pixel fragments.
    ///It **must** be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]`
    ///is not supported.
    ///If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to
    ///(1,1).
    fragment_shading_rate_with_conservative_rasterization: Bool32,
    ///[`fragment_shading_rate_with_fragment_shader_interlock`]
    ///specifies whether [fragment shader
    ///interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments.
    ///It **must** be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]`
    ///is not supported.
    ///If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to
    ///(1,1).
    fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    ///[`fragment_shading_rate_with_custom_sample_locations`]
    ///specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations)
    ///are supported for multi-pixel fragments.
    ///It **must** be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not
    ///supported.
    ///If this value is [`FALSE`], using [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) will clamp the fragment shading rate to
    ///(1,1).
    fragment_shading_rate_with_custom_sample_locations: Bool32,
    ///[`fragment_shading_rate_strict_multiply_combiner`] specifies whether
    ///`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a
    ///multiplication or not.
    ///Implementations where this value is [`FALSE`] will instead combine
    ///rates with an addition.
    ///If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`FALSE`],
    ///implementations **must** report this as [`FALSE`].
    ///If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`],
    ///implementations **should** report this as [`TRUE`].
    fragment_shading_rate_strict_multiply_combiner: Bool32,
}
///[VkPhysicalDeviceFragmentShadingRateKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateKHR.html) - Structure returning information about sample count specific additional multisampling capabilities
///# C Specifications
///The [`PhysicalDeviceFragmentShadingRateKHR`] structure is defined as
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef struct VkPhysicalDeviceFragmentShadingRateKHR {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkSampleCountFlags    sampleCounts;
///    VkExtent2D            fragmentSize;
///} VkPhysicalDeviceFragmentShadingRateKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sample_counts`] is a bitmask of sample counts for which the shading rate described by
///   [`fragment_size`] is supported.
/// - [`fragment_size`] is a [`Extent2D`] describing the width and height of a supported shading
///   rate.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`Extent2D`]
/// - [`SampleCountFlags`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceFragmentShadingRatesKHR`]
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
pub struct PhysicalDeviceFragmentShadingRateKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`sample_counts`] is a bitmask of sample counts for which the shading
    ///rate described by [`fragment_size`] is supported.
    sample_counts: SampleCountFlags,
    ///[`fragment_size`] is a [`Extent2D`] describing the width and height
    ///of a supported shading rate.
    fragment_size: Extent2D,
}
