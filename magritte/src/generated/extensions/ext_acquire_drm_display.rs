//![VK_EXT_acquire_drm_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html) - instance extension
//!# Description
//!This extension allows an application to take exclusive control of a display
//!using the Direct Rendering Manager (DRM) interface.
//!When acquired, the display will be under full control of the application
//!until the display is either released or the connector is unplugged.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`ext_direct_mode_display`]`
//!# Contacts
//! - Drew DeVault [sir@cmpwn.com]()
//!# New commands
//! - [`acquire_drm_display_ext`]
//! - [`get_drm_display_ext`]
//!# New constants
//! - [`EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME`]
//! - [`EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION`]
//!# Known issues & F.A.Q.
//!None.
//!# Version history
//! - Revision 1, 2021-05-11 (Simon Zeni)  - Initial draft
//!# Other information
//! * 2021-06-09
//! * No known IP claims.
//! * - Simon Zeni, Status Holdings, Ltd.
//!# Related
//! - [`acquire_drm_display_ext`]
//! - [`get_drm_display_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{Instance, PhysicalDevice, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, mem::MaybeUninit, sync::atomic::AtomicBool};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_acquire_drm_display");
///[vkAcquireDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) - Acquire access to a VkDisplayKHR using DRM
///# C Specifications
///To acquire permission to directly a display in Vulkan from the Direct
///Rendering Manager (DRM) interface, call:
///```c
///// Provided by VK_EXT_acquire_drm_display
///VkResult vkAcquireDrmDisplayEXT(
///    VkPhysicalDevice                            physicalDevice,
///    int32_t                                     drmFd,
///    VkDisplayKHR                                display);
///```
///# Parameters
/// - [`physical_device`] The physical device the display is on.
/// - [`drm_fd`] DRM primary file descriptor.
/// - [`display`] The display the caller wishes Vulkan to control.
///# Description
///All permissions necessary to control the display are granted to the Vulkan
///instance associated with the provided [`physical_device`] until the display
///is either released or the connector is unplugged.
///The provided [`drm_fd`] must correspond to the one owned by the
///[`physical_device`].
///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
///The DRM FD must have DRM master permissions.
///If any error is encountered during the acquisition of the display, the call
///must return the error code `VK_ERROR_INITIALIZATION_FAILED`.The provided DRM fd should not be
/// closed before the display is released,
///attempting to do it may result in undefined behaviour.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid [`DisplayKHR`] handle
/// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`ext_acquire_drm_display`]
/// - [`DisplayKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquireDrmDisplayEXT")]
pub type FNAcquireDrmDisplayExt = Option<
    unsafe extern "system" fn(physical_device: PhysicalDevice, drm_fd: i32, display: DisplayKHR) -> VulkanResultCodes,
