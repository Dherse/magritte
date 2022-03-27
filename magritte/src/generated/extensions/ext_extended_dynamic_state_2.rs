use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_extended_dynamic_state2");
///[VkPhysicalDeviceExtendedDynamicState2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html) - Structure describing what extended dynamic state can be used
///# C Specifications
///The [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_extended_dynamic_state2
///typedef struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           extendedDynamicState2;
///    VkBool32           extendedDynamicState2LogicOp;
///    VkBool32           extendedDynamicState2PatchControlPoints;
///} VkPhysicalDeviceExtendedDynamicState2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`extended_dynamic_state_2`] indicates that the implementation supports the following dynamic
///   states:  - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`  - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`
///   - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`
/// - [`extended_dynamic_state_2_logic_op`] indicates that the implementation supports the following
///   dynamic state:  - `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
/// - [`extended_dynamic_state_2_patch_control_points`] indicates that the implementation supports
///   the following dynamic state:  - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`
///If the [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_extended_dynamic_state2`]
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
pub struct PhysicalDeviceExtendedDynamicState2FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`extended_dynamic_state_2`] indicates
    ///that the implementation supports the following dynamic states:
    /// - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
    /// - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE`
    /// - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE`
    extended_dynamic_state_2: Bool32,
    ///[`extended_dynamic_state_2_logic_op`] indicates that the implementation
    ///supports the following dynamic state:
    /// - `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    extended_dynamic_state_2_logic_op: Bool32,
    ///[`extended_dynamic_state_2_patch_control_points`] indicates that the
    ///implementation supports the following dynamic state:
    /// - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`
    extended_dynamic_state_2_patch_control_points: Bool32,
}
