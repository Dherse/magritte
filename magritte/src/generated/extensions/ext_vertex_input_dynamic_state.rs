//![VK_EXT_vertex_input_dynamic_state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_input_dynamic_state.html) - device extension
//!# Description
//!One of the states that contributes to the combinatorial explosion of
//!pipeline state objects that need to be created, is the vertex input binding
//!and attribute descriptions.
//!By allowing them to be dynamic applications may reduce the number of
//!pipeline objects they need to create.This extension adds dynamic state support for what is
//! normally static state
//!in [`PipelineVertexInputStateCreateInfo`].
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_vertex_input_dynamic_state]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_vertex_input_dynamic_state extension>>)
//!# New functions & commands
//! - [`cmd_set_vertex_input_ext`]
//!# New structures
//! - [`VertexInputAttributeDescription2EXT`]
//! - [`VertexInputBindingDescription2EXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]
//!# New constants
//! - [`EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME`]
//! - [`EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`  -
//!   `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
//!# Version History
//! - Revision 2, 2020-11-05 (Piers Daniell)  - Make [`VertexInputBindingDescription2EXT`]
//!   extensible  - Add new [`VertexInputAttributeDescription2EXT`] struct for the
//!   `pVertexAttributeDescriptions` parameter to [`cmd_set_vertex_input_ext`] so it is also
//!   extensible
//! - Revision 1, 2020-08-21 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2020-08-21
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA  - Spencer Fricke, Samsung  - Stu Smith, AMD
//!# Related
//! - [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]
//! - [`VertexInputAttributeDescription2EXT`]
//! - [`VertexInputBindingDescription2EXT`]
//! - [`cmd_set_vertex_input_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, CommandBuffer, Device, Format, StructureType, VertexInputRate},
    AsRaw, SmallVec, Unique,
};
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
///[vkCmdSetVertexInputEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html) - Set the vertex input state dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the vertex input attribute
///and vertex input binding descriptions, call:
///```c
///// Provided by VK_EXT_vertex_input_dynamic_state
///void vkCmdSetVertexInputEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    vertexBindingDescriptionCount,
///    const VkVertexInputBindingDescription2EXT*  pVertexBindingDescriptions,
///    uint32_t                                    vertexAttributeDescriptionCount,
///    const VkVertexInputAttributeDescription2EXT* pVertexAttributeDescriptions);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`vertex_binding_description_count`] is the number of vertex binding descriptions provided in
///   [`p_vertex_binding_descriptions`].
/// - [`p_vertex_binding_descriptions`] is a pointer to an array of
///   [`VertexInputBindingDescription2EXT`] structures.
/// - [`vertex_attribute_description_count`] is the number of vertex attribute descriptions provided
///   in [`p_vertex_attribute_descriptions`].
/// - [`p_vertex_attribute_descriptions`] is a pointer to an array of
///   [`VertexInputAttributeDescription2EXT`] structures.
///# Description
///This command sets the vertex input attribute and vertex input binding
///descriptions state for subsequent drawing commands when the graphics
///pipeline is created with `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`GraphicsPipelineCreateInfo::vertex_input_state`] values used to
///create the currently active pipeline.If the bound pipeline state object was also created with
/// the
///`VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE` dynamic state enabled,
///then [`cmd_bind_vertex_buffers2`] can be used instead of
///[`cmd_set_vertex_input_ext`] to dynamically set the stride.
///## Valid Usage
/// - The [vertexInputDynamicState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexInputDynamicState)
///   feature  **must**  be enabled
/// - [`vertex_binding_description_count`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - [`vertex_attribute_description_count`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_attributes`]
/// - For every `binding` specified by each element of [`p_vertex_attribute_descriptions`], a
///   [`VertexInputBindingDescription2EXT`] **must**  exist in [`p_vertex_binding_descriptions`]
///   with the same value of `binding`
/// - All elements of [`p_vertex_binding_descriptions`] **must**  describe distinct binding numbers
/// - All elements of [`p_vertex_attribute_descriptions`] **must**  describe distinct attribute
///   locations
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`vertex_binding_description_count`] is not `0`, [`p_vertex_binding_descriptions`] **must**
///   be a valid pointer to an array of [`vertex_binding_description_count`] valid
///   [`VertexInputBindingDescription2EXT`] structures
/// - If [`vertex_attribute_description_count`] is not `0`, [`p_vertex_attribute_descriptions`]
///   **must**  be a valid pointer to an array of [`vertex_attribute_description_count`] valid
///   [`VertexInputAttributeDescription2EXT`] structures
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
/// - [`CommandBuffer`]
/// - [`VertexInputAttributeDescription2EXT`]
/// - [`VertexInputBindingDescription2EXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetVertexInputEXT")]
pub type FNCmdSetVertexInputExt = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'lt>,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'lt>,
    ),
