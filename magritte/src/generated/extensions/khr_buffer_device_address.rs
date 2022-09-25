//![VK_KHR_buffer_device_address](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_buffer_device_address.html) - device extension
//!# Description
//!This extension allows the application to query a 64-bit buffer device
//!address value for a buffer, which can be used to access the buffer memory
//!via the `PhysicalStorageBuffer` storage class in the
//![`GL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
//!GLSL extension and
//![`SPV_KHR_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_physical_storage_buffer.html)
//!SPIR-V extension.Another way to describe this extension is that it adds “pointers to buffer
//!memory in shaders”.
//!By calling [`get_buffer_device_address`] with a [`Buffer`], it will
//!return a [`DeviceAddress`] value which represents the address of the
//!start of the buffer.[`get_buffer_opaque_capture_address`] and
//![`get_device_memory_opaque_capture_address`] allow opaque addresses for
//!buffers and memory objects to be queried for the current process.
//!A trace capture and replay tool can then supply these addresses to be used
//!at replay time to match the addresses used when the trace was captured.
//!To enable tools to insert these queries, new memory allocation flags must be
//!specified for memory objects that will be bound to buffers accessed via the
//!`PhysicalStorageBuffer` storage class.
//!Note that this mechanism is intended only to support capture/replay tools,
//!and is not recommended for use in other applications.There are various use cases this extension
//! is designed for.
//!It is required for ray tracing, useful for DX12 portability, and by allowing
//!buffer addresses to be stored in memory it enables more complex data
//!structures to be created.This extension can also be used to hardcode a dedicated debug channel
//! into
//!all shaders by querying a pointer at startup and pushing that into shaders
//!as a run-time constant (e.g. specialization constant) that avoids impacting
//!other descriptor limits.There are examples of usage in the
//![`GL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
//!spec for how to use this in a high-level shading language such as GLSL.
//!The
//![`GL_EXT_buffer_reference2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference2.txt)
//!and
//![`GL_EXT_buffer_reference_uvec2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference_uvec2.txt)
//!extensions were also added to help cover a few additional edge cases.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_buffer_device_address]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_KHR_buffer_device_address extension>>)
# ! [doc = concat ! ("# " , "New commands")]
//! - [`get_buffer_device_address_khr`]
//! - [`get_buffer_opaque_capture_address_khr`]
//! - [`get_device_memory_opaque_capture_address_khr`]
# ! [doc = concat ! ("# " , "New structures")]
//! - [`BufferDeviceAddressInfoKHR`]
//! - [`DeviceMemoryOpaqueCaptureAddressInfoKHR`]
//! - Extending [`BufferCreateInfo`]:  - [`BufferOpaqueCaptureAddressCreateInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:  - [`MemoryOpaqueCaptureAddressAllocateInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceBufferDeviceAddressFeaturesKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME`]
//! - [`KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION`]
//! - Extending [`BufferCreateFlagBits`]:  -
//!   `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR`
//! - Extending [`MemoryAllocateFlagBits`]:  - `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR`  -
//!   `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2019-06-24 (Jan-Harald Fredriksen)  - Internal revisions based on
//!   VK_EXT_buffer_device_address
//!# Other info
//! * 2019-06-24
//! * No known IP claims.
//! * - Promoted to Vulkan 1.2 Core  - This extension requires [`SPV_KHR_physical_storage_buffer`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_physical_storage_buffer.html)
//!   - This extension provides API support for [`GL_EXT_buffer_reference`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference.txt)
//!   and [`GL_EXT_buffer_reference2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference2.txt)
//!   and [`GL_EXT_buffer_reference_uvec2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_buffer_reference_uvec2.txt)
//! * - Jeff Bolz, NVIDIA  - Neil Henning, AMD  - Tobias Hector, AMD  - Jason Ekstrand, Intel  -
//!   Baldur Karlsson, Valve  - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`BufferDeviceAddressInfoKHR`]
//! - [`BufferOpaqueCaptureAddressCreateInfoKHR`]
//! - [`DeviceMemoryOpaqueCaptureAddressInfoKHR`]
//! - [`MemoryOpaqueCaptureAddressAllocateInfoKHR`]
//! - [`PhysicalDeviceBufferDeviceAddressFeaturesKHR`]
//! - [`get_buffer_device_address_khr`]
//! - [`get_buffer_opaque_capture_address_khr`]
//! - [`get_device_memory_opaque_capture_address_khr`]
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
    vulkan1_2::{FNGetBufferDeviceAddress, FNGetBufferOpaqueCaptureAddress, FNGetDeviceMemoryOpaqueCaptureAddress},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_buffer_device_address");
///The V-table of [`Device`] for functions from `VK_KHR_buffer_device_address`
pub struct DeviceKhrBufferDeviceAddressVTable {
    ///See [`FNGetBufferOpaqueCaptureAddress`] for more information.
    pub get_buffer_opaque_capture_address_khr: FNGetBufferOpaqueCaptureAddress,
    ///See [`FNGetBufferDeviceAddress`] for more information.
    pub get_buffer_device_address_khr: FNGetBufferDeviceAddress,
    ///See [`FNGetDeviceMemoryOpaqueCaptureAddress`] for more information.
    pub get_device_memory_opaque_capture_address_khr: FNGetDeviceMemoryOpaqueCaptureAddress,
}
impl DeviceKhrBufferDeviceAddressVTable {
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
            get_buffer_opaque_capture_address_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetBufferOpaqueCaptureAddressKHR").as_ptr(),
                ))
            },
            get_buffer_device_address_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetBufferDeviceAddressKHR").as_ptr()))
            },
            get_device_memory_opaque_capture_address_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceMemoryOpaqueCaptureAddressKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_buffer_opaque_capture_address_khr`]. See
    /// [`FNGetBufferOpaqueCaptureAddress`] for more information.
    pub fn get_buffer_opaque_capture_address_khr(&self) -> FNGetBufferOpaqueCaptureAddress {
        self.get_buffer_opaque_capture_address_khr
    }
    ///Gets [`Self::get_buffer_device_address_khr`]. See [`FNGetBufferDeviceAddress`] for more
    /// information.
    pub fn get_buffer_device_address_khr(&self) -> FNGetBufferDeviceAddress {
        self.get_buffer_device_address_khr
    }
    ///Gets [`Self::get_device_memory_opaque_capture_address_khr`]. See
    /// [`FNGetDeviceMemoryOpaqueCaptureAddress`] for more information.
    pub fn get_device_memory_opaque_capture_address_khr(&self) -> FNGetDeviceMemoryOpaqueCaptureAddress {
        self.get_device_memory_opaque_capture_address_khr
    }
}
