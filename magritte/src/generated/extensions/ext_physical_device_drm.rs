use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION")]
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME")]
pub const EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_physical_device_drm");
///[VkPhysicalDeviceDrmPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html) - Structure containing DRM information of a physical device
///# C Specifications
///The [`PhysicalDeviceDrmPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_physical_device_drm
///typedef struct VkPhysicalDeviceDrmPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           hasPrimary;
///    VkBool32           hasRender;
///    int64_t            primaryMajor;
///    int64_t            primaryMinor;
///    int64_t            renderMajor;
///    int64_t            renderMinor;
///} VkPhysicalDeviceDrmPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`has_primary`] is a boolean indicating whether the physical device has a DRM primary node.
/// - [`has_render`] is a boolean indicating whether the physical device has a DRM render node.
/// - [`primary_major`] is the DRM primary node major number, if any.
/// - [`primary_minor`] is the DRM primary node minor number, if any.
/// - [`render_major`] is the DRM render node major number, if any.
/// - [`render_minor`] is the DRM render node minor number, if any.
///# Description
///If the [`PhysicalDeviceDrmPropertiesEXT`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These are properties of the DRM information of a
/// physical device.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_physical_device_drm`]
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
pub struct PhysicalDeviceDrmPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`has_primary`] is a boolean indicating whether the physical device has
    ///a DRM primary node.
    has_primary: Bool32,
    ///[`has_render`] is a boolean indicating whether the physical device has
    ///a DRM render node.
    has_render: Bool32,
    ///[`primary_major`] is the DRM primary node major number, if any.
    primary_major: i64,
    ///[`primary_minor`] is the DRM primary node minor number, if any.
    primary_minor: i64,
    ///[`render_major`] is the DRM render node major number, if any.
    render_major: i64,
    ///[`render_minor`] is the DRM render node minor number, if any.
    render_minor: i64,
}
impl<'lt> Default for PhysicalDeviceDrmPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            has_primary: 0,
            has_render: 0,
            primary_major: 0,
            primary_minor: 0,
            render_major: 0,
            render_minor: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDrmPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::has_primary`]
    pub fn has_primary_raw(&self) -> Bool32 {
        self.has_primary
    }
    ///Gets the raw value of [`Self::has_render`]
    pub fn has_render_raw(&self) -> Bool32 {
        self.has_render
    }
    ///Gets the raw value of [`Self::primary_major`]
    pub fn primary_major_raw(&self) -> i64 {
        self.primary_major
    }
    ///Gets the raw value of [`Self::primary_minor`]
    pub fn primary_minor_raw(&self) -> i64 {
        self.primary_minor
    }
    ///Gets the raw value of [`Self::render_major`]
    pub fn render_major_raw(&self) -> i64 {
        self.render_major
    }
    ///Gets the raw value of [`Self::render_minor`]
    pub fn render_minor_raw(&self) -> i64 {
        self.render_minor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::has_primary`]
    pub fn set_has_primary_raw(&mut self, value: Bool32) -> &mut Self {
        self.has_primary = value;
        self
    }
    ///Sets the raw value of [`Self::has_render`]
    pub fn set_has_render_raw(&mut self, value: Bool32) -> &mut Self {
        self.has_render = value;
        self
    }
    ///Sets the raw value of [`Self::primary_major`]
    pub fn set_primary_major_raw(&mut self, value: i64) -> &mut Self {
        self.primary_major = value;
        self
    }
    ///Sets the raw value of [`Self::primary_minor`]
    pub fn set_primary_minor_raw(&mut self, value: i64) -> &mut Self {
        self.primary_minor = value;
        self
    }
    ///Sets the raw value of [`Self::render_major`]
    pub fn set_render_major_raw(&mut self, value: i64) -> &mut Self {
        self.render_major = value;
        self
    }
    ///Sets the raw value of [`Self::render_minor`]
    pub fn set_render_minor_raw(&mut self, value: i64) -> &mut Self {
        self.render_minor = value;
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
    ///Gets the value of [`Self::has_primary`]
    pub fn has_primary(&self) -> bool {
        unsafe { std::mem::transmute(self.has_primary as u8) }
    }
    ///Gets the value of [`Self::has_render`]
    pub fn has_render(&self) -> bool {
        unsafe { std::mem::transmute(self.has_render as u8) }
    }
    ///Gets the value of [`Self::primary_major`]
    pub fn primary_major(&self) -> i64 {
        self.primary_major
    }
    ///Gets the value of [`Self::primary_minor`]
    pub fn primary_minor(&self) -> i64 {
        self.primary_minor
    }
    ///Gets the value of [`Self::render_major`]
    pub fn render_major(&self) -> i64 {
        self.render_major
    }
    ///Gets the value of [`Self::render_minor`]
    pub fn render_minor(&self) -> i64 {
        self.render_minor
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
    ///Gets a mutable reference to the value of [`Self::has_primary`]
    pub fn has_primary_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.has_primary as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.has_primary as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::has_render`]
    pub fn has_render_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.has_render as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.has_render as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::primary_major`]
    pub fn primary_major_mut(&mut self) -> &mut i64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::primary_minor`]
    pub fn primary_minor_mut(&mut self) -> &mut i64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::render_major`]
    pub fn render_major_mut(&mut self) -> &mut i64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::render_minor`]
    pub fn render_minor_mut(&mut self) -> &mut i64 {
        &mut getter
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
    ///Sets the raw value of [`Self::has_primary`]
    pub fn set_has_primary(&mut self, value: bool) -> &mut Self {
        self.has_primary = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::has_render`]
    pub fn set_has_render(&mut self, value: bool) -> &mut Self {
        self.has_render = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::primary_major`]
    pub fn set_primary_major(&mut self, value: i64) -> &mut Self {
        self.primary_major = value;
        self
    }
    ///Sets the raw value of [`Self::primary_minor`]
    pub fn set_primary_minor(&mut self, value: i64) -> &mut Self {
        self.primary_minor = value;
        self
    }
    ///Sets the raw value of [`Self::render_major`]
    pub fn set_render_major(&mut self, value: i64) -> &mut Self {
        self.render_major = value;
        self
    }
    ///Sets the raw value of [`Self::render_minor`]
    pub fn set_render_minor(&mut self, value: i64) -> &mut Self {
        self.render_minor = value;
        self
    }
}
