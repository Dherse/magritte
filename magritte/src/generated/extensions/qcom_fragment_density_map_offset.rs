//![VK_QCOM_fragment_density_map_offset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_fragment_density_map_offset.html) - device extension
//!# Description
//!This extension allows an application to specify offsets to a fragment
//!density map attachment, changing the framebuffer location where density
//!values are applied to without having to regenerate the fragment density map.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//! - Requires `[`ext_fragment_density_map`]`
//!# Contacts
//! - Matthew Netsch [mnetsch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_fragment_density_map_offset]
//!   @mnetsch%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_fragment_density_map_offset extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]
//! - Extending [`SubpassEndInfo`]:  - [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
//!# New constants
//! - [`QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME`]
//! - [`QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION`]
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`  -
//!   `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
//!# Version history
//! - Revision 1, 2021-09-03 (Matthew Netsch)  - Initial version
//!# Other information
//! * 2021-09-03
//! * - Matthew Netsch, Qualcomm Technologies, Inc.  - Jonathan Wicks, Qualcomm Technologies, Inc.
//!   - Jonathan Tinkham, Qualcomm Technologies, Inc.  - Jeff Leger, Qualcomm Technologies, Inc.
//!# Related
//! - [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]
//! - [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`]
//! - [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Extent2D, Offset2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_QCOM_fragment_density_map_offset");
///[VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html) - Structure describing fragment density map offet features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`] structure is
///defined as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMapOffset;
///} VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `fragmentDensityMapOffsets` specifies whether the implementation supports [fragment density map offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets)
///If the [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`
///# Related
/// - [`qcom_fragment_density_map_offset`]
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
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub fragment_density_map_offset: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM,
            p_next: std::ptr::null_mut(),
            fragment_density_map_offset: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::fragment_density_map_offset`]
    pub fn fragment_density_map_offset_raw(&self) -> Bool32 {
        self.fragment_density_map_offset
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_offset`]
    pub fn set_fragment_density_map_offset_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_map_offset = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_offset`]
    pub fn with_fragment_density_map_offset_raw(mut self, value: Bool32) -> Self {
        self.fragment_density_map_offset = value;
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
    ///Gets the value of [`Self::fragment_density_map_offset`]
    pub fn fragment_density_map_offset(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_map_offset as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_density_map_offset`]
    pub fn fragment_density_map_offset_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_map_offset as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_map_offset as *mut Bool32)
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
    ///Sets the value of [`Self::fragment_density_map_offset`]
    pub fn set_fragment_density_map_offset(&mut self, value: bool) -> &mut Self {
        self.fragment_density_map_offset = value as u8 as u32;
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
    ///Sets the value of [`Self::fragment_density_map_offset`]
    pub fn with_fragment_density_map_offset(mut self, value: bool) -> Self {
        self.fragment_density_map_offset = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html) - Structure describing fragment density map offset properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`] structure
///is defined as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         fragmentDensityOffsetGranularity;
///} VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_offset_granularity`] is the granularity for [fragment density offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets).
///# Description
///If the [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`
///# Related
/// - [`qcom_fragment_density_map_offset`]
/// - [`Extent2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_density_offset_granularity`] is the granularity for
    ///[fragment density offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets).
    pub fragment_density_offset_granularity: Extent2D,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM,
            p_next: std::ptr::null_mut(),
            fragment_density_offset_granularity: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::fragment_density_offset_granularity`]
    pub fn fragment_density_offset_granularity(&self) -> Extent2D {
        self.fragment_density_offset_granularity
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
    ///Gets a mutable reference to the value of [`Self::fragment_density_offset_granularity`]
    pub fn fragment_density_offset_granularity_mut(&mut self) -> &mut Extent2D {
        &mut self.fragment_density_offset_granularity
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
    ///Sets the value of [`Self::fragment_density_offset_granularity`]
    pub fn set_fragment_density_offset_granularity(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.fragment_density_offset_granularity = value;
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
    ///Sets the value of [`Self::fragment_density_offset_granularity`]
    pub fn with_fragment_density_offset_granularity(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.fragment_density_offset_granularity = value;
        self
    }
}
///[VkSubpassFragmentDensityMapOffsetEndInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html) - Structure specifying fragment density map offset subpass end information
///# C Specifications
///If the [`SubpassEndInfo`]::[`p_next`] chain includes a
///[`SubpassFragmentDensityMapOffsetEndInfoQCOM`] structure, then that
///structure includes an array of fragment density map offsets per layer for
///the render pass.The [`SubpassFragmentDensityMapOffsetEndInfoQCOM`] structure is defined
///as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
///    VkStructureType      sType;
///    const void*          pNext;
///    uint32_t             fragmentDensityOffsetCount;
///    const VkOffset2D*    pFragmentDensityOffsets;
///} VkSubpassFragmentDensityMapOffsetEndInfoQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_offset_count`] is the number of offsets being specified.
/// - [`fragment_density_offsets`] is a pointer to an array of [`Offset2D`] structs, each of which
///   describes the offset per layer.
///# Description
///The array elements are given per `layer` as defined by
///[Fetch Density Value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymap-fetch-density-value), where
///index = layer.
///Each (x,y) offset is in framebuffer pixels and shifts the fetch of the
///fragment density map by that amount.
///Offsets can be positive or negative.Offset values specified for any subpass that is not the last
/// subpass in the
///render pass are ignored.
///If the [`SubpassEndInfo`]::[`p_next`] chain for the last subpass of a
///renderpass does not include
///[`SubpassFragmentDensityMapOffsetEndInfoQCOM`], or if
///[`fragment_density_offset_count`] is zero, then the offset (0,0) is
///used for [Fetch Density Value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymap-fetch-density-value).
///## Valid Usage
/// - If the [`fragmentDensityMapOffsets`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentDensityMapOffsets)
///   feature is not enabled or fragment density map is not enabled in the render pass,
///   [`fragment_density_offset_count`] **must**  equal `0`.
/// - If [`SubpassDescription`]`::fragmentDensityMapAttachment` is not is not [`ATTACHMENT_UNUSED`]
///   and was not created with `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`] **must**  equal `0`.
/// - If [`SubpassDescription::depth_stencil_attachment`] is not is not [`ATTACHMENT_UNUSED`] and
///   was not created with `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`] **must**  equal `0`.
/// - If any element of [`SubpassDescription::input_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`, [`fragment_density_offset_count`]
///   **must**  equal `0`.
/// - If any element of [`SubpassDescription::color_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`, [`fragment_density_offset_count`]
///   **must**  equal `0`.
/// - If any element of [`SubpassDescription::resolve_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`, [`fragment_density_offset_count`]
///   **must**  equal `0`.
/// - If any element of [`SubpassDescription::preserve_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`, [`fragment_density_offset_count`]
///   **must**  equal `0`.
/// - If [`fragment_density_offset_count`] is not `0` and multiview is enabled for the render pass,
///   [`fragment_density_offset_count`] **must**  equal the `layerCount` that was specified in
///   creating the fragment density map attachment view.
/// - If [`fragment_density_offset_count`] is not `0` and multiview is not enabled for the render
///   pass, [`fragment_density_offset_count`] **must**  equal `1`.
/// - The `x` component of each element of [`fragment_density_offsets`] **must**  be an integer
///   multiple of `fragmentDensityOffsetGranularity.width`.
/// - The `y` component of each element of [`fragment_density_offsets`] **must**  be an integer
///   multiple of `fragmentDensityOffsetGranularity.height`.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
/// - If [`fragment_density_offset_count`] is not `0`, [`fragment_density_offsets`] **must**  be a
///   valid pointer to an array of [`fragment_density_offset_count`][`Offset2D`] structures
///# Related
/// - [`qcom_fragment_density_map_offset`]
/// - [`Offset2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSubpassFragmentDensityMapOffsetEndInfoQCOM")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOM<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`fragment_density_offset_count`] is the number of offsets being
    ///specified.
    pub fragment_density_offset_count: u32,
    ///[`fragment_density_offsets`] is a pointer to an array of
    ///[`Offset2D`] structs, each of which describes the offset per layer.
    pub fragment_density_offsets: *const Offset2D,
}
impl<'lt> Default for SubpassFragmentDensityMapOffsetEndInfoQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM,
            p_next: std::ptr::null(),
            fragment_density_offset_count: 0,
            fragment_density_offsets: std::ptr::null(),
        }
    }
}
impl<'lt> SubpassFragmentDensityMapOffsetEndInfoQCOM<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::fragment_density_offsets`]
    pub fn fragment_density_offsets_raw(&self) -> *const Offset2D {
        self.fragment_density_offsets
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_offsets`]
    pub fn set_fragment_density_offsets_raw(&mut self, value: *const Offset2D) -> &mut Self {
        self.fragment_density_offsets = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_offsets`]
    pub fn with_fragment_density_offsets_raw(mut self, value: *const Offset2D) -> Self {
        self.fragment_density_offsets = value;
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
    ///Gets the value of [`Self::fragment_density_offset_count`]
    pub fn fragment_density_offset_count(&self) -> u32 {
        self.fragment_density_offset_count
    }
    ///Gets the value of [`Self::fragment_density_offsets`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn fragment_density_offsets(&self) -> &[Offset2D] {
        std::slice::from_raw_parts(
            self.fragment_density_offsets,
            self.fragment_density_offset_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::fragment_density_offset_count`]
    pub fn fragment_density_offset_count_mut(&mut self) -> &mut u32 {
        &mut self.fragment_density_offset_count
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
    ///Sets the value of [`Self::fragment_density_offset_count`]
    pub fn set_fragment_density_offset_count(&mut self, value: u32) -> &mut Self {
        self.fragment_density_offset_count = value;
        self
    }
    ///Sets the value of [`Self::fragment_density_offsets`]
    pub fn set_fragment_density_offsets(&mut self, value: &'lt [crate::vulkan1_0::Offset2D]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.fragment_density_offsets = value.as_ptr();
        self.fragment_density_offset_count = len_;
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
    ///Sets the value of [`Self::fragment_density_offset_count`]
    pub fn with_fragment_density_offset_count(mut self, value: u32) -> Self {
        self.fragment_density_offset_count = value;
        self
    }
    ///Sets the value of [`Self::fragment_density_offsets`]
    pub fn with_fragment_density_offsets(mut self, value: &'lt [crate::vulkan1_0::Offset2D]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.fragment_density_offsets = value.as_ptr();
        self.fragment_density_offset_count = len_;
        self
    }
}
