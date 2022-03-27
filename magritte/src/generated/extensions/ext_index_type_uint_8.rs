use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_index_type_uint8");
///[VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) - Structure describing whether uint8 index type can be used
///# C Specifications
///The [`PhysicalDeviceIndexTypeUint8FeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_index_type_uint8
///typedef struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           indexTypeUint8;
///} VkPhysicalDeviceIndexTypeUint8FeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`index_type_uint_8`] indicates that `VK_INDEX_TYPE_UINT8_EXT` can be used with
///   [`CmdBindIndexBuffer`].
///If the [`PhysicalDeviceIndexTypeUint8FeaturesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceIndexTypeUint8FeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`
///# Related
/// - [`VK_EXT_index_type_uint8`]
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
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`index_type_uint_8`] indicates that
    ///`VK_INDEX_TYPE_UINT8_EXT` can be used with
    ///[`CmdBindIndexBuffer`].
    index_type_uint_8: Bool32,
}
