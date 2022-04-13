//![VK_KHR_device_group](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html) - device extension
//!# Description
//!This extension provides functionality to use a logical device that consists
//!of multiple physical devices, as created with the
//!`[`khr_device_group_creation`]` extension.
//!A device group can allocate memory across the subdevices, bind memory from
//!one subdevice to a resource on another subdevice, record command buffers
//!where some work executes on an arbitrary subset of the subdevices, and
//!potentially present a swapchain image from one or more subdevices.
//!# Revision
//!4
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_device_group_creation`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_device_group]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_device_group
//!   extension>>)
//!# New functions & commands
//! - [`cmd_dispatch_base_khr`]
//! - [`cmd_set_device_mask_khr`]
//! - [`get_device_group_peer_memory_features_khr`]
//!If [`khr_surface`] is supported:
//! - [`get_device_group_present_capabilities_khr`]
//! - [`get_device_group_surface_present_modes_khr`]
//! - [`get_physical_device_present_rectangles_khr`]
//!If [`khr_swapchain`] is supported:
//! - [`acquire_next_image2_khr`]
//!# New structures
//! - Extending [`BindSparseInfo`]:  - [`DeviceGroupBindSparseInfoKHR`]
//! - Extending [`CommandBufferBeginInfo`]:  - [`DeviceGroupCommandBufferBeginInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:  - [`MemoryAllocateFlagsInfoKHR`]
//! - Extending [`RenderPassBeginInfo`], [`RenderingInfo`]:  - [`DeviceGroupRenderPassBeginInfoKHR`]
//! - Extending [`SubmitInfo`]:  - [`DeviceGroupSubmitInfoKHR`]
//!If [`khr_bind_memory2`] is supported:
//! - Extending [`BindBufferMemoryInfo`]:  - [`BindBufferMemoryDeviceGroupInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImageMemoryDeviceGroupInfoKHR`]
//!If [`khr_surface`] is supported:
//! - [`DeviceGroupPresentCapabilitiesKHR`]
//!If [`khr_swapchain`] is supported:
//! - [`AcquireNextImageInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImageMemorySwapchainInfoKHR`]
//! - Extending [`ImageCreateInfo`]:  - [`ImageSwapchainCreateInfoKHR`]
//! - Extending [`PresentInfoKHR`]:  - [`DeviceGroupPresentInfoKHR`]
//! - Extending [`SwapchainCreateInfoKHR`]:  - [`DeviceGroupSwapchainCreateInfoKHR`]
//!# New enums
//! - [`MemoryAllocateFlagBitsKHR`]
//! - [`PeerMemoryFeatureFlagBitsKHR`]
//!If [`khr_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagBitsKHR`]
//!# New bitmasks
//! - [`MemoryAllocateFlagsKHR`]
//! - [`PeerMemoryFeatureFlagsKHR`]
//!If [`khr_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagsKHR`]
//!# New constants
//! - [`KHR_DEVICE_GROUP_EXTENSION_NAME`]
//! - [`KHR_DEVICE_GROUP_SPEC_VERSION`]
//! - Extending [`DependencyFlagBits`]:  - `VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR`
//! - Extending [`MemoryAllocateFlagBits`]:  - `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR`
//! - Extending [`PeerMemoryFeatureFlagBits`]:  - `VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR`  -
//!   `VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR`  - `VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR`  -
//!   `VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_DISPATCH_BASE_KHR`  -
//!   `VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR`
//!If [`khr_bind_memory2`] is supported:
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR`
//!   - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR`
//!If [`khr_surface`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
//!If [`khr_swapchain`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
//! - Extending [`SwapchainCreateFlagBitsKHR`]:  -
//!   `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//!# Version History
//! - Revision 1, 2016-10-19 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-05-19 (Tobias Hector)  - Removed extended memory bind functions to
//!   VK_KHR_bind_memory2, added dependency on that extension, and device-group-specific structs for
//!   those functions.
//! - Revision 3, 2017-10-06 (Ian Elliott)  - Corrected Vulkan 1.1 interactions with the WSI
//!   extensions. All Vulkan 1.1 WSI interactions are with the VK_KHR_swapchain extension.
//! - Revision 4, 2017-10-10 (Jeff Bolz)  - Rename “SFR” bits and structure members to use the
//!   phrase “split instance bind regions”.
//!# Other info
//! * 2017-10-10
//! * No known IP claims.
//! * - This extension requires [`SPV_KHR_device_group`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_device_group.html)
//!   - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA  - Tobias Hector, Imagination Technologies
//!# Related
//! - [`DeviceGroupBindSparseInfoKHR`]
//! - [`DeviceGroupCommandBufferBeginInfoKHR`]
//! - [`DeviceGroupRenderPassBeginInfoKHR`]
//! - [`DeviceGroupSubmitInfoKHR`]
//! - [`MemoryAllocateFlagBitsKHR`]
//! - [`MemoryAllocateFlagsInfoKHR`]
//! - [`MemoryAllocateFlagsKHR`]
//! - [`PeerMemoryFeatureFlagBitsKHR`]
//! - [`PeerMemoryFeatureFlagsKHR`]
//! - [`cmd_dispatch_base_khr`]
//! - [`cmd_set_device_mask_khr`]
//! - [`get_device_group_peer_memory_features_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::{khr_surface::SurfaceKHR, khr_swapchain::SwapchainKHR},
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Device, Fence, Instance, PhysicalDevice, Rect2D, Semaphore, StructureType,
        VulkanResultCodes,
    },
    vulkan1_1::{FNCmdDispatchBase, FNCmdSetDeviceMask, FNGetDeviceGroupPeerMemoryFeatures, MAX_DEVICE_GROUP_SIZE},
    AsRaw, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_device_group");
