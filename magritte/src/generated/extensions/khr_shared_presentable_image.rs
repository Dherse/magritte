//![VK_KHR_shared_presentable_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shared_presentable_image.html) - device extension
//!# Description
//!This extension extends `[`khr_swapchain`]` to enable creation of a
//!shared presentable image.
//!This allows the application to use the image while the presention engine is
//!accessing it, in order to reduce the latency between rendering and
//!presentation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`khr_get_surface_capabilities2`]`
//!# Contacts
//! - Alon Or-bach [alonorbach](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shared_presentable_image]
//!   @alonorbach%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shared_presentable_image extension>>)
//!# New functions & commands
//! - [`get_swapchain_status_khr`]
//!# New structures
//! - Extending [`SurfaceCapabilities2KHR`]:  - [`SharedPresentSurfaceCapabilitiesKHR`]
//!# New constants
//! - [`KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME`]
//! - [`KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION`]
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
//! - Extending [`PresentModeKHR`]:  - `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`  -
//!   `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
//!# Known issues & F.A.Q
//!1) Should we allow a Vulkan WSI swapchain to toggle between normal usage and
//!shared presentation usage? **RESOLVED** : No.
//!WSI swapchains are typically recreated with new properties instead of having
//!their properties changed.
//!This can also save resources, assuming that fewer images are needed for
//!shared presentation, and assuming that most VR applications do not need to
//!switch between normal and shared usage.2) Should we have a query for determining how the
//! presentation engine
//!refresh is triggered? **RESOLVED** : Yes.
//!This is done via which presentation modes a surface supports.3) Should the object representing a
//! shared presentable image be an extension
//!of a [`SwapchainKHR`] or a separate object? **RESOLVED** : Extension of a swapchain due to
//! overlap in creation properties
//!and to allow common functionality between shared and normal presentable
//!images and swapchains.4) What should we call the extension and the new structures it creates?
//! **RESOLVED** : Shared presentable image / shared present.5) Should the `minImageCount` and
//! `presentMode` values of the
//![`SwapchainCreateInfoKHR`] be ignored, or required to be compatible
//!values? **RESOLVED** : `minImageCount` must be set to 1, and `presentMode`
//!should be set to either `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
//!`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`.6) What should the layout of the shared
//! presentable image be? **RESOLVED** : After acquiring the shared presentable image, the
//! application
//!must transition it to the `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout
//!prior to it being used.
//!After this initial transition, any image usage that was requested during
//!swapchain creation  **can**  be performed on the image without layout transitions
//!being performed.7) Do we need a new API for the trigger to refresh new content? **RESOLVED** :
//! [`queue_present_khr`] to act as API to trigger a refresh, as
//!will allow combination with other compatible extensions to
//![`queue_present_khr`].8) How should an application detect a `VK_ERROR_OUT_OF_DATE_KHR` error
//!on a swapchain using the `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`
//!present mode? **RESOLVED** : Introduce [`get_swapchain_status_khr`] to allow applications to
//!query the status of a swapchain using a shared presentation mode.9) What should subsequent calls
//! to [`queue_present_khr`] for
//!`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` swapchains be defined to
//!do? **RESOLVED** : State that implementations may use it as a hint for updated
//!content.10) Can the ownership of a shared presentable image be transferred to a
//!different queue? **RESOLVED** : No.
//!It is not possible to transfer ownership of a shared presentable image
//!obtained from a swapchain created using `VK_SHARING_MODE_EXCLUSIVE`
//!after it has been presented.11) How should [`queue_submit`] behave if a command buffer uses an
//! image
//!from a `VK_ERROR_OUT_OF_DATE_KHR` swapchain? **RESOLVED** : [`queue_submit`] is expected to
//! return the
//!`VK_ERROR_DEVICE_LOST` error.12) Can Vulkan provide any guarantee on the order of rendering, to
//! enable
//!beam chasing? **RESOLVED** : This could be achieved via use of render passes to ensure strip
//!rendering.
//!# Version History
//! - Revision 1, 2017-03-20 (Alon Or-bach)  - Internal revisions
//!# Other info
//! * 2017-03-20
//! * No known IP claims.
//! * - Alon Or-bach, Samsung Electronics  - Ian Elliott, Google  - Jesse Hall, Google  - Pablo
//!   Ceballos, Google  - Chris Forbes, Google  - Jeff Juliano, NVIDIA  - James Jones, NVIDIA  -
//!   Daniel Rakos, AMD  - Tobias Hector, Imagination Technologies  - Graham Connor, Imagination
//!   Technologies  - Michael Worcester, Imagination Technologies  - Cass Everitt, Oculus  -
//!   Johannes Van Waveren, Oculus
//!# Related
//! - [`SharedPresentSurfaceCapabilitiesKHR`]
//! - [`get_swapchain_status_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Device, ImageUsageFlags, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shared_presentable_image");
///[vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html) - Get a swapchain's status
///# C Specifications
///In order to query a swapchain’s status when rendering to a shared
///presentable image, call:
///```c
///// Provided by VK_KHR_shared_presentable_image
///VkResult vkGetSwapchainStatusKHR(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to query.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
///# Related
/// - [`khr_shared_presentable_image`]
/// - [`Device`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetSwapchainStatusKHR")]
pub type FNGetSwapchainStatusKhr =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes>;
///[VkSharedPresentSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html) - Structure describing capabilities of a surface for shared presentation
///# C Specifications
///The [`SharedPresentSurfaceCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_shared_presentable_image
///typedef struct VkSharedPresentSurfaceCapabilitiesKHR {
///    VkStructureType      sType;
///    void*                pNext;
///    VkImageUsageFlags    sharedPresentSupportedUsageFlags;
///} VkSharedPresentSurfaceCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shared_present_supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing
///   the ways the application  **can**  use the shared presentable image from a swapchain created
///   with [`PresentModeKHR`] set to `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on the specified device.
///   `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set but implementations
///   **may**  support additional usages.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
///# Related
/// - [`khr_shared_presentable_image`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shared_present_supported_usage_flags`] is a bitmask of
    ///[`ImageUsageFlagBits`] representing the ways the application  **can**
    ///use the shared presentable image from a swapchain created with
    ///[`PresentModeKHR`] set to
    ///`VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
    ///`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on
    ///the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set
    ///but implementations  **may**  support additional usages.
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
impl<'lt> Default for SharedPresentSurfaceCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            p_next: std::ptr::null_mut(),
            shared_present_supported_usage_flags: Default::default(),
        }
    }
}
impl<'lt> SharedPresentSurfaceCapabilitiesKHR<'lt> {
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
    ///Gets the value of [`Self::shared_present_supported_usage_flags`]
    pub fn shared_present_supported_usage_flags(&self) -> ImageUsageFlags {
        self.shared_present_supported_usage_flags
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
    ///Gets a mutable reference to the value of [`Self::shared_present_supported_usage_flags`]
    pub fn shared_present_supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.shared_present_supported_usage_flags
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
    ///Sets the value of [`Self::shared_present_supported_usage_flags`]
    pub fn set_shared_present_supported_usage_flags(mut self, value: crate::vulkan1_0::ImageUsageFlags) -> Self {
        self.shared_present_supported_usage_flags = value;
        self
    }
}
impl Device {
    ///[vkGetSwapchainStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html) - Get a swapchain's status
    ///# C Specifications
    ///In order to query a swapchain’s status when rendering to a shared
    ///presentable image, call:
    ///```c
    ///// Provided by VK_KHR_shared_presentable_image
    ///VkResult vkGetSwapchainStatusKHR(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to query.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
    ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///# Related
    /// - [`khr_shared_presentable_image`]
    /// - [`Device`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetSwapchainStatusKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_swapchain_status_khr(self: &Unique<Device>, swapchain: SwapchainKHR) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_shared_presentable_image()
            .and_then(|vtable| vtable.get_swapchain_status_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_shared_presentable_image()
            .and_then(|vtable| vtable.get_swapchain_status_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), swapchain);
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::SUBOPTIMAL_KHR => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_shared_presentable_image`
pub struct DeviceKhrSharedPresentableImageVTable {
    ///See [`FNGetSwapchainStatusKhr`] for more information.
    pub get_swapchain_status_khr: FNGetSwapchainStatusKhr,
}
impl DeviceKhrSharedPresentableImageVTable {
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
            get_swapchain_status_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetSwapchainStatusKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_swapchain_status_khr`]. See [`FNGetSwapchainStatusKhr`] for more
    /// information.
    pub fn get_swapchain_status_khr(&self) -> FNGetSwapchainStatusKhr {
        self.get_swapchain_status_khr
    }
}
