//![VK_KHR_dynamic_rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html) - device extension
//!# Description
//!This extension allows applications to create single-pass render pass
//!instances without needing to create render pass objects or framebuffers.
//!Dynamic render passes can also span across multiple primary command buffers,
//!rather than relying on secondary command buffers.This extension also incorporates
//! `VK_ATTACHMENT_STORE_OP_NONE_KHR` from
//![`qcom_render_pass_store_ops`], enabling
//!applications to avoid unnecessary synchronization when an attachment is not
//!written during a render pass.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_dynamic_rendering]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_KHR_dynamic_rendering
//!   extension>>)
//!# New commands
//! - [`cmd_begin_rendering_khr`]
//! - [`cmd_end_rendering_khr`]
//!# New structures
//! - [`RenderingAttachmentInfoKHR`]
//! - [`RenderingInfoKHR`]
//! - Extending [`CommandBufferInheritanceInfo`]:  - [`CommandBufferInheritanceRenderingInfoKHR`]
//! - Extending [`GraphicsPipelineCreateInfo`]:  - [`PipelineRenderingCreateInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDynamicRenderingFeaturesKHR`]
//!If [`amd_mixed_attachment_samples`] is supported:
//! - Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`]:  -
//!   [`AttachmentSampleCountInfoAMD`]
//!If [`ext_fragment_density_map`] is supported:
//! - Extending [`RenderingInfo`]:  - [`RenderingFragmentDensityMapAttachmentInfoEXT`]
//!If [`khr_fragment_shading_rate`] is supported:
//! - Extending [`RenderingInfo`]:  - [`RenderingFragmentShadingRateAttachmentInfoKHR`]
//!If [`nv_framebuffer_mixed_samples`] is supported:
//! - Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`]:  -
//!   [`AttachmentSampleCountInfoNV`]
//!If [`nvx_multiview_per_view_attributes`] is supported:
//! - Extending [`CommandBufferInheritanceInfo`], [`GraphicsPipelineCreateInfo`], [`RenderingInfo`]:
//!   - [`MultiviewPerViewAttributesInfoNVX`]
//!# New enums
//! - [`RenderingFlagBitsKHR`]
//!# New bitmasks
//! - [`RenderingFlagsKHR`]
//!# New constants
//! - [`KHR_DYNAMIC_RENDERING_EXTENSION_NAME`]
//! - [`KHR_DYNAMIC_RENDERING_SPEC_VERSION`]
//! - Extending [`AttachmentStoreOp`]:  - `VK_ATTACHMENT_STORE_OP_NONE_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR`  - `VK_STRUCTURE_TYPE_RENDERING_INFO_KHR`
//!If [`amd_mixed_attachment_samples`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`
//!If [`ext_fragment_density_map`] is supported:
//! - Extending [`PipelineCreateFlagBits`]:  -
//!   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`  -
//!   `VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT`
//!If [`khr_fragment_shading_rate`] is supported:
//! - Extending [`PipelineCreateFlagBits`]:  -
//!   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`  -
//!   `VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
//!If [`nv_framebuffer_mixed_samples`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_NV`
//!If [`nvx_multiview_per_view_attributes`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`
//!# Version history
//! - Revision 1, 2021-10-06 (Tobias Hector)  - Initial revision
//!# Other information
//! * 2021-10-06
//! * - Promoted to Vulkan 1.3 Core
//! * - Tobias Hector, AMD  - Arseny Kapoulkine, Roblox  - François Duranleau, Gameloft  - Stuart
//!   Smith, AMD  - Hai Nguyen, Google  - Jean-François Roy, Google  - Jeff Leger, Qualcomm  -
//!   Jan-Harald Fredriksen, Arm  - Piers Daniell, Nvidia  - James Fitzpatrick, Imagination  - Piotr
//!   Byszewski, Mobica  - Jesse Hall, Google  - Mike Blumenkrantz, Valve
//!# Related
//! - [`CommandBufferInheritanceRenderingInfoKHR`]
//! - [`PhysicalDeviceDynamicRenderingFeaturesKHR`]
//! - [`PipelineRenderingCreateInfoKHR`]
//! - [`RenderingAttachmentInfoKHR`]
//! - [`RenderingFlagBitsKHR`]
//! - [`RenderingFlagsKHR`]
//! - [`RenderingInfoKHR`]
//! - [`cmd_begin_rendering_khr`]
//! - [`cmd_end_rendering_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{
        BaseInStructure, Bool32, Device, Extent2D, ImageLayout, ImageView, SampleCountFlagBits, StructureType,
    },
    vulkan1_3::{FNCmdBeginRendering, FNCmdEndRendering},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_dynamic_rendering");
