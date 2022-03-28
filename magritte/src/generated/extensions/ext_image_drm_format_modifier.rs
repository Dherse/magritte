//![VK_EXT_image_drm_format_modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_drm_format_modifier.html) - device extension
//!# Description
//!This extension provides the ability to use *DRM format modifiers* with
//!images, enabling Vulkan to better integrate with the Linux ecosystem of
//!graphics, video, and display APIs.Its functionality closely overlaps with
//!`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^
//!and
//!`EGL_MESA_image_dma_buf_export`<sup>[3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn3)</sup>^.
//!Unlike the EGL extensions, this extension does not require the use of a
//!specific handle type (such as a dma_buf) for external memory and provides
//!more explicit control of image creation.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_bind_memory2`]`
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_image_format_list`]`
//! - Requires `[`VK_KHR_sampler_ycbcr_conversion`]`
//!# Contacts
//! - Chad Versace [chadversary](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_image_drm_format_modifier]
//!   @chadversary%0A<<Here describe the issue or question you have about the
//!   VK_EXT_image_drm_format_modifier extension>>)
//!# New functions & commands
//! - [`GetImageDrmFormatModifierPropertiesEXT`]
//!# New structures
//! - [`DrmFormatModifierPropertiesEXT`]
//! - [`ImageDrmFormatModifierPropertiesEXT`]
//! - Extending [`FormatProperties2`]:  - [`DrmFormatModifierPropertiesListEXT`]
//! - Extending [`ImageCreateInfo`]:  - [`ImageDrmFormatModifierExplicitCreateInfoEXT`]  -
//!   [`ImageDrmFormatModifierListCreateInfoEXT`]
//! - Extending [`PhysicalDeviceImageFormatInfo2`]:  -
//!   [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - [`DrmFormatModifierProperties2EXT`]
//! - Extending [`FormatProperties2`]:  - [`DrmFormatModifierPropertiesList2EXT`]
//!# New constants
//! - [`EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME`]
//! - [`EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION`]
//! - Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT`  -
//!   `VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT`  - `VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT`  -
//!   `VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT`
//! - Extending [`ImageTiling`]:  - `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`
//!# Known issues & F.A.Q
//!1) Should this extension define a single DRM format modifier per
//![`Image`]? Or define one per plane?+ **RESOLVED** : There exists a single DRM format modifier
//! per [`Image`]. **DISCUSSION** : Prior art, such as
//!`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^,
//!`struct drm_mode_fb_cmd2`<sup>[4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn4)</sup>^, and
//!`struct
//!gbm_import_fd_modifier_data`<sup>[5](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn5)</sup>^,
//!allows defining one *modifier* per plane.
//!However, developers of the GBM and kernel APIs concede it was a mistake.
//!Beginning in Linux 4.10, the kernel requires that the application provide
//!the same DRM format *modifier* for each plane.
//!(See Linux commit
//![bae781b259269590109e8a4a8227331362b88212](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=bae781b259269590109e8a4a8227331362b88212)).
//!And GBM provides an entry point, `gbm_bo_get_modifier`, for querying the
//!*modifier* of the image but does not provide one to query the modifier of
//!individual planes.2) When creating an image with
//![`ImageDrmFormatModifierExplicitCreateInfoEXT`], which is typically used
//!when *importing* an image, should the application explicitly provide the
//!size of each plane?+ **RESOLVED** : No.
//!The application  **must**  not provide the size.
//!To enforce this, the API requires that
//![`ImageDrmFormatModifierExplicitCreateInfoEXT`]::`pPlaneLayouts->size` **must**  be 0. **DISCUSSION** : Prior art, such as
//!`EGL_EXT_image_dma_buf_import_modifiers`<sup>[2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn2)</sup>^,
//!`struct drm_mode_fb_cmd2`<sup>[4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn4)</sup>^, and
//!`struct
//!gbm_import_fd_modifier_data`<sup>[5](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_EXT_image_drm_format_modifier-fn5)</sup>^,
//!omits from the API the size of each plane.
//!Instead, the APIs infer each plane’s size from the import parameters, which
//!include the image’s pixel format and a dma_buf, offset, and row pitch for
//!each plane.However, Vulkan differs from EGL and GBM with regards to image creation in
//!the following ways:
//! - **Undedicated allocation by default.**  When importing or exporting a set of dma_bufs as an
//!   `EGLImage` or `gbm_bo`, common practice mandates that each dma_buf’s memory be dedicated (in
//!   the sense of [`VK_KHR_dedicated_allocation`]) to the image (though not necessarily dedicated
//!   to a single plane). In particular, neither the GBM documentation nor the EGL extension
//!   specifications explicitly state this requirement, but in light of common practice this is
//!   likely due to under-specification rather than intentional omission. In contrast,
//!   [`VK_EXT_image_drm_format_modifier`] permits, but does not require, the implementation to
//!   require dedicated allocations for images created with
//!   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`.
//! - **Separation of image creation and memory allocation.**  When importing a set of dma_bufs as
//!   an `EGLImage` or `gbm_bo`, EGL and GBM create the image resource and bind it to memory (the
//!   dma_bufs) simultaneously. This allows EGL and GBM to query each dma_buf’s size during image
//!   creation. In Vulkan, image creation and memory allocation are independent unless a dedicated
//!   allocation is used (as in [`VK_KHR_dedicated_allocation`]). Therefore, without requiring
//!   dedicated allocation, Vulkan cannot query the size of each dma_buf (or other external handle)
//!   when calculating the image’s memory layout. Even if dedication allocation were required,
//!   Vulkan cannot calculate the image’s memory layout until after the image is bound to its
//!   dma_ufs.
//!The above differences complicate the potential inference of plane size in
//!Vulkan.
//!Consider the following problematic cases:
//! - **Padding.**  Some plane of the image may require implementation-dependent padding.
//! - **Metadata.**  For some *modifiers*, the image may have a metadata plane which requires a
//!   non-trivial calculation to determine its size.
//! - **Mipmapped, array, and 3D images.**  The implementation may support
//!   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` for images whose `mipLevels`, `arrayLayers`, or
//!   `depth` is greater than 1. For such images with certain *modifiers*, the calculation of each
//!   plane’s size may be non-trivial.
//!However, an application-provided plane size solves none of the above
//!problems.For simplicity, consider an external image with a single memory plane.
//!The implementation is obviously capable calculating the image’s size when
//!its tiling is `VK_IMAGE_TILING_OPTIMAL`.
//!Likewise, any reasonable implementation is capable of calculating the
//!image’s size when its tiling uses a supported *modifier*.Suppose that the external image’s size
//! is smaller than the
//!implementation-calculated size.
//!If the application provided the external image’s size to
//![`CreateImage`], the implementation would observe the mismatched size
//!and recognize its inability to comprehend the external image’s layout
//!(unless the implementation used the application-provided size to select a
//!refinement of the tiling layout indicated by the *modifier*, which is
//!strongly discouraged).
//!The implementation would observe the conflict, and reject image creation
//!with `VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
//!On the other hand, if the application did not provide the external image’s
//!size to [`CreateImage`], then the application would observe after
//!calling [`GetImageMemoryRequirements`] that the external image’s size is
//!less than the size required by the implementation.
//!The application would observe the conflict and refuse to bind the
//![`Image`] to the external memory.
//!In both cases, the result is explicit failure.Suppose that the external image’s size is larger
//! than the
//!implementation-calculated size.
//!If the application provided the external image’s size to
//![`CreateImage`], for reasons similar to above the implementation would
//!observe the mismatched size and recognize its inability to comprehend the
//!image data residing in the extra size.
//!The implementation, however, must assume that image data resides in the
//!entire size provided by the application.
//!The implementation would observe the conflict and reject image creation with
//!`VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
//!On the other hand, if the application did not provide the external image’s
//!size to [`CreateImage`], then the application would observe after
//!calling [`GetImageMemoryRequirements`] that the external image’s size is
//!larger than the implementation-usable size.
//!The application would observe the conflict and refuse to bind the
//![`Image`] to the external memory.
//!In both cases, the result is explicit failure.Therefore, an application-provided size provides
//! no benefit, and this
//!extension should not require it.
//!This decision renders [`SubresourceLayout::size`] an unused field
//!during image creation, and thus introduces a risk that implementations may
//!require applications to submit sideband creation parameters in the unused
//!field.
//!To prevent implementations from relying on sideband data, this extension
//!*requires* the application to set `size` to 0.
//!### []()References
//!
//!0. [`EGL_EXT_image_dma_buf_import`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import.txt)
//!1. [`EGL_EXT_image_dma_buf_import_modifiers`](https://www.khronos.org/registry/EGL/extensions/EXT/EGL_EXT_image_dma_buf_import_modifiers.txt)
//!2. [`EGL_MESA_image_dma_buf_export`](https://www.khronos.org/registry/EGL/extensions/MESA/EGL_MESA_image_dma_buf_export.txt)
//!3. [`struct drm_mode_fb_cmd2`](https://git.kernel.org/cgit/linux/kernel/git/torvalds/linux.git/tree/include/uapi/drm/drm_mode.h?id=refs/tags/v4.10#n392)
//!4. [`gbm.h`](https://cgit.freedesktop.org/mesa/mesa/tree/src/gbm/main/gbm.h?id=refs/tags/mesa-18.0.0-rc1)
//!
//!### []()Version History
//!
//! - Revision 1, 2018-08-29 (Chad Versace)  - First stable revision
//! - Revision 2, 2021-09-30 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]`
//!   to `vk.xml`
//!# Other info
//! * 2021-09-30
//! * No known IP claims.
//! * - Antoine Labour, Google  - Bas Nieuwenhuizen, Google  - Chad Versace, Google  - James Jones,
//!   NVIDIA  - Jason Ekstrand, Intel  - Jőrg Wagner, ARM  - Kristian Høgsberg Kristensen, Google  -
//!   Ray Smith, ARM
//!# Related
//! - [`DrmFormatModifierPropertiesEXT`]
//! - [`DrmFormatModifierPropertiesListEXT`]
//! - [`ImageDrmFormatModifierExplicitCreateInfoEXT`]
//! - [`ImageDrmFormatModifierListCreateInfoEXT`]
//! - [`ImageDrmFormatModifierPropertiesEXT`]
//! - [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
//! - [`GetImageDrmFormatModifierPropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, FormatFeatureFlags, SharingMode, StructureType, SubresourceLayout},
    vulkan1_3::FormatFeatureFlags2,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_image_drm_format_modifier");
///[VkDrmFormatModifierPropertiesListEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesListEXT.html) - Structure specifying the list of DRM format modifiers supported for a format
///# C Specifications
///To obtain the list of [Linux DRM format
///modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) compatible with a [`Format`], add a
///[`DrmFormatModifierPropertiesListEXT`] structure to the [`p_next`]
///chain of [`FormatProperties2`].The [`DrmFormatModifierPropertiesListEXT`] structure is defined
/// as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkDrmFormatModifierPropertiesListEXT {
///    VkStructureType                      sType;
///    void*                                pNext;
///    uint32_t                             drmFormatModifierCount;
///    VkDrmFormatModifierPropertiesEXT*    pDrmFormatModifierProperties;
///} VkDrmFormatModifierPropertiesListEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier_count`] is an inout parameter related to the number of modifiers
///   compatible with the `format`, as described below.
/// - [`drm_format_modifier_properties`] is either `NULL` or a pointer to an array of
///   [`DrmFormatModifierPropertiesEXT`] structures.
///# Description
///If [`drm_format_modifier_properties`] is `NULL`, then the function returns
///in [`drm_format_modifier_count`] the number of modifiers compatible with the
///queried `format`.
///Otherwise, the application  **must**  set [`drm_format_modifier_count`] to the
///length of the array [`drm_format_modifier_properties`]; the function will
///write at most [`drm_format_modifier_count`] elements to the array, and will
///return in [`drm_format_modifier_count`] the number of elements written.Among the elements in
/// array [`drm_format_modifier_properties`], each
///returned `drmFormatModifier` **must**  be unique.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`DrmFormatModifierPropertiesEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`drm_format_modifier_count`] is an inout parameter related to the number
    ///of modifiers compatible with the `format`, as described below.
    pub drm_format_modifier_count: u32,
    ///[`drm_format_modifier_properties`] is either `NULL` or a pointer to an
    ///array of [`DrmFormatModifierPropertiesEXT`] structures.
    pub drm_format_modifier_properties: *mut DrmFormatModifierPropertiesEXT,
}
impl<'lt> Default for DrmFormatModifierPropertiesListEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: 0,
            drm_format_modifier_properties: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DrmFormatModifierPropertiesListEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn drm_format_modifier_properties_raw(&self) -> &*mut DrmFormatModifierPropertiesEXT {
        &self.drm_format_modifier_properties
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn set_drm_format_modifier_properties_raw(&mut self, value: *mut DrmFormatModifierPropertiesEXT) -> &mut Self {
        self.drm_format_modifier_properties = value;
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
    ///Gets the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count(&self) -> u32 {
        self.drm_format_modifier_count
    }
    ///Gets the value of [`Self::drm_format_modifier_properties`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn drm_format_modifier_properties(&self) -> &[DrmFormatModifierPropertiesEXT] {
        std::slice::from_raw_parts(
            self.drm_format_modifier_properties,
            self.drm_format_modifier_count as usize,
        )
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
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_count
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_properties`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn drm_format_modifier_properties_mut(&mut self) -> &mut [DrmFormatModifierPropertiesEXT] {
        std::slice::from_raw_parts_mut(
            self.drm_format_modifier_properties,
            self.drm_format_modifier_count as usize,
        )
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
    ///Sets the raw value of [`Self::drm_format_modifier_count`]
    pub fn set_drm_format_modifier_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_count = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn set_drm_format_modifier_properties(
        &mut self,
        value: &'lt mut [crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.drm_format_modifier_properties = value.as_mut_ptr();
        self.drm_format_modifier_count = len_;
        self
    }
}
///[VkDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesEXT.html) - Structure specifying properties of a format when combined with a DRM format modifier
///# C Specifications
///The [`DrmFormatModifierPropertiesEXT`] structure describes properties of
///a [`Format`] when that format is combined with a
///[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
///These properties, like those of [`FormatProperties2`], are independent
///of any particular image.The [`DrmFormatModifierPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkDrmFormatModifierPropertiesEXT {
///    uint64_t                drmFormatModifier;
///    uint32_t                drmFormatModifierPlaneCount;
///    VkFormatFeatureFlags    drmFormatModifierTilingFeatures;
///} VkDrmFormatModifierPropertiesEXT;
///```
///# Members
/// - [`drm_format_modifier`] is a *Linux DRM format modifier*.
/// - [`drm_format_modifier_plane_count`] is the number of *memory planes* in any image created with
///   `format` and [`drm_format_modifier`]. An image’s *memory planecount* is distinct from its
///   *format planecount*, as explained below.
/// - [`drm_format_modifier_tiling_features`] is a bitmask of [`FormatFeatureFlagBits`] that are
///   supported by any image created with `format` and [`drm_format_modifier`].
///# Description
///The returned [`drm_format_modifier_tiling_features`] **must**  contain at least
///one bit.The implementation  **must**  not return `DRM_FORMAT_MOD_INVALID` in
///[`drm_format_modifier`].An image’s *memory planecount* (as returned by
///[`drm_format_modifier_plane_count`]) is distinct from its *format planecount*
///(in the sense of [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///Y′C<sub>B</sub>C<sub>R</sub> formats).
///In [`ImageAspectFlags`], each
///`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` represents a *memory plane*
///and each `VK_IMAGE_ASPECT_PLANE*_i_*BIT` a *format plane*.An image’s set of *format planes* is
/// an ordered partition of the image’s
/// **content**  into separable groups of format components.
///The ordered partition is encoded in the name of each [`Format`].
///For example, `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM` contains two *format
///planes*; the first plane contains the green component and the second plane
///contains the blue component and red component.
///If the format name does not contain `PLANE`, then the format contains a
///single plane; for example, `VK_FORMAT_R8G8B8A8_UNORM`.
///Some commands, such as [`CmdCopyBufferToImage`], do not operate on all
///format components in the image, but instead operate only on the *format
///planes* explicitly chosen by the application and operate on each *format
///plane* independently.An image’s set of *memory planes* is an ordered partition of the image’s
/// **memory**  rather than the image’s  **content** .
///Each *memory plane* is a contiguous range of memory.
///The union of an image’s *memory planes* is not necessarily contiguous.If an image is [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition is
///the same for *memory planes* and for *format planes*.
///Therefore, if the returned [`drm_format_modifier`] is
///`DRM_FORMAT_MOD_LINEAR`, then [`drm_format_modifier_plane_count`] **must**
///equal the *format planecount*, and [`drm_format_modifier_tiling_features`] **must**  be
/// identical to the
///[`FormatProperties2`]`::linearTilingFeatures` returned in the same
///`pNext` chain.If an image is [non-linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition
///of the image’s  **memory**  into *memory planes* is implementation-specific and
/// **may**  be unrelated to the partition of the image’s  **content**  into *format
///planes*.
///For example, consider an image whose `format` is
///`VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM`, `tiling` is
///`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, whose [`drm_format_modifier`]
///is not `DRM_FORMAT_MOD_LINEAR`, and `flags` lacks
///`VK_IMAGE_CREATE_DISJOINT_BIT`.
///The image has 3 *format planes*, and commands such
///[`CmdCopyBufferToImage`] act on each *format plane* independently as if
///the data of each *format plane* were separable from the data of the other
///planes.
///In a straightforward implementation, the implementation  **may**  store the
///image’s content in 3 adjacent *memory planes* where each *memory plane*
///corresponds exactly to a *format plane*.
///However, the implementation  **may**  also store the image’s content in a single
///*memory plane* where all format components are combined using an
///implementation-private block-compressed format; or the implementation  **may**
///store the image’s content in a collection of 7 adjacent *memory planes*
///using an implementation-private sharding technique.
///Because the image is non-linear and non-disjoint, the implementation has
///much freedom when choosing the image’s placement in memory.The *memory planecount* applies to
/// function parameters and structures only
///when the API specifies an explicit requirement on
///[`drm_format_modifier_plane_count`].
///In all other cases, the *memory planecount* is ignored.
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`DrmFormatModifierPropertiesListEXT`]
/// - [`FormatFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    ///[`drm_format_modifier`] is a *Linux DRM format modifier*.
    pub drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///any image created with `format` and [`drm_format_modifier`].
    ///An image’s *memory planecount* is distinct from its *format planecount*,
    ///as explained below.
    pub drm_format_modifier_plane_count: u32,
    ///[`drm_format_modifier_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits`] that are supported by any image created
    ///with `format` and [`drm_format_modifier`].
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}
impl Default for DrmFormatModifierPropertiesEXT {
    fn default() -> Self {
        Self {
            drm_format_modifier: 0,
            drm_format_modifier_plane_count: 0,
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}
impl DrmFormatModifierPropertiesEXT {
    ///Gets the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Gets the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count(&self) -> u32 {
        self.drm_format_modifier_plane_count
    }
    ///Gets the value of [`Self::drm_format_modifier_tiling_features`]
    pub fn drm_format_modifier_tiling_features(&self) -> FormatFeatureFlags {
        self.drm_format_modifier_tiling_features
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_plane_count
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_tiling_features`]
    pub fn drm_format_modifier_tiling_features_mut(&mut self) -> &mut FormatFeatureFlags {
        &mut self.drm_format_modifier_tiling_features
    }
    ///Sets the raw value of [`Self::drm_format_modifier`]
    pub fn set_drm_format_modifier(&mut self, value: u64) -> &mut Self {
        self.drm_format_modifier = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_plane_count`]
    pub fn set_drm_format_modifier_plane_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_plane_count = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_tiling_features`]
    pub fn set_drm_format_modifier_tiling_features(
        &mut self,
        value: crate::vulkan1_0::FormatFeatureFlags,
    ) -> &mut Self {
        self.drm_format_modifier_tiling_features = value;
        self
    }
}
///[VkPhysicalDeviceImageDrmFormatModifierInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageDrmFormatModifierInfoEXT.html) - Structure specifying a DRM format modifier as image creation parameter
///# C Specifications
///To query the image capabilities that are compatible with a
///[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier), set
///[`PhysicalDeviceImageFormatInfo2::tiling`] to
///`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and add a
///[`PhysicalDeviceImageDrmFormatModifierInfoEXT`] structure to the
///[`p_next`] chain of [`PhysicalDeviceImageFormatInfo2`].The
/// [`PhysicalDeviceImageDrmFormatModifierInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint64_t           drmFormatModifier;
///    VkSharingMode      sharingMode;
///    uint32_t           queueFamilyIndexCount;
///    const uint32_t*    pQueueFamilyIndices;
///} VkPhysicalDeviceImageDrmFormatModifierInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier`] is the image’s *Linux DRM format modifier*, corresponding to
///   [`ImageDrmFormatModifierExplicitCreateInfoEXT`]`::modifier` or to
///   [`ImageDrmFormatModifierListCreateInfoEXT`]`::pModifiers`.
/// - [`sharing_mode`] specifies how the image will be accessed by multiple queue families.
/// - [`queue_family_index_count`] is the number of entries in the [`queue_family_indices`] array.
/// - [`queue_family_indices`] is a pointer to an array of queue families that will access the
///   image. It is ignored if [`sharing_mode`] is not `VK_SHARING_MODE_CONCURRENT`.
///# Description
///If the [`drm_format_modifier`] is incompatible with the parameters specified
///in [`PhysicalDeviceImageFormatInfo2`] and its [`p_next`] chain, then
///[`GetPhysicalDeviceImageFormatProperties2`] returns
///`VK_ERROR_FORMAT_NOT_SUPPORTED`.
///The implementation  **must**  support the query of any [`drm_format_modifier`],
///including unknown and invalid modifier values.
///## Valid Usage
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`queue_family_indices`] **must**
///   be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`queue_family_index_count`]
///   **must**  be greater than `1`
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of [`queue_family_indices`]
///   **must**  be unique and  **must**  be less than the `pQueueFamilyPropertyCount` returned by
///   [`GetPhysicalDeviceQueueFamilyProperties2`] for the `physicalDevice` that was used to create
///   `device`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
/// - [`sharing_mode`] **must**  be a valid [`SharingMode`] value
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`SharingMode`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`drm_format_modifier`] is the image’s *Linux DRM format modifier*,
    ///corresponding to
    ///[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::`modifier` or
    ///to [`ImageDrmFormatModifierListCreateInfoEXT`]::`pModifiers`.
    pub drm_format_modifier: u64,
    ///[`sharing_mode`] specifies how the image will be accessed by multiple
    ///queue families.
    pub sharing_mode: SharingMode,
    ///[`queue_family_index_count`] is the number of entries in the
    ///[`queue_family_indices`] array.
    pub queue_family_index_count: u32,
    ///[`queue_family_indices`] is a pointer to an array of queue families
    ///that will access the image.
    ///It is ignored if [`sharing_mode`] is not
    ///`VK_SHARING_MODE_CONCURRENT`.
    pub queue_family_indices: *const u32,
}
impl<'lt> Default for PhysicalDeviceImageDrmFormatModifierInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            drm_format_modifier: 0,
            sharing_mode: Default::default(),
            queue_family_index_count: 0,
            queue_family_indices: std::ptr::null(),
        }
    }
}
impl<'lt> PhysicalDeviceImageDrmFormatModifierInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::queue_family_indices`]
    pub fn queue_family_indices_raw(&self) -> *const u32 {
        self.queue_family_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.queue_family_indices = value;
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
    ///Gets the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Gets the value of [`Self::sharing_mode`]
    pub fn sharing_mode(&self) -> SharingMode {
        self.sharing_mode
    }
    ///Gets the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count(&self) -> u32 {
        self.queue_family_index_count
    }
    ///Gets the value of [`Self::queue_family_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn queue_family_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.queue_family_indices, self.queue_family_index_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Gets a mutable reference to the value of [`Self::sharing_mode`]
    pub fn sharing_mode_mut(&mut self) -> &mut SharingMode {
        &mut self.sharing_mode
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index_count
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
    ///Sets the raw value of [`Self::drm_format_modifier`]
    pub fn set_drm_format_modifier(&mut self, value: u64) -> &mut Self {
        self.drm_format_modifier = value;
        self
    }
    ///Sets the raw value of [`Self::sharing_mode`]
    pub fn set_sharing_mode(&mut self, value: crate::vulkan1_0::SharingMode) -> &mut Self {
        self.sharing_mode = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index_count`]
    pub fn set_queue_family_index_count(&mut self, value: u32) -> &mut Self {
        self.queue_family_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_family_indices = value.as_ptr();
        self.queue_family_index_count = len_;
        self
    }
}
///[VkImageDrmFormatModifierListCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierListCreateInfoEXT.html) - Specify that an image must be created with a DRM format modifier from the provided list
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageDrmFormatModifierListCreateInfoEXT`] structure, then the image
///will be created with one of the [Linux DRM
///format modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) listed in the structure.
///The choice of modifier is implementation-dependent.The
/// [`ImageDrmFormatModifierListCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkImageDrmFormatModifierListCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           drmFormatModifierCount;
///    const uint64_t*    pDrmFormatModifiers;
///} VkImageDrmFormatModifierListCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier_count`] is the length of the [`drm_format_modifiers`] array.
/// - [`drm_format_modifiers`] is a pointer to an array of *Linux DRM format modifiers*.
///# Description
///## Valid Usage
/// - Each *modifier* in [`drm_format_modifiers`] **must**  be compatible with the parameters in
///   [`ImageCreateInfo`] and its [`p_next`] chain, as determined by querying
///   [`PhysicalDeviceImageFormatInfo2`] extended with
///   [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`
/// - [`drm_format_modifiers`] **must**  be a valid pointer to an array of
///   [`drm_format_modifier_count`]`uint64_t` values
/// - [`drm_format_modifier_count`] **must**  be greater than `0`
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`drm_format_modifier_count`] is the length of the
    ///[`drm_format_modifiers`] array.
    pub drm_format_modifier_count: u32,
    ///[`drm_format_modifiers`] is a pointer to an array of *Linux DRM format
    ///modifiers*.
    pub drm_format_modifiers: *const u64,
}
impl<'lt> Default for ImageDrmFormatModifierListCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            drm_format_modifier_count: 0,
            drm_format_modifiers: std::ptr::null(),
        }
    }
}
impl<'lt> ImageDrmFormatModifierListCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::drm_format_modifiers`]
    pub fn drm_format_modifiers_raw(&self) -> *const u64 {
        self.drm_format_modifiers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifiers`]
    pub fn set_drm_format_modifiers_raw(&mut self, value: *const u64) -> &mut Self {
        self.drm_format_modifiers = value;
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
    ///Gets the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count(&self) -> u32 {
        self.drm_format_modifier_count
    }
    ///Gets the value of [`Self::drm_format_modifiers`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn drm_format_modifiers(&self) -> &[u64] {
        std::slice::from_raw_parts(self.drm_format_modifiers, self.drm_format_modifier_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_count
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
    ///Sets the raw value of [`Self::drm_format_modifier_count`]
    pub fn set_drm_format_modifier_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_count = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifiers`]
    pub fn set_drm_format_modifiers(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.drm_format_modifiers = value.as_ptr();
        self.drm_format_modifier_count = len_;
        self
    }
}
///[VkImageDrmFormatModifierExplicitCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierExplicitCreateInfoEXT.html) - Specify that an image be created with the provided DRM format modifier and explicit memory layout
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`] structure, then the
///image will be created with the [Linux DRM
///format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier) and memory layout defined by the structure.The [`ImageDrmFormatModifierExplicitCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
///    VkStructureType               sType;
///    const void*                   pNext;
///    uint64_t                      drmFormatModifier;
///    uint32_t                      drmFormatModifierPlaneCount;
///    const VkSubresourceLayout*    pPlaneLayouts;
///} VkImageDrmFormatModifierExplicitCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier`] is the *Linux DRM format modifier* with which the image will be
///   created.
/// - [`drm_format_modifier_plane_count`] is the number of *memory planes* in the image (as reported
///   by [`DrmFormatModifierPropertiesEXT`]) as well as the length of the [`plane_layouts`] array.
/// - [`plane_layouts`] is a pointer to an array of [`SubresourceLayout`] structures describing the
///   image’s *memory planes*.
///# Description
///The `i`<sup>th</sup> member of [`plane_layouts`] describes the layout of the
///image’s `i`<sup>th</sup>*memory plane* (that is,
///`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT`).
///In each element of [`plane_layouts`], the implementation  **must**  ignore
///`size`.
///The implementation calculates the size of each plane, which the application
/// **can**  query with [`GetImageSubresourceLayout`].When creating an image with
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`], it is the application’s
///responsibility to satisfy all valid usage requirements.
///However, the implementation  **must**  validate that the provided
///[`plane_layouts`], when combined with the provided [`drm_format_modifier`]
///and other creation parameters in [`ImageCreateInfo`] and its [`p_next`]
///chain, produce a valid image.
///(This validation is necessarily implementation-dependent and outside the
///scope of Vulkan, and therefore not described by valid usage requirements).
///If this validation fails, then [`CreateImage`] returns
///`VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.
///## Valid Usage
/// - [`drm_format_modifier`] **must**  be compatible with the parameters in [`ImageCreateInfo`] and
///   its [`p_next`] chain, as determined by querying [`PhysicalDeviceImageFormatInfo2`] extended
///   with [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
/// - [`drm_format_modifier_plane_count`] **must**  be equal to the
///   [`DrmFormatModifierPropertiesEXT`]::[`drm_format_modifier_plane_count`] associated with
///   [`ImageCreateInfo::format`] and [`drm_format_modifier`], as found by querying
///   [`DrmFormatModifierPropertiesListEXT`]
/// - For each element of [`plane_layouts`], `size` **must**  be 0
/// - For each element of [`plane_layouts`], `arrayPitch` **must**  be 0 if
///   [`ImageCreateInfo::array_layers`] is 1
/// - For each element of [`plane_layouts`], `depthPitch` **must**  be 0 if
///   [`ImageCreateInfo`]::`extent.depth` is 1
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`
/// - If [`drm_format_modifier_plane_count`] is not `0`, [`plane_layouts`] **must**  be a valid
///   pointer to an array of [`drm_format_modifier_plane_count`][`SubresourceLayout`] structures
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`StructureType`]
/// - [`SubresourceLayout`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`drm_format_modifier`] is the *Linux DRM format modifier* with which
    ///the image will be created.
    pub drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///the image (as reported by [`DrmFormatModifierPropertiesEXT`]) as
    ///well as the length of the [`plane_layouts`] array.
    pub drm_format_modifier_plane_count: u32,
    ///[`plane_layouts`] is a pointer to an array of
    ///[`SubresourceLayout`] structures describing the image’s *memory
    ///planes*.
    pub plane_layouts: *const SubresourceLayout,
}
impl<'lt> Default for ImageDrmFormatModifierExplicitCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            drm_format_modifier: 0,
            drm_format_modifier_plane_count: 0,
            plane_layouts: std::ptr::null(),
        }
    }
}
impl<'lt> ImageDrmFormatModifierExplicitCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::plane_layouts`]
    pub fn plane_layouts_raw(&self) -> *const SubresourceLayout {
        self.plane_layouts
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::plane_layouts`]
    pub fn set_plane_layouts_raw(&mut self, value: *const SubresourceLayout) -> &mut Self {
        self.plane_layouts = value;
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
    ///Gets the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Gets the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count(&self) -> u32 {
        self.drm_format_modifier_plane_count
    }
    ///Gets the value of [`Self::plane_layouts`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn plane_layouts(&self) -> &[SubresourceLayout] {
        std::slice::from_raw_parts(self.plane_layouts, self.drm_format_modifier_plane_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_plane_count
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
    ///Sets the raw value of [`Self::drm_format_modifier`]
    pub fn set_drm_format_modifier(&mut self, value: u64) -> &mut Self {
        self.drm_format_modifier = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_plane_count`]
    pub fn set_drm_format_modifier_plane_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_plane_count = value;
        self
    }
    ///Sets the raw value of [`Self::plane_layouts`]
    pub fn set_plane_layouts(&mut self, value: &'lt [crate::vulkan1_0::SubresourceLayout]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.plane_layouts = value.as_ptr();
        self.drm_format_modifier_plane_count = len_;
        self
    }
}
///[VkImageDrmFormatModifierPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageDrmFormatModifierPropertiesEXT.html) - Properties of an image's Linux DRM format modifier
///# C Specifications
///The [`ImageDrmFormatModifierPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_image_drm_format_modifier
///typedef struct VkImageDrmFormatModifierPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           drmFormatModifier;
///} VkImageDrmFormatModifierPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier`] returns the image’s [Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
///# Description
///If the `image` was created with
///[`ImageDrmFormatModifierListCreateInfoEXT`], then the returned
///[`drm_format_modifier`] **must**  belong to the list of modifiers provided at
///time of image creation in
///[`ImageDrmFormatModifierListCreateInfoEXT::drm_format_modifiers`].
///If the `image` was created with
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`], then the returned
///[`drm_format_modifier`] **must**  be the modifier provided at time of image
///creation in
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::[`drm_format_modifier`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`StructureType`]
/// - [`GetImageDrmFormatModifierPropertiesEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`drm_format_modifier`] returns the image’s
    ///[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
    pub drm_format_modifier: u64,
}
impl<'lt> Default for ImageDrmFormatModifierPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            drm_format_modifier: 0,
        }
    }
}
impl<'lt> ImageDrmFormatModifierPropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
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
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
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
    ///Sets the raw value of [`Self::drm_format_modifier`]
    pub fn set_drm_format_modifier(&mut self, value: u64) -> &mut Self {
        self.drm_format_modifier = value;
        self
    }
}
///[VkDrmFormatModifierPropertiesList2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html) - Structure specifying the list of DRM format modifiers supported for a format
///# C Specifications
///The list of [Linux DRM format modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier)
///compatible with a [`Format`] **can**  be obtained by adding a
///[`DrmFormatModifierPropertiesList2EXT`] structure to the [`p_next`]
///chain of [`FormatProperties2`].The [`DrmFormatModifierPropertiesList2EXT`] structure is defined
/// as:
///```c
///// Provided by VK_KHR_format_feature_flags2 with VK_EXT_image_drm_format_modifier
///typedef struct VkDrmFormatModifierPropertiesList2EXT {
///    VkStructureType                       sType;
///    void*                                 pNext;
///    uint32_t                              drmFormatModifierCount;
///    VkDrmFormatModifierProperties2EXT*    pDrmFormatModifierProperties;
///} VkDrmFormatModifierPropertiesList2EXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`drm_format_modifier_count`] is an inout parameter related to the number of modifiers
///   compatible with the `format`, as described below.
/// - [`drm_format_modifier_properties`] is either `NULL` or a pointer to an array of
///   [`DrmFormatModifierProperties2EXT`] structures.
///# Description
///If [`drm_format_modifier_properties`] is `NULL`, the number of modifiers
///compatible with the queried `format` is returned in
///[`drm_format_modifier_count`].
///Otherwise, the application  **must**  set [`drm_format_modifier_count`] to the
///length of the array [`drm_format_modifier_properties`]; the function will
///write at most [`drm_format_modifier_count`] elements to the array, and will
///return in [`drm_format_modifier_count`] the number of elements written.Among the elements in
/// array [`drm_format_modifier_properties`], each
///returned `drmFormatModifier` **must**  be unique.Among the elements in array
/// [`drm_format_modifier_properties`], the bits
///reported in `drmFormatModifierTilingFeatures` **must**  include the bits
///reported in the corresponding element of
///[`DrmFormatModifierPropertiesListEXT`]::[`drm_format_modifier_properties`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`VK_KHR_format_feature_flags2`]
/// - [`DrmFormatModifierProperties2EXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`drm_format_modifier_count`] is an inout parameter related to the number
    ///of modifiers compatible with the `format`, as described below.
    pub drm_format_modifier_count: u32,
    ///[`drm_format_modifier_properties`] is either `NULL` or a pointer to an
    ///array of [`DrmFormatModifierProperties2EXT`] structures.
    pub drm_format_modifier_properties: *mut DrmFormatModifierProperties2EXT,
}
impl<'lt> Default for DrmFormatModifierPropertiesList2EXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: 0,
            drm_format_modifier_properties: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DrmFormatModifierPropertiesList2EXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn drm_format_modifier_properties_raw(&self) -> &*mut DrmFormatModifierProperties2EXT {
        &self.drm_format_modifier_properties
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn set_drm_format_modifier_properties_raw(&mut self, value: *mut DrmFormatModifierProperties2EXT) -> &mut Self {
        self.drm_format_modifier_properties = value;
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
    ///Gets the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count(&self) -> u32 {
        self.drm_format_modifier_count
    }
    ///Gets the value of [`Self::drm_format_modifier_properties`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn drm_format_modifier_properties(&self) -> &[DrmFormatModifierProperties2EXT] {
        std::slice::from_raw_parts(
            self.drm_format_modifier_properties,
            self.drm_format_modifier_count as usize,
        )
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
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_count`]
    pub fn drm_format_modifier_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_count
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_properties`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn drm_format_modifier_properties_mut(&mut self) -> &mut [DrmFormatModifierProperties2EXT] {
        std::slice::from_raw_parts_mut(
            self.drm_format_modifier_properties,
            self.drm_format_modifier_count as usize,
        )
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
    ///Sets the raw value of [`Self::drm_format_modifier_count`]
    pub fn set_drm_format_modifier_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_count = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_properties`]
    pub fn set_drm_format_modifier_properties(
        &mut self,
        value: &'lt mut [crate::extensions::ext_image_drm_format_modifier::DrmFormatModifierProperties2EXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.drm_format_modifier_properties = value.as_mut_ptr();
        self.drm_format_modifier_count = len_;
        self
    }
}
///[VkDrmFormatModifierProperties2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierProperties2EXT.html) - Structure specifying properties of a format when combined with a DRM format modifier
///# C Specifications
///The [`DrmFormatModifierProperties2EXT`] structure describes properties
///of a [`Format`] when that format is combined with a
///[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
///These properties, like those of [`FormatProperties2`], are independent
///of any particular image.The [`DrmFormatModifierPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_KHR_format_feature_flags2 with VK_EXT_image_drm_format_modifier
///typedef struct VkDrmFormatModifierProperties2EXT {
///    uint64_t                 drmFormatModifier;
///    uint32_t                 drmFormatModifierPlaneCount;
///    VkFormatFeatureFlags2    drmFormatModifierTilingFeatures;
///} VkDrmFormatModifierProperties2EXT;
///```
///# Members
/// - [`drm_format_modifier`] is a *Linux DRM format modifier*.
/// - [`drm_format_modifier_plane_count`] is the number of *memory planes* in any image created with
///   `format` and [`drm_format_modifier`]. An image’s *memory planecount* is distinct from its
///   *format planecount*, as explained below.
/// - [`drm_format_modifier_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] that are
///   supported by any image created with `format` and [`drm_format_modifier`].
///# Related
/// - [`VK_EXT_image_drm_format_modifier`]
/// - [`VK_KHR_format_feature_flags2`]
/// - [`DrmFormatModifierPropertiesList2EXT`]
/// - [`FormatFeatureFlags2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDrmFormatModifierProperties2EXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    ///[`drm_format_modifier`] is a *Linux DRM format modifier*.
    pub drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///any image created with `format` and [`drm_format_modifier`].
    ///An image’s *memory planecount* is distinct from its *format planecount*,
    ///as explained below.
    pub drm_format_modifier_plane_count: u32,
    ///[`drm_format_modifier_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits2`] that are supported by any image created
    ///with `format` and [`drm_format_modifier`].
    pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
impl Default for DrmFormatModifierProperties2EXT {
    fn default() -> Self {
        Self {
            drm_format_modifier: 0,
            drm_format_modifier_plane_count: 0,
            drm_format_modifier_tiling_features: Default::default(),
        }
    }
}
impl DrmFormatModifierProperties2EXT {
    ///Gets the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Gets the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count(&self) -> u32 {
        self.drm_format_modifier_plane_count
    }
    ///Gets the value of [`Self::drm_format_modifier_tiling_features`]
    pub fn drm_format_modifier_tiling_features(&self) -> FormatFeatureFlags2 {
        self.drm_format_modifier_tiling_features
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier`]
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_plane_count`]
    pub fn drm_format_modifier_plane_count_mut(&mut self) -> &mut u32 {
        &mut self.drm_format_modifier_plane_count
    }
    ///Gets a mutable reference to the value of [`Self::drm_format_modifier_tiling_features`]
    pub fn drm_format_modifier_tiling_features_mut(&mut self) -> &mut FormatFeatureFlags2 {
        &mut self.drm_format_modifier_tiling_features
    }
    ///Sets the raw value of [`Self::drm_format_modifier`]
    pub fn set_drm_format_modifier(&mut self, value: u64) -> &mut Self {
        self.drm_format_modifier = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_plane_count`]
    pub fn set_drm_format_modifier_plane_count(&mut self, value: u32) -> &mut Self {
        self.drm_format_modifier_plane_count = value;
        self
    }
    ///Sets the raw value of [`Self::drm_format_modifier_tiling_features`]
    pub fn set_drm_format_modifier_tiling_features(
        &mut self,
        value: crate::vulkan1_3::FormatFeatureFlags2,
    ) -> &mut Self {
        self.drm_format_modifier_tiling_features = value;
        self
    }
}
