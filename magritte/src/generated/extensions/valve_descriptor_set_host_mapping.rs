//![VK_VALVE_descriptor_set_host_mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VALVE_descriptor_set_host_mapping.html) - device extension
//!# Description
//!This extension allows applications to directly query a host pointer for a
//![`DescriptorSet`] which  **can**  be used to copy descriptors between
//!descriptor sets without the use of an API command.
//!Memory offsets and sizes for descriptors  **can**  be queried from a
//![`DescriptorSetLayout`] as well.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Hans-Kristian Arntzen [HansKristian-Work](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_descriptor_set_host_mapping]
//!   @HansKristian-Work%0A<<Here describe the issue or question you have about the
//!   VK_VALVE_descriptor_set_host_mapping extension>>)
//!# New functions & commands
//! - [`GetDescriptorSetHostMappingVALVE`]
//! - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
//!# New structures
//! - [`DescriptorSetBindingReferenceVALVE`]
//! - [`DescriptorSetLayoutHostMappingInfoVALVE`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE`]
//!# New constants
//! - [`VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME`]
//! - [`VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`  -
//!   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
//!# Version History
//! - Revision 1, 2022-02-22 (Hans-Kristian Arntzen)  - Initial specification
//!# Other info
//! * 2022-02-22
//! * No known IP claims.
//! * - Hans-Kristian Arntzen, Valve
//!# Related
//! - [`DescriptorSetBindingReferenceVALVE`]
//! - [`DescriptorSetLayoutHostMappingInfoVALVE`]
//! - [`PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE`]
//! - [`GetDescriptorSetHostMappingVALVE`]
//! - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorSetLayout, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_VALVE_descriptor_set_host_mapping");
///[VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html) - Stub description of VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           descriptorSetHostMapping;
///} VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
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
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    descriptor_set_host_mapping: Bool32,
}
impl<'lt> Default for PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            descriptor_set_host_mapping: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::descriptor_set_host_mapping`]
    pub fn descriptor_set_host_mapping_raw(&self) -> Bool32 {
        self.descriptor_set_host_mapping
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_set_host_mapping`]
    pub fn set_descriptor_set_host_mapping_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_set_host_mapping = value;
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
    ///Gets the value of [`Self::descriptor_set_host_mapping`]
    pub fn descriptor_set_host_mapping(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_set_host_mapping as u8) }
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
    ///Gets a mutable reference to the value of [`Self::descriptor_set_host_mapping`]
    pub fn descriptor_set_host_mapping_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_set_host_mapping as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_set_host_mapping as *mut Bool32)
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
    ///Sets the raw value of [`Self::descriptor_set_host_mapping`]
    pub fn set_descriptor_set_host_mapping(&mut self, value: bool) -> &mut Self {
        self.descriptor_set_host_mapping = value as u8 as u32;
        self
    }
}
///[VkDescriptorSetBindingReferenceVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html) - Stub description of VkDescriptorSetBindingReferenceVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkDescriptorSetBindingReferenceVALVE {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkDescriptorSetLayout    descriptorSetLayout;
///    uint32_t                 binding;
///} VkDescriptorSetBindingReferenceVALVE;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
/// - [`p_next`] **must**  be `NULL`
/// - [`descriptor_set_layout`] **must**  be a valid [`DescriptorSetLayout`] handle
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
/// - [`DescriptorSetLayout`]
/// - [`StructureType`]
/// - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
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
pub struct DescriptorSetBindingReferenceVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`
    s_type: StructureType,
    ///[`p_next`] **must**  be `NULL`
    p_next: *const BaseInStructure<'lt>,
    ///[`descriptor_set_layout`] **must**  be a valid [`DescriptorSetLayout`] handle
    descriptor_set_layout: DescriptorSetLayout,
    ///No documentation found
    binding: u32,
}
impl<'lt> Default for DescriptorSetBindingReferenceVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            descriptor_set_layout: Default::default(),
            binding: 0,
        }
    }
}
impl<'lt> DescriptorSetBindingReferenceVALVE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::descriptor_set_layout`]
    pub fn descriptor_set_layout(&self) -> DescriptorSetLayout {
        self.descriptor_set_layout
    }
    ///Gets the value of [`Self::binding`]
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_set_layout`]
    pub fn descriptor_set_layout_mut(&mut self) -> &mut DescriptorSetLayout {
        &mut self.descriptor_set_layout
    }
    ///Gets a mutable reference to the value of [`Self::binding`]
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::descriptor_set_layout`]
    pub fn set_descriptor_set_layout(&mut self, value: crate::vulkan1_0::DescriptorSetLayout) -> &mut Self {
        self.descriptor_set_layout = value;
        self
    }
    ///Sets the raw value of [`Self::binding`]
    pub fn set_binding(&mut self, value: u32) -> &mut Self {
        self.binding = value;
        self
    }
}
///[VkDescriptorSetLayoutHostMappingInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html) - Stub description of VkDescriptorSetLayoutHostMappingInfoVALVE
///# C Specifications
///There is currently no specification language written for this type.
///This section acts only as placeholder and to avoid dead links in the
///specification and reference pages.
///```c
///// Provided by VK_VALVE_descriptor_set_host_mapping
///typedef struct VkDescriptorSetLayoutHostMappingInfoVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    size_t             descriptorOffset;
///    uint32_t           descriptorSize;
///} VkDescriptorSetLayoutHostMappingInfoVALVE;
///```
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_VALVE_descriptor_set_host_mapping`]
/// - [`StructureType`]
/// - [`GetDescriptorSetLayoutHostMappingInfoVALVE`]
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
pub struct DescriptorSetLayoutHostMappingInfoVALVE<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`
    s_type: StructureType,
    ///[`p_next`] **must**  be `NULL`
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    descriptor_offset: usize,
    ///No documentation found
    descriptor_size: u32,
}
impl<'lt> Default for DescriptorSetLayoutHostMappingInfoVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            descriptor_offset: 0,
            descriptor_size: 0,
        }
    }
}
impl<'lt> DescriptorSetLayoutHostMappingInfoVALVE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::descriptor_offset`]
    pub fn descriptor_offset(&self) -> usize {
        self.descriptor_offset
    }
    ///Gets the value of [`Self::descriptor_size`]
    pub fn descriptor_size(&self) -> u32 {
        self.descriptor_size
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
    ///Gets a mutable reference to the value of [`Self::descriptor_offset`]
    pub fn descriptor_offset_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_size`]
    pub fn descriptor_size_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::descriptor_offset`]
    pub fn set_descriptor_offset(&mut self, value: usize) -> &mut Self {
        self.descriptor_offset = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_size`]
    pub fn set_descriptor_size(&mut self, value: u32) -> &mut Self {
        self.descriptor_size = value;
        self
    }
}
