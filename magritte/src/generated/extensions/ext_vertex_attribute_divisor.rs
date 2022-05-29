//![VK_EXT_vertex_attribute_divisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_attribute_divisor.html) - device extension
//!# Description
//!This extension allows instance-rate vertex attributes to be repeated for
//!certain number of instances instead of advancing for every instance when
//!instanced rendering is enabled.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Vikram Kushwaha [vkushwaha](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_vertex_attribute_divisor]
//!   @vkushwaha%0A<<Here describe the issue or question you have about the
//!   VK_EXT_vertex_attribute_divisor extension>>)
//!# New structures
//! - [`VertexInputBindingDivisorDescriptionEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]
//! - Extending [`PipelineVertexInputStateCreateInfo`]:  -
//!   [`PipelineVertexInputDivisorStateCreateInfoEXT`]
//!# New constants
//! - [`EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME`]
//! - [`EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!1) What is the effect of a non-zero value for `firstInstance`? **RESOLVED** : The Vulkan API
//! should follow the OpenGL convention and offset
//!attribute fetching by `firstInstance` while computing vertex attribute
//!offsets.2) Should zero be an allowed divisor? **RESOLVED** : Yes.
//!A zero divisor means the vertex attribute is repeated for all instances.
//!# Version History
//! - Revision 1, 2017-12-04 (Vikram Kushwaha)  - First Version
//! - Revision 2, 2018-07-16 (Jason Ekstrand)  - Adjust the interaction between `divisor` and
//!   `firstInstance` to match the OpenGL convention.  - Disallow divisors of zero.
//! - Revision 3, 2018-08-03 (Vikram Kushwaha)  - Allow a zero divisor.  - Add a physical device
//!   features structure to query/enable this feature.
//!# Other info
//! * 2018-08-03
//! * No known IP claims.
//! * - Vikram Kushwaha, NVIDIA  - Jason Ekstrand, Intel
//!# Related
//! - [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`]
//! - [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]
//! - [`PipelineVertexInputDivisorStateCreateInfoEXT`]
//! - [`VertexInputBindingDivisorDescriptionEXT`]
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
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_vertex_attribute_divisor");
///[VkVertexInputBindingDivisorDescriptionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDivisorDescriptionEXT.html) - Structure specifying a divisor used in instanced rendering
///# C Specifications
///The individual divisor values per binding are specified using the
///[`VertexInputBindingDivisorDescriptionEXT`] structure which is defined
///as:
///```c
///// Provided by VK_EXT_vertex_attribute_divisor
///typedef struct VkVertexInputBindingDivisorDescriptionEXT {
///    uint32_t    binding;
///    uint32_t    divisor;
///} VkVertexInputBindingDivisorDescriptionEXT;
///```
/// # Members
/// - [`binding`] is the binding number for which the divisor is specified.
/// - [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0` **can**  be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.
/// # Description
/// If this structure is not used to define a divisor value for an attribute,
/// then the divisor has a logical default value of 1.
/// ## Valid Usage
/// - [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - If the `vertexAttributeInstanceRateZeroDivisor` feature is not enabled, [`divisor`] **must**
///   not be `0`
/// - If the `vertexAttributeInstanceRateDivisor` feature is not enabled, [`divisor`] **must**  be
///   `1`
/// - [`divisor`] **must**  be a value between `0` and
///   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
/// - [`VertexInputBindingDescription::input_rate`] **must**  be of type
///   `VK_VERTEX_INPUT_RATE_INSTANCE` for this [`binding`]
/// # Related
/// - [`ext_vertex_attribute_divisor`]
/// - [`PipelineVertexInputDivisorStateCreateInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVertexInputBindingDivisorDescriptionEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    ///[`binding`] is the binding number for which the divisor is specified.
    pub binding: u32,
    ///[`divisor`] is the number of successive instances that will use the
    ///same value of the vertex attribute when instanced rendering is enabled.
    ///For example, if the divisor is N, the same vertex attribute will be
    ///applied to N successive instances before moving on to the next vertex
    ///attribute.
    ///The maximum value of [`divisor`] is implementation-dependent and can
    ///be queried using
    ///[`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]::`maxVertexAttribDivisor`.
    ///A value of `0` **can**  be used for the divisor if the
    ///[`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor)
    ///feature is enabled.
    ///In this case, the same vertex attribute will be applied to all
    ///instances.
    pub divisor: u32,
}
impl Default for VertexInputBindingDivisorDescriptionEXT {
    fn default() -> Self {
        Self { binding: 0, divisor: 0 }
    }
}
impl VertexInputBindingDivisorDescriptionEXT {
    ///Gets the value of [`Self::binding`]
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Gets the value of [`Self::divisor`]
    pub fn divisor(&self) -> u32 {
        self.divisor
    }
    ///Gets a mutable reference to the value of [`Self::binding`]
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Gets a mutable reference to the value of [`Self::divisor`]
    pub fn divisor_mut(&mut self) -> &mut u32 {
        &mut self.divisor
    }
    ///Sets the value of [`Self::binding`]
    pub fn set_binding(&mut self, value: u32) -> &mut Self {
        self.binding = value;
        self
    }
    ///Sets the value of [`Self::divisor`]
    pub fn set_divisor(&mut self, value: u32) -> &mut Self {
        self.divisor = value;
        self
    }
    ///Sets the value of [`Self::binding`]
    pub fn with_binding(mut self, value: u32) -> Self {
        self.binding = value;
        self
    }
    ///Sets the value of [`Self::divisor`]
    pub fn with_divisor(mut self, value: u32) -> Self {
        self.divisor = value;
        self
    }
}
///[VkPipelineVertexInputDivisorStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineVertexInputDivisorStateCreateInfoEXT.html) - Structure specifying vertex attributes assignment during instanced rendering
///# C Specifications
///If
///[`vertexAttributeInstanceRateDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor)
///feature is enabled and the [`p_next`] chain of
///[`PipelineVertexInputStateCreateInfo`] includes a
///[`PipelineVertexInputDivisorStateCreateInfoEXT`] structure, then that
///structure controls how vertex attributes are assigned to an instance when
///instanced rendering is enabled.The [`PipelineVertexInputDivisorStateCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_vertex_attribute_divisor
///typedef struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    uint32_t                                            vertexBindingDivisorCount;
///    const VkVertexInputBindingDivisorDescriptionEXT*    pVertexBindingDivisors;
///} VkPipelineVertexInputDivisorStateCreateInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_binding_divisor_count`] is the number of elements in the [`vertex_binding_divisors`]
///   array.
/// - [`vertex_binding_divisors`] is a pointer to an array of
///   [`VertexInputBindingDivisorDescriptionEXT`] structures specifying the divisor value for each
///   binding.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
/// - [`vertex_binding_divisors`] **must**  be a valid pointer to an array of
///   [`vertex_binding_divisor_count`][`VertexInputBindingDivisorDescriptionEXT`] structures
/// - [`vertex_binding_divisor_count`] **must**  be greater than `0`
/// # Related
/// - [`ext_vertex_attribute_divisor`]
/// - [`StructureType`]
/// - [`VertexInputBindingDivisorDescriptionEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vertex_binding_divisor_count`] is the number of elements in the
    ///[`vertex_binding_divisors`] array.
    pub vertex_binding_divisor_count: u32,
    ///[`vertex_binding_divisors`] is a pointer to an array of
    ///[`VertexInputBindingDivisorDescriptionEXT`] structures specifying
    ///the divisor value for each binding.
    pub vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
