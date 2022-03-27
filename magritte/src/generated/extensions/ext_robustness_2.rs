use crate::vulkan1_0::{BaseOutStructure, Bool32, DeviceSize, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ROBUSTNESS_2_SPEC_VERSION")]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ROBUSTNESS_2_EXTENSION_NAME")]
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_robustness2");
///[VkPhysicalDeviceRobustness2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) - Structure describing the out-of-bounds behavior for an implementation
///# C Specifications
///The [`PhysicalDeviceRobustness2FeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_robustness2
///typedef struct VkPhysicalDeviceRobustness2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           robustBufferAccess2;
///    VkBool32           robustImageAccess2;
///    VkBool32           nullDescriptor;
///} VkPhysicalDeviceRobustness2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`robust_buffer_access_2`] indicates whether buffer accesses are tightly bounds-checked against the range of the descriptor. Uniform buffers **must** be bounds-checked to the range of the descriptor, where the range is rounded up to a multiple of [robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment). Storage buffers **must** be bounds-checked to the range of the descriptor, where the range is rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment). Out of bounds buffer loads will return zero values, and formatted loads will have (0,0,1) values inserted for missing G, B, or A components based on the format.
/// - [`robust_image_access_2`] indicates whether image accesses are tightly bounds-checked against
///   the dimensions of the image view. Out of bounds image loads will return zero values, with
///   (0,0,1) values [inserted for missing G, B, or A components]() based on the format.
/// - [`null_descriptor`] indicates whether descriptors **can** be written with a
///   [`crate::utils::Handle::null`] resource or view, which are considered valid to access and act
///   as if the descriptor were bound to nothing.
///If the [`PhysicalDeviceRobustness2FeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRobustness2FeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage
/// - If [`robust_buffer_access_2`] is enabled then [`robustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess)**must**
///   also be enabled
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_robustness2`]
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
pub struct PhysicalDeviceRobustness2FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`robust_buffer_access_2`] indicates
    ///whether buffer accesses are tightly bounds-checked against the range of
    ///the descriptor.
    ///Uniform buffers **must** be bounds-checked to the range of the descriptor,
    ///where the range is rounded up to a multiple of
    ///[robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment).
    ///Storage buffers **must** be bounds-checked to the range of the descriptor,
    ///where the range is rounded up to a multiple of
    ///[robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment).
    ///Out of bounds buffer loads will return zero values, and formatted loads
    ///will have (0,0,1) values inserted for missing G, B, or A
    ///components based on the format.
    robust_buffer_access_2: Bool32,
    ///[`robust_image_access_2`] indicates
    ///whether image accesses are tightly bounds-checked against the dimensions
    ///of the image view.
    ///Out of bounds image loads will return zero values, with (0,0,1)
    ///values [inserted for missing G, B, or A
    ///components]() based on the format.
    robust_image_access_2: Bool32,
    ///[`null_descriptor`] indicates whether
    ///descriptors **can** be written with a [`crate::utils::Handle::null`] resource or
    ///view, which are considered valid to access and act as if the descriptor
    ///were bound to nothing.
    null_descriptor: Bool32,
}
///[VkPhysicalDeviceRobustness2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) - Structure describing robust buffer access properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRobustness2PropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_robustness2
///typedef struct VkPhysicalDeviceRobustness2PropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       robustStorageBufferAccessSizeAlignment;
///    VkDeviceSize       robustUniformBufferAccessSizeAlignment;
///} VkPhysicalDeviceRobustness2PropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`robust_storage_buffer_access_size_alignment`] is the number of bytes that the range of a storage buffer descriptor is rounded up to when used for bounds-checking when [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled. This value **must** be either 1 or 4.
/// - [`robust_uniform_buffer_access_size_alignment`] is the number of bytes that the range of a uniform buffer descriptor is rounded up to when used for bounds-checking when [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled. This value **must** be a power of two in the range [1, 256].
///# Description
///If the [`PhysicalDeviceRobustness2PropertiesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_robustness2`]
/// - [`DeviceSize`]
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
pub struct PhysicalDeviceRobustness2PropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`robust_storage_buffer_access_size_alignment`] is the number of bytes that
    ///the range of a storage buffer descriptor is rounded up to when used for
    ///bounds-checking when
    ///[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled.
    ///This value **must** be either 1 or 4.
    robust_storage_buffer_access_size_alignment: DeviceSize,
    ///[`robust_uniform_buffer_access_size_alignment`] is the number of bytes that
    ///the range of a uniform buffer descriptor is rounded up to when used for
    ///bounds-checking when
    ///[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled.
    ///This value **must** be a power of two in the range [1, 256].
    robust_uniform_buffer_access_size_alignment: DeviceSize,
}
