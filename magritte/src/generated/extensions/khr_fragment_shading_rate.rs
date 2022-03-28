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
/// - [`fragment_shading_rate_attachment`] is `NULL` or a pointer to a [`AttachmentReference2`]
///   structure defining the fragment shading rate attachment for this subpass.
/// - [`shading_rate_attachment_texel_size`] specifies the size of the portion of the framebuffer
///   corresponding to each texel in [`fragment_shading_rate_attachment`].
///# Description
///If no shading rate attachment is specified, or if this structure is not
///specified, the implementation behaves as if a valid shading rate attachment
///was specified with all texels specifying a single pixel per fragment.
///## Valid Usage
/// - If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], its `layout` member  **must**  be equal to `VK_IMAGE_LAYOUT_GENERAL` or
///   `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
/// - If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width` **must**  be a power of two
///   value
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.width` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// - If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not
///   [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height` **must**  be a power of two
///   value
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], `shadingRateAttachmentTexelSize.height` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], the quotient of `shadingRateAttachmentTexelSize.width` and `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
/// -    If [`fragment_shading_rate_attachment`] is not `NULL` and its `attachment` member is not [`ATTACHMENT_UNUSED`], the quotient of `shadingRateAttachmentTexelSize.height` and `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
/// - If [`fragment_shading_rate_attachment`] is not `NULL`, [`fragment_shading_rate_attachment`]
///   **must**  be a valid pointer to a valid [`AttachmentReference2`] structure
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`fragment_shading_rate_attachment`] is `NULL` or a pointer to a
    ///[`AttachmentReference2`] structure defining the fragment shading
    ///rate attachment for this subpass.
    fragment_shading_rate_attachment: *const AttachmentReference2<'lt>,
    ///[`shading_rate_attachment_texel_size`] specifies the size of the portion
    ///of the framebuffer corresponding to each texel in
    ///[`fragment_shading_rate_attachment`].
    shading_rate_attachment_texel_size: Extent2D,
}
impl<'lt> Default for FragmentShadingRateAttachmentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            fragment_shading_rate_attachment: std::ptr::null(),
            shading_rate_attachment_texel_size: Default::default(),
        }
    }
}
impl<'lt> FragmentShadingRateAttachmentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_attachment`]
    pub fn fragment_shading_rate_attachment_raw(&self) -> *const AttachmentReference2<'lt> {
        self.fragment_shading_rate_attachment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_attachment`]
    pub fn set_fragment_shading_rate_attachment_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.fragment_shading_rate_attachment = value;
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::fragment_shading_rate_attachment`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn fragment_shading_rate_attachment(&self) -> &AttachmentReference2<'lt> {
        &*self.fragment_shading_rate_attachment
    }
    ///Gets the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.shading_rate_attachment_texel_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.shading_rate_attachment_texel_size
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_attachment`]
    pub fn set_fragment_shading_rate_attachment(
        &mut self,
        value: &'lt crate::vulkan1_2::AttachmentReference2<'lt>,
    ) -> &mut Self {
        self.fragment_shading_rate_attachment = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_attachment_texel_size`]
    pub fn set_shading_rate_attachment_texel_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.shading_rate_attachment_texel_size = value;
        self
    }
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
///to be equal to `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`
/// - Any given element of [`combiner_ops`] **must**  be a valid
///   [`FragmentShadingRateCombinerOpKHR`] value
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
impl<'lt> Default for PipelineFragmentShadingRateStateCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            fragment_size: Default::default(),
            combiner_ops: [Default::default(); 2],
        }
    }
}
impl<'lt> PipelineFragmentShadingRateStateCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::fragment_size`]
    pub fn fragment_size(&self) -> Extent2D {
        self.fragment_size
    }
    ///Gets the value of [`Self::combiner_ops`]
    pub fn combiner_ops(&self) -> &[FragmentShadingRateCombinerOpKHR; 2] {
        &getter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::fragment_size`]
    pub fn fragment_size_mut(&mut self) -> &mut Extent2D {
        &mut self.fragment_size
    }
    ///Gets a mutable reference to the value of [`Self::combiner_ops`]
    pub fn combiner_ops_mut(&mut self) -> &mut [FragmentShadingRateCombinerOpKHR; 2] {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::fragment_size`]
    pub fn set_fragment_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.fragment_size = value;
        self
    }
    ///Sets the raw value of [`Self::combiner_ops`]
    pub fn set_combiner_ops(
        &mut self,
        value: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2],
    ) -> &mut Self {
        self.combiner_ops = value;
        self
    }
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
///[`PhysicalDeviceFragmentShadingRateFeaturesKHR`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for PhysicalDeviceFragmentShadingRateFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            pipeline_fragment_shading_rate: 0,
            primitive_fragment_shading_rate: 0,
            attachment_fragment_shading_rate: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShadingRateFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::pipeline_fragment_shading_rate`]
    pub fn pipeline_fragment_shading_rate_raw(&self) -> Bool32 {
        self.pipeline_fragment_shading_rate
    }
    ///Gets the raw value of [`Self::primitive_fragment_shading_rate`]
    pub fn primitive_fragment_shading_rate_raw(&self) -> Bool32 {
        self.primitive_fragment_shading_rate
    }
    ///Gets the raw value of [`Self::attachment_fragment_shading_rate`]
    pub fn attachment_fragment_shading_rate_raw(&self) -> Bool32 {
        self.attachment_fragment_shading_rate
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_fragment_shading_rate`]
    pub fn set_pipeline_fragment_shading_rate_raw(&mut self, value: Bool32) -> &mut Self {
        self.pipeline_fragment_shading_rate = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_fragment_shading_rate`]
    pub fn set_primitive_fragment_shading_rate_raw(&mut self, value: Bool32) -> &mut Self {
        self.primitive_fragment_shading_rate = value;
        self
    }
    ///Sets the raw value of [`Self::attachment_fragment_shading_rate`]
    pub fn set_attachment_fragment_shading_rate_raw(&mut self, value: Bool32) -> &mut Self {
        self.attachment_fragment_shading_rate = value;
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
    ///Gets the value of [`Self::pipeline_fragment_shading_rate`]
    pub fn pipeline_fragment_shading_rate(&self) -> bool {
        unsafe { std::mem::transmute(self.pipeline_fragment_shading_rate as u8) }
    }
    ///Gets the value of [`Self::primitive_fragment_shading_rate`]
    pub fn primitive_fragment_shading_rate(&self) -> bool {
        unsafe { std::mem::transmute(self.primitive_fragment_shading_rate as u8) }
    }
    ///Gets the value of [`Self::attachment_fragment_shading_rate`]
    pub fn attachment_fragment_shading_rate(&self) -> bool {
        unsafe { std::mem::transmute(self.attachment_fragment_shading_rate as u8) }
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
    ///Gets a mutable reference to the value of [`Self::pipeline_fragment_shading_rate`]
    pub fn pipeline_fragment_shading_rate_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pipeline_fragment_shading_rate as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pipeline_fragment_shading_rate as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::primitive_fragment_shading_rate`]
    pub fn primitive_fragment_shading_rate_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.primitive_fragment_shading_rate as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.primitive_fragment_shading_rate as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::attachment_fragment_shading_rate`]
    pub fn attachment_fragment_shading_rate_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.attachment_fragment_shading_rate as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.attachment_fragment_shading_rate as *mut Bool32)
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
    ///Sets the raw value of [`Self::pipeline_fragment_shading_rate`]
    pub fn set_pipeline_fragment_shading_rate(&mut self, value: bool) -> &mut Self {
        self.pipeline_fragment_shading_rate = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::primitive_fragment_shading_rate`]
    pub fn set_primitive_fragment_shading_rate(&mut self, value: bool) -> &mut Self {
        self.primitive_fragment_shading_rate = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::attachment_fragment_shading_rate`]
    pub fn set_attachment_fragment_shading_rate(&mut self, value: bool) -> &mut Self {
        self.attachment_fragment_shading_rate = value as u8 as u32;
        self
    }
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
/// - [`min_fragment_shading_rate_attachment_texel_size`] indicates minimum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value  **must**  be less than or equal to the values in [`max_fragment_shading_rate_attachment_texel_size`]. Each value  **must**  be a power-of-two. It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`max_fragment_shading_rate_attachment_texel_size`] indicates maximum supported width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. Each value  **must**  be greater than or equal to the values in [`min_fragment_shading_rate_attachment_texel_size`]. Each value  **must**  be a power-of-two. It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the maximum ratio between the width and height of the portion of the framebuffer corresponding to each texel in a fragment shading rate attachment. [`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] **must**  be a power-of-two value, and  **must**  be less than or equal to max(`maxFragmentShadingRateAttachmentTexelSize.width` / `minFragmentShadingRateAttachmentTexelSize.height`, `maxFragmentShadingRateAttachmentTexelSize.height` / `minFragmentShadingRateAttachmentTexelSize.width`). It  **must**  be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`primitive_fragment_shading_rate_with_multiple_viewports`] specifies     whether the [primitive     fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive) **can**  be used when multiple viewports are used.     If this value is [`FALSE`], only a single viewport  **must**  be used,     and applications  **must**  not write to the     `ViewportMaskNV` or     `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.     It  **must**  be [`FALSE`] if     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.
/// - [`layered_shading_rate_attachments`] specifies whether a shading rate     attachment image view  **can**  be created with multiple layers.     If this value is [`FALSE`], when creating an image view with a     `usage` that includes     `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,     `layerCount` **must**  be `1`.     It  **must**  be [`FALSE`] if     the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview) feature,     the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,     the `[`VK_EXT_shader_viewport_index_layer`]` extension, or     the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not     supported, or if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
/// - [`fragment_shading_rate_non_trivial_combiner_ops`] specifies whether [`FragmentShadingRateCombinerOpKHR`] enums other than `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR` **can**  be used. It  **must**  be [`FALSE`] unless either the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.
/// - [`max_fragment_size`] indicates the maximum supported width and height of a fragment. Its
///   `width` and `height` members  **must**  both be power-of-two values. This limit is purely
///   informational, and is not validated.
/// - [`max_fragment_size_aspect_ratio`] indicates the maximum ratio between the width and height of
///   a fragment. [`max_fragment_size_aspect_ratio`] **must**  be a power-of-two value, and
///   **must**  be less than or equal to the maximum of the `width` and `height` members of
///   [`max_fragment_size`]. This limit is purely informational, and is not validated.
/// - [`max_fragment_shading_rate_coverage_samples`] specifies the maximum number of coverage
///   samples supported in a single fragment. [`max_fragment_shading_rate_coverage_samples`]
///   **must**  be less than or equal to the product of the `width` and `height` members of
///   [`max_fragment_size`], and the sample count reported by
///   [`max_fragment_shading_rate_rasterization_samples`].
///   [`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal to
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
///   setting valid bits of [`PipelineMultisampleStateCreateInfo::sample_mask`] to `0` for
///   multi-pixel fragments. If this value is [`FALSE`], zeroing valid bits in the sample mask will
///   clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_shader_sample_mask`] specifies whether the implementation
///   supports reading or writing [`SampleMask`] for multi-pixel fragments. If this value is
///   [`FALSE`], using that built-in will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_conservative_rasterization`] specifies whether [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) is supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]` is not supported. If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_fragment_shader_interlock`] specifies whether [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]` is not supported. If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_with_custom_sample_locations`] specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) are supported for multi-pixel fragments. It  **must**  be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not supported. If this value is [`FALSE`], using [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations) will clamp the fragment shading rate to (1,1).
/// - [`fragment_shading_rate_strict_multiply_combiner`] specifies whether
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR` accurately performs a multiplication or not.
///   Implementations where this value is [`FALSE`] will instead combine rates with an addition. If
///   [`fragment_shading_rate_non_trivial_combiner_ops`] is [`FALSE`], implementations  **must**
///   report this as [`FALSE`]. If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`],
///   implementations  **should**  report this as [`TRUE`].
///# Description
///If the [`PhysicalDeviceFragmentShadingRatePropertiesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties are related to [fragment
///shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`min_fragment_shading_rate_attachment_texel_size`] indicates minimum
    ///supported width and height of the portion of the framebuffer
    ///corresponding to each texel in a fragment shading rate attachment.
    ///Each value  **must**  be less than or equal to the values in
    ///[`max_fragment_shading_rate_attachment_texel_size`].
    ///Each value  **must**  be a power-of-two.
    ///It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    min_fragment_shading_rate_attachment_texel_size: Extent2D,
    ///[`max_fragment_shading_rate_attachment_texel_size`] indicates maximum
    ///supported width and height of the portion of the framebuffer
    ///corresponding to each texel in a fragment shading rate attachment.
    ///Each value  **must**  be greater than or equal to the values in
    ///[`min_fragment_shading_rate_attachment_texel_size`].
    ///Each value  **must**  be a power-of-two.
    ///It  **must**  be (0,0) if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    max_fragment_shading_rate_attachment_texel_size: Extent2D,
    ///[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] indicates the
    ///maximum ratio between the width and height of the portion of the
    ///framebuffer corresponding to each texel in a fragment shading rate
    ///attachment.
    ///[`max_fragment_shading_rate_attachment_texel_size_aspect_ratio`] **must**  be a
    ///power-of-two value, and  **must**  be less than or equal to
    ///max(`maxFragmentShadingRateAttachmentTexelSize.width` /
    ///`minFragmentShadingRateAttachmentTexelSize.height`,
    ///`maxFragmentShadingRateAttachmentTexelSize.height` /
    ///`minFragmentShadingRateAttachmentTexelSize.width`).
    ///It  **must**  be 0 if the [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is not supported.
    max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    ///[`primitive_fragment_shading_rate_with_multiple_viewports`] specifies
    ///    whether the [primitive
    ///    fragment shading rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive) **can**  be used when multiple viewports are used.
    ///    If this value is [`FALSE`], only a single viewport  **must**  be used,
    ///    and applications  **must**  not write to the
    ///    `ViewportMaskNV` or
    ///    `ViewportIndex` built-in when setting `PrimitiveShadingRateKHR`.
    ///    It  **must**  be [`FALSE`] if
    ///    the [`shaderOutputViewportIndex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderOutputViewportIndex) feature,
    ///    the `[`VK_EXT_shader_viewport_index_layer`]` extension,
    ///or
    ///    the [`geometryShader`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) feature is not
    ///    supported, or if the [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) feature is not supported.
    primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    ///[`layered_shading_rate_attachments`] specifies whether a shading rate
    ///    attachment image view  **can**  be created with multiple layers.
    ///    If this value is [`FALSE`], when creating an image view with a
    ///    `usage` that includes
    ///    `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`,
    ///    `layerCount` **must**  be `1`.
    ///    It  **must**  be [`FALSE`] if
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
    ///`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR` **can**  be used.
    ///It  **must**  be [`FALSE`] unless either the
    ///[`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate) or
    ///[`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate) feature is supported.
    fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    ///[`max_fragment_size`] indicates the maximum
    ///supported width and height of a fragment.
    ///Its `width` and `height` members  **must**  both be power-of-two
    ///values.
    ///This limit is purely informational, and is not validated.
    max_fragment_size: Extent2D,
    ///[`max_fragment_size_aspect_ratio`]
    ///indicates the maximum ratio between the width and height of a fragment.
    ///[`max_fragment_size_aspect_ratio`] **must**  be a power-of-two value, and
    /// **must**  be less than or equal to the maximum of the `width` and
    ///`height` members of [`max_fragment_size`].
    ///This limit is purely informational, and is not validated.
    max_fragment_size_aspect_ratio: u32,
    ///[`max_fragment_shading_rate_coverage_samples`] specifies the maximum number
    ///of coverage samples supported in a single fragment.
    ///[`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal
    ///to the product of the `width` and `height` members of
    ///[`max_fragment_size`], and the sample count reported by
    ///[`max_fragment_shading_rate_rasterization_samples`].
    ///[`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal
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
    ///It  **must**  be [`FALSE`] if `[`VK_EXT_conservative_rasterization`]`
    ///is not supported.
    ///If this value is [`FALSE`], using [conservative rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-conservativeraster) will clamp the fragment shading rate to
    ///(1,1).
    fragment_shading_rate_with_conservative_rasterization: Bool32,
    ///[`fragment_shading_rate_with_fragment_shader_interlock`]
    ///specifies whether [fragment shader
    ///interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) is supported for multi-pixel fragments.
    ///It  **must**  be [`FALSE`] if `[`VK_EXT_fragment_shader_interlock`]`
    ///is not supported.
    ///If this value is [`FALSE`], using [fragment shader interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-shader-interlock) will clamp the fragment shading rate to
    ///(1,1).
    fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    ///[`fragment_shading_rate_with_custom_sample_locations`]
    ///specifies whether [custom sample locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-samplelocations)
    ///are supported for multi-pixel fragments.
    ///It  **must**  be [`FALSE`] if `[`VK_EXT_sample_locations`]` is not
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
    ///implementations  **must**  report this as [`FALSE`].
    ///If [`fragment_shading_rate_non_trivial_combiner_ops`] is [`TRUE`],
    ///implementations  **should**  report this as [`TRUE`].
    fragment_shading_rate_strict_multiply_combiner: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentShadingRatePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            min_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size: Default::default(),
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: 0,
            primitive_fragment_shading_rate_with_multiple_viewports: 0,
            layered_shading_rate_attachments: 0,
            fragment_shading_rate_non_trivial_combiner_ops: 0,
            max_fragment_size: Default::default(),
            max_fragment_size_aspect_ratio: 0,
            max_fragment_shading_rate_coverage_samples: 0,
            max_fragment_shading_rate_rasterization_samples: Default::default(),
            fragment_shading_rate_with_shader_depth_stencil_writes: 0,
            fragment_shading_rate_with_sample_mask: 0,
            fragment_shading_rate_with_shader_sample_mask: 0,
            fragment_shading_rate_with_conservative_rasterization: 0,
            fragment_shading_rate_with_fragment_shader_interlock: 0,
            fragment_shading_rate_with_custom_sample_locations: 0,
            fragment_shading_rate_strict_multiply_combiner: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShadingRatePropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::primitive_fragment_shading_rate_with_multiple_viewports`]
    pub fn primitive_fragment_shading_rate_with_multiple_viewports_raw(&self) -> Bool32 {
        self.primitive_fragment_shading_rate_with_multiple_viewports
    }
    ///Gets the raw value of [`Self::layered_shading_rate_attachments`]
    pub fn layered_shading_rate_attachments_raw(&self) -> Bool32 {
        self.layered_shading_rate_attachments
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_non_trivial_combiner_ops`]
    pub fn fragment_shading_rate_non_trivial_combiner_ops_raw(&self) -> Bool32 {
        self.fragment_shading_rate_non_trivial_combiner_ops
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_shader_depth_stencil_writes`]
    pub fn fragment_shading_rate_with_shader_depth_stencil_writes_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_shader_depth_stencil_writes
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_sample_mask`]
    pub fn fragment_shading_rate_with_sample_mask_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_sample_mask
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_shader_sample_mask`]
    pub fn fragment_shading_rate_with_shader_sample_mask_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_shader_sample_mask
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_conservative_rasterization`]
    pub fn fragment_shading_rate_with_conservative_rasterization_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_conservative_rasterization
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_fragment_shader_interlock`]
    pub fn fragment_shading_rate_with_fragment_shader_interlock_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_fragment_shader_interlock
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_with_custom_sample_locations`]
    pub fn fragment_shading_rate_with_custom_sample_locations_raw(&self) -> Bool32 {
        self.fragment_shading_rate_with_custom_sample_locations
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_strict_multiply_combiner`]
    pub fn fragment_shading_rate_strict_multiply_combiner_raw(&self) -> Bool32 {
        self.fragment_shading_rate_strict_multiply_combiner
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_fragment_shading_rate_with_multiple_viewports`]
    pub fn set_primitive_fragment_shading_rate_with_multiple_viewports_raw(&mut self, value: Bool32) -> &mut Self {
        self.primitive_fragment_shading_rate_with_multiple_viewports = value;
        self
    }
    ///Sets the raw value of [`Self::layered_shading_rate_attachments`]
    pub fn set_layered_shading_rate_attachments_raw(&mut self, value: Bool32) -> &mut Self {
        self.layered_shading_rate_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_non_trivial_combiner_ops`]
    pub fn set_fragment_shading_rate_non_trivial_combiner_ops_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_non_trivial_combiner_ops = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_shader_depth_stencil_writes`]
    pub fn set_fragment_shading_rate_with_shader_depth_stencil_writes_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_shader_depth_stencil_writes = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_sample_mask`]
    pub fn set_fragment_shading_rate_with_sample_mask_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_sample_mask = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_shader_sample_mask`]
    pub fn set_fragment_shading_rate_with_shader_sample_mask_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_shader_sample_mask = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_conservative_rasterization`]
    pub fn set_fragment_shading_rate_with_conservative_rasterization_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_conservative_rasterization = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_fragment_shader_interlock`]
    pub fn set_fragment_shading_rate_with_fragment_shader_interlock_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_fragment_shader_interlock = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_custom_sample_locations`]
    pub fn set_fragment_shading_rate_with_custom_sample_locations_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_with_custom_sample_locations = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_strict_multiply_combiner`]
    pub fn set_fragment_shading_rate_strict_multiply_combiner_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shading_rate_strict_multiply_combiner = value;
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
    ///Gets the value of [`Self::min_fragment_shading_rate_attachment_texel_size`]
    pub fn min_fragment_shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.min_fragment_shading_rate_attachment_texel_size
    }
    ///Gets the value of [`Self::max_fragment_shading_rate_attachment_texel_size`]
    pub fn max_fragment_shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.max_fragment_shading_rate_attachment_texel_size
    }
    ///Gets the value of [`Self::max_fragment_shading_rate_attachment_texel_size_aspect_ratio`]
    pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio(&self) -> u32 {
        self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio
    }
    ///Gets the value of [`Self::primitive_fragment_shading_rate_with_multiple_viewports`]
    pub fn primitive_fragment_shading_rate_with_multiple_viewports(&self) -> bool {
        unsafe { std::mem::transmute(self.primitive_fragment_shading_rate_with_multiple_viewports as u8) }
    }
    ///Gets the value of [`Self::layered_shading_rate_attachments`]
    pub fn layered_shading_rate_attachments(&self) -> bool {
        unsafe { std::mem::transmute(self.layered_shading_rate_attachments as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_non_trivial_combiner_ops`]
    pub fn fragment_shading_rate_non_trivial_combiner_ops(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_non_trivial_combiner_ops as u8) }
    }
    ///Gets the value of [`Self::max_fragment_size`]
    pub fn max_fragment_size(&self) -> Extent2D {
        self.max_fragment_size
    }
    ///Gets the value of [`Self::max_fragment_size_aspect_ratio`]
    pub fn max_fragment_size_aspect_ratio(&self) -> u32 {
        self.max_fragment_size_aspect_ratio
    }
    ///Gets the value of [`Self::max_fragment_shading_rate_coverage_samples`]
    pub fn max_fragment_shading_rate_coverage_samples(&self) -> u32 {
        self.max_fragment_shading_rate_coverage_samples
    }
    ///Gets the value of [`Self::max_fragment_shading_rate_rasterization_samples`]
    pub fn max_fragment_shading_rate_rasterization_samples(&self) -> SampleCountFlagBits {
        self.max_fragment_shading_rate_rasterization_samples
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_shader_depth_stencil_writes`]
    pub fn fragment_shading_rate_with_shader_depth_stencil_writes(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_shader_depth_stencil_writes as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_sample_mask`]
    pub fn fragment_shading_rate_with_sample_mask(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_sample_mask as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_shader_sample_mask`]
    pub fn fragment_shading_rate_with_shader_sample_mask(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_shader_sample_mask as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_conservative_rasterization`]
    pub fn fragment_shading_rate_with_conservative_rasterization(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_conservative_rasterization as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_fragment_shader_interlock`]
    pub fn fragment_shading_rate_with_fragment_shader_interlock(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_fragment_shader_interlock as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_with_custom_sample_locations`]
    pub fn fragment_shading_rate_with_custom_sample_locations(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_with_custom_sample_locations as u8) }
    }
    ///Gets the value of [`Self::fragment_shading_rate_strict_multiply_combiner`]
    pub fn fragment_shading_rate_strict_multiply_combiner(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_strict_multiply_combiner as u8) }
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
    ///Gets a mutable reference to the value of
    /// [`Self::min_fragment_shading_rate_attachment_texel_size`]
    pub fn min_fragment_shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.min_fragment_shading_rate_attachment_texel_size
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_fragment_shading_rate_attachment_texel_size`]
    pub fn max_fragment_shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.max_fragment_shading_rate_attachment_texel_size
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_fragment_shading_rate_attachment_texel_size_aspect_ratio`]
    pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::primitive_fragment_shading_rate_with_multiple_viewports`]
    pub fn primitive_fragment_shading_rate_with_multiple_viewports_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.primitive_fragment_shading_rate_with_multiple_viewports as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.primitive_fragment_shading_rate_with_multiple_viewports as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::layered_shading_rate_attachments`]
    pub fn layered_shading_rate_attachments_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.layered_shading_rate_attachments as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.layered_shading_rate_attachments as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_non_trivial_combiner_ops`]
    pub fn fragment_shading_rate_non_trivial_combiner_ops_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_non_trivial_combiner_ops as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_non_trivial_combiner_ops as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_fragment_size`]
    pub fn max_fragment_size_mut(&mut self) -> &mut Extent2D {
        &mut self.max_fragment_size
    }
    ///Gets a mutable reference to the value of [`Self::max_fragment_size_aspect_ratio`]
    pub fn max_fragment_size_aspect_ratio_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_fragment_shading_rate_coverage_samples`]
    pub fn max_fragment_shading_rate_coverage_samples_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_fragment_shading_rate_rasterization_samples`]
    pub fn max_fragment_shading_rate_rasterization_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.max_fragment_shading_rate_rasterization_samples
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_with_shader_depth_stencil_writes`]
    pub fn fragment_shading_rate_with_shader_depth_stencil_writes_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_shader_depth_stencil_writes as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_shader_depth_stencil_writes as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::fragment_shading_rate_with_sample_mask`]
    pub fn fragment_shading_rate_with_sample_mask_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_sample_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_sample_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_with_shader_sample_mask`]
    pub fn fragment_shading_rate_with_shader_sample_mask_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_shader_sample_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_shader_sample_mask as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_with_conservative_rasterization`]
    pub fn fragment_shading_rate_with_conservative_rasterization_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_conservative_rasterization as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_conservative_rasterization as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_with_fragment_shader_interlock`]
    pub fn fragment_shading_rate_with_fragment_shader_interlock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_fragment_shader_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_fragment_shader_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_with_custom_sample_locations`]
    pub fn fragment_shading_rate_with_custom_sample_locations_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_with_custom_sample_locations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_with_custom_sample_locations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_shading_rate_strict_multiply_combiner`]
    pub fn fragment_shading_rate_strict_multiply_combiner_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_strict_multiply_combiner as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_strict_multiply_combiner as *mut Bool32)
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
    ///Sets the raw value of [`Self::min_fragment_shading_rate_attachment_texel_size`]
    pub fn set_min_fragment_shading_rate_attachment_texel_size(
        &mut self,
        value: crate::vulkan1_0::Extent2D,
    ) -> &mut Self {
        self.min_fragment_shading_rate_attachment_texel_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_shading_rate_attachment_texel_size`]
    pub fn set_max_fragment_shading_rate_attachment_texel_size(
        &mut self,
        value: crate::vulkan1_0::Extent2D,
    ) -> &mut Self {
        self.max_fragment_shading_rate_attachment_texel_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_shading_rate_attachment_texel_size_aspect_ratio`]
    pub fn set_max_fragment_shading_rate_attachment_texel_size_aspect_ratio(&mut self, value: u32) -> &mut Self {
        self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_fragment_shading_rate_with_multiple_viewports`]
    pub fn set_primitive_fragment_shading_rate_with_multiple_viewports(&mut self, value: bool) -> &mut Self {
        self.primitive_fragment_shading_rate_with_multiple_viewports = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::layered_shading_rate_attachments`]
    pub fn set_layered_shading_rate_attachments(&mut self, value: bool) -> &mut Self {
        self.layered_shading_rate_attachments = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_non_trivial_combiner_ops`]
    pub fn set_fragment_shading_rate_non_trivial_combiner_ops(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_non_trivial_combiner_ops = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_size`]
    pub fn set_max_fragment_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_fragment_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_size_aspect_ratio`]
    pub fn set_max_fragment_size_aspect_ratio(&mut self, value: u32) -> &mut Self {
        self.max_fragment_size_aspect_ratio = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_shading_rate_coverage_samples`]
    pub fn set_max_fragment_shading_rate_coverage_samples(&mut self, value: u32) -> &mut Self {
        self.max_fragment_shading_rate_coverage_samples = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_shading_rate_rasterization_samples`]
    pub fn set_max_fragment_shading_rate_rasterization_samples(
        &mut self,
        value: crate::vulkan1_0::SampleCountFlagBits,
    ) -> &mut Self {
        self.max_fragment_shading_rate_rasterization_samples = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_shader_depth_stencil_writes`]
    pub fn set_fragment_shading_rate_with_shader_depth_stencil_writes(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_shader_depth_stencil_writes = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_sample_mask`]
    pub fn set_fragment_shading_rate_with_sample_mask(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_sample_mask = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_shader_sample_mask`]
    pub fn set_fragment_shading_rate_with_shader_sample_mask(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_shader_sample_mask = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_conservative_rasterization`]
    pub fn set_fragment_shading_rate_with_conservative_rasterization(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_conservative_rasterization = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_fragment_shader_interlock`]
    pub fn set_fragment_shading_rate_with_fragment_shader_interlock(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_fragment_shader_interlock = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_with_custom_sample_locations`]
    pub fn set_fragment_shading_rate_with_custom_sample_locations(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_with_custom_sample_locations = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_strict_multiply_combiner`]
    pub fn set_fragment_shading_rate_strict_multiply_combiner(&mut self, value: bool) -> &mut Self {
        self.fragment_shading_rate_strict_multiply_combiner = value as u8 as u32;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
/// - [`p_next`] **must**  be `NULL`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`sample_counts`] is a bitmask of sample counts for which the shading
    ///rate described by [`fragment_size`] is supported.
    sample_counts: SampleCountFlags,
    ///[`fragment_size`] is a [`Extent2D`] describing the width and height
    ///of a supported shading rate.
    fragment_size: Extent2D,
}
impl<'lt> Default for PhysicalDeviceFragmentShadingRateKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            sample_counts: Default::default(),
            fragment_size: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShadingRateKHR<'lt> {
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
    ///Gets the value of [`Self::sample_counts`]
    pub fn sample_counts(&self) -> SampleCountFlags {
        self.sample_counts
    }
    ///Gets the value of [`Self::fragment_size`]
    pub fn fragment_size(&self) -> Extent2D {
        self.fragment_size
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
    ///Gets a mutable reference to the value of [`Self::sample_counts`]
    pub fn sample_counts_mut(&mut self) -> &mut SampleCountFlags {
        &mut self.sample_counts
    }
    ///Gets a mutable reference to the value of [`Self::fragment_size`]
    pub fn fragment_size_mut(&mut self) -> &mut Extent2D {
        &mut self.fragment_size
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
    ///Sets the raw value of [`Self::sample_counts`]
    pub fn set_sample_counts(&mut self, value: crate::vulkan1_0::SampleCountFlags) -> &mut Self {
        self.sample_counts = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_size`]
    pub fn set_fragment_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.fragment_size = value;
        self
    }
}
