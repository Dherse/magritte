use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
pub const EXT_METAL_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_metal_surface");
///[CAMetalLayer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/CAMetalLayer.html) - CoreAnimation native layer type for Metal
///# C Specifications
///To remove an unnecessary compile-time dependency, an incomplete type
///definition of [`CaMetalLayer`] is provided in the Vulkan headers:
///```c
///// Provided by VK_EXT_metal_surface
///
///#ifdef __OBJC__
///@class CAMetalLayer;
///#else
///typedef void CAMetalLayer;
///#endif
///```
///# Related
/// - [`VK_EXT_metal_surface`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "CAMetalLayer")]
pub type CaMetalLayer = c_void;
///[VkMetalSurfaceCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMetalSurfaceCreateInfoEXT.html) - Structure specifying parameters of a newly created Metal surface object
///# C Specifications
///The [`MetalSurfaceCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_metal_surface
///typedef struct VkMetalSurfaceCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkMetalSurfaceCreateFlagsEXT    flags;
///    const CAMetalLayer*             pLayer;
///} VkMetalSurfaceCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`layer`] is a reference to a [`CaMetalLayer`] object representing a renderable surface.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_EXT_metal_surface`]
/// - [`MetalSurfaceCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateMetalSurfaceEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: MetalSurfaceCreateFlagsEXT,
    ///[`layer`] is a reference to a [`CaMetalLayer`] object
    ///representing a renderable surface.
    layer: *const CaMetalLayer,
}
impl<'lt> Default for MetalSurfaceCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            layer: std::ptr::null(),
        }
    }
}
impl<'lt> MetalSurfaceCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::layer`]
    pub fn layer_raw(&self) -> *const CaMetalLayer {
        self.layer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer_raw(&mut self, value: *const CaMetalLayer) -> &mut Self {
        self.layer = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> MetalSurfaceCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::layer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn layer(&self) -> &CaMetalLayer {
        &*self.layer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut MetalSurfaceCreateFlagsEXT {
        &mut self.flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::ext_metal_surface::MetalSurfaceCreateFlagsEXT) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer(&mut self, value: &'lt crate::extensions::ext_metal_surface::CaMetalLayer) -> &mut Self {
        self.layer = value as *const _;
        self
    }
}