///[vkGetDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) - Query present capabilities from other physical devices
///# C Specifications
///A logical device that represents multiple physical devices  **may**  support
///presenting from images on more than one physical device, or combining images
///from multiple physical devices.To query these capabilities, call:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///VkResult vkGetDeviceGroupPresentCapabilitiesKHR(
///    VkDevice                                    device,
///    VkDeviceGroupPresentCapabilitiesKHR*        pDeviceGroupPresentCapabilities);
///```
///# Parameters
/// - [`device`] is the logical device.
/// - [`p_device_group_present_capabilities`] is a pointer to a
///   [`DeviceGroupPresentCapabilitiesKHR`] structure in which the device’s capabilities are
///   returned.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_device_group_present_capabilities`] **must**  be a valid pointer to a
///   [`DeviceGroupPresentCapabilitiesKHR`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`Device`]
/// - [`DeviceGroupPresentCapabilitiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
pub type FNGetDeviceGroupPresentCapabilitiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetDeviceGroupSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) - Query present capabilities for a surface
///# C Specifications
///Some surfaces  **may**  not be capable of using all the device group present
///modes.To query the supported device group present modes for a particular surface,
///call:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///VkResult vkGetDeviceGroupSurfacePresentModesKHR(
///    VkDevice                                    device,
///    VkSurfaceKHR                                surface,
///    VkDeviceGroupPresentModeFlagsKHR*           pModes);
///```
///# Parameters
/// - [`device`] is the logical device.
/// - [`surface`] is the surface.
/// - [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported device
///   group present modes for the surface are returned.
///# Description
///The modes returned by this command are not invariant, and  **may**  change in
///response to the surface being moved, resized, or occluded.
///These modes  **must**  be a subset of the modes returned by
///[`get_device_group_present_capabilities_khr`].
///## Valid Usage
/// - [`surface`] **must**  be supported by all physical devices associated with [`device`], as
///   reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific
///   mechanism
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value
/// - Both of [`device`], and [`surface`] **must**  have been created, allocated, or retrieved from
///   the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`surface`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`Device`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
pub type FNGetDeviceGroupSurfacePresentModesKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VulkanResultCodes,
>;
///[vkAcquireNextImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html) - Retrieve the index of the next available presentable image
///# C Specifications
///To acquire an available presentable image to use, and retrieve the index of
///that image, call:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///VkResult vkAcquireNextImage2KHR(
///    VkDevice                                    device,
///    const VkAcquireNextImageInfoKHR*            pAcquireInfo,
///    uint32_t*                                   pImageIndex);
///```
///# Parameters
/// - [`device`] is the device associated with `swapchain`.
/// - [`p_acquire_info`] is a pointer to a [`AcquireNextImageInfoKHR`] structure containing
///   parameters of the acquire.
/// - [`p_image_index`] is a pointer to a `uint32_t` that is set to the index of the next image to
///   use.
///# Description
///## Valid Usage
/// - If the number of currently acquired images is greater than the difference between the number
///   of images in the `swapchain` member of [`p_acquire_info`] and the value of
///   [`SurfaceCapabilitiesKHR::min_image_count`] as returned by a call to
///   [`get_physical_device_surface_capabilities2_khr`] with the `surface` used to create
///   `swapchain`, the `timeout` member of [`p_acquire_info`] **must**  not be `UINT64_MAX`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_acquire_info`] **must**  be a valid pointer to a valid [`AcquireNextImageInfoKHR`]
///   structure
/// - [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`AcquireNextImageInfoKHR`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquireNextImage2KHR")]
pub type FNAcquireNextImage2Khr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR<'lt>,
        p_image_index: *mut u32,
    ) -> VulkanResultCodes,
>;
///[vkGetPhysicalDevicePresentRectanglesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) - Query present rectangles for a surface on a physical device
///# C Specifications
///When using `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`,
///the application  **may**  need to know which regions of the surface are used when
///presenting locally on each physical device.
///Presentation of swapchain images to this surface need only have valid
///contents in the regions returned by this command.To query a set of rectangles used in
/// presentation on the physical device,
///call:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///VkResult vkGetPhysicalDevicePresentRectanglesKHR(
///    VkPhysicalDevice                            physicalDevice,
///    VkSurfaceKHR                                surface,
///    uint32_t*                                   pRectCount,
///    VkRect2D*                                   pRects);
///```
///# Parameters
/// - [`physical_device`] is the physical device.
/// - [`surface`] is the surface.
/// - [`p_rect_count`] is a pointer to an integer related to the number of rectangles available or
///   queried, as described below.
/// - [`p_rects`] is either `NULL` or a pointer to an array of [`Rect2D`] structures.
///# Description
///If [`p_rects`] is `NULL`, then the number of rectangles used when
///presenting the given [`surface`] is returned in [`p_rect_count`].
///Otherwise, [`p_rect_count`] **must**  point to a variable set by the user to the
///number of elements in the [`p_rects`] array, and on return the variable is
///overwritten with the number of structures actually written to [`p_rects`].
///If the value of [`p_rect_count`] is less than the number of rectangles, at
///most [`p_rect_count`] structures will be written, and `VK_INCOMPLETE`
///will be returned instead of `VK_SUCCESS`, to indicate that not all the
///available rectangles were returned.The values returned by this command are not invariant, and
/// **may**  change in
///response to the surface being moved, resized, or occluded.The rectangles returned by this
/// command  **must**  not overlap.
///## Valid Usage
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`surface`] **must**  be supported by [`physical_device`], as reported by
///   [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism
///
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`p_rect_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_rect_count`] is not `0`, and [`p_rects`] is not `NULL`,
///   [`p_rects`] **must**  be a valid pointer to an array of [`p_rect_count`][`Rect2D`] structures
/// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
///   retrieved from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`surface`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDevice`]
/// - [`Rect2D`]
/// - [`SurfaceKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
pub type FNGetPhysicalDevicePresentRectanglesKhr = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
    ) -> VulkanResultCodes,
