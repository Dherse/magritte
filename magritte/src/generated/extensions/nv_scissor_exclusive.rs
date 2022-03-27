use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Rect2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION")]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME")]
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_scissor_exclusive");
///[VkPhysicalDeviceExclusiveScissorFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html) - Structure describing exclusive scissor features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceExclusiveScissorFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_scissor_exclusive
///typedef struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           exclusiveScissor;
///} VkPhysicalDeviceExclusiveScissorFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`exclusive_scissor`] indicates that the implementation supports the exclusive scissor test.
///See [Exclusive Scissor Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-exclusive-scissor) for more
///information.If the [`PhysicalDeviceExclusiveScissorFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExclusiveScissorFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV`
///# Related
/// - [`VK_NV_scissor_exclusive`]
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
pub struct PhysicalDeviceExclusiveScissorFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`exclusive_scissor`] indicates that the
    ///implementation supports the exclusive scissor test.
    exclusive_scissor: Bool32,
}
///[VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html) - Structure specifying parameters controlling exclusive scissor testing
///# C Specifications
///The [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_scissor_exclusive
///typedef struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           exclusiveScissorCount;
///    const VkRect2D*    pExclusiveScissors;
///} VkPipelineViewportExclusiveScissorStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`exclusive_scissor_count`] is the number of exclusive scissor rectangles.
/// - [`p_exclusive_scissors`] is a pointer to an array of [`Rect2D`] structures defining exclusive
///   scissor rectangles.
///# Description
///If the `VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV` dynamic state is enabled
///for a pipeline, the [`p_exclusive_scissors`] member is ignored.When this structure is included
/// in the [`p_next`] chain of
///[`GraphicsPipelineCreateInfo`], it defines parameters of the exclusive
///scissor test.
///If this structure is not included in the [`p_next`] chain, it is equivalent
///to specifying this structure with a [`exclusive_scissor_count`] of `0`.Valid Usage
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`exclusive_scissor_count`]**must** be `0` or `1`
/// - [`exclusive_scissor_count`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_viewports`]
/// - [`exclusive_scissor_count`]**must** be `0` or greater than or equal to the `viewportCount`
///   member of [`PipelineViewportStateCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_scissor_exclusive`]
/// - [`Rect2D`]
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
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`exclusive_scissor_count`] is the number of exclusive scissor
    ///rectangles.
    exclusive_scissor_count: u32,
    ///[`p_exclusive_scissors`] is a pointer to an array of [`Rect2D`]
    ///structures defining exclusive scissor rectangles.
    p_exclusive_scissors: *mut Rect2D,
}
