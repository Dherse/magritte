use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION")]
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME")]
pub const EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_image_view_min_lod");
///[VkPhysicalDeviceImageViewMinLodFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewMinLodFeaturesEXT.html) - Structure describing whether clamping the min lod of a image view is supported by the implementation
///# C Specifications
///The [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_image_view_min_lod
///typedef struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           minLod;
///} VkPhysicalDeviceImageViewMinLodFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`min_lod`] indicates whether the implementation supports clamping the minimum LOD value during [Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by [`ImageViewMinLodCreateInfoEXT`]::[`min_lod`].
///If the [`PhysicalDeviceImageViewMinLodFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceImageViewMinLodFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
///# Related
/// - [`VK_EXT_image_view_min_lod`]
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
pub struct PhysicalDeviceImageViewMinLodFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`min_lod`] indicates whether the implementation
    ///supports clamping the minimum LOD value during
    ///[Image Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and
    ///[Integer Texel Coordinate
    ///Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations) with a given [`ImageView`] by
    ///[`ImageViewMinLodCreateInfoEXT`]::[`min_lod`].
    min_lod: Bool32,
}
///[VkImageViewMinLodCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewMinLodCreateInfoEXT.html) - Structure describing the minimum lod of an image view
///# C Specifications
///If the [`p_next`] chain includes a [`ImageViewMinLodCreateInfoEXT`]
///structure, then that structure includes a parameter specifying a value to
///clamp the minimum LOD value during [Image
///Level(s) Selection](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-image-level-selection) and [Integer
///Texel Coordinate Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-integer-coordinate-operations).The [`ImageViewMinLodCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_image_view_min_lod
///typedef struct VkImageViewMinLodCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    float              minLod;
///} VkImageViewMinLodCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_lod`] is the value to clamp the minimum LOD accessible by this [`ImageView`].
///# Description
///Valid Usage
/// - If the [[`min_lod`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-minLod)
///   feature is not enabled, [`min_lod`]**must** be `0.0`.
/// - [`min_lod`]**must** be less or equal to the index of the last mipmap level accessible to the
///   view.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT`
///# Related
/// - [`VK_EXT_image_view_min_lod`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageViewMinLodCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`min_lod`] is the value to clamp the minimum LOD accessible by this
    ///[`ImageView`].
    min_lod: f32,
}
