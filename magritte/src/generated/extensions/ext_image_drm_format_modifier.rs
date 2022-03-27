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
/// - [`p_drm_format_modifier_properties`] is either `NULL` or a pointer to an array of
///   [`DrmFormatModifierPropertiesEXT`] structures.
///# Description
///If [`p_drm_format_modifier_properties`] is `NULL`, then the function returns
///in [`drm_format_modifier_count`] the number of modifiers compatible with the
///queried `format`.
///Otherwise, the application **must** set [`drm_format_modifier_count`] to the
///length of the array [`p_drm_format_modifier_properties`]; the function will
///write at most [`drm_format_modifier_count`] elements to the array, and will
///return in [`drm_format_modifier_count`] the number of elements written.Among the elements in
/// array [`p_drm_format_modifier_properties`], each
///returned `drmFormatModifier`**must** be unique.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesListEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`drm_format_modifier_count`] is an inout parameter related to the number
    ///of modifiers compatible with the `format`, as described below.
    drm_format_modifier_count: u32,
    ///[`p_drm_format_modifier_properties`] is either `NULL` or a pointer to an
    ///array of [`DrmFormatModifierPropertiesEXT`] structures.
    p_drm_format_modifier_properties: *const DrmFormatModifierPropertiesEXT,
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
///The returned [`drm_format_modifier_tiling_features`]**must** contain at least
///one bit.The implementation **must** not return `DRM_FORMAT_MOD_INVALID` in
///[`drm_format_modifier`].An image’s *memory planecount* (as returned by
///[`drm_format_modifier_plane_count`]) is distinct from its *format planecount*
///(in the sense of [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///Y′C<sub>B</sub>C<sub>R</sub> formats).
///In [`ImageAspectFlags`], each
///`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` represents a *memory plane*
///and each `VK_IMAGE_ASPECT_PLANE*_i_*BIT` a *format plane*.An image’s set of *format planes* is
/// an ordered partition of the image’s
///**content** into separable groups of format components.
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
///**memory** rather than the image’s **content**.
///Each *memory plane* is a contiguous range of memory.
///The union of an image’s *memory planes* is not necessarily contiguous.If an image is [linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition is
///the same for *memory planes* and for *format planes*.
///Therefore, if the returned [`drm_format_modifier`] is
///`DRM_FORMAT_MOD_LINEAR`, then [`drm_format_modifier_plane_count`]**must**
///equal the *format planecount*, and [`drm_format_modifier_tiling_features`]**must** be identical
/// to the
///[`FormatProperties2`]`::linearTilingFeatures` returned in the same
///`pNext` chain.If an image is [non-linear](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-linear-resource), then the partition
///of the image’s **memory** into *memory planes* is implementation-specific and
///**may** be unrelated to the partition of the image’s **content** into *format
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
///In a straightforward implementation, the implementation **may** store the
///image’s content in 3 adjacent *memory planes* where each *memory plane*
///corresponds exactly to a *format plane*.
///However, the implementation **may** also store the image’s content in a single
///*memory plane* where all format components are combined using an
///implementation-private block-compressed format; or the implementation **may**
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    ///[`drm_format_modifier`] is a *Linux DRM format modifier*.
    drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///any image created with `format` and [`drm_format_modifier`].
    ///An image’s *memory planecount* is distinct from its *format planecount*,
    ///as explained below.
    drm_format_modifier_plane_count: u32,
    ///[`drm_format_modifier_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits`] that are supported by any image created
    ///with `format` and [`drm_format_modifier`].
    drm_format_modifier_tiling_features: FormatFeatureFlags,
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
/// - [`queue_family_index_count`] is the number of entries in the [`p_queue_family_indices`] array.
/// - [`p_queue_family_indices`] is a pointer to an array of queue families that will access the
///   image. It is ignored if [`sharing_mode`] is not `VK_SHARING_MODE_CONCURRENT`.
///# Description
///If the [`drm_format_modifier`] is incompatible with the parameters specified
///in [`PhysicalDeviceImageFormatInfo2`] and its [`p_next`] chain, then
///[`GetPhysicalDeviceImageFormatProperties2`] returns
///`VK_ERROR_FORMAT_NOT_SUPPORTED`.
///The implementation **must** support the query of any [`drm_format_modifier`],
///including unknown and invalid modifier values.Valid Usage
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`p_queue_family_indices`]**must**
///   be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, then [`queue_family_index_count`]**must**
///   be greater than `1`
/// - If [`sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of
///   [`p_queue_family_indices`]**must** be unique and **must** be less than the
///   `pQueueFamilyPropertyCount` returned by [`GetPhysicalDeviceQueueFamilyProperties2`] for the
///   `physicalDevice` that was used to create `device`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT`
/// - [`sharing_mode`]**must** be a valid [`SharingMode`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`drm_format_modifier`] is the image’s *Linux DRM format modifier*,
    ///corresponding to
    ///[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::`modifier` or
    ///to [`ImageDrmFormatModifierListCreateInfoEXT`]::`pModifiers`.
    drm_format_modifier: u64,
    ///[`sharing_mode`] specifies how the image will be accessed by multiple
    ///queue families.
    sharing_mode: SharingMode,
    ///[`queue_family_index_count`] is the number of entries in the
    ///[`p_queue_family_indices`] array.
    queue_family_index_count: u32,
    ///[`p_queue_family_indices`] is a pointer to an array of queue families
    ///that will access the image.
    ///It is ignored if [`sharing_mode`] is not
    ///`VK_SHARING_MODE_CONCURRENT`.
    p_queue_family_indices: *mut u32,
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
/// - [`drm_format_modifier_count`] is the length of the [`p_drm_format_modifiers`] array.
/// - [`p_drm_format_modifiers`] is a pointer to an array of *Linux DRM format modifiers*.
///# Description
///Valid Usage
/// - Each *modifier* in [`p_drm_format_modifiers`]**must** be compatible with the parameters in
///   [`ImageCreateInfo`] and its [`p_next`] chain, as determined by querying
///   [`PhysicalDeviceImageFormatInfo2`] extended with
///   [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT`
/// - [`p_drm_format_modifiers`]**must** be a valid pointer to an array of
///   [`drm_format_modifier_count`]`uint64_t` values
/// - [`drm_format_modifier_count`]**must** be greater than `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageDrmFormatModifierListCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`drm_format_modifier_count`] is the length of the
    ///[`p_drm_format_modifiers`] array.
    drm_format_modifier_count: u32,
    ///[`p_drm_format_modifiers`] is a pointer to an array of *Linux DRM format
    ///modifiers*.
    p_drm_format_modifiers: *mut u64,
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
///   by [`DrmFormatModifierPropertiesEXT`]) as well as the length of the [`p_plane_layouts`] array.
/// - [`p_plane_layouts`] is a pointer to an array of [`SubresourceLayout`] structures describing
///   the image’s *memory planes*.
///# Description
///The `i`<sup>th</sup> member of [`p_plane_layouts`] describes the layout of the
///image’s `i`<sup>th</sup>*memory plane* (that is,
///`VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT`).
///In each element of [`p_plane_layouts`], the implementation **must** ignore
///`size`.
///The implementation calculates the size of each plane, which the application
///**can** query with [`GetImageSubresourceLayout`].When creating an image with
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`], it is the application’s
///responsibility to satisfy all valid usage requirements.
///However, the implementation **must** validate that the provided
///[`p_plane_layouts`], when combined with the provided [`drm_format_modifier`]
///and other creation parameters in [`ImageCreateInfo`] and its [`p_next`]
///chain, produce a valid image.
///(This validation is necessarily implementation-dependent and outside the
///scope of Vulkan, and therefore not described by valid usage requirements).
///If this validation fails, then [`CreateImage`] returns
///`VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT`.Valid Usage
/// - [`drm_format_modifier`]**must** be compatible with the parameters in [`ImageCreateInfo`] and
///   its [`p_next`] chain, as determined by querying [`PhysicalDeviceImageFormatInfo2`] extended
///   with [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
/// - [`drm_format_modifier_plane_count`]**must** be equal to the
///   [`DrmFormatModifierPropertiesEXT`]::[`drm_format_modifier_plane_count`] associated with
///   [`ImageCreateInfo::format`] and [`drm_format_modifier`], as found by querying
///   [`DrmFormatModifierPropertiesListEXT`]
/// - For each element of [`p_plane_layouts`], `size`**must** be 0
/// - For each element of [`p_plane_layouts`], `arrayPitch`**must** be 0 if
///   [`ImageCreateInfo::array_layers`] is 1
/// - For each element of [`p_plane_layouts`], `depthPitch`**must** be 0 if
///   [`ImageCreateInfo`]::`extent.depth` is 1
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT`
/// - If [`drm_format_modifier_plane_count`] is not `0`, [`p_plane_layouts`]**must** be a valid
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`drm_format_modifier`] is the *Linux DRM format modifier* with which
    ///the image will be created.
    drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///the image (as reported by [`DrmFormatModifierPropertiesEXT`]) as
    ///well as the length of the [`p_plane_layouts`] array.
    drm_format_modifier_plane_count: u32,
    ///[`p_plane_layouts`] is a pointer to an array of
    ///[`SubresourceLayout`] structures describing the image’s *memory
    ///planes*.
    p_plane_layouts: *mut SubresourceLayout,
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
///[`drm_format_modifier`]**must** belong to the list of modifiers provided at
///time of image creation in
///[`ImageDrmFormatModifierListCreateInfoEXT::p_drm_format_modifiers`].
///If the `image` was created with
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`], then the returned
///[`drm_format_modifier`]**must** be the modifier provided at time of image
///creation in
///[`ImageDrmFormatModifierExplicitCreateInfoEXT`]::[`drm_format_modifier`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageDrmFormatModifierPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`drm_format_modifier`] returns the image’s
    ///[Linux DRM format modifier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier).
    drm_format_modifier: u64,
}
///[VkDrmFormatModifierPropertiesList2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrmFormatModifierPropertiesList2EXT.html) - Structure specifying the list of DRM format modifiers supported for a format
///# C Specifications
///The list of [Linux DRM format modifiers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-drm-format-modifier)
///compatible with a [`Format`]**can** be obtained by adding a
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
/// - [`p_drm_format_modifier_properties`] is either `NULL` or a pointer to an array of
///   [`DrmFormatModifierProperties2EXT`] structures.
///# Description
///If [`p_drm_format_modifier_properties`] is `NULL`, the number of modifiers
///compatible with the queried `format` is returned in
///[`drm_format_modifier_count`].
///Otherwise, the application **must** set [`drm_format_modifier_count`] to the
///length of the array [`p_drm_format_modifier_properties`]; the function will
///write at most [`drm_format_modifier_count`] elements to the array, and will
///return in [`drm_format_modifier_count`] the number of elements written.Among the elements in
/// array [`p_drm_format_modifier_properties`], each
///returned `drmFormatModifier`**must** be unique.Among the elements in array
/// [`p_drm_format_modifier_properties`], the bits
///reported in `drmFormatModifierTilingFeatures`**must** include the bits
///reported in the corresponding element of
///[`DrmFormatModifierPropertiesListEXT`]::[`p_drm_format_modifier_properties`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesList2EXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`drm_format_modifier_count`] is an inout parameter related to the number
    ///of modifiers compatible with the `format`, as described below.
    drm_format_modifier_count: u32,
    ///[`p_drm_format_modifier_properties`] is either `NULL` or a pointer to an
    ///array of [`DrmFormatModifierProperties2EXT`] structures.
    p_drm_format_modifier_properties: *const DrmFormatModifierProperties2EXT,
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    ///[`drm_format_modifier`] is a *Linux DRM format modifier*.
    drm_format_modifier: u64,
    ///[`drm_format_modifier_plane_count`] is the number of *memory planes* in
    ///any image created with `format` and [`drm_format_modifier`].
    ///An image’s *memory planecount* is distinct from its *format planecount*,
    ///as explained below.
    drm_format_modifier_plane_count: u32,
    ///[`drm_format_modifier_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits2`] that are supported by any image created
    ///with `format` and [`drm_format_modifier`].
    drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
