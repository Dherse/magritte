//![VK_NV_external_memory_rdma](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_rdma.html) - device extension
//!# Description
//!This extension adds support for allocating memory which can be used for
//!remote direct memory access (RDMA) from other devices.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - Carsten Rohde [crohde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_rdma]
//!   @crohde%0A<<Here describe the issue or question you have about the VK_NV_external_memory_rdma
//!   extension>>)
//!# New functions & commands
//! - [`get_memory_remote_address_nv`]
//!# New structures
//! - [`MemoryGetRemoteAddressInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV`
//! - Extending [`MemoryPropertyFlagBits`]:  - `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
//!# Version History
//! - Revision 1, 2020-12-15 (Carsten Rohde)  - Internal revisions
//!# Other info
//! * 2021-04-19
//! * No known IP claims.
//! * - Carsten Rohde, NVIDIA
//!# Related
//! - [`MemoryGetRemoteAddressInfoNV`]
//! - [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]
//! - [`RemoteAddressNV`]
//! - [`get_memory_remote_address_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory_rdma");
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VkRemoteAddressNV")]
pub type RemoteAddressNV = c_void;
///[vkGetMemoryRemoteAddressNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryRemoteAddressNV.html) - Get an address for a memory object accessible by remote devices
///# C Specifications
///To export an address representing the payload of a Vulkan device memory
///object accessible by remote devices, call:
///```c
///// Provided by VK_NV_external_memory_rdma
///VkResult vkGetMemoryRemoteAddressNV(
///    VkDevice                                    device,
///    const VkMemoryGetRemoteAddressInfoNV*       pMemoryGetRemoteAddressInfo,
///    VkRemoteAddressNV*                          pAddress);
///```
/// # Parameters
/// - [`device`] is the logical device that created the device memory being exported.
/// - [`p_memory_get_remote_address_info`] is a pointer to a [`MemoryGetRemoteAddressInfoNV`]
///   structure containing parameters of the export operation.
/// - [`p_address`] will return the address representing the payload of the device memory object.
/// # Description
/// More communication may be required between the kernel-mode drivers of the
/// devices involved.
/// This information is out of scope of this documentation and should be
/// requested from the vendors of the devices.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_memory_get_remote_address_info`] **must**  be a valid pointer to a valid
///   [`MemoryGetRemoteAddressInfoNV`] structure
/// - [`p_address`] **must**  be a valid pointer to a [`RemoteAddressNV`] value
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
/// # Related
/// - [`VK_NV_external_memory_rdma`]
/// - [`Device`]
/// - [`MemoryGetRemoteAddressInfoNV`]
/// - [`RemoteAddressNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryRemoteAddressNV")]
pub type FNGetMemoryRemoteAddressNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'lt>,
        p_address: *mut RemoteAddressNV,
    ) -> VulkanResultCodes,
>;
///[VkPhysicalDeviceExternalMemoryRDMAFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryRDMAFeaturesNV.html) - Structure describing the external memory RDMA features supported by the implementation
///# C Specifications
///The [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_external_memory_rdma
///typedef struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           externalMemoryRDMA;
///} VkPhysicalDeviceExternalMemoryRDMAFeaturesNV;
///```
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`external_memory_rdma`] indicates whether the implementation has support for the
///   `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV` memory property and the
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV` external memory handle type.
/// If the [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
/// # Related
/// - [`VK_NV_external_memory_rdma`]
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
#[doc(alias = "VkPhysicalDeviceExternalMemoryRDMAFeaturesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryRdmaFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`external_memory_rdma`] indicates
    ///whether the implementation has support for the
    ///`VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV` memory property and the
    ///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV` external memory
    ///handle type.
    pub external_memory_rdma: Bool32,
}
impl<'lt> Default for PhysicalDeviceExternalMemoryRdmaFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExternalMemoryRdmaFeaturesNv,
            p_next: std::ptr::null_mut(),
            external_memory_rdma: 0,
        }
    }
}
impl<'lt> PhysicalDeviceExternalMemoryRdmaFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::external_memory_rdma`]
    pub fn external_memory_rdma_raw(&self) -> Bool32 {
        self.external_memory_rdma
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::external_memory_rdma`]
    pub fn set_external_memory_rdma_raw(&mut self, value: Bool32) -> &mut Self {
        self.external_memory_rdma = value;
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
    ///Gets the value of [`Self::external_memory_rdma`]
    pub fn external_memory_rdma(&self) -> bool {
        unsafe { std::mem::transmute(self.external_memory_rdma as u8) }
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
    ///Gets a mutable reference to the value of [`Self::external_memory_rdma`]
    pub fn external_memory_rdma_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.external_memory_rdma as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.external_memory_rdma as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::external_memory_rdma`]
    pub fn set_external_memory_rdma(&mut self, value: bool) -> &mut Self {
        self.external_memory_rdma = value as u8 as u32;
        self
    }
}
///[VkMemoryGetRemoteAddressInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetRemoteAddressInfoNV.html) - Structure describing a remote accessible address export operation
///# C Specifications
///The [`MemoryGetRemoteAddressInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory_rdma
///typedef struct VkMemoryGetRemoteAddressInfoNV {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceMemory                        memory;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkMemoryGetRemoteAddressInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the remote accessible address will be exported.
/// - [`handle_type`] is the type of handle requested.
/// # Description
/// ## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`]
///   when [`memory`] was created
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
/// # Related
/// - [`VK_NV_external_memory_rdma`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`get_memory_remote_address_nv`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryGetRemoteAddressInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryGetRemoteAddressInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the remote accessible
    ///address will be exported.
    pub memory: DeviceMemory,
    ///[`handle_type`] is the type of handle requested.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for MemoryGetRemoteAddressInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryGetRemoteAddressInfoNv,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> MemoryGetRemoteAddressInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
///The V-table of [`Device`] for functions from VK_NV_external_memory_rdma
pub struct DeviceNvExternalMemoryRdmaVTable {
    ///See [`FNGetMemoryRemoteAddressNv`] for more information.
    pub get_memory_remote_address_nv: FNGetMemoryRemoteAddressNv,
}
impl DeviceNvExternalMemoryRdmaVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            get_memory_remote_address_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetMemoryRemoteAddressNV")))
            },
        }
    }
    ///Gets [`Self::get_memory_remote_address_nv`]. See [`FNGetMemoryRemoteAddressNv`] for more
    /// information.
    pub fn get_memory_remote_address_nv(&self) -> FNGetMemoryRemoteAddressNv {
        self.get_memory_remote_address_nv
    }
}
