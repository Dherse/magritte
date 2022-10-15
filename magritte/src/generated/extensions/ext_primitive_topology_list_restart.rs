//![VK_EXT_primitive_topology_list_restart](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_primitive_topology_list_restart.html) - device extension
//!# Description
//!This extension allows list primitives to use the primitive restart index
//!value.
//!This provides a more efficient implementation when layering OpenGL
//!functionality on Vulkan by avoiding emulation which incurs data copies.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_primitive_topology_list_restart]
//!   @syoussefi%0A<<Here describe the issue or question you have about the
//!   VK_EXT_primitive_topology_list_restart extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]
//!# New constants
//! - [`EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME`]
//! - [`EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`
//!# Version history
//! - Revision 0, 2020-09-14 (Courtney Goeltzenleuchter)  - Internal revisions
//! - Revision 1, 2021-01-11 (Shahbaz Youssefi)  - Add the `primitiveTopologyPatchListRestart`
//!   feature  - Internal revisions
//!# Other information
//! * 2021-01-11
//! * No known IP claims.
//! * - Courtney Goeltzenleuchter, Google  - Shahbaz Youssefi, Google
//!# Related
//! - [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]
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
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_primitive_topology_list_restart");
///[VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html) - Structure describing whether list type primitives can support primitive restart
///# C Specifications
///The [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure
///is defined as:
///```c
///// Provided by VK_EXT_primitive_topology_list_restart
///typedef struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           primitiveTopologyListRestart;
///    VkBool32           primitiveTopologyPatchListRestart;
///} VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
///```
/// # Members
/// The members of the
/// [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure
/// describe the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`primitive_topology_list_restart`] indicates that list type primitives,
///   `VK_PRIMITIVE_TOPOLOGY_POINT_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST`,
///   `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY` and
///   `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY`,  **can**  use the primitive restart
///   index value in index buffers.
/// - [`primitive_topology_patch_list_restart`] indicates that the
///   `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST` topology  **can**  use the primitive restart index value in
///   index buffers.
/// If the [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] **can**  also be used in the
/// [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`
/// # Related
/// - [`ext_primitive_topology_list_restart`]
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
#[doc(alias = "VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`primitive_topology_list_restart`] indicates that list type primitives,
    ///`VK_PRIMITIVE_TOPOLOGY_POINT_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_LINE_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY` and
    ///`VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY`,  **can**  use the
    ///primitive restart index value in index buffers.
    pub primitive_topology_list_restart: Bool32,
    ///[`primitive_topology_patch_list_restart`] indicates that the
    ///`VK_PRIMITIVE_TOPOLOGY_PATCH_LIST` topology  **can**  use the primitive
    ///restart index value in index buffers.
    pub primitive_topology_patch_list_restart: Bool32,
}
impl<'lt> Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            primitive_topology_list_restart: 0,
            primitive_topology_patch_list_restart: 0,
        }
    }
}
impl<'lt> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::primitive_topology_list_restart`]
    pub fn primitive_topology_list_restart_raw(&self) -> Bool32 {
        self.primitive_topology_list_restart
    }
    ///Gets the raw value of [`Self::primitive_topology_patch_list_restart`]
    pub fn primitive_topology_patch_list_restart_raw(&self) -> Bool32 {
        self.primitive_topology_patch_list_restart
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_topology_list_restart`]
    pub fn set_primitive_topology_list_restart_raw(&mut self, value: Bool32) -> &mut Self {
        self.primitive_topology_list_restart = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_topology_patch_list_restart`]
    pub fn set_primitive_topology_patch_list_restart_raw(&mut self, value: Bool32) -> &mut Self {
        self.primitive_topology_patch_list_restart = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_topology_list_restart`]
    pub fn with_primitive_topology_list_restart_raw(mut self, value: Bool32) -> Self {
        self.primitive_topology_list_restart = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_topology_patch_list_restart`]
    pub fn with_primitive_topology_patch_list_restart_raw(mut self, value: Bool32) -> Self {
        self.primitive_topology_patch_list_restart = value;
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
    ///Gets the value of [`Self::primitive_topology_list_restart`]
    pub fn primitive_topology_list_restart(&self) -> bool {
        unsafe { std::mem::transmute(self.primitive_topology_list_restart as u8) }
    }
    ///Gets the value of [`Self::primitive_topology_patch_list_restart`]
    pub fn primitive_topology_patch_list_restart(&self) -> bool {
        unsafe { std::mem::transmute(self.primitive_topology_patch_list_restart as u8) }
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
    ///Gets a mutable reference to the value of [`Self::primitive_topology_list_restart`]
    pub fn primitive_topology_list_restart_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.primitive_topology_list_restart as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.primitive_topology_list_restart as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::primitive_topology_patch_list_restart`]
    pub fn primitive_topology_patch_list_restart_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.primitive_topology_patch_list_restart as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.primitive_topology_patch_list_restart as *mut Bool32)
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
    ///Sets the value of [`Self::primitive_topology_list_restart`]
    pub fn set_primitive_topology_list_restart(&mut self, value: bool) -> &mut Self {
        self.primitive_topology_list_restart = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::primitive_topology_patch_list_restart`]
    pub fn set_primitive_topology_patch_list_restart(&mut self, value: bool) -> &mut Self {
        self.primitive_topology_patch_list_restart = value as u8 as u32;
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
    ///Sets the value of [`Self::primitive_topology_list_restart`]
    pub fn with_primitive_topology_list_restart(mut self, value: bool) -> Self {
        self.primitive_topology_list_restart = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::primitive_topology_patch_list_restart`]
    pub fn with_primitive_topology_patch_list_restart(mut self, value: bool) -> Self {
        self.primitive_topology_patch_list_restart = value as u8 as u32;
        self
    }
}
