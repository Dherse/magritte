//![VK_KHR_shader_subgroup_uniform_control_flow](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_uniform_control_flow.html) - device extension
//!# Description
//!This extension allows the use of the `SPV_KHR_subgroup_uniform_control_flow`
//!SPIR-V extension in shader modules.
//!`SPV_KHR_subgroup_uniform_control_flow` provides stronger guarantees that
//!diverged subgroups will reconverge.Developers should utilize this extension if they use subgroup
//! operations to
//!reduce the work performed by a uniform subgroup.
//!This extension will guarantee that uniform subgroup will reconverge in the
//!same manner as invocation groups (see “Uniform Control Flow” in the
//![Khronos SPIR-V Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirv-spec)).
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Alan Baker [alan-baker](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_subgroup_uniform_control_flow]
//!   @alan-baker%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shader_subgroup_uniform_control_flow extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME`]
//! - [`KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
//!# Version history
//! - Revision 1, 2020-08-27 (Alan Baker)  - Internal draft version
//!# Other information
//! * 2020-08-27
//! * No known IP claims.
//! * - Requires SPIR-V 1.3.  - This extension requires [`SPV_KHR_subgroup_uniform_control_flow`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_subgroup_uniform_control_flow.html)
//!   - This extension provides API support for [`GL_EXT_subgroupuniform_qualifier`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_subgroupuniform_qualifier.txt)
//! * - Alan Baker, Google  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
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
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_shader_subgroup_uniform_control_flow");
///[VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR.html) - Structure describing support for shader subgroup uniform control flow by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`]
///structure is defined as:
///```c
///// Provided by VK_KHR_shader_subgroup_uniform_control_flow
///typedef struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderSubgroupUniformControlFlow;
///} VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR;
///```
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`shader_subgroup_uniform_control_flow`] specifies whether the implementation supports the
///   shader execution mode `SubgroupUniformControlFlowKHR`
/// If the [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`] structure is included in
/// the [`p_next`] chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR`] **can**  also be used in the
/// [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
/// # Related
/// - [`khr_shader_subgroup_uniform_control_flow`]
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
#[doc(alias = "VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_subgroup_uniform_control_flow`] specifies whether the
    ///implementation supports the shader execution mode
    ///`SubgroupUniformControlFlowKHR`
    pub shader_subgroup_uniform_control_flow: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            shader_subgroup_uniform_control_flow: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn shader_subgroup_uniform_control_flow_raw(&self) -> Bool32 {
        self.shader_subgroup_uniform_control_flow
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn set_shader_subgroup_uniform_control_flow_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_subgroup_uniform_control_flow = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn with_shader_subgroup_uniform_control_flow_raw(mut self, value: Bool32) -> Self {
        self.shader_subgroup_uniform_control_flow = value;
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
    ///Gets the value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn shader_subgroup_uniform_control_flow(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_subgroup_uniform_control_flow as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn shader_subgroup_uniform_control_flow_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_subgroup_uniform_control_flow as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_subgroup_uniform_control_flow as *mut Bool32)
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
    ///Sets the value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn set_shader_subgroup_uniform_control_flow(&mut self, value: bool) -> &mut Self {
        self.shader_subgroup_uniform_control_flow = value as u8 as u32;
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
    ///Sets the value of [`Self::shader_subgroup_uniform_control_flow`]
    pub fn with_shader_subgroup_uniform_control_flow(mut self, value: bool) -> Self {
        self.shader_subgroup_uniform_control_flow = value as u8 as u32;
        self
    }
}
