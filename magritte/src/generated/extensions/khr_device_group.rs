//![VK_KHR_device_group](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html) - device extension
//!# Description
//!This extension provides functionality to use a logical device that consists
//!of multiple physical devices, as created with the
//!`[`VK_KHR_device_group_creation`]` extension.
//!A device group can allocate memory across the subdevices, bind memory from
//!one subdevice to a resource on another subdevice, record command buffers
//!where some work executes on an arbitrary subset of the subdevices, and
//!potentially present a swapchain image from one or more subdevices.
//!# Revision
//!4
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_device_group_creation`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_device_group]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_device_group
//!   extension>>)
//!# New functions & commands
//! - [`CmdDispatchBaseKHR`]
//! - [`CmdSetDeviceMaskKHR`]
//! - [`GetDeviceGroupPeerMemoryFeaturesKHR`]If [`VK_KHR_surface`] is supported:
//! - [`GetDeviceGroupPresentCapabilitiesKHR`]
//! - [`GetDeviceGroupSurfacePresentModesKHR`]
//! - [`GetPhysicalDevicePresentRectanglesKHR`]If [`VK_KHR_swapchain`] is supported:
//! - [`AcquireNextImage2KHR`]
//!# New structures
//! - Extending [`BindSparseInfo`]:
//! - [`DeviceGroupBindSparseInfoKHR`]
//! - Extending [`CommandBufferBeginInfo`]:
//! - [`DeviceGroupCommandBufferBeginInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:
//! - [`MemoryAllocateFlagsInfoKHR`]
//! - Extending [`RenderPassBeginInfo`], [`RenderingInfo`]:
//! - [`DeviceGroupRenderPassBeginInfoKHR`]
//! - Extending [`SubmitInfo`]:
//! - [`DeviceGroupSubmitInfoKHR`]If [`VK_KHR_bind_memory2`] is supported:
//! - Extending [`BindBufferMemoryInfo`]:
//! - [`BindBufferMemoryDeviceGroupInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:
//! - [`BindImageMemoryDeviceGroupInfoKHR`]If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentCapabilitiesKHR`]If [`VK_KHR_swapchain`] is supported:
//! - [`AcquireNextImageInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:
//! - [`BindImageMemorySwapchainInfoKHR`]
//! - Extending [`ImageCreateInfo`]:
//! - [`ImageSwapchainCreateInfoKHR`]
//! - Extending [`PresentInfoKHR`]:
//! - [`DeviceGroupPresentInfoKHR`]
//! - Extending [`SwapchainCreateInfoKHR`]:
//! - [`DeviceGroupSwapchainCreateInfoKHR`]
//!# New enums
//! - [`MemoryAllocateFlagBitsKHR`]
//! - [`PeerMemoryFeatureFlagBitsKHR`]If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagBitsKHR`]
//!# New bitmasks
//! - [`MemoryAllocateFlagsKHR`]
//! - [`PeerMemoryFeatureFlagsKHR`]If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagsKHR`]
//!# New constants
//! - [`KHR_DEVICE_GROUP_EXTENSION_NAME`]
//! - [`KHR_DEVICE_GROUP_SPEC_VERSION`]
//! - Extending [`DependencyFlagBits`]:
//! - `VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR`
//! - Extending [`MemoryAllocateFlagBits`]:
//! - `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR`
//! - Extending [`PeerMemoryFeatureFlagBits`]:
//! - `VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR`
//! - `VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR`
//! - `VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR`
//! - `VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR`
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_DISPATCH_BASE_KHR`
//! - `VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR`If [`VK_KHR_bind_memory2`] is supported:
//! - Extending [`ImageCreateFlagBits`]:
//! - `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR`If [`VK_KHR_surface`] is supported:
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`If [`VK_KHR_swapchain`] is supported:
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
//! - Extending [`SwapchainCreateFlagBitsKHR`]:
//! - `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//!# Version History
//! - Revision 1, 2016-10-19 (Jeff Bolz)
//! - Internal revisions
//! - Revision 2, 2017-05-19 (Tobias Hector)
//! - Removed extended memory bind functions to VK_KHR_bind_memory2, added
//!dependency on that extension, and device-group-specific structs for
//!those functions.
//! - Revision 3, 2017-10-06 (Ian Elliott)
//! - Corrected Vulkan 1.1 interactions with the WSI extensions.
//!All Vulkan 1.1 WSI interactions are with the VK_KHR_swapchain
//!extension.
//! - Revision 4, 2017-10-10 (Jeff Bolz)
//! - Rename “SFR” bits and structure members to use the phrase “split
//!instance bind regions”.
//!# Other info
//! * 2017-10-10
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!*
//! - Jeff Bolz, NVIDIA
//! - Tobias Hector, Imagination Technologies
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
//! - [`CmdDispatchBaseKHR`]
//! - [`CmdSetDeviceMaskKHR`]
//! - [`GetDeviceGroupPeerMemoryFeaturesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_device_group");
