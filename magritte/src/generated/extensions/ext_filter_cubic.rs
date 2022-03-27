use crate::vulkan1_0::{BaseOutStructure, Bool32, ImageViewType, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_filter_cubic");
///[VkPhysicalDeviceImageViewImageFormatInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html) - Structure for providing image view type
///# C Specifications
///The [`PhysicalDeviceImageViewImageFormatInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_filter_cubic
///typedef struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkImageViewType    imageViewType;
///} VkPhysicalDeviceImageViewImageFormatInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view_type`] is a [`ImageViewType`] value specifying the type of the image view.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT`
/// - [`image_view_type`]**must** be a valid [`ImageViewType`] value
///# Related
/// - [`VK_EXT_filter_cubic`]
/// - [`ImageViewType`]
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
pub struct PhysicalDeviceImageViewImageFormatInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`image_view_type`] is a [`ImageViewType`] value specifying the type
    ///of the image view.
    image_view_type: ImageViewType,
}
///[VkFilterCubicImageViewImageFormatPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html) - Structure for querying cubic filtering capabilities of an image view type
///# C Specifications
///The [`FilterCubicImageViewImageFormatPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_filter_cubic
///typedef struct VkFilterCubicImageViewImageFormatPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           filterCubic;
///    VkBool32           filterCubicMinmax;
///} VkFilterCubicImageViewImageFormatPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`filter_cubic`] tells if image format, image type and image view type **can** be used with
///   cubic filtering. This field is set by the implementation. User-specified value is ignored.
/// - [`filter_cubic_minmax`] tells if image format, image type and image view type **can** be used
///   with cubic filtering and minmax filtering. This field is set by the implementation.
///   User-specified value is ignored.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT`
///Valid Usage
/// - If the [`p_next`] chain of the [`ImageFormatProperties2`] structure includes a
///   [`FilterCubicImageViewImageFormatPropertiesEXT`] structure, the [`p_next`] chain of the
///   [`PhysicalDeviceImageFormatInfo2`] structure **must** include a
///   [`PhysicalDeviceImageViewImageFormatInfoEXT`] structure with an `imageViewType` that is
///   compatible with `imageType`
///# Related
/// - [`VK_EXT_filter_cubic`]
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
pub struct FilterCubicImageViewImageFormatPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`filter_cubic`] tells if image format, image type and image view type
    ///**can** be used with cubic filtering.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    filter_cubic: Bool32,
    ///[`filter_cubic_minmax`] tells if image format, image type and image view
    ///type **can** be used with cubic filtering and minmax filtering.
    ///This field is set by the implementation.
    ///User-specified value is ignored.
    filter_cubic_minmax: Bool32,
}
