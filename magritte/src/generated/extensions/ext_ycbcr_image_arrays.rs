//![VK_EXT_ycbcr_image_arrays](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_image_arrays.html) - device extension
//!# Description
//!This extension allows images of a format that requires
//![Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) to be
//!created with multiple array layers, which is otherwise restricted.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_sampler_ycbcr_conversion`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_ycbcr_image_arrays]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_ycbcr_image_arrays extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`]
//!# New constants
//! - [`EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME`]
//! - [`EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-01-15 (Piers Daniell)  - Initial revision
//!# Other info
//! * 2019-01-15
//! * - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`]
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
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION")]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME")]
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_ycbcr_image_arrays");
///[VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html) - Structure describing extended Y′C<sub>B</sub>C<sub>R</sub> image creation features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_ycbcr_image_arrays
///typedef struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           ycbcrImageArrays;
///} VkPhysicalDeviceYcbcrImageArraysFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ycbcr_image_arrays`] indicates that the implementation supports creating images with a format that requires [Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and has multiple array layers.
///If the [`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceYcbcrImageArraysFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT`
///# Related
/// - [`ext_ycbcr_image_arrays`]
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
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`ycbcr_image_arrays`] indicates that the
    ///implementation supports creating images with a format that requires
    ///[Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) and
    ///has multiple array layers.
    pub ycbcr_image_arrays: Bool32,
}
impl<'lt> Default for PhysicalDeviceYcbcrImageArraysFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            ycbcr_image_arrays: 0,
        }
    }
}
impl<'lt> PhysicalDeviceYcbcrImageArraysFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::ycbcr_image_arrays`]
    pub fn ycbcr_image_arrays_raw(&self) -> Bool32 {
        self.ycbcr_image_arrays
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ycbcr_image_arrays`]
    pub fn set_ycbcr_image_arrays_raw(mut self, value: Bool32) -> Self {
        self.ycbcr_image_arrays = value;
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
    ///Gets the value of [`Self::ycbcr_image_arrays`]
    pub fn ycbcr_image_arrays(&self) -> bool {
        unsafe { std::mem::transmute(self.ycbcr_image_arrays as u8) }
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
    ///Gets a mutable reference to the value of [`Self::ycbcr_image_arrays`]
    pub fn ycbcr_image_arrays_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ycbcr_image_arrays as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ycbcr_image_arrays as *mut Bool32)
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
    ///Sets the value of [`Self::ycbcr_image_arrays`]
    pub fn set_ycbcr_image_arrays(mut self, value: bool) -> Self {
        self.ycbcr_image_arrays = value as u8 as u32;
        self
    }
}
