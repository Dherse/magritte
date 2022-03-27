use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_display_native_hdr");
///[VkDisplayNativeHdrSurfaceCapabilitiesAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayNativeHdrSurfaceCapabilitiesAMD.html) - Structure describing display native HDR specific capabilities of a surface
///# C Specifications
///The [`DisplayNativeHdrSurfaceCapabilitiesAMD`] structure is defined as:
///```c
///// Provided by VK_AMD_display_native_hdr
///typedef struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           localDimmingSupport;
///} VkDisplayNativeHdrSurfaceCapabilitiesAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`local_dimming_support`] specifies whether the surface supports local dimming. If this is
///   [`TRUE`], [`SwapchainDisplayNativeHdrCreateInfoAMD`]**can** be used to explicitly enable or
///   disable local dimming for the surface. Local dimming may also be overriden by
///   [`SetLocalDimmingAMD`] during the lifetime of the swapchain.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`
///# Related
/// - [`VK_AMD_display_native_hdr`]
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
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`local_dimming_support`] specifies whether the surface supports local
    ///dimming.
    ///If this is [`TRUE`], [`SwapchainDisplayNativeHdrCreateInfoAMD`]**can** be used to explicitly
    /// enable or disable local dimming for the surface.
    ///Local dimming may also be overriden by [`SetLocalDimmingAMD`] during
    ///the lifetime of the swapchain.
    local_dimming_support: Bool32,
}
impl<'lt> Default for DisplayNativeHdrSurfaceCapabilitiesAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            local_dimming_support: 0,
        }
    }
}
impl<'lt> DisplayNativeHdrSurfaceCapabilitiesAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::local_dimming_support`]
    pub fn local_dimming_support_raw(&self) -> Bool32 {
        self.local_dimming_support
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::local_dimming_support`]
    pub fn set_local_dimming_support_raw(&mut self, value: Bool32) -> &mut Self {
        self.local_dimming_support = value;
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
    ///Gets the value of [`Self::local_dimming_support`]
    pub fn local_dimming_support(&self) -> bool {
        unsafe { std::mem::transmute(self.local_dimming_support as u8) }
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
    ///Gets a mutable reference to the value of [`Self::local_dimming_support`]
    pub fn local_dimming_support_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.local_dimming_support as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.local_dimming_support as *mut Bool32)
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
    ///Sets the raw value of [`Self::local_dimming_support`]
    pub fn set_local_dimming_support(&mut self, value: bool) -> &mut Self {
        self.local_dimming_support = value as u8 as u32;
        self
    }
}
///[VkSwapchainDisplayNativeHdrCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainDisplayNativeHdrCreateInfoAMD.html) - Structure specifying display native HDR parameters of a newly created swapchain object
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`SwapchainDisplayNativeHdrCreateInfoAMD`] structure, then that
///structure includes additional swapchain creation parameters specific to
///display native HDR support.The [`SwapchainDisplayNativeHdrCreateInfoAMD`] structure is defined
/// as:
///```c
///// Provided by VK_AMD_display_native_hdr
///typedef struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           localDimmingEnable;
///} VkSwapchainDisplayNativeHdrCreateInfoAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`local_dimming_enable`] specifies whether local dimming is enabled for the swapchain.
///# Description
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] does not include
///this structure, the default value for [`local_dimming_enable`] is
///[`TRUE`], meaning local dimming is initially enabled for the swapchain.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`
///Valid Usage
/// - It is only valid to set [`local_dimming_enable`] to [`TRUE`] if
///   [`DisplayNativeHdrSurfaceCapabilitiesAMD::local_dimming_support`] is supported
///# Related
/// - [`VK_AMD_display_native_hdr`]
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`local_dimming_enable`] specifies whether local dimming is enabled for
    ///the swapchain.
    local_dimming_enable: Bool32,
}
impl<'lt> Default for SwapchainDisplayNativeHdrCreateInfoAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            local_dimming_enable: 0,
        }
    }
}
impl<'lt> SwapchainDisplayNativeHdrCreateInfoAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::local_dimming_enable`]
    pub fn local_dimming_enable_raw(&self) -> Bool32 {
        self.local_dimming_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::local_dimming_enable`]
    pub fn set_local_dimming_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.local_dimming_enable = value;
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
    ///Gets the value of [`Self::local_dimming_enable`]
    pub fn local_dimming_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.local_dimming_enable as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::local_dimming_enable`]
    pub fn local_dimming_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.local_dimming_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.local_dimming_enable as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::local_dimming_enable`]
    pub fn set_local_dimming_enable(&mut self, value: bool) -> &mut Self {
        self.local_dimming_enable = value as u8 as u32;
        self
    }
}
