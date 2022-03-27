use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_multi_draw");
///[VkMultiDrawInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawInfoEXT {
///    uint32_t    firstVertex;
///    uint32_t    vertexCount;
///} VkMultiDrawInfoEXT;
///```
///# Members
/// - [`first_vertex`] is the first vertex to draw.
/// - [`vertex_count`] is the number of vertices to draw.
///# Description
///The members of [`MultiDrawInfoEXT`] have the same meaning as the
///[`first_vertex`] and [`vertex_count`] parameters in [`CmdDraw`].
///# Related
/// - [`VK_EXT_multi_draw`]
/// - [`CmdDrawMultiEXT`]
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
pub struct MultiDrawInfoEXT {
    ///[`first_vertex`] is the first vertex to draw.
    first_vertex: u32,
    ///[`vertex_count`] is the number of vertices to draw.
    vertex_count: u32,
}
///[VkMultiDrawIndexedInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawIndexedInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawIndexedInfoEXT {
///    uint32_t    firstIndex;
///    uint32_t    indexCount;
///    int32_t     vertexOffset;
///} VkMultiDrawIndexedInfoEXT;
///```
///# Members
/// - [`first_index`] is the first index to draw.
/// - [`index_count`] is the number of vertices to draw.
/// - [`vertex_offset`] is the value added to the vertex index before indexing into the vertex
///   buffer for indexed multidraws.
///# Description
///The [`first_index`], [`index_count`], and [`vertex_offset`] members of
///[`MultiDrawIndexedInfoEXT`] have the same meaning as the
///[`first_index`], [`index_count`], and [`vertex_offset`] parameters,
///respectively, of [`CmdDrawIndexed`].
///# Related
/// - [`VK_EXT_multi_draw`]
/// - [`CmdDrawMultiIndexedEXT`]
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
pub struct MultiDrawIndexedInfoEXT {
    ///[`first_index`] is the first index to draw.
    first_index: u32,
    ///[`index_count`] is the number of vertices to draw.
    index_count: u32,
    ///[`vertex_offset`] is the value added to the vertex index before
    ///indexing into the vertex buffer for indexed multidraws.
    vertex_offset: i32,
}
///[VkPhysicalDeviceMultiDrawPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) - Structure describing multidraw limits of an implementation
///# C Specifications
///The [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxMultiDrawCount;
///} VkPhysicalDeviceMultiDrawPropertiesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure
///describe the following features:
///# Description
/// - [`max_multi_draw_count`] indicates the maximum number of draw calls which **can** be batched
///   into a single multidraw.
///If the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_multi_draw`]
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
pub struct PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_multi_draw_count`] indicates the
    ///maximum number of draw calls which **can** be batched into a single
    ///multidraw.
    max_multi_draw_count: u32,
}
///[VkPhysicalDeviceMultiDrawFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) - Structure describing whether the implementation supports multi draw functionality
///# C Specifications
///The [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           multiDraw;
///} VkPhysicalDeviceMultiDrawFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure
///describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`multi_draw`] indicates that the implementation supports [`CmdDrawMultiEXT`] and
///   [`CmdDrawMultiIndexedEXT`].
///If the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMultiDrawFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`
///# Related
/// - [`VK_EXT_multi_draw`]
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
pub struct PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`multi_draw`] indicates that the implementation
    ///supports [`CmdDrawMultiEXT`] and [`CmdDrawMultiIndexedEXT`].
    multi_draw: Bool32,
}
