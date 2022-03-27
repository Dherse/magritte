use crate::vulkan1_0::{BaseOutStructure, Bool32, Format, StructureType, VertexInputRate};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_vertex_input_dynamic_state");
///[VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html) - Structure describing whether the dynamic vertex input state can be used
///# C Specifications
///The [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_vertex_input_dynamic_state
///typedef struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           vertexInputDynamicState;
///} VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_input_dynamic_state`] indicates that the implementation supports the following
///   dynamic states:  - `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///If the [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT`
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
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
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`vertex_input_dynamic_state`]
    ///indicates that the implementation supports the following dynamic states:
    /// - `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
    vertex_input_dynamic_state: Bool32,
}
///[VkVertexInputBindingDescription2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputBindingDescription2EXT.html) - Structure specifying the extended vertex input binding description
///# C Specifications
///The [`VertexInputBindingDescription2EXT`] structure is defined as:
///```c
///// Provided by VK_EXT_vertex_input_dynamic_state
///typedef struct VkVertexInputBindingDescription2EXT {
///    VkStructureType      sType;
///    void*                pNext;
///    uint32_t             binding;
///    uint32_t             stride;
///    VkVertexInputRate    inputRate;
///    uint32_t             divisor;
///} VkVertexInputBindingDescription2EXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`binding`] is the binding number that this structure describes.
/// - [`stride`] is the byte stride between consecutive elements within the buffer.
/// - [`input_rate`] is a [`VertexInputRate`] value specifying whether vertex attribute addressing
///   is a function of the vertex index or of the instance index.
/// - [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. This member **can** be set to a value other than `1` if the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0`**can** be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.
///# Description
///Valid Usage
/// - [`binding`]**must** be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - [`stride`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_binding_stride`]
/// - If the [vertexAttributeInstanceRateZeroDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor)
///   feature is not enabled, [`divisor`]**must** not be `0`
/// - If the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor)
///   feature is not enabled, [`divisor`]**must** be `1`
/// - [`divisor`]**must** be a value between `0` and
///   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
/// - If [`divisor`] is not `1` then [`input_rate`]**must** be of type
///   `VK_VERTEX_INPUT_RATE_INSTANCE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
/// - [`input_rate`]**must** be a valid [`VertexInputRate`] value
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
/// - [`StructureType`]
/// - [`VertexInputRate`]
/// - [`CmdSetVertexInputEXT`]
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
pub struct VertexInputBindingDescription2EXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`binding`] is the binding number that this structure describes.
    binding: u32,
    ///[`stride`] is the byte stride between consecutive elements within the
    ///buffer.
    stride: u32,
    ///[`input_rate`] is a [`VertexInputRate`] value specifying whether
    ///vertex attribute addressing is a function of the vertex index or of the
    ///instance index.
    input_rate: VertexInputRate,
    ///[`divisor`] is the number of successive instances that will use the
    ///same value of the vertex attribute when instanced rendering is enabled.
    ///This member **can** be set to a value other than `1` if the
    ///[vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled.
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
///[VkVertexInputAttributeDescription2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVertexInputAttributeDescription2EXT.html) - Structure specifying the extended vertex input attribute description
///# C Specifications
///The [`VertexInputAttributeDescription2EXT`] structure is defined as:
///```c
///// Provided by VK_EXT_vertex_input_dynamic_state
///typedef struct VkVertexInputAttributeDescription2EXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           location;
///    uint32_t           binding;
///    VkFormat           format;
///    uint32_t           offset;
///} VkVertexInputAttributeDescription2EXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`location`] is the shader input location number for this attribute.
/// - [`binding`] is the binding number which this attribute takes its data from.
/// - [`format`] is the size and type of the vertex attribute data.
/// - [`offset`] is a byte offset of this attribute relative to the start of an element in the
///   vertex input binding.
///# Description
///Valid Usage
/// - [`location`]**must** be less than [`PhysicalDeviceLimits::max_vertex_input_attributes`]
/// - [`binding`]**must** be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - [`offset`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_attribute_offset`]
/// - [`format`]**must** be allowed as a vertex buffer format, as specified by the
///   `VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT` flag in [`FormatProperties::buffer_features`] returned
///   by [`GetPhysicalDeviceFormatProperties`]
/// - If the `[`VK_KHR_portability_subset`]` extension is enabled, and
///   [`PhysicalDevicePortabilitySubsetFeaturesKHR::vertex_attribute_access_beyond_stride`] is
///   [`FALSE`], the sum of [`offset`] plus the size of the vertex attribute data described by
///   [`format`]**must** not be greater than `stride` in the [`VertexInputBindingDescription2EXT`]
///   referenced in [`binding`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`
/// - [`format`]**must** be a valid [`Format`] value
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`CmdSetVertexInputEXT`]
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
pub struct VertexInputAttributeDescription2EXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`location`] is the shader input location number for this attribute.
    location: u32,
    ///[`binding`] is the binding number which this attribute takes its data
    ///from.
    binding: u32,
    ///[`format`] is the size and type of the vertex attribute data.
    format: Format,
    ///[`offset`] is a byte offset of this attribute relative to the start of
    ///an element in the vertex input binding.
    offset: u32,
}