>;
///[vkGetDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) - Query the VkDisplayKHR corresponding to a DRM connector ID
///# C Specifications
///Before acquiring a display from the DRM interface, the caller may want to
///select a specific [`DisplayKHR`] handle by identifying it using a
///[`connector_id`].
///To do so, call:
///```c
///// Provided by VK_EXT_acquire_drm_display
///VkResult vkGetDrmDisplayEXT(
///    VkPhysicalDevice                            physicalDevice,
///    int32_t                                     drmFd,
///    uint32_t                                    connectorId,
///    VkDisplayKHR*                               display);
///```
///# Parameters
/// - [`physical_device`] The physical device to query the display from.
/// - [`drm_fd`] DRM primary file descriptor.
/// - [`connector_id`] Identifier of the specified DRM connector.
/// - [`display`] The corresponding [`DisplayKHR`] handle will be returned here.
///# Description
///If there is no [`DisplayKHR`] corresponding to the [`connector_id`] on
///the [`physical_device`], the returning [`display`] must be set to
///[`crate::Handle::null`].
///The provided [`drm_fd`] must correspond to the one owned by the
///[`physical_device`].
///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
///Master permissions are not required, because the file descriptor is just
///used for information gathering purposes.
///The given [`connector_id`] must be a resource owned by the provided
///[`drm_fd`].
///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
///If any error is encountered during the identification of the display, the
///call must return the error code `VK_ERROR_INITIALIZATION_FAILED`.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`display`] **must**  be a valid pointer to a [`DisplayKHR`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`ext_acquire_drm_display`]
/// - [`DisplayKHR`]
/// - [`PhysicalDevice`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDrmDisplayEXT")]
pub type FNGetDrmDisplayExt = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> VulkanResultCodes,
>;
impl PhysicalDevice {
    ///[vkAcquireDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) - Acquire access to a VkDisplayKHR using DRM
    ///# C Specifications
    ///To acquire permission to directly a display in Vulkan from the Direct
    ///Rendering Manager (DRM) interface, call:
    ///```c
    ///// Provided by VK_EXT_acquire_drm_display
    ///VkResult vkAcquireDrmDisplayEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    int32_t                                     drmFd,
    ///    VkDisplayKHR                                display);
    ///```
    ///# Parameters
    /// - [`physical_device`] The physical device the display is on.
    /// - [`drm_fd`] DRM primary file descriptor.
    /// - [`display`] The display the caller wishes Vulkan to control.
    ///# Description
    ///All permissions necessary to control the display are granted to the Vulkan
    ///instance associated with the provided [`physical_device`] until the display
    ///is either released or the connector is unplugged.
    ///The provided [`drm_fd`] must correspond to the one owned by the
    ///[`physical_device`].
    ///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
    ///The DRM FD must have DRM master permissions.
    ///If any error is encountered during the acquisition of the display, the call
    ///must return the error code `VK_ERROR_INITIALIZATION_FAILED`.The provided DRM fd should not
    /// be closed before the display is released,
    ///attempting to do it may result in undefined behaviour.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid [`DisplayKHR`] handle
    /// - [`display`] **must**  have been created, allocated, or retrieved from [`physical_device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`ext_acquire_drm_display`]
    /// - [`DisplayKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_drm_display_ext(
        self: &Unique<PhysicalDevice>,
        drm_fd: Option<i32>,
        display: DisplayKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_acquire_drm_display()
            .and_then(|vtable| vtable.acquire_drm_display_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_acquire_drm_display()
            .and_then(|vtable| vtable.acquire_drm_display_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), drm_fd.unwrap_or_default() as _, display);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl PhysicalDevice {
    ///[vkGetDrmDisplayEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) - Query the VkDisplayKHR corresponding to a DRM connector ID
    ///# C Specifications
    ///Before acquiring a display from the DRM interface, the caller may want to
    ///select a specific [`DisplayKHR`] handle by identifying it using a
    ///[`connector_id`].
    ///To do so, call:
    ///```c
    ///// Provided by VK_EXT_acquire_drm_display
    ///VkResult vkGetDrmDisplayEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    int32_t                                     drmFd,
    ///    uint32_t                                    connectorId,
    ///    VkDisplayKHR*                               display);
    ///```
    ///# Parameters
    /// - [`physical_device`] The physical device to query the display from.
    /// - [`drm_fd`] DRM primary file descriptor.
    /// - [`connector_id`] Identifier of the specified DRM connector.
    /// - [`display`] The corresponding [`DisplayKHR`] handle will be returned here.
    ///# Description
    ///If there is no [`DisplayKHR`] corresponding to the [`connector_id`] on
    ///the [`physical_device`], the returning [`display`] must be set to
    ///[`crate::Handle::null`].
    ///The provided [`drm_fd`] must correspond to the one owned by the
    ///[`physical_device`].
    ///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
    ///Master permissions are not required, because the file descriptor is just
    ///used for information gathering purposes.
    ///The given [`connector_id`] must be a resource owned by the provided
    ///[`drm_fd`].
    ///If not, the error code `VK_ERROR_UNKNOWN` must be returned.
    ///If any error is encountered during the identification of the display, the
    ///call must return the error code `VK_ERROR_INITIALIZATION_FAILED`.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`display`] **must**  be a valid pointer to a [`DisplayKHR`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`ext_acquire_drm_display`]
    /// - [`DisplayKHR`]
    /// - [`PhysicalDevice`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDrmDisplayEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_drm_display_ext(
        self: &Unique<PhysicalDevice>,
        drm_fd: Option<i32>,
        connector_id: Option<u32>,
    ) -> VulkanResult<Unique<DisplayKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_acquire_drm_display()
            .and_then(|vtable| vtable.get_drm_display_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_acquire_drm_display()
            .and_then(|vtable| vtable.get_drm_display_ext())
            .unwrap_unchecked();
        let mut display = MaybeUninit::<DisplayKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            drm_fd.unwrap_or_default() as _,
            connector_id.unwrap_or_default() as _,
            display.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => {
                VulkanResult::Success(_return, Unique::new(self, display.assume_init(), AtomicBool::default()))
            },
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_acquire_drm_display`
pub struct InstanceExtAcquireDrmDisplayVTable {
    ///See [`FNAcquireDrmDisplayExt`] for more information.
    pub acquire_drm_display_ext: FNAcquireDrmDisplayExt,
    ///See [`FNGetDrmDisplayExt`] for more information.
    pub get_drm_display_ext: FNGetDrmDisplayExt,
}
impl InstanceExtAcquireDrmDisplayVTable {
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
            acquire_drm_display_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireDrmDisplayEXT").as_ptr()))
            },
            get_drm_display_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetDrmDisplayEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::acquire_drm_display_ext`]. See [`FNAcquireDrmDisplayExt`] for more information.
    pub fn acquire_drm_display_ext(&self) -> FNAcquireDrmDisplayExt {
        self.acquire_drm_display_ext
    }
    ///Gets [`Self::get_drm_display_ext`]. See [`FNGetDrmDisplayExt`] for more information.
    pub fn get_drm_display_ext(&self) -> FNGetDrmDisplayExt {
        self.get_drm_display_ext
    }
}
