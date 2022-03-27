use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_ID_SPEC_VERSION")]
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_ID_EXTENSION_NAME")]
pub const KHR_PRESENT_ID_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_present_id");
///[VkPhysicalDevicePresentIdFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html) - Structure indicating support for present id
///# C Specifications
///The [`PhysicalDevicePresentIdFeaturesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_present_id
///typedef struct VkPhysicalDevicePresentIdFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           presentId;
///} VkPhysicalDevicePresentIdFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`present_id`] indicates that the implementation supports specifying present ID values in the
///   [`PresentIdKHR`] extension to the [`PresentInfoKHR`] struct.
///If the [`PhysicalDevicePresentIdFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePresentIdFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`
///# Related
/// - [`VK_KHR_present_id`]
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
pub struct PhysicalDevicePresentIdFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`present_id`] indicates that the implementation
    ///supports specifying present ID values in the [`PresentIdKHR`]
    ///extension to the [`PresentInfoKHR`] struct.
    present_id: Bool32,
}
///[VkPresentIdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentIdKHR.html) - The list of presentation identifiers
///# C Specifications
///The [`PresentIdKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_present_id
///typedef struct VkPresentIdKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           swapchainCount;
///    const uint64_t*    pPresentIds;
///} VkPresentIdKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to the [`QueuePresentKHR`]
///   command.
/// - [`p_present_ids`] is `NULL` or a pointer to an array of uint64_t with [`swapchain_count`]
///   entries. If not `NULL`, each non-zero value in [`p_present_ids`] specifies the present id to
///   be associated with the presentation of the swapchain with the same index in the
///   [`QueuePresentKHR`] call.
///# Description
///For applications to be able to reference specific presentation events queued
///by a call to [`QueuePresentKHR`], an identifier needs to be associated
///with them.
///When the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId) feature is enabled,
///applications **can** include the [`PresentIdKHR`] structure in the
///[`p_next`] chain of the [`PresentInfoKHR`] structure to supply
///identifiers.Each [`SwapchainKHR`] has a presentId associated with it.
///This value is initially set to zero when the [`SwapchainKHR`] is
///created.When a [`PresentIdKHR`] structure with a non-NULL [`p_present_ids`] is
///included in the [`p_next`] chain of a [`PresentInfoKHR`] structure,
///each `pSwapchains` entry has a presentId associated in the
///[`p_present_ids`] array at the same index as the swapchain in the
///`pSwapchains` array.
///If this presentId is non-zero, then the application **can** later use this
///value to refer to that image presentation.
///A value of zero indicates that this presentation has no associated
///presentId.
///A non-zero presentId **must** be greater than any non-zero presentId passed
///previously by the application for the same swapchain.There is no requirement for any precise
/// timing relationship between the
///presentation of the image to the user and the update of the presentId value,
///but implementations **should** make this as close as possible to the
///presentation of the first pixel in the new image to the user.Valid Usage
/// - [`swapchain_count`]**must** be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where this [`PresentIdKHR`] is in the [`p_next`] chain of the [`PresentInfoKHR`] structure
/// - Each `presentIds` entry **must** be greater than any previous `presentIds` entry passed for
///   the associated `pSwapchains` entry
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
/// - If [`p_present_ids`] is not `NULL`, [`p_present_ids`]**must** be a valid pointer to an array
///   of [`swapchain_count`]`uint64_t` values
/// - [`swapchain_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_present_id`]
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
pub struct PresentIdKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to the
    ///[`QueuePresentKHR`] command.
    swapchain_count: u32,
    ///[`p_present_ids`] is `NULL` or a pointer to an array of uint64_t with
    ///[`swapchain_count`] entries.
    ///If not `NULL`, each non-zero value in [`p_present_ids`] specifies the
    ///present id to be associated with the presentation of the swapchain with
    ///the same index in the [`QueuePresentKHR`] call.
    p_present_ids: *mut u64,
}
