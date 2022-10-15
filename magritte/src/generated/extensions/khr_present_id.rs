//![VK_KHR_present_id](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_id.html) - device extension
//!# Description
//!This device extension allows an application that uses the
//!`[`khr_swapchain`]` extension to provide an identifier for present
//!operations on a swapchain.
//!An application  **can**  use this to reference specific present operations in
//!other extensions.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//!# Contacts
//! - Keith Packard [keithp](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_present_id]
//!   @keithp%0A<<Here describe the issue or question you have about the VK_KHR_present_id
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePresentIdFeaturesKHR`]
//! - Extending [`PresentInfoKHR`]:  - [`PresentIdKHR`]
//!# New constants
//! - [`KHR_PRESENT_ID_EXTENSION_NAME`]
//! - [`KHR_PRESENT_ID_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
//!# Known issues & F.A.Q.
//!None.
//!# Version history
//! - Revision 1, 2019-05-15 (Keith Packard)  - Initial version
//!# Other information
//! * 2019-05-15
//! * No known IP claims.
//! * - Keith Packard, Valve  - Ian Elliott, Google  - Alon Or-bach, Samsung
//!# Related
//! - [`PhysicalDevicePresentIdFeaturesKHR`]
//! - [`PresentIdKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`present_id`] indicates that the implementation supports specifying present ID values in the
///   [`PresentIdKHR`] extension to the [`PresentInfoKHR`] struct.
/// If the [`PhysicalDevicePresentIdFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDevicePresentIdFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR`
/// # Related
/// - [`khr_present_id`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePresentIdFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePresentIdFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`present_id`] indicates that the implementation
    ///supports specifying present ID values in the [`PresentIdKHR`]
    ///extension to the [`PresentInfoKHR`] struct.
    pub present_id: Bool32,
}
impl<'lt> Default for PhysicalDevicePresentIdFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            present_id: 0,
        }
    }
}
impl<'lt> PhysicalDevicePresentIdFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::present_id`]
    pub fn present_id_raw(&self) -> Bool32 {
        self.present_id
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_id`]
    pub fn set_present_id_raw(&mut self, value: Bool32) -> &mut Self {
        self.present_id = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_id`]
    pub fn with_present_id_raw(mut self, value: Bool32) -> Self {
        self.present_id = value;
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
    ///Gets the value of [`Self::present_id`]
    pub fn present_id(&self) -> bool {
        unsafe { std::mem::transmute(self.present_id as u8) }
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
    ///Gets a mutable reference to the value of [`Self::present_id`]
    pub fn present_id_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.present_id as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.present_id as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::present_id`]
    pub fn set_present_id(&mut self, value: bool) -> &mut Self {
        self.present_id = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::present_id`]
    pub fn with_present_id(mut self, value: bool) -> Self {
        self.present_id = value as u8 as u32;
        self
    }
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to the [`queue_present_khr`]
///   command.
/// - [`present_ids`] is `NULL` or a pointer to an array of uint64_t with [`swapchain_count`]
///   entries. If not `NULL`, each non-zero value in [`present_ids`] specifies the present id to be
///   associated with the presentation of the swapchain with the same index in the
///   [`queue_present_khr`] call.
/// # Description
/// For applications to be able to reference specific presentation events queued
/// by a call to [`queue_present_khr`], an identifier needs to be associated
/// with them.
/// When the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId) feature is enabled,
/// applications  **can**  include the [`PresentIdKHR`] structure in the
/// [`p_next`] chain of the [`PresentInfoKHR`] structure to supply
/// identifiers.Each [`SwapchainKHR`] has a presentId associated with it.
/// This value is initially set to zero when the [`SwapchainKHR`] is
/// created.When a [`PresentIdKHR`] structure with a non-NULL [`present_ids`] is
/// included in the [`p_next`] chain of a [`PresentInfoKHR`] structure,
/// each `pSwapchains` entry has a presentId associated in the
/// [`present_ids`] array at the same index as the swapchain in the
/// `pSwapchains` array.
/// If this presentId is non-zero, then the application  **can**  later use this
/// value to refer to that image presentation.
/// A value of zero indicates that this presentation has no associated
/// presentId.
/// A non-zero presentId  **must**  be greater than any non-zero presentId passed
/// previously by the application for the same swapchain.There is no requirement for any precise
/// timing relationship between the
/// presentation of the image to the user and the update of the presentId value,
/// but implementations  **should**  make this as close as possible to the
/// presentation of the first pixel in the new image to the user.
/// ## Valid Usage
/// - [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where this [`PresentIdKHR`] is in the [`p_next`] chain of the [`PresentInfoKHR`] structure
/// - Each `presentIds` entry  **must**  be greater than any previous `presentIds` entry passed for
///   the associated `pSwapchains` entry
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_ID_KHR`
/// - If [`present_ids`] is not `NULL`, [`present_ids`] **must**  be a valid pointer to an array of
///   [`swapchain_count`]`uint64_t` values
/// - [`swapchain_count`] **must**  be greater than `0`
/// # Related
/// - [`khr_present_id`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentIdKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PresentIdKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to the
    ///[`queue_present_khr`] command.
    pub swapchain_count: u32,
    ///[`present_ids`] is `NULL` or a pointer to an array of uint64_t with
    ///[`swapchain_count`] entries.
    ///If not `NULL`, each non-zero value in [`present_ids`] specifies the
    ///present id to be associated with the presentation of the swapchain with
    ///the same index in the [`queue_present_khr`] call.
    pub present_ids: *const u64,
}
impl<'lt> Default for PresentIdKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PRESENT_ID_KHR,
            p_next: std::ptr::null(),
            swapchain_count: 0,
            present_ids: std::ptr::null(),
        }
    }
}
impl<'lt> PresentIdKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::present_ids`]
    pub fn present_ids_raw(&self) -> *const u64 {
        self.present_ids
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_ids`]
    pub fn set_present_ids_raw(&mut self, value: *const u64) -> &mut Self {
        self.present_ids = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_ids`]
    pub fn with_present_ids_raw(mut self, value: *const u64) -> Self {
        self.present_ids = value;
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
    ///Gets the value of [`Self::present_ids`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn present_ids(&self) -> &[u64] {
        std::slice::from_raw_parts(self.present_ids, self.swapchain_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut self.swapchain_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::present_ids`]
    pub fn set_present_ids(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.present_ids = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn with_swapchain_count(mut self, value: u32) -> Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::present_ids`]
    pub fn with_present_ids(mut self, value: &'lt [u64]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.present_ids = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
}
