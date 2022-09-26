//![VK_KHR_workgroup_memory_explicit_layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_workgroup_memory_explicit_layout.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_KHR_workgroup_memory_explicit_layout`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_workgroup_memory_explicit_layout.html)
//!SPIR-V extension, which allows shaders to explicitly define the layout of
//!`Workgroup` storage class memory and create aliases between variables
//!from that storage class in a compute shader.The aliasing feature allows different “views” on the
//! same data, so the
//!shader can bulk copy data from another storage class using one type (e.g. an
//!array of large vectors), and then use the data with a more specific type.
//!It also enables reducing the amount of workgroup memory consumed by allowing
//!the shader to alias data whose lifetimes do not overlap.The explicit layout support and some
//! form of aliasing is also required for
//!layering OpenCL on top of Vulkan.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Caio Marcelo de Oliveira Filho [cmarcelo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_workgroup_memory_explicit_layout]
//!   @cmarcelo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_workgroup_memory_explicit_layout extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]
//!# New constants
//! - [`KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME`]
//! - [`KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
//!# Version history
//! - Revision 1, 2020-06-01 (Caio Marcelo de Oliveira Filho)  - Initial version
//!# Other information
//! * 2020-06-01
//! * No known IP claims.
//! * - This extension requires [`SPV_KHR_workgroup_memory_explicit_layout`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_workgroup_memory_explicit_layout.html)
//!   - This extension provides API support for [`GL_EXT_shared_memory_block`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shared_memory_block.txt)
//! * - Caio Marcelo de Oliveira Filho, Intel  - Jeff Bolz, NVIDIA  - Graeme Leese, Broadcom  -
//!   Jason Ekstrand, Intel  - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_workgroup_memory_explicit_layout");
///[VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html) - Structure describing the workgroup storage explicit layout features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] structure
///is defined as:
///```c
///// Provided by VK_KHR_workgroup_memory_explicit_layout
///typedef struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           workgroupMemoryExplicitLayout;
///    VkBool32           workgroupMemoryExplicitLayoutScalarBlockLayout;
///    VkBool32           workgroupMemoryExplicitLayout8BitAccess;
///    VkBool32           workgroupMemoryExplicitLayout16BitAccess;
///} VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`workgroup_memory_explicit_layout`] indicates whether the implementation supports the SPIR-V
///   `WorkgroupMemoryExplicitLayoutKHR` capability.
/// - [`workgroup_memory_explicit_layout_scalar_block_layout`] indicates whether the implementation
///   supports scalar alignment for laying out Workgroup Blocks.
/// - [`workgroup_memory_explicit_layout8_bit_access`] indicates whether objects in the `Workgroup`
///   storage class with the `Block` decoration  **can**  have 8-bit integer members. If this
///   feature is not enabled, 8-bit integer members  **must**  not be used in such objects. This
///   also indicates whether shader modules  **can**  declare the
///   `WorkgroupMemoryExplicitLayout8BitAccessKHR` capability.
/// - [`workgroup_memory_explicit_layout16_bit_access`] indicates whether objects in the `Workgroup`
///   storage class with the `Block` decoration  **can**  have 16-bit integer and 16-bit
///   floating-point members. If this feature is not enabled, 16-bit integer or 16-bit
///   floating-point members  **must**  not be used in such objects. This also indicates whether
///   shader modules  **can**  declare the `WorkgroupMemoryExplicitLayout16BitAccessKHR` capability.
///If the [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
///# Related
/// - [`khr_workgroup_memory_explicit_layout`]
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
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`workgroup_memory_explicit_layout`] indicates whether the implementation
    ///supports the SPIR-V `WorkgroupMemoryExplicitLayoutKHR` capability.
    pub workgroup_memory_explicit_layout: Bool32,
    ///[`workgroup_memory_explicit_layout_scalar_block_layout`] indicates whether
    ///the implementation supports scalar alignment for laying out Workgroup
    ///Blocks.
    pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    ///[`workgroup_memory_explicit_layout8_bit_access`] indicates whether objects
    ///in the `Workgroup` storage class with the `Block` decoration  **can**
    ///have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`WorkgroupMemoryExplicitLayout8BitAccessKHR` capability.
    pub workgroup_memory_explicit_layout8_bit_access: Bool32,
    ///[`workgroup_memory_explicit_layout16_bit_access`] indicates whether objects
    ///in the `Workgroup` storage class with the `Block` decoration  **can**
    ///have 16-bit integer and 16-bit floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members  **must**  not be used in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`WorkgroupMemoryExplicitLayout16BitAccessKHR` capability.
    pub workgroup_memory_explicit_layout16_bit_access: Bool32,
}
impl<'lt> Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            workgroup_memory_explicit_layout: 0,
            workgroup_memory_explicit_layout_scalar_block_layout: 0,
            workgroup_memory_explicit_layout8_bit_access: 0,
            workgroup_memory_explicit_layout16_bit_access: 0,
        }
    }
}
impl<'lt> PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::workgroup_memory_explicit_layout`]
    pub fn workgroup_memory_explicit_layout_raw(&self) -> Bool32 {
        self.workgroup_memory_explicit_layout
    }
    ///Gets the raw value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn workgroup_memory_explicit_layout_scalar_block_layout_raw(&self) -> Bool32 {
        self.workgroup_memory_explicit_layout_scalar_block_layout
    }
    ///Gets the raw value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn workgroup_memory_explicit_layout8_bit_access_raw(&self) -> Bool32 {
        self.workgroup_memory_explicit_layout8_bit_access
    }
    ///Gets the raw value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn workgroup_memory_explicit_layout16_bit_access_raw(&self) -> Bool32 {
        self.workgroup_memory_explicit_layout16_bit_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout`]
    pub fn set_workgroup_memory_explicit_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.workgroup_memory_explicit_layout = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn set_workgroup_memory_explicit_layout_scalar_block_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn set_workgroup_memory_explicit_layout8_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.workgroup_memory_explicit_layout8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn set_workgroup_memory_explicit_layout16_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.workgroup_memory_explicit_layout16_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout`]
    pub fn with_workgroup_memory_explicit_layout_raw(mut self, value: Bool32) -> Self {
        self.workgroup_memory_explicit_layout = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn with_workgroup_memory_explicit_layout_scalar_block_layout_raw(mut self, value: Bool32) -> Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn with_workgroup_memory_explicit_layout8_bit_access_raw(mut self, value: Bool32) -> Self {
        self.workgroup_memory_explicit_layout8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn with_workgroup_memory_explicit_layout16_bit_access_raw(mut self, value: Bool32) -> Self {
        self.workgroup_memory_explicit_layout16_bit_access = value;
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
    ///Gets the value of [`Self::workgroup_memory_explicit_layout`]
    pub fn workgroup_memory_explicit_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.workgroup_memory_explicit_layout as u8) }
    }
    ///Gets the value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn workgroup_memory_explicit_layout_scalar_block_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.workgroup_memory_explicit_layout_scalar_block_layout as u8) }
    }
    ///Gets the value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn workgroup_memory_explicit_layout8_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.workgroup_memory_explicit_layout8_bit_access as u8) }
    }
    ///Gets the value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn workgroup_memory_explicit_layout16_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.workgroup_memory_explicit_layout16_bit_access as u8) }
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
    ///Gets a mutable reference to the value of [`Self::workgroup_memory_explicit_layout`]
    pub fn workgroup_memory_explicit_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.workgroup_memory_explicit_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.workgroup_memory_explicit_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn workgroup_memory_explicit_layout_scalar_block_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.workgroup_memory_explicit_layout_scalar_block_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.workgroup_memory_explicit_layout_scalar_block_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn workgroup_memory_explicit_layout8_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.workgroup_memory_explicit_layout8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.workgroup_memory_explicit_layout8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn workgroup_memory_explicit_layout16_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.workgroup_memory_explicit_layout16_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.workgroup_memory_explicit_layout16_bit_access as *mut Bool32)
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
    ///Sets the value of [`Self::workgroup_memory_explicit_layout`]
    pub fn set_workgroup_memory_explicit_layout(&mut self, value: bool) -> &mut Self {
        self.workgroup_memory_explicit_layout = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn set_workgroup_memory_explicit_layout_scalar_block_layout(&mut self, value: bool) -> &mut Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn set_workgroup_memory_explicit_layout8_bit_access(&mut self, value: bool) -> &mut Self {
        self.workgroup_memory_explicit_layout8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn set_workgroup_memory_explicit_layout16_bit_access(&mut self, value: bool) -> &mut Self {
        self.workgroup_memory_explicit_layout16_bit_access = value as u8 as u32;
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
    ///Sets the value of [`Self::workgroup_memory_explicit_layout`]
    pub fn with_workgroup_memory_explicit_layout(mut self, value: bool) -> Self {
        self.workgroup_memory_explicit_layout = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout_scalar_block_layout`]
    pub fn with_workgroup_memory_explicit_layout_scalar_block_layout(mut self, value: bool) -> Self {
        self.workgroup_memory_explicit_layout_scalar_block_layout = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout8_bit_access`]
    pub fn with_workgroup_memory_explicit_layout8_bit_access(mut self, value: bool) -> Self {
        self.workgroup_memory_explicit_layout8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::workgroup_memory_explicit_layout16_bit_access`]
    pub fn with_workgroup_memory_explicit_layout16_bit_access(mut self, value: bool) -> Self {
        self.workgroup_memory_explicit_layout16_bit_access = value as u8 as u32;
        self
    }
}
