use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_ARM_rasterization_order_attachment_access");
///[VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html) - Structure describing whether rasterization order attachment access can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure is defined as:
///```c
///// Provided by VK_ARM_rasterization_order_attachment_access
///typedef struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rasterizationOrderColorAttachmentAccess;
///    VkBool32           rasterizationOrderDepthAttachmentAccess;
///    VkBool32           rasterizationOrderStencilAttachmentAccess;
///} VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
///```
///# Members
///The members of the
///[`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure describe the following features:
///# Description
/// - [`rasterization_order_color_attachment_access`] indicates that rasterization order access to
///   color and input attachments is supported by the implementation.
/// - [`rasterization_order_depth_attachment_access`] indicates that rasterization order access to
///   the depth aspect of depth/stencil and input attachments is supported by the implementation.
/// - [`rasterization_order_stencil_attachment_access`] indicates that rasterization order access to
///   the stencil aspect of depth/stencil and input attachments is supported by the implementation.
///If the [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure is included in the [`p_next`] chain of
///[`PhysicalDeviceFeatures2`], it is filled with values indicating whether
///the feature is supported.
///[`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]**can**
///also be used in the [`p_next`] chain of [`DeviceCreateInfo`] to enable
///features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
///# Related
/// - [`VK_ARM_rasterization_order_attachment_access`]
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
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`rasterization_order_color_attachment_access`] indicates that
    ///rasterization order access to color and input attachments is supported
    ///by the implementation.
    rasterization_order_color_attachment_access: Bool32,
    ///[`rasterization_order_depth_attachment_access`] indicates that
    ///rasterization order access to the depth aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    rasterization_order_depth_attachment_access: Bool32,
    ///[`rasterization_order_stencil_attachment_access`] indicates that
    ///rasterization order access to the stencil aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    rasterization_order_stencil_attachment_access: Bool32,
}
impl<'lt> Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            rasterization_order_color_attachment_access: 0,
            rasterization_order_depth_attachment_access: 0,
            rasterization_order_stencil_attachment_access: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_color_attachment_access
    }
    ///Gets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_depth_attachment_access
    }
    ///Gets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_stencil_attachment_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn set_rasterization_order_color_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_color_attachment_access = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn set_rasterization_order_depth_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_depth_attachment_access = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn set_rasterization_order_stencil_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_stencil_attachment_access = value;
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
    ///Gets the value of [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_color_attachment_access as u8) }
    }
    ///Gets the value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_depth_attachment_access as u8) }
    }
    ///Gets the value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_stencil_attachment_access as u8) }
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
    ///Gets a mutable reference to the value of
    /// [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_color_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_color_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_depth_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_depth_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_stencil_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_stencil_attachment_access as *mut Bool32)
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
    ///Sets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn set_rasterization_order_color_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_color_attachment_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn set_rasterization_order_depth_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_depth_attachment_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn set_rasterization_order_stencil_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_stencil_attachment_access = value as u8 as u32;
        self
    }
}
