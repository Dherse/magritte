//![VK_EXT_buffer_device_address](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_buffer_device_address.html) - device extension
//!# Description
//!This extension allows the application to query a 64-bit buffer device
//!address value for a buffer, which can be used to access the buffer memory
//!via the `PhysicalStorageBufferEXT` storage class in the
//![`GL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
//!GLSL extension and
//![`SPV_EXT_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_physical_storage_buffer.html)
//!SPIR-V extension.It also allows buffer device addresses to be provided by a trace replay
//!tool, so that it matches the address used when the trace was captured.
//!# Revision
//!2
//!# Dependencies
//! - *Deprecated* by `[`khr_buffer_device_address`]` extension  - Which in turn was *promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_buffer_device_address]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_buffer_device_address extension>>)
//!# New functions & commands
//! - [`get_buffer_device_address_ext`]
//!# New structures
//! - [`BufferDeviceAddressInfoEXT`]
//! - Extending [`BufferCreateInfo`]:  - [`BufferDeviceAddressCreateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceBufferAddressFeaturesEXT`]  - [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]
//!# New constants
//! - [`EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME`]
//! - [`EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION`]
//! - Extending [`BufferCreateFlagBits`]:  -
//!   `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_DEVICE_ADDRESS_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Where is VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT
//!and VkPhysicalDeviceBufferAddressFeaturesEXT? **RESOLVED** : They were renamed as
//!`VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
//!and [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] accordingly for
//!consistency.
//!Even though, the old names can still be found in the generated header files
//!for compatibility.
//!# Version History
//! - Revision 1, 2018-11-01 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2019-01-06 (Jon Leech)  - Minor updates to appendix for publication
//!# Other info
//! * 2019-01-06
//! * No known IP claims.
//! * - This extension requires [`SPV_EXT_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_physical_storage_buffer.html)
//!   - This extension provides API support for [`GLSL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
//!   and [`GLSL_EXT_buffer_reference_uvec2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference_uvec2.txt)
//! * - Jeff Bolz, NVIDIA  - Neil Henning, AMD  - Tobias Hector, AMD  - Jason Ekstrand, Intel  -
//!   Baldur Karlsson, Valve
//!# Related
//! - [`BufferDeviceAddressCreateInfoEXT`]
//! - [`BufferDeviceAddressInfoEXT`]
//! - [`PhysicalDeviceBufferAddressFeaturesEXT`]
//! - [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]
//! - [`get_buffer_device_address_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, DeviceAddress, StructureType},
    vulkan1_2::FNGetBufferDeviceAddress,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_buffer_device_address");
///[VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) - Structure describing buffer address features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           bufferDeviceAddress;
///    VkBool32           bufferDeviceAddressCaptureReplay;
///    VkBool32           bufferDeviceAddressMultiDevice;
///} VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
///```
///
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef VkPhysicalDeviceBufferDeviceAddressFeaturesEXT VkPhysicalDeviceBufferAddressFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer_device_address`] indicates that the implementation supports accessing buffer memory
///   in shaders as storage buffers via an address queried from [`get_buffer_device_address_ext`].
/// - [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and
///   reusing buffer addresses, e.g. for trace capture and replay.
/// - [`buffer_device_address_multi_device`] indicates that the implementation supports the
///   [`buffer_device_address`] feature for logical devices created with multiple physical devices.
///   If this feature is not supported, buffer addresses  **must**  not be queried on a logical
///   device created with more than one physical device.
///If the [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
///# Related
/// - [`ext_buffer_device_address`]
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
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`buffer_device_address`] indicates
    ///that the implementation supports accessing buffer memory in shaders as
    ///storage buffers via an address queried from
    ///[`get_buffer_device_address_ext`].
    pub buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer addresses, e.g. for trace capture and
    ///replay.
    pub buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`] feature for logical devices
    ///created with multiple physical devices.
    ///If this feature is not supported, buffer addresses  **must**  not be queried
    ///on a logical device created with more than one physical device.
    pub buffer_device_address_multi_device: Bool32,
}
impl<'lt> Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            buffer_device_address: 0,
            buffer_device_address_capture_replay: 0,
            buffer_device_address_multi_device: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBufferDeviceAddressFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_raw(&self) -> Bool32 {
        self.buffer_device_address
    }
    ///Gets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_raw(&self) -> Bool32 {
        self.buffer_device_address_capture_replay
    }
    ///Gets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_raw(&self) -> Bool32 {
        self.buffer_device_address_multi_device
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address_raw(mut self, value: Bool32) -> Self {
        self.buffer_device_address = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay_raw(mut self, value: Bool32) -> Self {
        self.buffer_device_address_capture_replay = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device_raw(mut self, value: Bool32) -> Self {
        self.buffer_device_address_multi_device = value;
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
    ///Gets the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_capture_replay as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_multi_device as u8) }
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
    ///Gets a mutable reference to the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address(mut self, value: bool) -> Self {
        self.buffer_device_address = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay(mut self, value: bool) -> Self {
        self.buffer_device_address_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device(mut self, value: bool) -> Self {
        self.buffer_device_address_multi_device = value as u8 as u32;
        self
    }
}
///[VkBufferDeviceAddressCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) - Request a specific address for a buffer
///# C Specifications
///Alternatively, to
///request a specific device address for a buffer, add a
///[`BufferDeviceAddressCreateInfoEXT`] structure to the [`p_next`] chain
///of the [`BufferCreateInfo`] structure.
///The [`BufferDeviceAddressCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef struct VkBufferDeviceAddressCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceAddress    deviceAddress;
///} VkBufferDeviceAddressCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_address`] is the device address requested for the buffer.
///# Description
///If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not
/// zero, then it  **must**  be an address retrieved
///from an identically created buffer on the same implementation.
///The buffer  **must**  also be bound to an identically created
///[`DeviceMemory`] object.If this structure is not present, it is as if [`device_address`] is
/// zero.Apps  **should**  avoid creating buffers with app-provided addresses and
///implementation-provided addresses in the same process, to reduce the
///likelihood of `VK_ERROR_INVALID_DEVICE_ADDRESS_EXT` errors.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`
///# Related
/// - [`ext_buffer_device_address`]
/// - [`DeviceAddress`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`device_address`] is the device address requested for the buffer.
    pub device_address: DeviceAddress,
}
impl<'lt> Default for BufferDeviceAddressCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            device_address: Default::default(),
        }
    }
}
impl<'lt> BufferDeviceAddressCreateInfoEXT<'lt> {
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
    ///Gets the value of [`Self::device_address`]
    pub fn device_address(&self) -> DeviceAddress {
        self.device_address
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_address`]
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
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
    ///Sets the value of [`Self::device_address`]
    pub fn set_device_address(mut self, value: crate::vulkan1_0::DeviceAddress) -> Self {
        self.device_address = value;
        self
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_buffer_device_address`
pub struct DeviceExtBufferDeviceAddressVTable {
    ///See [`FNGetBufferDeviceAddress`] for more information.
    pub get_buffer_device_address_ext: FNGetBufferDeviceAddress,
}
impl DeviceExtBufferDeviceAddressVTable {
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
            get_buffer_device_address_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetBufferDeviceAddressEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_buffer_device_address_ext`]. See [`FNGetBufferDeviceAddress`] for more
    /// information.
    pub fn get_buffer_device_address_ext(&self) -> FNGetBufferDeviceAddress {
        self.get_buffer_device_address_ext
    }
}
