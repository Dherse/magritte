//![VK_NV_dedicated_allocation_image_aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation_image_aliasing.html) - device extension
//!# Description
//!This extension allows applications to alias images on dedicated allocations,
//!subject to specific restrictions: the extent and the number of layers in the
//!image being aliased must be smaller than or equal to those of the original
//!image for which the allocation was created, and every other image parameter
//!must match.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_dedicated_allocation`]`
//!# Contacts
//! - Nuno Subtil [nsubtil](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_dedicated_allocation_image_aliasing]
//!   @nsubtil%0A<<Here describe the issue or question you have about the
//!   VK_NV_dedicated_allocation_image_aliasing extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]
//!# New constants
//! - [`NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME`]
//! - [`NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`
//!# Version History
//! - Revision 1, 2019-01-04 (Nuno Subtil)  - Internal revisions
//!# Other info
//! * 2019-01-04
//! * - Nuno Subtil, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA  - Axel Gneiting, id
//!   Software
//!# Related
//! - [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]
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
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_dedicated_allocation_image_aliasing");
///[VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html) - Structure describing dedicated allocation image aliasing features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`]
///structure is defined as:
///```c
///// Provided by VK_NV_dedicated_allocation_image_aliasing
///typedef struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           dedicatedAllocationImageAliasing;
///} VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`dedicated_allocation_image_aliasing`] indicates that the implementation supports aliasing of
///   compatible image objects on a dedicated allocation.
///If the [`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV`] **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV`
///# Related
/// - [`VK_NV_dedicated_allocation_image_aliasing`]
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
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`dedicated_allocation_image_aliasing`] indicates that the implementation
    ///supports aliasing of compatible image objects on a dedicated allocation.
    pub dedicated_allocation_image_aliasing: Bool32,
}
impl<'lt> Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNv,
            p_next: std::ptr::null_mut(),
            dedicated_allocation_image_aliasing: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dedicated_allocation_image_aliasing`]
    pub fn dedicated_allocation_image_aliasing_raw(&self) -> Bool32 {
        self.dedicated_allocation_image_aliasing
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dedicated_allocation_image_aliasing`]
    pub fn set_dedicated_allocation_image_aliasing_raw(mut self, value: Bool32) -> Self {
        self.dedicated_allocation_image_aliasing = value;
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
    ///Gets the value of [`Self::dedicated_allocation_image_aliasing`]
    pub fn dedicated_allocation_image_aliasing(&self) -> bool {
        unsafe { std::mem::transmute(self.dedicated_allocation_image_aliasing as u8) }
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
    ///Gets a mutable reference to the value of [`Self::dedicated_allocation_image_aliasing`]
    pub fn dedicated_allocation_image_aliasing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dedicated_allocation_image_aliasing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dedicated_allocation_image_aliasing as *mut Bool32)
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
    ///Sets the value of [`Self::dedicated_allocation_image_aliasing`]
    pub fn set_dedicated_allocation_image_aliasing(mut self, value: bool) -> Self {
        self.dedicated_allocation_image_aliasing = value as u8 as u32;
        self
    }
}
