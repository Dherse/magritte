//![VK_VALVE_mutable_descriptor_type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VALVE_mutable_descriptor_type.html) - device extension
//!# Description
//!This extension allows applications to reduce descriptor memory footprint by
//!allowing a descriptor to be able to mutate to a given list of descriptor
//!types depending on which descriptor types are written into, or copied into a
//!descriptor set.The main use case this extension intends to address is descriptor indexing
//!with `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` where the
//!descriptor types are completely generic, as this means applications can
//!allocate one large descriptor set, rather than having one large descriptor
//!set per descriptor type, which significantly bloats descriptor memory usage
//!and causes performance issues.This extension also adds a mechanism to declare that a descriptor
//! pool, and
//!therefore the descriptor sets that are allocated from it, reside only in
//!host memory; as such these descriptors can only be updated/copied, but not
//!bound.These features together allow much more efficient emulation of the raw D3D12
//!binding model.
//!This extension is primarily intended to be useful for API layering efforts.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_maintenance3`]`
//!# Contacts
//! - Joshua Ashton [Joshua-Ashton](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_mutable_descriptor_type]
//!   @Joshua-Ashton%0A<<Here describe the issue or question you have about the
//!   VK_VALVE_mutable_descriptor_type extension>>)
//! - Hans-Kristian Arntzen [HansKristian-Work](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_mutable_descriptor_type]
//!   @HansKristian-Work%0A<<Here describe the issue or question you have about the
//!   VK_VALVE_mutable_descriptor_type extension>>)
//!# New structures
//! - [`MutableDescriptorTypeListVALVE`]
//! - Extending [`DescriptorSetLayoutCreateInfo`], [`DescriptorPoolCreateInfo`]:  -
//!   [`MutableDescriptorTypeCreateInfoVALVE`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]
//!# New constants
//! - [`VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME`]
//! - [`VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION`]
//! - Extending [`DescriptorPoolCreateFlagBits`]:  - `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE`
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:  -
//!   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE`
//! - Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`
//!# Version History
//! - Revision 1, 2020-12-01 (Joshua Ashton, Hans-Kristian Arntzen)  - Initial specification,
//!   squashed from public draft.
//!# Other info
//! * 2020-12-02
//! * No known IP claims.
//! * - Joshua Ashton, Valve  - Hans-Kristian Arntzen, Valve
//!# Related
//! - [`MutableDescriptorTypeCreateInfoVALVE`]
//! - [`MutableDescriptorTypeListVALVE`]
//! - [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DescriptorType, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_VALVE_mutable_descriptor_type");
///[VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html) - Structure describing whether the mutable descriptor type is supported
///# C Specifications
///The [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is
///defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           mutableDescriptorType;
///} VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mutable_descriptor_type`] indicates that the implementation  **must**  support using the
///   [`DescriptorType`] of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` with at least the following
///   descriptor types, where any combination of the types  **must**  be supported:  -
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`  - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`  -
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`  -
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`  - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`
/// - Additionally, [`mutable_descriptor_type`] indicates that:  - Non-uniform descriptor indexing
///   **must**  be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding non-uniform indexing features
///   enabled in [`PhysicalDeviceDescriptorIndexingFeatures`].  -
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` with `descriptorType` of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` relaxes the list of required descriptor types to the
///   descriptor types which have the corresponding update-after-bind feature enabled in
///   [`PhysicalDeviceDescriptorIndexingFeatures`].  - Dynamically uniform descriptor indexing
///   **must**  be supported if all descriptor types in a [`MutableDescriptorTypeListVALVE`] for
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` have the corresponding dynamic indexing features enabled.
///   - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE` **must**  be supported.  -
///   `VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` **must**  be supported.
///If the [`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMutableDescriptorTypeFeaturesVALVE`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE`
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
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
#[doc(alias = "VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`mutable_descriptor_type`] indicates
    ///that the implementation  **must**  support using the [`DescriptorType`]
    ///of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` with at least the following
    ///descriptor types, where any combination of the types  **must**  be supported:
    /// - `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`
    /// - `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`
    /// - `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`
    pub mutable_descriptor_type: Bool32,
}
impl<'lt> Default for PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            mutable_descriptor_type: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::mutable_descriptor_type`]
    pub fn mutable_descriptor_type_raw(&self) -> Bool32 {
        self.mutable_descriptor_type
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::mutable_descriptor_type`]
    pub fn set_mutable_descriptor_type_raw(&mut self, value: Bool32) -> &mut Self {
        self.mutable_descriptor_type = value;
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
    ///Gets the value of [`Self::mutable_descriptor_type`]
    pub fn mutable_descriptor_type(&self) -> bool {
        unsafe { std::mem::transmute(self.mutable_descriptor_type as u8) }
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
    ///Gets a mutable reference to the value of [`Self::mutable_descriptor_type`]
    pub fn mutable_descriptor_type_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.mutable_descriptor_type as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.mutable_descriptor_type as *mut Bool32)
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
    ///Sets the raw value of [`Self::mutable_descriptor_type`]
    pub fn set_mutable_descriptor_type(&mut self, value: bool) -> &mut Self {
        self.mutable_descriptor_type = value as u8 as u32;
        self
    }
}
///[VkMutableDescriptorTypeListVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeListVALVE.html) - Structure describing descriptor types that a given descriptor may mutate to
///# C Specifications
///The list of potential descriptor types a given mutable descriptor  **can**
///mutate to is passed in a [`MutableDescriptorTypeListVALVE`] structure.The
/// [`MutableDescriptorTypeListVALVE`] structure is defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkMutableDescriptorTypeListVALVE {
///    uint32_t                   descriptorTypeCount;
///    const VkDescriptorType*    pDescriptorTypes;
///} VkMutableDescriptorTypeListVALVE;
///```
///# Members
/// - [`descriptor_type_count`] is the number of elements in [`descriptor_types`].
/// - [`descriptor_types`] is `NULL` or a pointer to an array of
///   [`descriptor_type_count`][`DescriptorType`] values defining which descriptor types a given
///   binding may mutate to.
///# Description
///## Valid Usage
/// - [`descriptor_type_count`] **must**  not be `0` if the corresponding binding is of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`descriptor_types`] **must**  be a valid pointer to an array of [`descriptor_type_count`]
///   valid, unique [`DescriptorType`] values if the given binding is of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` type
/// - [`descriptor_type_count`] **must**  be `0` if the corresponding binding is not of
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
/// - [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
/// - [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`
/// - [`descriptor_types`] **must**  not contain `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
///
///## Valid Usage (Implicit)
/// - If [`descriptor_type_count`] is not `0`, [`descriptor_types`] **must**  be a valid pointer to
///   an array of [`descriptor_type_count`] valid [`DescriptorType`] values
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
/// - [`DescriptorType`]
/// - [`MutableDescriptorTypeCreateInfoVALVE`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMutableDescriptorTypeListVALVE")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MutableDescriptorTypeListVALVE<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`descriptor_type_count`] is the number of elements in
    ///[`descriptor_types`].
    pub descriptor_type_count: u32,
    ///[`descriptor_types`] is `NULL` or a pointer to an array of
    ///[`descriptor_type_count`][`DescriptorType`] values defining which
    ///descriptor types a given binding may mutate to.
    pub descriptor_types: *const DescriptorType,
}
impl<'lt> Default for MutableDescriptorTypeListVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            descriptor_type_count: 0,
            descriptor_types: std::ptr::null(),
        }
    }
}
impl<'lt> MutableDescriptorTypeListVALVE<'lt> {
    ///Gets the raw value of [`Self::descriptor_types`]
    pub fn descriptor_types_raw(&self) -> *const DescriptorType {
        self.descriptor_types
    }
    ///Sets the raw value of [`Self::descriptor_types`]
    pub fn set_descriptor_types_raw(&mut self, value: *const DescriptorType) -> &mut Self {
        self.descriptor_types = value;
        self
    }
    ///Gets the value of [`Self::descriptor_type_count`]
    pub fn descriptor_type_count(&self) -> u32 {
        self.descriptor_type_count
    }
    ///Gets the value of [`Self::descriptor_types`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn descriptor_types(&self) -> &[DescriptorType] {
        std::slice::from_raw_parts(self.descriptor_types, self.descriptor_type_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_type_count`]
    pub fn descriptor_type_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::descriptor_type_count`]
    pub fn set_descriptor_type_count(&mut self, value: u32) -> &mut Self {
        self.descriptor_type_count = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_types`]
    pub fn set_descriptor_types(&mut self, value: &'lt [crate::vulkan1_0::DescriptorType]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.descriptor_types = value.as_ptr();
        self.descriptor_type_count = len_;
        self
    }
}
///[VkMutableDescriptorTypeCreateInfoVALVE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMutableDescriptorTypeCreateInfoVALVE.html) - Structure describing the list of possible active descriptor types for mutable type descriptors
///# C Specifications
///Information about the possible descriptor types for mutable descriptor types
///is passed in a [`MutableDescriptorTypeCreateInfoVALVE`] structure as a
///[`p_next`] to a [`DescriptorSetLayoutCreateInfo`] structure or a
///[`DescriptorPoolCreateInfo`] structure.The [`MutableDescriptorTypeCreateInfoVALVE`] structure is
/// defined as:
///```c
///// Provided by VK_VALVE_mutable_descriptor_type
///typedef struct VkMutableDescriptorTypeCreateInfoVALVE {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   mutableDescriptorTypeListCount;
///    const VkMutableDescriptorTypeListVALVE*    pMutableDescriptorTypeLists;
///} VkMutableDescriptorTypeCreateInfoVALVE;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`mutable_descriptor_type_list_count`] is the number of elements in
///   [`mutable_descriptor_type_lists`].
/// - [`mutable_descriptor_type_lists`] is a pointer to an array of
///   [`MutableDescriptorTypeListVALVE`] structures.
///# Description
///If [`mutable_descriptor_type_list_count`] is zero or if this structure is not
///included in the [`p_next`] chain, the
///[`MutableDescriptorTypeListVALVE`] for each element is considered to be
///zero or `NULL` for each member.
///Otherwise, the descriptor set layout binding at
///[`DescriptorSetLayoutCreateInfo::bindings`][i] uses the
///descriptor type lists in
///[`MutableDescriptorTypeCreateInfoVALVE`]::[`mutable_descriptor_type_lists`][i].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE`
/// - If [`mutable_descriptor_type_list_count`] is not `0`, [`mutable_descriptor_type_lists`]
///   **must**  be a valid pointer to an array of [`mutable_descriptor_type_list_count`] valid
///   [`MutableDescriptorTypeListVALVE`] structures
///# Related
/// - [`VK_VALVE_mutable_descriptor_type`]
/// - [`MutableDescriptorTypeListVALVE`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMutableDescriptorTypeCreateInfoVALVE")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoVALVE<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`mutable_descriptor_type_list_count`] is the number of elements in
    ///[`mutable_descriptor_type_lists`].
    pub mutable_descriptor_type_list_count: u32,
    ///[`mutable_descriptor_type_lists`] is a pointer to an array of
    ///[`MutableDescriptorTypeListVALVE`] structures.
    pub mutable_descriptor_type_lists: *const MutableDescriptorTypeListVALVE<'lt>,
}
impl<'lt> Default for MutableDescriptorTypeCreateInfoVALVE<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            mutable_descriptor_type_list_count: 0,
            mutable_descriptor_type_lists: std::ptr::null(),
        }
    }
}
impl<'lt> MutableDescriptorTypeCreateInfoVALVE<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::mutable_descriptor_type_lists`]
    pub fn mutable_descriptor_type_lists_raw(&self) -> *const MutableDescriptorTypeListVALVE<'lt> {
        self.mutable_descriptor_type_lists
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::mutable_descriptor_type_lists`]
    pub fn set_mutable_descriptor_type_lists_raw(
        &mut self,
        value: *const MutableDescriptorTypeListVALVE<'lt>,
    ) -> &mut Self {
        self.mutable_descriptor_type_lists = value;
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
    ///Gets the value of [`Self::mutable_descriptor_type_list_count`]
    pub fn mutable_descriptor_type_list_count(&self) -> u32 {
        self.mutable_descriptor_type_list_count
    }
    ///Gets the value of [`Self::mutable_descriptor_type_lists`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn mutable_descriptor_type_lists(&self) -> &[MutableDescriptorTypeListVALVE<'lt>] {
        std::slice::from_raw_parts(
            self.mutable_descriptor_type_lists,
            self.mutable_descriptor_type_list_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::mutable_descriptor_type_list_count`]
    pub fn mutable_descriptor_type_list_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::mutable_descriptor_type_list_count`]
    pub fn set_mutable_descriptor_type_list_count(&mut self, value: u32) -> &mut Self {
        self.mutable_descriptor_type_list_count = value;
        self
    }
    ///Sets the raw value of [`Self::mutable_descriptor_type_lists`]
    pub fn set_mutable_descriptor_type_lists(
        &mut self,
        value: &'lt [crate::extensions::valve_mutable_descriptor_type::MutableDescriptorTypeListVALVE<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.mutable_descriptor_type_lists = value.as_ptr();
        self.mutable_descriptor_type_list_count = len_;
        self
    }
}
