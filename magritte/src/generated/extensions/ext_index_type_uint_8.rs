//![VK_EXT_index_type_uint8](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_index_type_uint8.html) - device extension
//!# Description
//!This extension allows `uint8_t` indices to be used with
//![`CmdBindIndexBuffer`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_index_type_uint8]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_index_type_uint8 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceIndexTypeUint8FeaturesEXT`]
//!# New constants
//! - [`EXT_INDEX_TYPE_UINT8_EXTENSION_NAME`]
//! - [`EXT_INDEX_TYPE_UINT8_SPEC_VERSION`]
//! - Extending [`IndexType`]:  - `VK_INDEX_TYPE_UINT8_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-05-02 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2019-05-02
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceIndexTypeUint8FeaturesEXT`]
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
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_index_type_uint8");
///[VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html) - Structure describing whether uint8 index type can be used
///# C Specifications
///The [`PhysicalDeviceIndexTypeUint8FeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_index_type_uint8
///typedef struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           indexTypeUint8;
///} VkPhysicalDeviceIndexTypeUint8FeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`index_type_uint_8`] indicates that `VK_INDEX_TYPE_UINT8_EXT` can be used with
///   [`CmdBindIndexBuffer`].
///If the [`PhysicalDeviceIndexTypeUint8FeaturesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceIndexTypeUint8FeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT`
///# Related
/// - [`VK_EXT_index_type_uint8`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`index_type_uint_8`] indicates that
    ///`VK_INDEX_TYPE_UINT8_EXT` can be used with
    ///[`CmdBindIndexBuffer`].
    index_type_uint_8: Bool32,
}
impl<'lt> Default for PhysicalDeviceIndexTypeUint8FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            index_type_uint_8: 0,
        }
    }
}
impl<'lt> PhysicalDeviceIndexTypeUint8FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::index_type_uint_8`]
    pub fn index_type_uint_8_raw(&self) -> Bool32 {
        self.index_type_uint_8
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::index_type_uint_8`]
    pub fn set_index_type_uint_8_raw(&mut self, value: Bool32) -> &mut Self {
        self.index_type_uint_8 = value;
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
    ///Gets the value of [`Self::index_type_uint_8`]
    pub fn index_type_uint_8(&self) -> bool {
        unsafe { std::mem::transmute(self.index_type_uint_8 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::index_type_uint_8`]
    pub fn index_type_uint_8_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.index_type_uint_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.index_type_uint_8 as *mut Bool32)
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
    ///Sets the raw value of [`Self::index_type_uint_8`]
    pub fn set_index_type_uint_8(&mut self, value: bool) -> &mut Self {
        self.index_type_uint_8 = value as u8 as u32;
        self
    }
}
