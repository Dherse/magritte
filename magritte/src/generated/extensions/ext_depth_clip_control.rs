//![VK_EXT_depth_clip_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_control.html) - device extension
//!# Description
//!This extension allows the application to use the OpenGL depth range in NDC,
//!i.e. with depth in range [-1, 1], as opposed to Vulkan’s default of
//![0, 1].
//!The purpose of this extension is to allow efficient layering of OpenGL over
//!Vulkan, by avoiding emulation in the pre-rasterization shader stages.
//!This emulation, which effectively duplicates gl_Position but with a
//!different depth value, costs ALU and consumes shader output components that
//!the implementation may not have to spare to meet OpenGL minimum
//!requirements.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_depth_clip_control]
//!   @syoussefi%0A<<Here describe the issue or question you have about the
//!   VK_EXT_depth_clip_control extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDepthClipControlFeaturesEXT`]
//! - Extending [`PipelineViewportStateCreateInfo`]:  -
//!   [`PipelineViewportDepthClipControlCreateInfoEXT`]
//!# New constants
//! - [`EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME`]
//! - [`EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`
//!# Known issues & F.A.Q.
//!1) Should this extension include an origin control option to match
//!GL_LOWER_LEFT found in ARB_clip_control? **RESOLVED** : No.
//!The fix for porting over the origin is a simple y-axis flip.
//!The depth clip control is a much harder problem to solve than what this
//!extension is aimed to solve.
//!Adding an equivalent to GL_LOWER_LEFT would require more testing.2) Should this pipeline state
//! be dynamic? **RESOLVED** : Yes.
//!The purpose of this extension is to emulate the OpenGL depth range, which is
//!expected to be globally fixed (in case of OpenGL ES) or very infrequently
//!changed (with `glClipControl` in OpenGL).3) Should the control provided in this extension be an
//! enum that could be
//!extended in the future? **RESOLVED** : No.
//!It is highly unlikely that the depth range is changed to anything other than
//![0, 1] in the future.
//!Should that happen a new extension will be required to extend such an enum,
//!and that extension might as well add a new struct to chain to
//![`PipelineViewportStateCreateInfo::p_next`] instead.
//!# Version history
//! - Revision 0, 2020-10-01 (Spencer Fricke)  - Internal revisions
//! - Revision 1, 2020-11-26 (Shahbaz Youssefi)  - Language fixes
//!# Other information
//! * 2021-11-09
//! * - Spencer Fricke, Samsung Electronics  - Shahbaz Youssefi, Google  - Ralph Potter, Samsung
//!   Electronics
//!# Related
//! - [`PhysicalDeviceDepthClipControlFeaturesEXT`]
//! - [`PipelineViewportDepthClipControlCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_clip_control");
///[VkPhysicalDeviceDepthClipControlFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html) - Structure describing additional depth clip control supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDepthClipControlFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_depth_clip_control
///typedef struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           depthClipControl;
///} VkPhysicalDeviceDepthClipControlFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceDepthClipControlFeaturesEXT`]
///structure describe the following features:
///# Description
/// - [`depth_clip_control`] indicates that the implementation supports setting
///   [`PipelineViewportDepthClipControlCreateInfoEXT::negative_one_to_one`] to [`TRUE`].
///If the [`PhysicalDeviceDepthClipControlFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDepthClipControlFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
///# Related
/// - [`ext_depth_clip_control`]
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
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`depth_clip_control`] indicates that the
    ///implementation supports setting
    ///[`PipelineViewportDepthClipControlCreateInfoEXT`]::`negativeOneToOne`
    ///to [`TRUE`].
    pub depth_clip_control: Bool32,
}
impl<'lt> Default for PhysicalDeviceDepthClipControlFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_control: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDepthClipControlFeaturesEXT<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceDepthClipControlFeaturesEXT<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::depth_clip_control`]
    pub fn depth_clip_control_raw(&self) -> Bool32 {
        self.depth_clip_control
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::depth_clip_control`]
    pub fn set_depth_clip_control_raw(&mut self, value: Bool32) -> &mut Self {
        self.depth_clip_control = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::depth_clip_control`]
    pub fn with_depth_clip_control_raw(mut self, value: Bool32) -> Self {
        self.depth_clip_control = value;
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
    ///Gets the value of [`Self::depth_clip_control`]
    pub fn depth_clip_control(&self) -> bool {
        unsafe { std::mem::transmute(self.depth_clip_control as u8) }
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
    ///Gets a mutable reference to the value of [`Self::depth_clip_control`]
    pub fn depth_clip_control_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.depth_clip_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.depth_clip_control as *mut Bool32)
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
    ///Sets the value of [`Self::depth_clip_control`]
    pub fn set_depth_clip_control(&mut self, value: bool) -> &mut Self {
        self.depth_clip_control = value as u8 as u32;
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
    ///Sets the value of [`Self::depth_clip_control`]
    pub fn with_depth_clip_control(mut self, value: bool) -> Self {
        self.depth_clip_control = value as u8 as u32;
        self
    }
}
///[VkPipelineViewportDepthClipControlCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html) - Structure specifying parameters of a newly created pipeline depth clip control state
///# C Specifications
///The [`PipelineViewportDepthClipControlCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_depth_clip_control
///typedef struct VkPipelineViewportDepthClipControlCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           negativeOneToOne;
///} VkPipelineViewportDepthClipControlCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`negative_one_to_one`] sets the z<sub>m</sub> in the *view volume* to -w<sub>c</sub>
///# Description
///## Valid Usage
/// - If [depthClipControl](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-depthClipControl)
///   is not enabled, [`negative_one_to_one`] **must**  be [`FALSE`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT`
///# Related
/// - [`ext_depth_clip_control`]
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
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`negative_one_to_one`] sets the z<sub>m</sub> in the *view volume* to
    ///-w<sub>c</sub>
    pub negative_one_to_one: Bool32,
}
impl<'lt> Default for PipelineViewportDepthClipControlCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            negative_one_to_one: 0,
        }
    }
}
impl<'lt> PipelineViewportDepthClipControlCreateInfoEXT<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PipelineViewportDepthClipControlCreateInfoEXT<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::negative_one_to_one`]
    pub fn negative_one_to_one_raw(&self) -> Bool32 {
        self.negative_one_to_one
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::negative_one_to_one`]
    pub fn set_negative_one_to_one_raw(&mut self, value: Bool32) -> &mut Self {
        self.negative_one_to_one = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::negative_one_to_one`]
    pub fn with_negative_one_to_one_raw(mut self, value: Bool32) -> Self {
        self.negative_one_to_one = value;
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
    ///Gets the value of [`Self::negative_one_to_one`]
    pub fn negative_one_to_one(&self) -> bool {
        unsafe { std::mem::transmute(self.negative_one_to_one as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::negative_one_to_one`]
    pub fn negative_one_to_one_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.negative_one_to_one as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.negative_one_to_one as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::negative_one_to_one`]
    pub fn set_negative_one_to_one(&mut self, value: bool) -> &mut Self {
        self.negative_one_to_one = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::negative_one_to_one`]
    pub fn with_negative_one_to_one(mut self, value: bool) -> Self {
        self.negative_one_to_one = value as u8 as u32;
        self
    }
}
