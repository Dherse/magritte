use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Buffer, DeviceSize, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_conditional_rendering");
///[VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) - Structure specifying conditional rendering begin information
///# C Specifications
///The [`ConditionalRenderingBeginInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkConditionalRenderingBeginInfoEXT {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkBuffer                          buffer;
///    VkDeviceSize                      offset;
///    VkConditionalRenderingFlagsEXT    flags;
///} VkConditionalRenderingBeginInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is a buffer containing the predicate for conditional rendering.
/// - [`offset`] is the byte offset into [`buffer`] where the predicate is located.
/// - [`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`] specifying the behavior of
///   conditional rendering.
///# Description
///If the 32-bit value at [`offset`] in [`buffer`] memory is zero, then the
///rendering commands are discarded, otherwise they are executed as normal.
///If the value of the predicate in buffer memory changes while conditional
///rendering is active, the rendering commands **may** be discarded in an
///implementation-dependent way.
///Some implementations may latch the value of the predicate upon beginning
///conditional rendering while others may read it before every rendering
///command.Valid Usage
/// - If [`buffer`] is non-sparse then it **must** be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// - [`buffer`]**must** have been created with the `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT`
///   bit set
/// - [`offset`]**must** be less than the size of [`buffer`] by at least 32 bits
/// - [`offset`]**must** be a multiple of 4
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`buffer`]**must** be a valid [`Buffer`] handle
/// - [`flags`]**must** be a valid combination of [`ConditionalRenderingFlagBitsEXT`] values
///# Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`Buffer`]
/// - [`ConditionalRenderingFlagsEXT`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`CmdBeginConditionalRenderingEXT`]
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
pub struct ConditionalRenderingBeginInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`buffer`] is a buffer containing the predicate for conditional
    ///rendering.
    buffer: Buffer,
    ///[`offset`] is the byte offset into [`buffer`] where the predicate is
    ///located.
    offset: DeviceSize,
    ///[`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`]
    ///specifying the behavior of conditional rendering.
    flags: ConditionalRenderingFlagsEXT,
}
///[VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) - Structure specifying command buffer inheritance information
///# C Specifications
///If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
///[`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure, then
///that structure controls whether a command buffer **can** be executed while
///conditional rendering is [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) in the
///primary command buffer.The [`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           conditionalRenderingEnable;
///} VkCommandBufferInheritanceConditionalRenderingInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering_enable`] specifies whether the command buffer **can** be executed
///   while conditional rendering is active in the primary command buffer. If this is [`TRUE`], then
///   this command buffer **can** be executed whether the primary command buffer has active
///   conditional rendering or not. If this is [`FALSE`], then the primary command buffer **must**
///   not have conditional rendering active.
///# Description
///If this structure is not present, the behavior is as if
///[`conditional_rendering_enable`] is [`FALSE`].Valid Usage
/// - If the [inherited conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedConditionalRendering)
///   feature is not enabled, [`conditional_rendering_enable`]**must** be [`FALSE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`
///# Related
/// - [`VK_EXT_conditional_rendering`]
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
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`conditional_rendering_enable`] specifies whether the command buffer
    ///**can** be executed while conditional rendering is active in the primary
    ///command buffer.
    ///If this is [`TRUE`], then this command buffer **can** be executed
    ///whether the primary command buffer has active conditional rendering or
    ///not.
    ///If this is [`FALSE`], then the primary command buffer **must** not
    ///have conditional rendering active.
    conditional_rendering_enable: Bool32,
}
///[VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) - Structure describing if a secondary command buffer can be executed if conditional rendering is active in the primary command buffer
///# C Specifications
///The [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           conditionalRendering;
///    VkBool32           inheritedConditionalRendering;
///} VkPhysicalDeviceConditionalRenderingFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering`] specifies whether conditional rendering is supported.
/// - [`inherited_conditional_rendering`] specifies whether a secondary command buffer **can** be
///   executed while conditional rendering is active in the primary command buffer.
///If the [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceConditionalRenderingFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
///# Related
/// - [`VK_EXT_conditional_rendering`]
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
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`conditional_rendering`] specifies
    ///whether conditional rendering is supported.
    conditional_rendering: Bool32,
    ///[`inherited_conditional_rendering`] specifies whether a secondary
    ///command buffer **can** be executed while conditional rendering is active in
    ///the primary command buffer.
    inherited_conditional_rendering: Bool32,
}