///[VkRenderingFragmentShadingRateAttachmentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html) - Structure specifying fragment shading rate attachment information
///# C Specifications
///The [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_dynamic_rendering with VK_KHR_fragment_shading_rate
///typedef struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImageView        imageView;
///    VkImageLayout      imageLayout;
///    VkExtent2D         shadingRateAttachmentTexelSize;
///} VkRenderingFragmentShadingRateAttachmentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view`] is the image view that will be used as a fragment shading rate attachment.
/// - [`image_layout`] is the layout that [`image_view`] will be in during rendering.
/// - [`shading_rate_attachment_texel_size`] specifies the number of pixels corresponding to each
///   texel in [`image_view`].
///# Description
///This structure can be included in the [`p_next`] chain of
///[`RenderingInfo`] to define a
///[fragment shading rate
///attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment).
///If [`image_view`] is [`crate::Handle::null`], or if this structure is not
///specified, the implementation behaves as if a valid shading rate attachment
///was specified with all texels specifying a single pixel per fragment.
///## Valid Usage
/// - If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  be
///   `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with
///   `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width`
///   **must**  be a power of two value
/// -    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.width` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.width`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// - If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height`
///   **must**  be a power of two value
/// -    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSize)
/// -    If [`image_view`] is not [`crate::Handle::null`], `shadingRateAttachmentTexelSize.height` **must**  be greater than or equal to [`minFragmentShadingRateAttachmentTexelSize.height`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minFragmentShadingRateAttachmentTexelSize)
/// -    If [`image_view`] is not [`crate::Handle::null`], the quotient of `shadingRateAttachmentTexelSize.width` and `shadingRateAttachmentTexelSize.height` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
/// -    If [`image_view`] is not [`crate::Handle::null`], the quotient of `shadingRateAttachmentTexelSize.height` and `shadingRateAttachmentTexelSize.width` **must**  be less than or equal to [`maxFragmentShadingRateAttachmentTexelSizeAspectRatio`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFragmentShadingRateAttachmentTexelSizeAspectRatio)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
/// - If [`image_view`] is not [`crate::Handle::null`], [`image_view`] **must**  be a valid
///   [`ImageView`] handle
/// - [`image_layout`] **must**  be a valid [`ImageLayout`] value
///# Related
/// - [`khr_dynamic_rendering`]
/// - [`khr_fragment_shading_rate`]
/// - [`Extent2D`]
/// - [`ImageLayout`]
/// - [`ImageView`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image_view`] is the image view that will be used as a fragment
    ///shading rate attachment.
    pub image_view: ImageView,
    ///[`image_layout`] is the layout that [`image_view`] will be in during
    ///rendering.
    pub image_layout: ImageLayout,
    ///[`shading_rate_attachment_texel_size`] specifies the number of pixels
    ///corresponding to each texel in [`image_view`].
    pub shading_rate_attachment_texel_size: Extent2D,
}
impl<'lt> Default for RenderingFragmentShadingRateAttachmentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            shading_rate_attachment_texel_size: Default::default(),
        }
    }
}
impl<'lt> RenderingFragmentShadingRateAttachmentInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> RenderingFragmentShadingRateAttachmentInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::image_view`]
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }
    ///Gets the value of [`Self::image_layout`]
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Gets the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.shading_rate_attachment_texel_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_view`]
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Gets a mutable reference to the value of [`Self::image_layout`]
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.shading_rate_attachment_texel_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_view`]
    pub fn set_image_view(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view = value;
        self
    }
    ///Sets the value of [`Self::image_layout`]
    pub fn set_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.image_layout = value;
        self
    }
    ///Sets the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn set_shading_rate_attachment_texel_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.shading_rate_attachment_texel_size = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_view`]
    pub fn with_image_view(mut self, value: crate::vulkan1_0::ImageView) -> Self {
        self.image_view = value;
        self
    }
    ///Sets the value of [`Self::image_layout`]
    pub fn with_image_layout(mut self, value: crate::vulkan1_0::ImageLayout) -> Self {
        self.image_layout = value;
        self
    }
    ///Sets the value of [`Self::shading_rate_attachment_texel_size`]
    pub fn with_shading_rate_attachment_texel_size(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.shading_rate_attachment_texel_size = value;
        self
    }
}
///[VkRenderingFragmentDensityMapAttachmentInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html) - Structure specifying fragment shading rate attachment information
///# C Specifications
///The [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_KHR_dynamic_rendering with VK_EXT_fragment_density_map
///typedef struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImageView        imageView;
///    VkImageLayout      imageLayout;
///} VkRenderingFragmentDensityMapAttachmentInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view`] is the image view that will be used as a fragment shading rate attachment.
/// - [`image_layout`] is the layout that [`image_view`] will be in during rendering.
///# Description
///This structure can be included in the [`p_next`] chain of
///[`RenderingInfo`] to define a fragment density map.
///If this structure is not included in the [`p_next`] chain, [`image_view`]
///is treated as [`crate::Handle::null`].
///## Valid Usage
/// - If [`image_view`] is not [`crate::Handle::null`], `layout` **must**  be
///   `VK_IMAGE_LAYOUT_GENERAL` or `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  have been created with
///   `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
/// - If [`image_view`] is not [`crate::Handle::null`], it  **must**  not have been created with
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT`
/// - [`image_view`] **must**  be a valid [`ImageView`] handle
/// - [`image_layout`] **must**  be a valid [`ImageLayout`] value
///# Related
/// - [`ext_fragment_density_map`]
/// - [`khr_dynamic_rendering`]
/// - [`ImageLayout`]
/// - [`ImageView`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image_view`] is the image view that will be used as a fragment
    ///shading rate attachment.
    pub image_view: ImageView,
    ///[`image_layout`] is the layout that [`image_view`] will be in during
    ///rendering.
    pub image_layout: ImageLayout,
}
impl<'lt> Default for RenderingFragmentDensityMapAttachmentInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT,
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
        }
    }
}
impl<'lt> RenderingFragmentDensityMapAttachmentInfoEXT<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> RenderingFragmentDensityMapAttachmentInfoEXT<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::image_view`]
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }
    ///Gets the value of [`Self::image_layout`]
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_view`]
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Gets a mutable reference to the value of [`Self::image_layout`]
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_view`]
    pub fn set_image_view(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view = value;
        self
    }
    ///Sets the value of [`Self::image_layout`]
    pub fn set_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.image_layout = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::image_view`]
    pub fn with_image_view(mut self, value: crate::vulkan1_0::ImageView) -> Self {
        self.image_view = value;
        self
    }
    ///Sets the value of [`Self::image_layout`]
    pub fn with_image_layout(mut self, value: crate::vulkan1_0::ImageLayout) -> Self {
        self.image_layout = value;
        self
    }
}
///[VkAttachmentSampleCountInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html) - Structure specifying command buffer inheritance info for dynamic render pass instances
///# C Specifications
///The
///[`AttachmentSampleCountInfoAMD`]
///or
///[`AttachmentSampleCountInfoNV`]
///structure is defined as:
///```c
///// Provided by VK_KHR_dynamic_rendering with VK_AMD_mixed_attachment_samples
///typedef struct VkAttachmentSampleCountInfoAMD {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    uint32_t                        colorAttachmentCount;
///    const VkSampleCountFlagBits*    pColorAttachmentSamples;
///    VkSampleCountFlagBits           depthStencilAttachmentSamples;
///} VkAttachmentSampleCountInfoAMD;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering with VK_NV_framebuffer_mixed_samples
///typedef VkAttachmentSampleCountInfoAMD VkAttachmentSampleCountInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`color_attachment_count`] is the number of color attachments specified in a render pass
///   instance.
/// - [`color_attachment_samples`] is a pointer to an array of [`SampleCountFlagBits`] values
///   defining the sample count of color attachments.
/// - [`depth_stencil_attachment_samples`] is a [`SampleCountFlagBits`] value defining the sample
///   count of a depth/stencil attachment.
///# Description
///If [`CommandBufferInheritanceInfo::render_pass`] is
///[`crate::Handle::null`], `VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT`
///is specified in [`CommandBufferBeginInfo::flags`], and the
///[`p_next`] chain of [`CommandBufferInheritanceInfo`] includes
///[`AttachmentSampleCountInfoAMD`], then this structure defines the sample
///counts of each attachment within the render pass instance.
///If [`AttachmentSampleCountInfoAMD`] is not included, the value of
///[`CommandBufferInheritanceRenderingInfo::rasterization_samples`] is
///used as the sample count for each attachment.
///If [`CommandBufferInheritanceInfo::render_pass`] is not
///[`crate::Handle::null`], or
///`VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` is not specified in
///[`CommandBufferBeginInfo::flags`], parameters of this structure
///are ignored.[`AttachmentSampleCountInfoAMD`] **can**  also be included in the
///[`p_next`] chain of [`GraphicsPipelineCreateInfo`].
///When a graphics pipeline is created without a [`RenderPass`], if this
///structure is present in the [`p_next`] chain of
///[`GraphicsPipelineCreateInfo`], it specifies the sample count of
///attachments used for rendering.
///If this structure is not specified, and the pipeline does not include a
///[`RenderPass`], the value of
///[`PipelineMultisampleStateCreateInfo::rasterization_samples`] is
///used as the sample count for each attachment.
///If a graphics pipeline is created with a valid [`RenderPass`],
///parameters of this structure are ignored.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD`
/// - If [`color_attachment_count`] is not `0`, [`color_attachment_samples`] **must**  be a valid
///   pointer to an array of [`color_attachment_count`] valid [`SampleCountFlagBits`] values
/// - If [`depth_stencil_attachment_samples`] is not `0`, [`depth_stencil_attachment_samples`]
///   **must**  be a valid [`SampleCountFlagBits`] value
///# Related
/// - [`amd_mixed_attachment_samples`]
/// - [`khr_dynamic_rendering`]
/// - [`nv_framebuffer_mixed_samples`]
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
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    pub p_next: *const BaseInStructure<'lt>,
    ///[`color_attachment_count`] is the number of color attachments specified
    ///in a render pass instance.
    pub color_attachment_count: u32,
    ///[`color_attachment_samples`] is a pointer to an array of
    ///[`SampleCountFlagBits`] values defining the sample count of color
    ///attachments.
    pub color_attachment_samples: *const SampleCountFlagBits,
    ///[`depth_stencil_attachment_samples`] is a [`SampleCountFlagBits`]
    ///value defining the sample count of a depth/stencil attachment.
    pub depth_stencil_attachment_samples: SampleCountFlagBits,
}
impl<'lt> Default for AttachmentSampleCountInfoAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
            p_next: std::ptr::null(),
            color_attachment_count: 0,
            color_attachment_samples: std::ptr::null(),
            depth_stencil_attachment_samples: Default::default(),
        }
    }
}
impl<'lt> AttachmentSampleCountInfoAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::color_attachment_samples`]
    pub fn color_attachment_samples_raw(&self) -> *const SampleCountFlagBits {
        self.color_attachment_samples
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_samples`]
    pub fn set_color_attachment_samples_raw(&mut self, value: *const SampleCountFlagBits) -> &mut Self {
        self.color_attachment_samples = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_samples`]
    pub fn with_color_attachment_samples_raw(mut self, value: *const SampleCountFlagBits) -> Self {
        self.color_attachment_samples = value;
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
    ///Gets the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the value of [`Self::color_attachment_samples`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_attachment_samples(&self) -> &[SampleCountFlagBits] {
        std::slice::from_raw_parts(self.color_attachment_samples, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::depth_stencil_attachment_samples`]
    pub fn depth_stencil_attachment_samples(&self) -> SampleCountFlagBits {
        self.depth_stencil_attachment_samples
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.color_attachment_count
    }
    ///Gets a mutable reference to the value of [`Self::depth_stencil_attachment_samples`]
    pub fn depth_stencil_attachment_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.depth_stencil_attachment_samples
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the value of [`Self::color_attachment_samples`]
    pub fn set_color_attachment_samples(&mut self, value: &'lt [crate::vulkan1_0::SampleCountFlagBits]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachment_samples = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the value of [`Self::depth_stencil_attachment_samples`]
    pub fn set_depth_stencil_attachment_samples(&mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> &mut Self {
        self.depth_stencil_attachment_samples = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::color_attachment_count`]
    pub fn with_color_attachment_count(mut self, value: u32) -> Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the value of [`Self::color_attachment_samples`]
    pub fn with_color_attachment_samples(mut self, value: &'lt [crate::vulkan1_0::SampleCountFlagBits]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachment_samples = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the value of [`Self::depth_stencil_attachment_samples`]
    pub fn with_depth_stencil_attachment_samples(mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> Self {
        self.depth_stencil_attachment_samples = value;
        self
    }
}
///[VkMultiviewPerViewAttributesInfoNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html) - Structure specifying the multiview per-attribute properties
///# C Specifications
///The [`MultiviewPerViewAttributesInfoNVX`] structure is defined as:
///```c
///// Provided by VK_KHR_dynamic_rendering with VK_NVX_multiview_per_view_attributes
///typedef struct VkMultiviewPerViewAttributesInfoNVX {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           perViewAttributes;
///    VkBool32           perViewAttributesPositionXOnly;
///} VkMultiviewPerViewAttributesInfoNVX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`per_view_attributes`] specifies that shaders compiled for this pipeline write the attributes
///   for all views in a single invocation of each vertex processing stage. All pipelines executed
///   within a render pass instance that includes this bit  **must**  write per-view attributes to
///   the `*PerViewNV[]` shader outputs, in addition to the non-per-view (e.g. `Position`) outputs.
/// - [`per_view_attributes_position_x_only`] specifies that shaders compiled for this pipeline use
///   per-view positions which only differ in value in the x component. Per-view viewport mask
///   **can**  also be used.
///# Description
///When dynamic render pass instances are being used, instead of specifying
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` or
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX` in the subpass
///description flags, the per-attibute properties of the render pass instance
/// **must**  be specified by the [`MultiviewPerViewAttributesInfoNVX`]
///structure Include the [`MultiviewPerViewAttributesInfoNVX`] structure in
///the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] when creating a
///graphics pipeline for dynamic rendering, [`RenderingInfo`] when starting
///a dynamic render pass instance, and [`CommandBufferInheritanceInfo`]
///when specifying the dynamic render pass instance parameters for secondary
///command buffers.
///## Valid Usage
/// - If [`per_view_attributes_position_x_only`] is [`TRUE`] then [`per_view_attributes`] **must**
///   also be [`TRUE`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX`
///# Related
/// - [`khr_dynamic_rendering`]
/// - [`nvx_multiview_per_view_attributes`]
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
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`per_view_attributes`] specifies that shaders compiled for this
    ///pipeline write the attributes for all views in a single invocation of
    ///each vertex processing stage.
    ///All pipelines executed within a render pass instance that includes this
    ///bit  **must**  write per-view attributes to the `*PerViewNV[]` shader
    ///outputs, in addition to the non-per-view (e.g. `Position`) outputs.
    pub per_view_attributes: Bool32,
    ///[`per_view_attributes_position_x_only`] specifies that shaders compiled for
    ///this pipeline use per-view positions which only differ in value in the x
    ///component.
    ///Per-view viewport mask  **can**  also be used.
    pub per_view_attributes_position_x_only: Bool32,
}
impl<'lt> Default for MultiviewPerViewAttributesInfoNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
            p_next: std::ptr::null(),
            per_view_attributes: 0,
            per_view_attributes_position_x_only: 0,
        }
    }
}
impl<'lt> MultiviewPerViewAttributesInfoNVX<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> MultiviewPerViewAttributesInfoNVX<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::per_view_attributes`]
    pub fn per_view_attributes_raw(&self) -> Bool32 {
        self.per_view_attributes
    }
    ///Gets the raw value of [`Self::per_view_attributes_position_x_only`]
    pub fn per_view_attributes_position_x_only_raw(&self) -> Bool32 {
        self.per_view_attributes_position_x_only
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_attributes`]
    pub fn set_per_view_attributes_raw(&mut self, value: Bool32) -> &mut Self {
        self.per_view_attributes = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_attributes_position_x_only`]
    pub fn set_per_view_attributes_position_x_only_raw(&mut self, value: Bool32) -> &mut Self {
        self.per_view_attributes_position_x_only = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_attributes`]
    pub fn with_per_view_attributes_raw(mut self, value: Bool32) -> Self {
        self.per_view_attributes = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_attributes_position_x_only`]
    pub fn with_per_view_attributes_position_x_only_raw(mut self, value: Bool32) -> Self {
        self.per_view_attributes_position_x_only = value;
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
    ///Gets the value of [`Self::per_view_attributes`]
    pub fn per_view_attributes(&self) -> bool {
        unsafe { std::mem::transmute(self.per_view_attributes as u8) }
    }
    ///Gets the value of [`Self::per_view_attributes_position_x_only`]
    pub fn per_view_attributes_position_x_only(&self) -> bool {
        unsafe { std::mem::transmute(self.per_view_attributes_position_x_only as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::per_view_attributes`]
    pub fn per_view_attributes_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.per_view_attributes as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.per_view_attributes as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::per_view_attributes_position_x_only`]
    pub fn per_view_attributes_position_x_only_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.per_view_attributes_position_x_only as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.per_view_attributes_position_x_only as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::per_view_attributes`]
    pub fn set_per_view_attributes(&mut self, value: bool) -> &mut Self {
        self.per_view_attributes = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::per_view_attributes_position_x_only`]
    pub fn set_per_view_attributes_position_x_only(&mut self, value: bool) -> &mut Self {
        self.per_view_attributes_position_x_only = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::per_view_attributes`]
    pub fn with_per_view_attributes(mut self, value: bool) -> Self {
        self.per_view_attributes = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::per_view_attributes_position_x_only`]
    pub fn with_per_view_attributes_position_x_only(mut self, value: bool) -> Self {
        self.per_view_attributes_position_x_only = value as u8 as u32;
        self
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_dynamic_rendering`
pub struct DeviceKhrDynamicRenderingVTable {
    ///See [`FNCmdBeginRendering`] for more information.
    pub cmd_begin_rendering_khr: FNCmdBeginRendering,
    ///See [`FNCmdEndRendering`] for more information.
    pub cmd_end_rendering_khr: FNCmdEndRendering,
}
impl DeviceKhrDynamicRenderingVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_begin_rendering_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBeginRenderingKHR").as_ptr()))
            },
            cmd_end_rendering_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndRenderingKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_begin_rendering_khr`]. See [`FNCmdBeginRendering`] for more information.
    pub fn cmd_begin_rendering_khr(&self) -> FNCmdBeginRendering {
        self.cmd_begin_rendering_khr
    }
    ///Gets [`Self::cmd_end_rendering_khr`]. See [`FNCmdEndRendering`] for more information.
    pub fn cmd_end_rendering_khr(&self) -> FNCmdEndRendering {
        self.cmd_end_rendering_khr
    }
}
