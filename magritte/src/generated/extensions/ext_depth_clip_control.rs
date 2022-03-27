use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_clip_control");
///[VkPhysicalDeviceDepthClipControlFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html) - Structure describing additional depth clip control supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDepthClipControlFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_depth_clip_control
///typedef struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           depthClipControl;
///} VkPhysicalDeviceDepthClipControlFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceDepthClipControlFeaturesEXT`]
///structure describe the following features:
///# Description
/// - [`depth_clip_control`] indicates that the implementation supports setting
///   [`PipelineViewportDepthClipControlCreateInfoEXT::negative_one_to_one`] to [`TRUE`].
///If the [`PhysicalDeviceDepthClipControlFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDepthClipControlFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
///# Related
/// - [`VK_EXT_depth_clip_control`]
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
pub struct PhysicalDeviceDepthClipControlFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`depth_clip_control`] indicates that the
    ///implementation supports setting
    ///[`PipelineViewportDepthClipControlCreateInfoEXT`]::`negativeOneToOne`
    ///to [`TRUE`].
    depth_clip_control: Bool32,
}
///[VkPipelineViewportDepthClipControlCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) - Structure specifying parameters of a newly created pipeline depth clip control state
///# C Specifications
///The [`PipelineViewportDepthClipControlCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_depth_clip_control
///typedef struct VkPipelineViewportDepthClipControlCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           negativeOneToOne;
///} VkPipelineViewportDepthClipControlCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`negative_one_to_one`] sets the z<sub>m</sub> in the *view volume* to -w<sub>c</sub>
///# Description
///Valid Usage
/// - If [depthClipControl](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthClipControl)
///   is not enabled, [`negative_one_to_one`]**must** be [`FALSE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`
///# Related
/// - [`VK_EXT_depth_clip_control`]
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
pub struct PipelineViewportDepthClipControlCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`negative_one_to_one`] sets the z<sub>m</sub> in the *view volume* to
    ///-w<sub>c</sub>
    negative_one_to_one: Bool32,
}