impl<'lt> Default for PipelineVertexInputDivisorStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            vertex_binding_divisor_count: 0,
            vertex_binding_divisors: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineVertexInputDivisorStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::vertex_binding_divisors`]
    pub fn vertex_binding_divisors_raw(&self) -> *const VertexInputBindingDivisorDescriptionEXT {
        self.vertex_binding_divisors
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_binding_divisors`]
    pub fn set_vertex_binding_divisors_raw(
        &mut self,
        value: *const VertexInputBindingDivisorDescriptionEXT,
    ) -> &mut Self {
        self.vertex_binding_divisors = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_binding_divisors`]
    pub fn with_vertex_binding_divisors_raw(mut self, value: *const VertexInputBindingDivisorDescriptionEXT) -> Self {
        self.vertex_binding_divisors = value;
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
    ///Gets the value of [`Self::vertex_binding_divisor_count`]
    pub fn vertex_binding_divisor_count(&self) -> u32 {
        self.vertex_binding_divisor_count
    }
    ///Gets the value of [`Self::vertex_binding_divisors`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn vertex_binding_divisors(&self) -> &[VertexInputBindingDivisorDescriptionEXT] {
        std::slice::from_raw_parts(self.vertex_binding_divisors, self.vertex_binding_divisor_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vertex_binding_divisor_count`]
    pub fn vertex_binding_divisor_count_mut(&mut self) -> &mut u32 {
        &mut self.vertex_binding_divisor_count
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
    ///Sets the value of [`Self::vertex_binding_divisor_count`]
    pub fn set_vertex_binding_divisor_count(&mut self, value: u32) -> &mut Self {
        self.vertex_binding_divisor_count = value;
        self
    }
    ///Sets the value of [`Self::vertex_binding_divisors`]
    pub fn set_vertex_binding_divisors(
        &mut self,
        value: &'lt [crate::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.vertex_binding_divisors = value.as_ptr();
        self.vertex_binding_divisor_count = len_;
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
    ///Sets the value of [`Self::vertex_binding_divisor_count`]
    pub fn with_vertex_binding_divisor_count(mut self, value: u32) -> Self {
        self.vertex_binding_divisor_count = value;
        self
    }
    ///Sets the value of [`Self::vertex_binding_divisors`]
    pub fn with_vertex_binding_divisors(
        mut self,
        value: &'lt [crate::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.vertex_binding_divisors = value.as_ptr();
        self.vertex_binding_divisor_count = len_;
        self
    }
}
///[VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT.html) - Structure describing max value of vertex attribute divisor that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_vertex_attribute_divisor
///typedef struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxVertexAttribDivisor;
///} VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_vertex_attrib_divisor`] is the maximum value of the number of instances that will repeat
///   the value of vertex attribute data when instanced rendering is enabled.
/// # Description
/// If the [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`
/// # Related
/// - [`ext_vertex_attribute_divisor`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_vertex_attrib_divisor`] is the
    ///maximum value of the number of instances that will repeat the value of
    ///vertex attribute data when instanced rendering is enabled.
    pub max_vertex_attrib_divisor: u32,
}
impl<'lt> Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_vertex_attrib_divisor: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::max_vertex_attrib_divisor`]
    pub fn max_vertex_attrib_divisor(&self) -> u32 {
        self.max_vertex_attrib_divisor
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
    ///Gets a mutable reference to the value of [`Self::max_vertex_attrib_divisor`]
    pub fn max_vertex_attrib_divisor_mut(&mut self) -> &mut u32 {
        &mut self.max_vertex_attrib_divisor
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
    ///Sets the value of [`Self::max_vertex_attrib_divisor`]
    pub fn set_max_vertex_attrib_divisor(&mut self, value: u32) -> &mut Self {
        self.max_vertex_attrib_divisor = value;
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
    ///Sets the value of [`Self::max_vertex_attrib_divisor`]
    pub fn with_max_vertex_attrib_divisor(mut self, value: u32) -> Self {
        self.max_vertex_attrib_divisor = value;
        self
    }
}
///[VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html) - Structure describing if fetching of vertex attribute may be repeated for instanced rendering
///# C Specifications
///The [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_vertex_attribute_divisor
///typedef struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           vertexAttributeInstanceRateDivisor;
///    VkBool32           vertexAttributeInstanceRateZeroDivisor;
///} VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT;
///```
/// # Members
/// This structure describes the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_attribute_instance_rate_divisor`] specifies whether vertex attribute fetching may be
///   repeated in case of instanced rendering.
/// - [`vertex_attribute_instance_rate_zero_divisor`] specifies whether a zero value for
///   [`VertexInputBindingDivisorDescriptionEXT::divisor`] is supported.
/// If the [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] **can**  also be used in the [`p_next`]
/// chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`
/// # Related
/// - [`ext_vertex_attribute_divisor`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`vertex_attribute_instance_rate_divisor`] specifies whether vertex
    ///attribute fetching may be repeated in case of instanced rendering.
    pub vertex_attribute_instance_rate_divisor: Bool32,
    ///[`vertex_attribute_instance_rate_zero_divisor`] specifies whether a zero
    ///value for [`VertexInputBindingDivisorDescriptionEXT`]::`divisor`
    ///is supported.
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}
impl<'lt> Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_attribute_instance_rate_divisor: 0,
            vertex_attribute_instance_rate_zero_divisor: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn vertex_attribute_instance_rate_divisor_raw(&self) -> Bool32 {
        self.vertex_attribute_instance_rate_divisor
    }
    ///Gets the raw value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn vertex_attribute_instance_rate_zero_divisor_raw(&self) -> Bool32 {
        self.vertex_attribute_instance_rate_zero_divisor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn set_vertex_attribute_instance_rate_divisor_raw(&mut self, value: Bool32) -> &mut Self {
        self.vertex_attribute_instance_rate_divisor = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn set_vertex_attribute_instance_rate_zero_divisor_raw(&mut self, value: Bool32) -> &mut Self {
        self.vertex_attribute_instance_rate_zero_divisor = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn with_vertex_attribute_instance_rate_divisor_raw(mut self, value: Bool32) -> Self {
        self.vertex_attribute_instance_rate_divisor = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn with_vertex_attribute_instance_rate_zero_divisor_raw(mut self, value: Bool32) -> Self {
        self.vertex_attribute_instance_rate_zero_divisor = value;
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
    ///Gets the value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn vertex_attribute_instance_rate_divisor(&self) -> bool {
        unsafe { std::mem::transmute(self.vertex_attribute_instance_rate_divisor as u8) }
    }
    ///Gets the value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn vertex_attribute_instance_rate_zero_divisor(&self) -> bool {
        unsafe { std::mem::transmute(self.vertex_attribute_instance_rate_zero_divisor as u8) }
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
    ///Gets a mutable reference to the value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn vertex_attribute_instance_rate_divisor_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vertex_attribute_instance_rate_divisor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vertex_attribute_instance_rate_divisor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn vertex_attribute_instance_rate_zero_divisor_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vertex_attribute_instance_rate_zero_divisor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vertex_attribute_instance_rate_zero_divisor as *mut Bool32)
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
    ///Sets the value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn set_vertex_attribute_instance_rate_divisor(&mut self, value: bool) -> &mut Self {
        self.vertex_attribute_instance_rate_divisor = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn set_vertex_attribute_instance_rate_zero_divisor(&mut self, value: bool) -> &mut Self {
        self.vertex_attribute_instance_rate_zero_divisor = value as u8 as u32;
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
    ///Sets the value of [`Self::vertex_attribute_instance_rate_divisor`]
    pub fn with_vertex_attribute_instance_rate_divisor(mut self, value: bool) -> Self {
        self.vertex_attribute_instance_rate_divisor = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::vertex_attribute_instance_rate_zero_divisor`]
    pub fn with_vertex_attribute_instance_rate_zero_divisor(mut self, value: bool) -> Self {
        self.vertex_attribute_instance_rate_zero_divisor = value as u8 as u32;
        self
    }
}
