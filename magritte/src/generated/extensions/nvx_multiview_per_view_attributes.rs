//![VK_NVX_multiview_per_view_attributes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_multiview_per_view_attributes.html) - device extension
//!# Description
//!This extension adds a new way to write shaders to be used with multiview
//!subpasses, where the attributes for all views are written out by a single
//!invocation of the
//![pre-rasterization shader
//!stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
//!Related SPIR-V and GLSL extensions `SPV_NVX_multiview_per_view_attributes`
//!and `GL_NVX_multiview_per_view_attributes` introduce per-view position and
//!viewport mask attributes arrays, and this extension defines how those
//!per-view attribute arrays are interpreted by Vulkan.
//!Pipelines using per-view attributes  **may**  only execute the
//![pre-rasterization shader
//!stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) once for all views rather than once per-view, which reduces
//!redundant shading work.A subpass creation flag controls whether the subpass uses this extension.
//!A subpass  **must**  either exclusively use this extension or not use it at all.Some Vulkan
//! implementations only support the position attribute varying
//!between views in the X component.
//!A subpass can declare via a second creation flag whether all pipelines
//!compiled for this subpass will obey this restriction.Shaders that use the new per-view outputs
//! (e.g. `gl_PositionPerViewNV`)
//! **must**  also write the non-per-view output (`gl_Position`), and the values
//!written  **must**  be such that `gl_Position =
//!gl_PositionPerViewNV[gl_ViewIndex]` for all views in the subpass.
//!Implementations are free to either use the per-view outputs or the
//!non-per-view outputs, whichever would be more efficient.If `[`nv_viewport_array2`]` is not also
//! supported and enabled, the
//!per-view viewport mask  **must**  not be used.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_multiview`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_multiview_per_view_attributes]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NVX_multiview_per_view_attributes extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`]
//!# New constants
//! - [`NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME`]
//! - [`NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
//! - Extending [`SubpassDescriptionFlagBits`]:  -
//!   `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`  -
//!   `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`
//!# Version history
//! - Revision 1, 2017-01-13 (Jeff Bolz)  - Internal revisions
//!# Other information
//! * 2017-01-13
//! * No known IP claims.
//! * - This extension requires [`SPV_NVX_multiview_per_view_attributes`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NVX_multiview_per_view_attributes.html)
//!   - This extension provides API support for [`GL_NVX_multiview_per_view_attributes`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nvx/GL_NVX_multiview_per_view_attributes.txt)
//!   - This extension interacts with `[`nv_viewport_array2`]`.
//! * - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`]
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
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NVX_multiview_per_view_attributes");
///[VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) - Structure describing multiview limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`] structure
///is defined as:
///```c
///// Provided by VK_NVX_multiview_per_view_attributes
///typedef struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           perViewPositionAllComponents;
///} VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`per_view_position_all_components`] is [`TRUE`] if the implementation supports per-view
///   position values that differ in components other than the X component.
///# Description
///If the [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
///# Related
/// - [`nvx_multiview_per_view_attributes`]
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
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`per_view_position_all_components`] is [`TRUE`] if the
    ///implementation supports per-view position values that differ in
    ///components other than the X component.
    pub per_view_position_all_components: Bool32,
}
impl<'lt> Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
            p_next: std::ptr::null_mut(),
            per_view_position_all_components: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::per_view_position_all_components`]
    pub fn per_view_position_all_components_raw(&self) -> Bool32 {
        self.per_view_position_all_components
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_position_all_components`]
    pub fn set_per_view_position_all_components_raw(&mut self, value: Bool32) -> &mut Self {
        self.per_view_position_all_components = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::per_view_position_all_components`]
    pub fn with_per_view_position_all_components_raw(mut self, value: Bool32) -> Self {
        self.per_view_position_all_components = value;
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
    ///Gets the value of [`Self::per_view_position_all_components`]
    pub fn per_view_position_all_components(&self) -> bool {
        unsafe { std::mem::transmute(self.per_view_position_all_components as u8) }
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
    ///Gets a mutable reference to the value of [`Self::per_view_position_all_components`]
    pub fn per_view_position_all_components_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.per_view_position_all_components as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.per_view_position_all_components as *mut Bool32)
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
    ///Sets the value of [`Self::per_view_position_all_components`]
    pub fn set_per_view_position_all_components(&mut self, value: bool) -> &mut Self {
        self.per_view_position_all_components = value as u8 as u32;
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
    ///Sets the value of [`Self::per_view_position_all_components`]
    pub fn with_per_view_position_all_components(mut self, value: bool) -> Self {
        self.per_view_position_all_components = value as u8 as u32;
        self
    }
}
