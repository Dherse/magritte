use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NVX_multiview_per_view_attributes");
///[VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) - Structure describing multiview limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`] structure
///is defined as:
///```c
///// Provided by VK_NVX_multiview_per_view_attributes
///typedef struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           perViewPositionAllComponents;
///} VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`per_view_position_all_components`] is [`TRUE`] if the implementation supports per-view
///   position values that differ in components other than the X component.
///# Description
///If the [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
///# Related
/// - [`VK_NVX_multiview_per_view_attributes`]
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
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`per_view_position_all_components`] is [`TRUE`] if the
    ///implementation supports per-view position values that differ in
    ///components other than the X component.
    per_view_position_all_components: Bool32,
}
