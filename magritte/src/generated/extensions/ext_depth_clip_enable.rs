use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_clip_enable");
///[VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) - Structure indicating support for explicit enable of depth clip
///# C Specifications
///The [`PhysicalDeviceDepthClipEnableFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_depth_clip_enable
///typedef struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           depthClipEnable;
///} VkPhysicalDeviceDepthClipEnableFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`depth_clip_enable`] indicates that the implementation supports setting the depth clipping
///   operation explicitly via the [`PipelineRasterizationDepthClipStateCreateInfoEXT`] pipeline
///   state. Otherwise depth clipping is only enabled when
///   [`PipelineRasterizationStateCreateInfo::depth_clamp_enable`] is set to [`FALSE`].
///If the [`PhysicalDeviceDepthClipEnableFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDepthClipEnableFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_depth_clip_enable`]
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
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`depth_clip_enable`] indicates that the
    ///implementation supports setting the depth clipping operation explicitly
    ///via the [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
    ///pipeline state.
    ///Otherwise depth clipping is only enabled when
    ///[`PipelineRasterizationStateCreateInfo`]::`depthClampEnable` is
    ///set to [`FALSE`].
    depth_clip_enable: Bool32,
}
///[VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) - Structure specifying depth clipping state
///# C Specifications
///If the [`p_next`] chain of [`PipelineRasterizationStateCreateInfo`]
///includes a [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
///structure, then that structure controls whether depth clipping is enabled or
///disabled.The [`PipelineRasterizationDepthClipStateCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_depth_clip_enable
///typedef struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    VkPipelineRasterizationDepthClipStateCreateFlagsEXT    flags;
///    VkBool32                                               depthClipEnable;
///} VkPipelineRasterizationDepthClipStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`depth_clip_enable`] controls whether depth clipping is enabled as described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_EXT_depth_clip_enable`]
/// - [`Bool32`]
/// - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
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
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    ///[`depth_clip_enable`] controls whether depth clipping is enabled as
    ///described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).
    depth_clip_enable: Bool32,
}
