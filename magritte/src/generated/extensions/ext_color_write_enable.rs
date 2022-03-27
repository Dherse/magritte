use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_color_write_enable");
///[VkPhysicalDeviceColorWriteEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html) - Structure describing whether writes to color attachments can be enabled and disabled dynamically
///# C Specifications
///The [`PhysicalDeviceColorWriteEnableFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_color_write_enable
///typedef struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           colorWriteEnable;
///} VkPhysicalDeviceColorWriteEnableFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`color_write_enable`] indicates that the implementation supports the dynamic state
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`.
///If the [`PhysicalDeviceColorWriteEnableFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceColorWriteEnableFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_color_write_enable`]
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
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`color_write_enable`] indicates that the
    ///implementation supports the dynamic state
    ///`VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`.
    color_write_enable: Bool32,
}
///[VkPipelineColorWriteCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) - Structure specifying color write state of a newly created pipeline
///# C Specifications
///The [`PipelineColorWriteCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_color_write_enable
///typedef struct VkPipelineColorWriteCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           attachmentCount;
///    const VkBool32*    pColorWriteEnables;
///} VkPipelineColorWriteCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment_count`] is the number of [`Bool32`] elements in [`p_color_write_enables`].
/// - [`p_color_write_enables`] is a pointer to an array of per target attachment boolean values
///   specifying whether color writes are enabled for the given attachment.
///# Description
///When this structure is included in the [`p_next`] chain of
///[`PipelineColorBlendStateCreateInfo`], it defines per-attachment color
///write state.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with [`attachment_count`] equal to the
///[`attachment_count`] member of [`PipelineColorBlendStateCreateInfo`],
///and [`p_color_write_enables`] pointing to an array of as many [`TRUE`]
///values.If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable) feature is not enabled
///on the device, all [`Bool32`] elements in the
///[`p_color_write_enables`] array **must** be [`TRUE`].Color Write Enable interacts with the
/// [Color
///Write Mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-mask) as follows:
/// - If `colorWriteEnable` is [`TRUE`], writes to the attachment are determined by the
///   `colorWriteMask`.
/// - If `colorWriteEnable` is [`FALSE`], the `colorWriteMask` is ignored and writes to all
///   components of the attachment are disabled. This is equivalent to specifying a `colorWriteMask`
///   of 0.
///Valid Usage
/// - If the [colorWriteEnable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-colorWriteEnable)
///   feature is not enabled, all elements of [`p_color_write_enables`]**must** be [`TRUE`]
/// - [`attachment_count`]**must** be equal to the [`attachment_count`] member of the
///   [`PipelineColorBlendStateCreateInfo`] structure specified during pipeline creation
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
/// - If [`attachment_count`] is not `0`, [`p_color_write_enables`]**must** be a valid pointer to an
///   array of [`attachment_count`][`Bool32`] values
///# Related
/// - [`VK_EXT_color_write_enable`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`attachment_count`] is the number of [`Bool32`] elements in
    ///[`p_color_write_enables`].
    attachment_count: u32,
    ///[`p_color_write_enables`] is a pointer to an array of per target
    ///attachment boolean values specifying whether color writes are enabled
    ///for the given attachment.
    p_color_write_enables: *mut Bool32,
}
