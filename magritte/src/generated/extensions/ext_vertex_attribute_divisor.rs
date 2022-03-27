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
///# Members
/// - [`binding`] is the binding number for which the divisor is specified.
/// - [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0`**can** be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.
///# Description
///If this structure is not used to define a divisor value for an attribute,
///then the divisor has a logical default value of 1.Valid Usage
/// - [`binding`]**must** be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - If the `vertexAttributeInstanceRateZeroDivisor` feature is not enabled, [`divisor`]**must**
///   not be `0`
/// - If the `vertexAttributeInstanceRateDivisor` feature is not enabled, [`divisor`]**must** be `1`
/// - [`divisor`]**must** be a value between `0` and
///   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
/// - [`VertexInputBindingDescription::input_rate`]**must** be of type
///   `VK_VERTEX_INPUT_RATE_INSTANCE` for this [`binding`]
///# Related
/// - [`VK_EXT_vertex_attribute_divisor`]
/// - [`PipelineVertexInputDivisorStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    ///[`binding`] is the binding number for which the divisor is specified.
    binding: u32,
    ///[`divisor`] is the number of successive instances that will use the
    ///same value of the vertex attribute when instanced rendering is enabled.
    ///For example, if the divisor is N, the same vertex attribute will be
    ///applied to N successive instances before moving on to the next vertex
    ///attribute.
    ///The maximum value of [`divisor`] is implementation-dependent and can
    ///be queried using
    ///[`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]::`maxVertexAttribDivisor`.
    ///A value of `0`**can** be used for the divisor if the
    ///[`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor)
    ///feature is enabled.
    ///In this case, the same vertex attribute will be applied to all
    ///instances.
    divisor: u32,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_binding_divisor_count`] is the number of elements in the
///   [`p_vertex_binding_divisors`] array.
/// - [`p_vertex_binding_divisors`] is a pointer to an array of
///   [`VertexInputBindingDivisorDescriptionEXT`] structures specifying the divisor value for each
///   binding.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`
/// - [`p_vertex_binding_divisors`]**must** be a valid pointer to an array of
///   [`vertex_binding_divisor_count`][`VertexInputBindingDivisorDescriptionEXT`] structures
/// - [`vertex_binding_divisor_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_vertex_attribute_divisor`]
/// - [`StructureType`]
/// - [`VertexInputBindingDivisorDescriptionEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vertex_binding_divisor_count`] is the number of elements in the
    ///[`p_vertex_binding_divisors`] array.
    vertex_binding_divisor_count: u32,
    ///[`p_vertex_binding_divisors`] is a pointer to an array of
    ///[`VertexInputBindingDivisorDescriptionEXT`] structures specifying
    ///the divisor value for each binding.
    p_vertex_binding_divisors: *mut VertexInputBindingDivisorDescriptionEXT,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_vertex_attrib_divisor`] is the maximum value of the number of instances that will repeat
///   the value of vertex attribute data when instanced rendering is enabled.
///# Description
///If the [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_vertex_attribute_divisor`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_vertex_attrib_divisor`] is the
    ///maximum value of the number of instances that will repeat the value of
    ///vertex attribute data when instanced rendering is enabled.
    max_vertex_attrib_divisor: u32,
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
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_attribute_instance_rate_divisor`] specifies whether vertex attribute fetching may be
///   repeated in case of instanced rendering.
/// - [`vertex_attribute_instance_rate_zero_divisor`] specifies whether a zero value for
///   [`VertexInputBindingDivisorDescriptionEXT::divisor`] is supported.
///If the [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`
///# Related
/// - [`VK_EXT_vertex_attribute_divisor`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`vertex_attribute_instance_rate_divisor`] specifies whether vertex
    ///attribute fetching may be repeated in case of instanced rendering.
    vertex_attribute_instance_rate_divisor: Bool32,
    ///[`vertex_attribute_instance_rate_zero_divisor`] specifies whether a zero
    ///value for [`VertexInputBindingDivisorDescriptionEXT`]::`divisor`
    ///is supported.
    vertex_attribute_instance_rate_zero_divisor: Bool32,
}