>;
///[VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) - Bitmask specifying supported device group present modes
///# C Specifications
///Bits which  **may**  be set in
///[`DeviceGroupPresentCapabilitiesKHR::modes`], indicating which
///device group presentation modes are supported, are:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef enum VkDeviceGroupPresentModeFlagBitsKHR {
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 0x00000001,
///    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 0x00000002,
///    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 0x00000004,
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
///} VkDeviceGroupPresentModeFlagBitsKHR;
///```
///# Description
/// - [`LOCAL`] specifies that any physical device with a presentation engine  **can**  present its
///   own swapchain images.
/// - [`REMOTE`] specifies that any physical device with a presentation engine  **can**  present
///   swapchain images from any physical device in its `presentMask`.
/// - [`SUM`] specifies that any physical device with a presentation engine  **can**  present the
///   sum of swapchain images from any physical devices in its `presentMask`.
/// - [`LOCAL_MULTI_DEVICE`] specifies that multiple physical devices with a presentation engine
///   **can**  each present their own swapchain images.
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentInfoKHR`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagBitsKHR(u32);
impl const Default for DeviceGroupPresentModeFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl DeviceGroupPresentModeFlagBitsKHR {
    ///[`LOCAL`] specifies that any
    ///physical device with a presentation engine  **can**  present its own
    ///swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    ///[`REMOTE`] specifies that any
    ///physical device with a presentation engine  **can**  present swapchain images
    ///from any physical device in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    ///[`SUM`] specifies that any
    ///physical device with a presentation engine  **can**  present the sum of
    ///swapchain images from any physical devices in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    ///[`LOCAL_MULTI_DEVICE`] specifies
    ///that multiple physical devices with a presentation engine  **can**  each
    ///present their own swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DeviceGroupPresentModeFlagBitsKHR))
            .field(match *self {
                #[cfg(feature = "VK_KHR_swapchain")]
                Self::LOCAL => &"LOCAL",
                #[cfg(feature = "VK_KHR_swapchain")]
                Self::REMOTE => &"REMOTE",
                #[cfg(feature = "VK_KHR_swapchain")]
                Self::SUM => &"SUM",
                #[cfg(feature = "VK_KHR_swapchain")]
                Self::LOCAL_MULTI_DEVICE => &"LOCAL_MULTI_DEVICE",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(DeviceGroupPresentModeFlagBitsKHR),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for DeviceGroupPresentModeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::LOCAL => &"LOCAL",
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::REMOTE => &"REMOTE",
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::SUM => &"SUM",
            #[cfg(feature = "VK_KHR_swapchain")]
            Self::LOCAL_MULTI_DEVICE => &"LOCAL_MULTI_DEVICE",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(DeviceGroupPresentModeFlagBitsKHR),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) - Bitmask specifying supported device group present modes
///# C Specifications
///Bits which  **may**  be set in
///[`DeviceGroupPresentCapabilitiesKHR::modes`], indicating which
///device group presentation modes are supported, are:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef enum VkDeviceGroupPresentModeFlagBitsKHR {
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 0x00000001,
///    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 0x00000002,
///    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 0x00000004,
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
///} VkDeviceGroupPresentModeFlagBitsKHR;
///```
///# Description
/// - [`LOCAL`] specifies that any physical device with a presentation engine  **can**  present its
///   own swapchain images.
/// - [`REMOTE`] specifies that any physical device with a presentation engine  **can**  present
///   swapchain images from any physical device in its `presentMask`.
/// - [`SUM`] specifies that any physical device with a presentation engine  **can**  present the
///   sum of swapchain images from any physical devices in its `presentMask`.
/// - [`LOCAL_MULTI_DEVICE`] specifies that multiple physical devices with a presentation engine
///   **can**  each present their own swapchain images.
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentInfoKHR`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagsKHR(u32);
impl const Default for DeviceGroupPresentModeFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from(from: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl DeviceGroupPresentModeFlagsKHR {
    ///[`LOCAL`] specifies that any
    ///physical device with a presentation engine  **can**  present its own
    ///swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    ///[`REMOTE`] specifies that any
    ///physical device with a presentation engine  **can**  present swapchain images
    ///from any physical device in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    ///[`SUM`] specifies that any
    ///physical device with a presentation engine  **can**  present the sum of
    ///swapchain images from any physical devices in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    ///[`LOCAL_MULTI_DEVICE`] specifies
    ///that multiple physical devices with a presentation engine  **can**  each
    ///present their own swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::REMOTE;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::SUM;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL_MULTI_DEVICE;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DeviceGroupPresentModeFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceGroupPresentModeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceGroupPresentModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(LOCAL))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::REMOTE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(REMOTE))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::SUM) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUM))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL_MULTI_DEVICE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(LOCAL_MULTI_DEVICE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceGroupPresentModeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) - Present capabilities from other physical devices
///# C Specifications
///The [`DeviceGroupPresentCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef struct VkDeviceGroupPresentCapabilitiesKHR {
///    VkStructureType                     sType;
///    void*                               pNext;
///    uint32_t                            presentMask[VK_MAX_DEVICE_GROUP_SIZE];
///    VkDeviceGroupPresentModeFlagsKHR    modes;
///} VkDeviceGroupPresentCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`present_mask`] is an array of [`MAX_DEVICE_GROUP_SIZE`]`uint32_t` masks, where the mask at
///   element i is non-zero if physical device i has a presentation engine, and where bit j is set
///   in element i if physical device i **can**  present swapchain images from physical device j. If
///   element i is non-zero, then bit i **must**  be set.
/// - [`modes`] is a bitmask of [`DeviceGroupPresentModeFlagBitsKHR`] indicating which device group
///   presentation modes are supported.
///# Description
///[`modes`] always has `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR` set.The present mode flags are
/// also used when presenting an image, in
///[`DeviceGroupPresentInfoKHR::mode`].If a device group only includes a single physical device,
/// then [`modes`] **must**  equal `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`khr_device_group`]
/// - [`khr_surface`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`StructureType`]
/// - [`get_device_group_present_capabilities_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`present_mask`] is an array of [`MAX_DEVICE_GROUP_SIZE`]`uint32_t` masks, where the mask at
    /// element i is non-zero if physical device i has a presentation engine, and where bit j
    ///is set in element i if physical device i **can**  present
    ///swapchain images from physical device j.
    ///If element i is non-zero, then bit i **must**  be set.
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    ///[`modes`] is a bitmask of [`DeviceGroupPresentModeFlagBitsKHR`]
    ///indicating which device group presentation modes are supported.
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl<'lt> Default for DeviceGroupPresentCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            present_mask: [0; MAX_DEVICE_GROUP_SIZE as usize],
            modes: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupPresentCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::present_mask`]
    pub fn present_mask(&self) -> &[u32; MAX_DEVICE_GROUP_SIZE as usize] {
        &self.present_mask
    }
    ///Gets the value of [`Self::modes`]
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::present_mask`]
    pub fn present_mask_mut(&mut self) -> &mut [u32; MAX_DEVICE_GROUP_SIZE as usize] {
        &mut self.present_mask
    }
    ///Gets a mutable reference to the value of [`Self::modes`]
    pub fn modes_mut(&mut self) -> &mut DeviceGroupPresentModeFlagsKHR {
        &mut self.modes
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::present_mask`]
    pub fn set_present_mask(mut self, value: [u32; crate::vulkan1_1::MAX_DEVICE_GROUP_SIZE as usize]) -> Self {
        self.present_mask = value;
        self
    }
    ///Sets the value of [`Self::modes`]
    pub fn set_modes(mut self, value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.modes = value;
        self
    }
}
///[VkImageSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) - Specify that an image will be bound to swapchain memory
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageSwapchainCreateInfoKHR`] structure, then that structure includes
///a swapchain handle indicating that the image will be bound to memory from
///that swapchain.The [`ImageSwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkImageSwapchainCreateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///} VkImageSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is [`crate::Handle::null`] or a handle of a swapchain that the image will be
///   bound to.
///# Description
///## Valid Usage
/// -    If [`swapchain`] is not [`crate::Handle::null`], the fields of [`ImageCreateInfo`] **must**  match the [implied image creation parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#swapchain-wsi-image-create-info) of the swapchain
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
/// - If [`swapchain`] is not [`crate::Handle::null`], [`swapchain`] **must**  be a valid
///   [`SwapchainKHR`] handle
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is [`crate::Handle::null`] or a handle of a swapchain that
    ///the image will be bound to.
    pub swapchain: SwapchainKHR,
}
impl<'lt> Default for ImageSwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
        }
    }
}
impl<'lt> ImageSwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain`]
    pub fn set_swapchain(mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.swapchain = value;
        self
    }
}
///[VkBindImageMemorySwapchainInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) - Structure specifying swapchain image memory to bind to
///# C Specifications
///If the [`p_next`] chain of [`BindImageMemoryInfo`] includes a
///[`BindImageMemorySwapchainInfoKHR`] structure, then that structure
///includes a swapchain handle and image index indicating that the image will
///be bound to memory from that swapchain.The [`BindImageMemorySwapchainInfoKHR`] structure is
/// defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkBindImageMemorySwapchainInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///    uint32_t           imageIndex;
///} VkBindImageMemorySwapchainInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is [`crate::Handle::null`] or a swapchain handle.
/// - [`image_index`] is an image index within [`swapchain`].
///# Description
///If [`swapchain`] is not `NULL`, the [`swapchain`] and [`image_index`]
///are used to determine the memory that the image is bound to, instead of
///`memory` and `memoryOffset`.Memory  **can**  be bound to a swapchain and use the
/// `pDeviceIndices` or
///`pSplitInstanceBindRegions` members of
///[`BindImageMemoryDeviceGroupInfo`].
///## Valid Usage
/// - [`image_index`] **must**  be less than the number of images in [`swapchain`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is [`crate::Handle::null`] or a swapchain handle.
    pub swapchain: SwapchainKHR,
    ///[`image_index`] is an image index within [`swapchain`].
    pub image_index: u32,
}
impl<'lt> Default for BindImageMemorySwapchainInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            image_index: 0,
        }
    }
}
impl<'lt> BindImageMemorySwapchainInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets the value of [`Self::image_index`]
    pub fn image_index(&self) -> u32 {
        self.image_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::image_index`]
    pub fn image_index_mut(&mut self) -> &mut u32 {
        &mut self.image_index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain`]
    pub fn set_swapchain(mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.swapchain = value;
        self
    }
    ///Sets the value of [`Self::image_index`]
    pub fn set_image_index(mut self, value: u32) -> Self {
        self.image_index = value;
        self
    }
}
///[VkAcquireNextImageInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html) - Structure specifying parameters of the acquire
///# C Specifications
///The [`AcquireNextImageInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkAcquireNextImageInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///    uint64_t           timeout;
///    VkSemaphore        semaphore;
///    VkFence            fence;
///    uint32_t           deviceMask;
///} VkAcquireNextImageInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is a non-retired swapchain from which an image is acquired.
/// - [`timeout`] specifies how long the function waits, in nanoseconds, if no image is available.
/// - [`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
/// - [`fence`] is [`crate::Handle::null`] or a fence to signal.
/// - [`device_mask`] is a mask of physical devices for which the swapchain image will be ready to
///   use when the semaphore or fence is signaled.
///# Description
///If [`acquire_next_image_khr`] is used, the device mask is considered to
///include all physical devices in the logical device.
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  be unsignaled
/// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  not have any uncompleted signal
///   or wait operations pending
/// - If [`fence`] is not [`crate::Handle::null`] it  **must**  be unsignaled and  **must**  not be
///   associated with any other queue command that has not yet completed execution on that queue
/// - [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::Handle::null`]
/// - [`device_mask`] **must**  be a valid device mask
/// - [`device_mask`] **must**  not be zero
/// - [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - If [`semaphore`] is not [`crate::Handle::null`], [`semaphore`] **must**  be a valid
///   [`Semaphore`] handle
/// - If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
/// - Each of [`fence`], [`semaphore`], and [`swapchain`] that are valid handles of non-ignored
///   parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
/// - Host access to [`semaphore`] **must**  be externally synchronized
/// - Host access to [`fence`] **must**  be externally synchronized
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`Fence`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
/// - [`acquire_next_image2_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is a non-retired swapchain from which an image is
    ///acquired.
    pub swapchain: SwapchainKHR,
    ///[`timeout`] specifies how long the function waits, in nanoseconds, if
    ///no image is available.
    pub timeout: u64,
    ///[`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
    pub semaphore: Semaphore,
    ///[`fence`] is [`crate::Handle::null`] or a fence to signal.
    pub fence: Fence,
    ///[`device_mask`] is a mask of physical devices for which the swapchain
    ///image will be ready to use when the semaphore or fence is signaled.
    pub device_mask: u32,
}
impl<'lt> Default for AcquireNextImageInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            timeout: 0,
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: 0,
        }
    }
}
impl<'lt> AcquireNextImageInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets the value of [`Self::timeout`]
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::fence`]
    pub fn fence(&self) -> Fence {
        self.fence
    }
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::timeout`]
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut self.timeout
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::fence`]
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain`]
    pub fn set_swapchain(mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.swapchain = value;
        self
    }
    ///Sets the value of [`Self::timeout`]
    pub fn set_timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }
    ///Sets the value of [`Self::semaphore`]
    pub fn set_semaphore(mut self, value: crate::vulkan1_0::Semaphore) -> Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::fence`]
    pub fn set_fence(mut self, value: crate::vulkan1_0::Fence) -> Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::device_mask`]
    pub fn set_device_mask(mut self, value: u32) -> Self {
        self.device_mask = value;
        self
    }
}
///[VkDeviceGroupPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) - Mode and mask controlling which physical devices' images are presented
///# C Specifications
///If the [`p_next`] chain of [`PresentInfoKHR`] includes a
///[`DeviceGroupPresentInfoKHR`] structure, then that structure includes an
///array of device masks and a device group present mode.The [`DeviceGroupPresentInfoKHR`]
/// structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkDeviceGroupPresentInfoKHR {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    uint32_t                               swapchainCount;
///    const uint32_t*                        pDeviceMasks;
///    VkDeviceGroupPresentModeFlagBitsKHR    mode;
///} VkDeviceGroupPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is zero or the number of elements in [`device_masks`].
/// - [`device_masks`] is a pointer to an array of device masks, one for each element of
///   [`PresentInfoKHR`]::pSwapchains.
/// - [`mode`] is a [`DeviceGroupPresentModeFlagBitsKHR`] value specifying the device group present
///   mode that will be used for this present.
///# Description
///If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each
///element of [`device_masks`] selects which instance of the swapchain image
///is presented.
///Each element of [`device_masks`] **must**  have exactly one bit set, and the
///corresponding physical device  **must**  have a presentation engine as reported
///by [`DeviceGroupPresentCapabilitiesKHR`].If [`mode`] is
/// `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then
///each element of [`device_masks`] selects which instance of the swapchain
///image is presented.
///Each element of [`device_masks`] **must**  have exactly one bit set, and some
///physical device in the logical device  **must**  include that bit in its
///[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is
/// `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each
///element of [`device_masks`] selects which instances of the swapchain image
///are component-wise summed and the sum of those images is presented.
///If the sum in any component is outside the representable range, the value of
///that component is undefined.
///Each element of [`device_masks`] **must**  have a value for which all set bits
///are set in one of the elements of
///[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is
///`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then each
///element of [`device_masks`] selects which instance(s) of the swapchain
///images are presented.
///For each bit set in each element of [`device_masks`], the corresponding
///physical device  **must**  have a presentation engine as reported by
///[`DeviceGroupPresentCapabilitiesKHR`].If [`DeviceGroupPresentInfoKHR`] is not provided or
/// [`swapchain_count`]
///is zero then the masks are considered to be `1`.
///If [`DeviceGroupPresentInfoKHR`] is not provided, [`mode`] is
///considered to be `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage
/// - [`swapchain_count`] **must**  equal `0` or [`PresentInfoKHR`]::[`swapchain_count`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have exactly one bit set, and the corresponding element of
///   [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have exactly one bit set, and some physical device in the logical
///   device  **must**  include that bit in its [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have a value for which all set bits are set in one of the elements
///   of [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then for each bit
///   set in each element of [`device_masks`], the corresponding element of
///   [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
/// - The value of each element of [`device_masks`] **must**  be equal to the device mask passed in
///   [`AcquireNextImageInfoKHR::device_mask`] when the image index was last acquired
/// - [`mode`] **must**  have exactly one bit set, and that bit  **must**  have been included in
///   [`DeviceGroupSwapchainCreateInfoKHR::modes`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
/// - If [`swapchain_count`] is not `0`, [`device_masks`] **must**  be a valid pointer to an array
///   of [`swapchain_count`]`uint32_t` values
/// - [`mode`] **must**  be a valid [`DeviceGroupPresentModeFlagBitsKHR`] value
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagBitsKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain_count`] is zero or the number of elements in
    ///[`device_masks`].
    pub swapchain_count: u32,
    ///[`device_masks`] is a pointer to an array of device masks, one for
    ///each element of [`PresentInfoKHR`]::pSwapchains.
    pub device_masks: *const u32,
    ///[`mode`] is a [`DeviceGroupPresentModeFlagBitsKHR`] value
    ///specifying the device group present mode that will be used for this
    ///present.
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
}
impl<'lt> Default for DeviceGroupPresentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            swapchain_count: 0,
            device_masks: std::ptr::null(),
            mode: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupPresentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_masks`]
    pub fn device_masks_raw(&self) -> *const u32 {
        self.device_masks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_masks`]
    pub fn set_device_masks_raw(mut self, value: *const u32) -> Self {
        self.device_masks = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::device_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.device_masks, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> DeviceGroupPresentModeFlagBitsKHR {
        self.mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut self.swapchain_count
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut DeviceGroupPresentModeFlagBitsKHR {
        &mut self.mode
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(mut self, value: u32) -> Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::device_masks`]
    pub fn set_device_masks(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_masks = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the value of [`Self::mode`]
    pub fn set_mode(mut self, value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagBitsKHR) -> Self {
        self.mode = value;
        self
    }
}
///[VkDeviceGroupSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) - Structure specifying parameters of a newly created swapchain object
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`DeviceGroupSwapchainCreateInfoKHR`] structure, then that structure
///includes a set of device group present modes that the swapchain  **can**  be used
///with.The [`DeviceGroupSwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkDeviceGroupSwapchainCreateInfoKHR {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkDeviceGroupPresentModeFlagsKHR    modes;
///} VkDeviceGroupSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`modes`] is a bitfield of modes that the swapchain  **can**  be used with.
///# Description
///If this structure is not present, [`modes`] is considered to be
///`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
/// - [`modes`] **must**  be a valid combination of [`DeviceGroupPresentModeFlagBitsKHR`] values
/// - [`modes`] **must**  not be `0`
///# Related
/// - [`khr_device_group`]
/// - [`khr_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`modes`] is a bitfield of modes that the swapchain  **can**  be used with.
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl<'lt> Default for DeviceGroupSwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            modes: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupSwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::modes`]
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::modes`]
    pub fn modes_mut(&mut self) -> &mut DeviceGroupPresentModeFlagsKHR {
        &mut self.modes
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::modes`]
    pub fn set_modes(mut self, value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.modes = value;
        self
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDevicePresentRectanglesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html) - Query present rectangles for a surface on a physical device
    ///# C Specifications
    ///When using `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`,
    ///the application  **may**  need to know which regions of the surface are used when
    ///presenting locally on each physical device.
    ///Presentation of swapchain images to this surface need only have valid
    ///contents in the regions returned by this command.To query a set of rectangles used in
    /// presentation on the physical device,
    ///call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
    ///VkResult vkGetPhysicalDevicePresentRectanglesKHR(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkSurfaceKHR                                surface,
    ///    uint32_t*                                   pRectCount,
    ///    VkRect2D*                                   pRects);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device.
    /// - [`surface`] is the surface.
    /// - [`p_rect_count`] is a pointer to an integer related to the number of rectangles available
    ///   or queried, as described below.
    /// - [`p_rects`] is either `NULL` or a pointer to an array of [`Rect2D`] structures.
    ///# Description
    ///If [`p_rects`] is `NULL`, then the number of rectangles used when
    ///presenting the given [`surface`] is returned in [`p_rect_count`].
    ///Otherwise, [`p_rect_count`] **must**  point to a variable set by the user to the
    ///number of elements in the [`p_rects`] array, and on return the variable is
    ///overwritten with the number of structures actually written to [`p_rects`].
    ///If the value of [`p_rect_count`] is less than the number of rectangles, at
    ///most [`p_rect_count`] structures will be written, and `VK_INCOMPLETE`
    ///will be returned instead of `VK_SUCCESS`, to indicate that not all the
    ///available rectangles were returned.The values returned by this command are not invariant,
    /// and  **may**  change in
    ///response to the surface being moved, resized, or occluded.The rectangles returned by this
    /// command  **must**  not overlap.
    ///## Valid Usage
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`surface`] **must**  be supported by [`physical_device`], as reported by
    ///   [`get_physical_device_surface_support_khr`] or an equivalent platform-specific mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`p_rect_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_rect_count`] is not `0`, and [`p_rects`] is not `NULL`,
    ///   [`p_rects`] **must**  be a valid pointer to an array of [`p_rect_count`][`Rect2D`]
    ///   structures
    /// - Both of [`physical_device`], and [`surface`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`surface`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`khr_device_group`]
    /// - [`khr_surface`]
    /// - [`khr_swapchain`]
    /// - [`crate::vulkan1_1`]
    /// - [`PhysicalDevice`]
    /// - [`Rect2D`]
    /// - [`SurfaceKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_present_rectangles_khr(
        self: &Unique<PhysicalDevice>,
        surface: SurfaceKHR,
        p_rect_count: Option<usize>,
    ) -> VulkanResult<SmallVec<Rect2D>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_physical_device_present_rectangles_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_physical_device_present_rectangles_khr())
            .unwrap_unchecked();
        let mut p_rect_count = match p_rect_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), surface, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_rects = SmallVec::<Rect2D>::from_elem(Default::default(), p_rect_count as usize);
        let _return = _function(self.as_raw(), surface, &mut p_rect_count, p_rects.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_rects),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html) - Query present capabilities from other physical devices
    ///# C Specifications
    ///A logical device that represents multiple physical devices  **may**  support
    ///presenting from images on more than one physical device, or combining images
    ///from multiple physical devices.To query these capabilities, call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
    ///VkResult vkGetDeviceGroupPresentCapabilitiesKHR(
    ///    VkDevice                                    device,
    ///    VkDeviceGroupPresentCapabilitiesKHR*        pDeviceGroupPresentCapabilities);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device.
    /// - [`p_device_group_present_capabilities`] is a pointer to a
    ///   [`DeviceGroupPresentCapabilitiesKHR`] structure in which the device’s capabilities are
    ///   returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_device_group_present_capabilities`] **must**  be a valid pointer to a
    ///   [`DeviceGroupPresentCapabilitiesKHR`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`khr_device_group`]
    /// - [`khr_surface`]
    /// - [`khr_swapchain`]
    /// - [`crate::vulkan1_1`]
    /// - [`Device`]
    /// - [`DeviceGroupPresentCapabilitiesKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_device_group_present_capabilities_khr<'lt>(
        self: &Unique<Device>,
        p_device_group_present_capabilities: Option<DeviceGroupPresentCapabilitiesKHR<'lt>>,
    ) -> VulkanResult<DeviceGroupPresentCapabilitiesKHR<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_device_group_present_capabilities_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_device_group_present_capabilities_khr())
            .unwrap_unchecked();
        let mut p_device_group_present_capabilities = p_device_group_present_capabilities.unwrap_or_default();
        let _return = _function(self.as_raw(), &mut p_device_group_present_capabilities);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_device_group_present_capabilities.p_next = std::ptr::null_mut();
                p_device_group_present_capabilities
            }),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetDeviceGroupSurfacePresentModesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html) - Query present capabilities for a surface
    ///# C Specifications
    ///Some surfaces  **may**  not be capable of using all the device group present
    ///modes.To query the supported device group present modes for a particular surface,
    ///call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
    ///VkResult vkGetDeviceGroupSurfacePresentModesKHR(
    ///    VkDevice                                    device,
    ///    VkSurfaceKHR                                surface,
    ///    VkDeviceGroupPresentModeFlagsKHR*           pModes);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device.
    /// - [`surface`] is the surface.
    /// - [`p_modes`] is a pointer to a [`DeviceGroupPresentModeFlagsKHR`] in which the supported
    ///   device group present modes for the surface are returned.
    ///# Description
    ///The modes returned by this command are not invariant, and  **may**  change in
    ///response to the surface being moved, resized, or occluded.
    ///These modes  **must**  be a subset of the modes returned by
    ///[`get_device_group_present_capabilities_khr`].
    ///## Valid Usage
    /// - [`surface`] **must**  be supported by all physical devices associated with [`device`], as
    ///   reported by [`get_physical_device_surface_support_khr`] or an equivalent platform-specific
    ///   mechanism
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
    /// - [`p_modes`] **must**  be a valid pointer to a [`DeviceGroupPresentModeFlagsKHR`] value
    /// - Both of [`device`], and [`surface`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`surface`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`khr_device_group`]
    /// - [`khr_surface`]
    /// - [`khr_swapchain`]
    /// - [`crate::vulkan1_1`]
    /// - [`Device`]
    /// - [`DeviceGroupPresentModeFlagsKHR`]
    /// - [`SurfaceKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes_khr(
        self: &Unique<Device>,
        surface: SurfaceKHR,
        p_modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_device_group_surface_present_modes_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.get_device_group_surface_present_modes_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), surface, p_modes as *mut DeviceGroupPresentModeFlagsKHR);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkAcquireNextImage2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html) - Retrieve the index of the next available presentable image
    ///# C Specifications
    ///To acquire an available presentable image to use, and retrieve the index of
    ///that image, call:
    ///```c
    ///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with
    ///// VK_KHR_swapchain
    ///VkResult vkAcquireNextImage2KHR(
    ///    VkDevice                                    device,
    ///    const VkAcquireNextImageInfoKHR*            pAcquireInfo,
    ///    uint32_t*                                   pImageIndex);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with `swapchain`.
    /// - [`p_acquire_info`] is a pointer to a [`AcquireNextImageInfoKHR`] structure containing
    ///   parameters of the acquire.
    /// - [`p_image_index`] is a pointer to a `uint32_t` that is set to the index of the next image
    ///   to use.
    ///# Description
    ///## Valid Usage
    /// - If the number of currently acquired images is greater than the difference between the
    ///   number of images in the `swapchain` member of [`p_acquire_info`] and the value of
    ///   [`SurfaceCapabilitiesKHR::min_image_count`] as returned by a call to
    ///   [`get_physical_device_surface_capabilities2_khr`] with the `surface` used to create
    ///   `swapchain`, the `timeout` member of [`p_acquire_info`] **must**  not be `UINT64_MAX`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_acquire_info`] **must**  be a valid pointer to a valid [`AcquireNextImageInfoKHR`]
    ///   structure
    /// - [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
    ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///# Related
    /// - [`khr_device_group`]
    /// - [`khr_swapchain`]
    /// - [`crate::vulkan1_1`]
    /// - [`AcquireNextImageInfoKHR`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkAcquireNextImage2KHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_next_image2_khr<'lt>(
        self: &Unique<Device>,
        p_acquire_info: &AcquireNextImageInfoKHR<'lt>,
    ) -> VulkanResult<u32> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.acquire_next_image2_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_device_group()
            .and_then(|vtable| vtable.acquire_next_image2_khr())
            .unwrap_unchecked();
        let mut p_image_index = Default::default();
        let _return = _function(
            self.as_raw(),
            p_acquire_info as *const AcquireNextImageInfoKHR<'lt>,
            &mut p_image_index,
        );
        match _return {
            VulkanResultCodes::SUCCESS
            | VulkanResultCodes::TIMEOUT
            | VulkanResultCodes::NOT_READY
            | VulkanResultCodes::SUBOPTIMAL_KHR => VulkanResult::Success(_return, p_image_index),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_KHR_device_group`
pub struct InstanceKhrDeviceGroupVTable {
    ///See [`FNGetPhysicalDevicePresentRectanglesKhr`] for more information.
    pub get_physical_device_present_rectangles_khr: FNGetPhysicalDevicePresentRectanglesKhr,
}
impl InstanceKhrDeviceGroupVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_present_rectangles_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDevicePresentRectanglesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_present_rectangles_khr`]. See
    /// [`FNGetPhysicalDevicePresentRectanglesKhr`] for more information.
    pub fn get_physical_device_present_rectangles_khr(&self) -> FNGetPhysicalDevicePresentRectanglesKhr {
        self.get_physical_device_present_rectangles_khr
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_device_group`
pub struct DeviceKhrDeviceGroupVTable {
    ///See [`FNGetDeviceGroupPresentCapabilitiesKhr`] for more information.
    pub get_device_group_present_capabilities_khr: FNGetDeviceGroupPresentCapabilitiesKhr,
    ///See [`FNGetDeviceGroupSurfacePresentModesKhr`] for more information.
    pub get_device_group_surface_present_modes_khr: FNGetDeviceGroupSurfacePresentModesKhr,
    ///See [`FNAcquireNextImage2Khr`] for more information.
    pub acquire_next_image2_khr: FNAcquireNextImage2Khr,
    ///See [`FNGetDeviceGroupPeerMemoryFeatures`] for more information.
    pub get_device_group_peer_memory_features_khr: FNGetDeviceGroupPeerMemoryFeatures,
    ///See [`FNCmdSetDeviceMask`] for more information.
    pub cmd_set_device_mask_khr: FNCmdSetDeviceMask,
    ///See [`FNCmdDispatchBase`] for more information.
    pub cmd_dispatch_base_khr: FNCmdDispatchBase,
}
impl DeviceKhrDeviceGroupVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_device_group_present_capabilities_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceGroupPresentCapabilitiesKHR").as_ptr(),
                ))
            },
            get_device_group_surface_present_modes_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceGroupSurfacePresentModesKHR").as_ptr(),
                ))
            },
            acquire_next_image2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireNextImage2KHR").as_ptr()))
            },
            get_device_group_peer_memory_features_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceGroupPeerMemoryFeaturesKHR").as_ptr(),
                ))
            },
            cmd_set_device_mask_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetDeviceMaskKHR").as_ptr()))
            },
            cmd_dispatch_base_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdDispatchBaseKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_device_group_present_capabilities_khr`]. See
    /// [`FNGetDeviceGroupPresentCapabilitiesKhr`] for more information.
    pub fn get_device_group_present_capabilities_khr(&self) -> FNGetDeviceGroupPresentCapabilitiesKhr {
        self.get_device_group_present_capabilities_khr
    }
    ///Gets [`Self::get_device_group_surface_present_modes_khr`]. See
    /// [`FNGetDeviceGroupSurfacePresentModesKhr`] for more information.
    pub fn get_device_group_surface_present_modes_khr(&self) -> FNGetDeviceGroupSurfacePresentModesKhr {
        self.get_device_group_surface_present_modes_khr
    }
    ///Gets [`Self::acquire_next_image2_khr`]. See [`FNAcquireNextImage2Khr`] for more information.
    pub fn acquire_next_image2_khr(&self) -> FNAcquireNextImage2Khr {
        self.acquire_next_image2_khr
    }
    ///Gets [`Self::get_device_group_peer_memory_features_khr`]. See
    /// [`FNGetDeviceGroupPeerMemoryFeatures`] for more information.
    pub fn get_device_group_peer_memory_features_khr(&self) -> FNGetDeviceGroupPeerMemoryFeatures {
        self.get_device_group_peer_memory_features_khr
    }
    ///Gets [`Self::cmd_set_device_mask_khr`]. See [`FNCmdSetDeviceMask`] for more information.
    pub fn cmd_set_device_mask_khr(&self) -> FNCmdSetDeviceMask {
        self.cmd_set_device_mask_khr
    }
    ///Gets [`Self::cmd_dispatch_base_khr`]. See [`FNCmdDispatchBase`] for more information.
    pub fn cmd_dispatch_base_khr(&self) -> FNCmdDispatchBase {
        self.cmd_dispatch_base_khr
    }
}
