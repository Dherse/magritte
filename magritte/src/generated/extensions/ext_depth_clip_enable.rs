//![VK_EXT_depth_clip_enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_enable.html) - device extension
//!# Description
//!This extension allows the depth clipping operation, that is normally
//!implicitly controlled by
//![`PipelineRasterizationStateCreateInfo::depth_clamp_enable`], to
//!instead be controlled explicitly by
//![`PipelineRasterizationDepthClipStateCreateInfoEXT::depth_clip_enable`].This is useful for
//! translating DX content which assumes depth clamping is
//!always enabled, but depth clip can be controlled by the DepthClipEnable
//!rasterization state (D3D12_RASTERIZER_DESC).
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_depth_clip_enable]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_depth_clip_enable extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDepthClipEnableFeaturesEXT`]
//! - Extending [`PipelineRasterizationStateCreateInfo`]:  -
//!   [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
//!# New bitmasks
//! - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME`]
//! - [`EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2018-12-20 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2018-12-20
//! * - Daniel Rakos, AMD  - Henri Verbeet, CodeWeavers  - Jeff Bolz, NVIDIA  - Philip Rebohle, DXVK
//!   - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceDepthClipEnableFeaturesEXT`]
//! - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
//! - [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_clip_enable");
///[VkPipelineRasterizationDepthClipStateCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_depth_clip_enable
///typedef VkFlags VkPipelineRasterizationDepthClipStateCreateFlagsEXT;
///```
///# Related
/// - [`VK_EXT_depth_clip_enable`]
/// - [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(u32);
impl const Default for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineRasterizationDepthClipStateCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html) - Structure indicating support for explicit enable of depth clip
///# C Specifications
///The [`PhysicalDeviceDepthClipEnableFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_depth_clip_enable
///typedef struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           depthClipEnable;
///} VkPhysicalDeviceDepthClipEnableFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`depth_clip_enable`] indicates that the implementation supports setting the depth clipping
///   operation explicitly via the [`PipelineRasterizationDepthClipStateCreateInfoEXT`] pipeline
///   state. Otherwise depth clipping is only enabled when
///   [`PipelineRasterizationStateCreateInfo::depth_clamp_enable`] is set to [`FALSE`].
///If the [`PhysicalDeviceDepthClipEnableFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDepthClipEnableFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_depth_clip_enable`]
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
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`depth_clip_enable`] indicates that the
    ///implementation supports setting the depth clipping operation explicitly
    ///via the [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
    ///pipeline state.
    ///Otherwise depth clipping is only enabled when
    ///[`PipelineRasterizationStateCreateInfo`]::`depthClampEnable` is
    ///set to [`FALSE`].
    pub depth_clip_enable: Bool32,
}
impl<'lt> Default for PhysicalDeviceDepthClipEnableFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            depth_clip_enable: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDepthClipEnableFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable_raw(&self) -> Bool32 {
        self.depth_clip_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::depth_clip_enable`]
    pub fn set_depth_clip_enable_raw(mut self, value: Bool32) -> Self {
        self.depth_clip_enable = value;
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
    ///Gets the value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.depth_clip_enable as u8) }
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
    ///Gets a mutable reference to the value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.depth_clip_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.depth_clip_enable as *mut Bool32)
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
    ///Sets the value of [`Self::depth_clip_enable`]
    pub fn set_depth_clip_enable(mut self, value: bool) -> Self {
        self.depth_clip_enable = value as u8 as u32;
        self
    }
}
///[VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html) - Structure specifying depth clipping state
///# C Specifications
///If the [`p_next`] chain of [`PipelineRasterizationStateCreateInfo`]
///includes a [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
///structure, then that structure controls whether depth clipping is enabled or
///disabled.The [`PipelineRasterizationDepthClipStateCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_depth_clip_enable
///typedef struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
///    VkStructureType                                        sType;
///    const void*                                            pNext;
///    VkPipelineRasterizationDepthClipStateCreateFlagsEXT    flags;
///    VkBool32                                               depthClipEnable;
///} VkPipelineRasterizationDepthClipStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`depth_clip_enable`] controls whether depth clipping is enabled as described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`VK_EXT_depth_clip_enable`]
/// - [`Bool32`]
/// - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    ///[`depth_clip_enable`] controls whether depth clipping is enabled as
    ///described in [Primitive Clipping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-clipping).
    pub depth_clip_enable: Bool32,
}
impl<'lt> Default for PipelineRasterizationDepthClipStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            depth_clip_enable: 0,
        }
    }
}
impl<'lt> PipelineRasterizationDepthClipStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable_raw(&self) -> Bool32 {
        self.depth_clip_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::depth_clip_enable`]
    pub fn set_depth_clip_enable_raw(mut self, value: Bool32) -> Self {
        self.depth_clip_enable = value;
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
    pub fn flags(&self) -> PipelineRasterizationDepthClipStateCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.depth_clip_enable as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationDepthClipStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::depth_clip_enable`]
    pub fn depth_clip_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.depth_clip_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.depth_clip_enable as *mut Bool32)
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
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::ext_depth_clip_enable::PipelineRasterizationDepthClipStateCreateFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::depth_clip_enable`]
    pub fn set_depth_clip_enable(mut self, value: bool) -> Self {
        self.depth_clip_enable = value as u8 as u32;
        self
    }
}