>;
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
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`vertex_input_dynamic_state`]
    ///indicates that the implementation supports the following dynamic states:
    /// - `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
    pub vertex_input_dynamic_state: Bool32,
}
impl<'lt> Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            vertex_input_dynamic_state: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVertexInputDynamicStateFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::vertex_input_dynamic_state`]
    pub fn vertex_input_dynamic_state_raw(&self) -> Bool32 {
        self.vertex_input_dynamic_state
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_input_dynamic_state`]
    pub fn set_vertex_input_dynamic_state_raw(mut self, value: Bool32) -> Self {
        self.vertex_input_dynamic_state = value;
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
    ///Gets the value of [`Self::vertex_input_dynamic_state`]
    pub fn vertex_input_dynamic_state(&self) -> bool {
        unsafe { std::mem::transmute(self.vertex_input_dynamic_state as u8) }
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
    ///Gets a mutable reference to the value of [`Self::vertex_input_dynamic_state`]
    pub fn vertex_input_dynamic_state_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vertex_input_dynamic_state as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vertex_input_dynamic_state as *mut Bool32)
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
    ///Sets the value of [`Self::vertex_input_dynamic_state`]
    pub fn set_vertex_input_dynamic_state(mut self, value: bool) -> Self {
        self.vertex_input_dynamic_state = value as u8 as u32;
        self
    }
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
/// - [`divisor`] is the number of successive instances that will use the same value of the vertex attribute when instanced rendering is enabled. This member  **can**  be set to a value other than `1` if the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled. For example, if the divisor is N, the same vertex attribute will be applied to N successive instances before moving on to the next vertex attribute. The maximum value of [`divisor`] is implementation-dependent and can be queried using [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`]. A value of `0` **can**  be used for the divisor if the [`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor) feature is enabled. In this case, the same vertex attribute will be applied to all instances.
///# Description
///## Valid Usage
/// - [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - [`stride`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_binding_stride`]
/// - If the [vertexAttributeInstanceRateZeroDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor)
///   feature is not enabled, [`divisor`] **must**  not be `0`
/// - If the [vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor)
///   feature is not enabled, [`divisor`] **must**  be `1`
/// - [`divisor`] **must**  be a value between `0` and
///   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT::max_vertex_attrib_divisor`], inclusive
/// - If [`divisor`] is not `1` then [`input_rate`] **must**  be of type
///   `VK_VERTEX_INPUT_RATE_INSTANCE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT`
/// - [`input_rate`] **must**  be a valid [`VertexInputRate`] value
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
/// - [`StructureType`]
/// - [`VertexInputRate`]
/// - [`cmd_set_vertex_input_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VertexInputBindingDescription2EXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`binding`] is the binding number that this structure describes.
    pub binding: u32,
    ///[`stride`] is the byte stride between consecutive elements within the
    ///buffer.
    pub stride: u32,
    ///[`input_rate`] is a [`VertexInputRate`] value specifying whether
    ///vertex attribute addressing is a function of the vertex index or of the
    ///instance index.
    pub input_rate: VertexInputRate,
    ///[`divisor`] is the number of successive instances that will use the
    ///same value of the vertex attribute when instanced rendering is enabled.
    ///This member  **can**  be set to a value other than `1` if the
    ///[vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled.
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
impl<'lt> Default for VertexInputBindingDescription2EXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VERTEX_INPUT_BINDING_DESCRIPTION2_EXT,
            p_next: std::ptr::null_mut(),
            binding: 0,
            stride: 0,
            input_rate: Default::default(),
            divisor: 0,
        }
    }
}
impl<'lt> VertexInputBindingDescription2EXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::binding`]
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Gets the value of [`Self::input_rate`]
    pub fn input_rate(&self) -> VertexInputRate {
        self.input_rate
    }
    ///Gets the value of [`Self::divisor`]
    pub fn divisor(&self) -> u32 {
        self.divisor
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
    ///Gets a mutable reference to the value of [`Self::binding`]
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Gets a mutable reference to the value of [`Self::input_rate`]
    pub fn input_rate_mut(&mut self) -> &mut VertexInputRate {
        &mut self.input_rate
    }
    ///Gets a mutable reference to the value of [`Self::divisor`]
    pub fn divisor_mut(&mut self) -> &mut u32 {
        &mut self.divisor
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
    ///Sets the value of [`Self::binding`]
    pub fn set_binding(mut self, value: u32) -> Self {
        self.binding = value;
        self
    }
    ///Sets the value of [`Self::stride`]
    pub fn set_stride(mut self, value: u32) -> Self {
        self.stride = value;
        self
    }
    ///Sets the value of [`Self::input_rate`]
    pub fn set_input_rate(mut self, value: crate::vulkan1_0::VertexInputRate) -> Self {
        self.input_rate = value;
        self
    }
    ///Sets the value of [`Self::divisor`]
    pub fn set_divisor(mut self, value: u32) -> Self {
        self.divisor = value;
        self
    }
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
///## Valid Usage
/// - [`location`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_attributes`]
/// - [`binding`] **must**  be less than [`PhysicalDeviceLimits::max_vertex_input_bindings`]
/// - [`offset`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_vertex_input_attribute_offset`]
/// - [`format`] **must**  be allowed as a vertex buffer format, as specified by the
///   `VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT` flag in [`FormatProperties::buffer_features`] returned
///   by [`get_physical_device_format_properties`]
/// - If the `[`VK_KHR_portability_subset`]` extension is enabled, and
///   [`PhysicalDevicePortabilitySubsetFeaturesKHR::vertex_attribute_access_beyond_stride`] is
///   [`FALSE`], the sum of [`offset`] plus the size of the vertex attribute data described by
///   [`format`] **must**  not be greater than `stride` in the [`VertexInputBindingDescription2EXT`]
///   referenced in [`binding`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT`
/// - [`format`] **must**  be a valid [`Format`] value
///# Related
/// - [`VK_EXT_vertex_input_dynamic_state`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`cmd_set_vertex_input_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`location`] is the shader input location number for this attribute.
    pub location: u32,
    ///[`binding`] is the binding number which this attribute takes its data
    ///from.
    pub binding: u32,
    ///[`format`] is the size and type of the vertex attribute data.
    pub format: Format,
    ///[`offset`] is a byte offset of this attribute relative to the start of
    ///an element in the vertex input binding.
    pub offset: u32,
}
impl<'lt> Default for VertexInputAttributeDescription2EXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION2_EXT,
            p_next: std::ptr::null_mut(),
            location: 0,
            binding: 0,
            format: Default::default(),
            offset: 0,
        }
    }
}
impl<'lt> VertexInputAttributeDescription2EXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::location`]
    pub fn location(&self) -> u32 {
        self.location
    }
    ///Gets the value of [`Self::binding`]
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> u32 {
        self.offset
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
    ///Gets a mutable reference to the value of [`Self::location`]
    pub fn location_mut(&mut self) -> &mut u32 {
        &mut self.location
    }
    ///Gets a mutable reference to the value of [`Self::binding`]
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.offset
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
    ///Sets the value of [`Self::location`]
    pub fn set_location(mut self, value: u32) -> Self {
        self.location = value;
        self
    }
    ///Sets the value of [`Self::binding`]
    pub fn set_binding(mut self, value: u32) -> Self {
        self.binding = value;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::offset`]
    pub fn set_offset(mut self, value: u32) -> Self {
        self.offset = value;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdSetVertexInputEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html) - Set the vertex input state dynamically for a command buffer
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the vertex input attribute
    ///and vertex input binding descriptions, call:
    ///```c
    ///// Provided by VK_EXT_vertex_input_dynamic_state
    ///void vkCmdSetVertexInputEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    vertexBindingDescriptionCount,
    ///    const VkVertexInputBindingDescription2EXT*  pVertexBindingDescriptions,
    ///    uint32_t                                    vertexAttributeDescriptionCount,
    ///    const VkVertexInputAttributeDescription2EXT* pVertexAttributeDescriptions);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`vertex_binding_description_count`] is the number of vertex binding descriptions provided
    ///   in [`p_vertex_binding_descriptions`].
    /// - [`p_vertex_binding_descriptions`] is a pointer to an array of
    ///   [`VertexInputBindingDescription2EXT`] structures.
    /// - [`vertex_attribute_description_count`] is the number of vertex attribute descriptions
    ///   provided in [`p_vertex_attribute_descriptions`].
    /// - [`p_vertex_attribute_descriptions`] is a pointer to an array of
    ///   [`VertexInputAttributeDescription2EXT`] structures.
    ///# Description
    ///This command sets the vertex input attribute and vertex input binding
    ///descriptions state for subsequent drawing commands when the graphics
    ///pipeline is created with `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, this state is specified by the
    ///[`GraphicsPipelineCreateInfo::vertex_input_state`] values used to
    ///create the currently active pipeline.If the bound pipeline state object was also created
    /// with the
    ///`VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE` dynamic state enabled,
    ///then [`cmd_bind_vertex_buffers2`] can be used instead of
    ///[`cmd_set_vertex_input_ext`] to dynamically set the stride.
    ///## Valid Usage
    /// - The [vertexInputDynamicState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexInputDynamicState)
    ///   feature  **must**  be enabled
    /// - [`vertex_binding_description_count`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_vertex_input_bindings`]
    /// - [`vertex_attribute_description_count`] **must**  be less than or equal to
    ///   [`PhysicalDeviceLimits::max_vertex_input_attributes`]
    /// - For every `binding` specified by each element of [`p_vertex_attribute_descriptions`], a
    ///   [`VertexInputBindingDescription2EXT`] **must**  exist in [`p_vertex_binding_descriptions`]
    ///   with the same value of `binding`
    /// - All elements of [`p_vertex_binding_descriptions`] **must**  describe distinct binding
    ///   numbers
    /// - All elements of [`p_vertex_attribute_descriptions`] **must**  describe distinct attribute
    ///   locations
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - If [`vertex_binding_description_count`] is not `0`, [`p_vertex_binding_descriptions`]
    ///   **must**  be a valid pointer to an array of [`vertex_binding_description_count`] valid
    ///   [`VertexInputBindingDescription2EXT`] structures
    /// - If [`vertex_attribute_description_count`] is not `0`, [`p_vertex_attribute_descriptions`]
    ///   **must**  be a valid pointer to an array of [`vertex_attribute_description_count`] valid
    ///   [`VertexInputAttributeDescription2EXT`] structures
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_EXT_vertex_input_dynamic_state`]
    /// - [`CommandBuffer`]
    /// - [`VertexInputAttributeDescription2EXT`]
    /// - [`VertexInputBindingDescription2EXT`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_vertex_input_ext<'a: 'this, 'this, 'lt>(
        self: &'this mut Unique<'a, CommandBuffer>,
        p_vertex_binding_descriptions : & [crate :: extensions :: ext_vertex_input_dynamic_state :: VertexInputBindingDescription2EXT < 'lt >],
        p_vertex_attribute_descriptions : & [crate :: extensions :: ext_vertex_input_dynamic_state :: VertexInputAttributeDescription2EXT < 'lt >],
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_vertex_input_dynamic_state()
            .and_then(|vtable| vtable.cmd_set_vertex_input_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_vertex_input_dynamic_state()
            .and_then(|vtable| vtable.cmd_set_vertex_input_ext())
            .unwrap_unchecked();
        let vertex_binding_description_count = (|len: usize| len)(p_vertex_binding_descriptions.len()) as _;
        let vertex_attribute_description_count = (|len: usize| len)(p_vertex_attribute_descriptions.len()) as _;
        let _return = _function(
            self.as_raw(),
            vertex_binding_description_count,
            p_vertex_binding_descriptions.as_ptr(),
            vertex_attribute_description_count,
            p_vertex_attribute_descriptions.as_ptr(),
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_vertex_input_dynamic_state`
pub struct DeviceExtVertexInputDynamicStateVTable {
    ///See [`FNCmdSetVertexInputExt`] for more information.
    pub cmd_set_vertex_input_ext: FNCmdSetVertexInputExt,
}
impl DeviceExtVertexInputDynamicStateVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            cmd_set_vertex_input_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetVertexInputEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_set_vertex_input_ext`]. See [`FNCmdSetVertexInputExt`] for more
    /// information.
    pub fn cmd_set_vertex_input_ext(&self) -> FNCmdSetVertexInputExt {
        self.cmd_set_vertex_input_ext
    }
}
