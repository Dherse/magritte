//![VK_KHR_get_physical_device_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_physical_device_properties2.html) - instance extension
//!# Description
//!This extension provides new entry points to query device features, device
//!properties, and format properties in a way that can be easily extended by
//!other extensions, without introducing any further entry points.
//!The Vulkan 1.0 feature/limit/formatproperty structures do not include
//!`sType`/`pNext` members.
//!This extension wraps them in new structures with `sType`/`pNext`
//!members, so an application can query a chain of feature/limit/formatproperty
//!structures by constructing the chain and letting the implementation fill
//!them in.
//!A new command is added for each `vkGetPhysicalDevice*` command in core
//!Vulkan 1.0.
//!The new feature structure (and a `pNext` chain of extending structures)
//!can also be passed in to device creation to enable features.This extension also allows
//! applications to use the physical-device
//!components of device extensions before [`create_device`] is called.
//!# Revision
//!2
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_physical_device_properties2]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_physical_device_properties2 extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceFeatures2KHR`]
//! - [`GetPhysicalDeviceFormatProperties2KHR`]
//! - [`GetPhysicalDeviceImageFormatProperties2KHR`]
//! - [`GetPhysicalDeviceMemoryProperties2KHR`]
//! - [`GetPhysicalDeviceProperties2KHR`]
//! - [`GetPhysicalDeviceQueueFamilyProperties2KHR`]
//! - [`GetPhysicalDeviceSparseImageFormatProperties2KHR`]
//!# New structures
//! - [`FormatProperties2KHR`]
//! - [`ImageFormatProperties2KHR`]
//! - [`PhysicalDeviceImageFormatInfo2KHR`]
//! - [`PhysicalDeviceMemoryProperties2KHR`]
//! - [`PhysicalDeviceProperties2KHR`]
//! - [`PhysicalDeviceSparseImageFormatInfo2KHR`]
//! - [`QueueFamilyProperties2KHR`]
//! - [`SparseImageFormatProperties2KHR`]
//! - Extending [`DeviceCreateInfo`]:  - [`PhysicalDeviceFeatures2KHR`]
//!# New constants
//! - [`KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME`]
//! - [`KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR`
//!# Version History
//! - Revision 1, 2016-09-12 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2016-11-02 (Ian Elliott)  - Added ability for applications to use the
//!   physical-device components of device extensions before vkCreateDevice is called.
//!# Other info
//! * 2017-09-05
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA  - Ian Elliott, Google
//!# Related
//! - [`FormatProperties2KHR`]
//! - [`ImageFormatProperties2KHR`]
//! - [`PhysicalDeviceFeatures2KHR`]
//! - [`PhysicalDeviceImageFormatInfo2KHR`]
//! - [`PhysicalDeviceMemoryProperties2KHR`]
//! - [`PhysicalDeviceProperties2KHR`]
//! - [`PhysicalDeviceSparseImageFormatInfo2KHR`]
//! - [`QueueFamilyProperties2KHR`]
//! - [`SparseImageFormatProperties2KHR`]
//! - [`GetPhysicalDeviceFeatures2KHR`]
//! - [`GetPhysicalDeviceFormatProperties2KHR`]
//! - [`GetPhysicalDeviceImageFormatProperties2KHR`]
//! - [`GetPhysicalDeviceMemoryProperties2KHR`]
//! - [`GetPhysicalDeviceProperties2KHR`]
//! - [`GetPhysicalDeviceQueueFamilyProperties2KHR`]
//! - [`GetPhysicalDeviceSparseImageFormatProperties2KHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::Instance,
    vulkan1_1::{
        FNGetPhysicalDeviceFeatures2, FNGetPhysicalDeviceFormatProperties2, FNGetPhysicalDeviceImageFormatProperties2,
        FNGetPhysicalDeviceMemoryProperties2, FNGetPhysicalDeviceProperties2,
        FNGetPhysicalDeviceQueueFamilyProperties2, FNGetPhysicalDeviceSparseImageFormatProperties2,
    },
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_get_physical_device_properties2");
///The V-table of [`Instance`] for functions from `VK_KHR_get_physical_device_properties2`
pub struct InstanceKhrGetPhysicalDeviceProperties2VTable {
    ///See [`FNGetPhysicalDeviceFeatures2`] for more information.
    pub get_physical_device_features2: FNGetPhysicalDeviceFeatures2,
    ///See [`FNGetPhysicalDeviceProperties2`] for more information.
    pub get_physical_device_properties2: FNGetPhysicalDeviceProperties2,
    ///See [`FNGetPhysicalDeviceFormatProperties2`] for more information.
    pub get_physical_device_format_properties2: FNGetPhysicalDeviceFormatProperties2,
    ///See [`FNGetPhysicalDeviceImageFormatProperties2`] for more information.
    pub get_physical_device_image_format_properties2: FNGetPhysicalDeviceImageFormatProperties2,
    ///See [`FNGetPhysicalDeviceQueueFamilyProperties2`] for more information.
    pub get_physical_device_queue_family_properties2: FNGetPhysicalDeviceQueueFamilyProperties2,
    ///See [`FNGetPhysicalDeviceMemoryProperties2`] for more information.
    pub get_physical_device_memory_properties2: FNGetPhysicalDeviceMemoryProperties2,
    ///See [`FNGetPhysicalDeviceSparseImageFormatProperties2`] for more information.
    pub get_physical_device_sparse_image_format_properties2: FNGetPhysicalDeviceSparseImageFormatProperties2,
}
impl InstanceKhrGetPhysicalDeviceProperties2VTable {
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
            get_physical_device_features2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceFeatures2KHR").as_ptr(),
                ))
            },
            get_physical_device_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_format_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceFormatProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_image_format_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceImageFormatProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_queue_family_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceQueueFamilyProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_memory_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceMemoryProperties2KHR").as_ptr(),
                ))
            },
            get_physical_device_sparse_image_format_properties2: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSparseImageFormatProperties2KHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_features2`]. See [`FNGetPhysicalDeviceFeatures2`] for more
    /// information.
    pub fn get_physical_device_features2(&self) -> FNGetPhysicalDeviceFeatures2 {
        self.get_physical_device_features2
    }
    ///Gets [`Self::get_physical_device_properties2`]. See [`FNGetPhysicalDeviceProperties2`] for
    /// more information.
    pub fn get_physical_device_properties2(&self) -> FNGetPhysicalDeviceProperties2 {
        self.get_physical_device_properties2
    }
    ///Gets [`Self::get_physical_device_format_properties2`]. See
    /// [`FNGetPhysicalDeviceFormatProperties2`] for more information.
    pub fn get_physical_device_format_properties2(&self) -> FNGetPhysicalDeviceFormatProperties2 {
        self.get_physical_device_format_properties2
    }
    ///Gets [`Self::get_physical_device_image_format_properties2`]. See
    /// [`FNGetPhysicalDeviceImageFormatProperties2`] for more information.
    pub fn get_physical_device_image_format_properties2(&self) -> FNGetPhysicalDeviceImageFormatProperties2 {
        self.get_physical_device_image_format_properties2
    }
    ///Gets [`Self::get_physical_device_queue_family_properties2`]. See
    /// [`FNGetPhysicalDeviceQueueFamilyProperties2`] for more information.
    pub fn get_physical_device_queue_family_properties2(&self) -> FNGetPhysicalDeviceQueueFamilyProperties2 {
        self.get_physical_device_queue_family_properties2
    }
    ///Gets [`Self::get_physical_device_memory_properties2`]. See
    /// [`FNGetPhysicalDeviceMemoryProperties2`] for more information.
    pub fn get_physical_device_memory_properties2(&self) -> FNGetPhysicalDeviceMemoryProperties2 {
        self.get_physical_device_memory_properties2
    }
    ///Gets [`Self::get_physical_device_sparse_image_format_properties2`]. See
    /// [`FNGetPhysicalDeviceSparseImageFormatProperties2`] for more information.
    pub fn get_physical_device_sparse_image_format_properties2(
        &self,
    ) -> FNGetPhysicalDeviceSparseImageFormatProperties2 {
        self.get_physical_device_sparse_image_format_properties2
    }
}
