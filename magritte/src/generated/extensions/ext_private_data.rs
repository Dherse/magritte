//![VK_EXT_private_data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_private_data.html) - device extension
//!# Description
//!This extension is a device extension which enables attaching arbitrary
//!payloads to Vulkan objects.
//!It introduces the idea of private data slots as a means of storing a 64-bit
//!unsigned integer of application defined data.
//!Private data slots can be created or destroyed any time an associated device
//!is available.
//!Private data slots can be reserved at device creation time, and limiting use
//!to the amount reserved will allow the extension to exhibit better
//!performance characteristics.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Matthew Rusch [mattruschnv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_private_data]
//!   @mattruschnv%0A<<Here describe the issue or question you have about the VK_EXT_private_data
//!   extension>>)
//!# New handles
//! - [`PrivateDataSlotEXT`]
//!# New functions & commands
//! - [`CreatePrivateDataSlotEXT`]
//! - [`DestroyPrivateDataSlotEXT`]
//! - [`GetPrivateDataEXT`]
//! - [`SetPrivateDataEXT`]
//!# New structures
//! - [`PrivateDataSlotCreateInfoEXT`]
//! - Extending [`DeviceCreateInfo`]:  - [`DevicePrivateDataCreateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePrivateDataFeaturesEXT`]
//!# New bitmasks
//! - [`PrivateDataSlotCreateFlagsEXT`]
//!# New constants
//! - [`EXT_PRIVATE_DATA_EXTENSION_NAME`]
//! - [`EXT_PRIVATE_DATA_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-01-15 (Matthew Rusch)  - Initial draft
//!# Other info
//! * 2020-03-25
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Matthew Rusch, NVIDIA  - Nuno Subtil, NVIDIA  - Piers Daniell, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`DevicePrivateDataCreateInfoEXT`]
//! - [`PhysicalDevicePrivateDataFeaturesEXT`]
//! - [`PrivateDataSlotCreateFlagsEXT`]
//! - [`PrivateDataSlotCreateInfoEXT`]
//! - [`PrivateDataSlotEXT`]
//! - [`CreatePrivateDataSlotEXT`]
//! - [`DestroyPrivateDataSlotEXT`]
//! - [`GetPrivateDataEXT`]
//! - [`SetPrivateDataEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::Device,
    vulkan1_3::{FNCreatePrivateDataSlot, FNDestroyPrivateDataSlot, FNGetPrivateData, FNSetPrivateData},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PRIVATE_DATA_SPEC_VERSION")]
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PRIVATE_DATA_EXTENSION_NAME")]
pub const EXT_PRIVATE_DATA_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_private_data");
///The V-table of [`Device`] for functions from `VK_EXT_private_data`
pub struct DeviceExtPrivateDataVTable {
    ///See [`FNCreatePrivateDataSlot`] for more information.
    pub create_private_data_slot: FNCreatePrivateDataSlot,
    ///See [`FNDestroyPrivateDataSlot`] for more information.
    pub destroy_private_data_slot: FNDestroyPrivateDataSlot,
    ///See [`FNSetPrivateData`] for more information.
    pub set_private_data: FNSetPrivateData,
    ///See [`FNGetPrivateData`] for more information.
    pub get_private_data: FNGetPrivateData,
}
impl DeviceExtPrivateDataVTable {
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
            create_private_data_slot: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreatePrivateDataSlotEXT").as_ptr()))
            },
            destroy_private_data_slot: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroyPrivateDataSlotEXT").as_ptr()))
            },
            set_private_data: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSetPrivateDataEXT").as_ptr()))
            },
            get_private_data: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetPrivateDataEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_private_data_slot`]. See [`FNCreatePrivateDataSlot`] for more
    /// information.
    pub fn create_private_data_slot(&self) -> FNCreatePrivateDataSlot {
        self.create_private_data_slot
    }
    ///Gets [`Self::destroy_private_data_slot`]. See [`FNDestroyPrivateDataSlot`] for more
    /// information.
    pub fn destroy_private_data_slot(&self) -> FNDestroyPrivateDataSlot {
        self.destroy_private_data_slot
    }
    ///Gets [`Self::set_private_data`]. See [`FNSetPrivateData`] for more information.
    pub fn set_private_data(&self) -> FNSetPrivateData {
        self.set_private_data
    }
    ///Gets [`Self::get_private_data`]. See [`FNGetPrivateData`] for more information.
    pub fn get_private_data(&self) -> FNGetPrivateData {
        self.get_private_data
    }
}
