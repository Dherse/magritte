use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceMemory, StructureType},
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
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`external_memory_rdma`] indicates whether the implementation has support for the
///   `VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV` memory property and the
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV` external memory handle type.
///If the [`PhysicalDeviceExternalMemoryRdmaFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceExternalMemoryRdmaFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV`
///# Related
/// - [`VK_NV_external_memory_rdma`]
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
pub struct PhysicalDeviceExternalMemoryRdmaFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`external_memory_rdma`] indicates
    ///whether the implementation has support for the
    ///`VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV` memory property and the
    ///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV` external memory
    ///handle type.
    external_memory_rdma: Bool32,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the remote accessible address will be exported.
/// - [`handle_type`] is the type of handle requested.
///# Description
///Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportMemoryAllocateInfo::handle_types`] when
///   [`memory`] was created
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_NV_external_memory_rdma`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetMemoryRemoteAddressNV`]
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
pub struct MemoryGetRemoteAddressInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the remote accessible
    ///address will be exported.
    memory: DeviceMemory,
    ///[`handle_type`] is the type of handle requested.
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
