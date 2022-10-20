//![VK_NV_compute_shader_derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_compute_shader_derivatives.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_NV_compute_shader_derivatives`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_compute_shader_derivatives.html)
//!SPIR-V extension.The SPIR-V extension provides two new execution modes, both of which allow
//!compute shaders to use built-ins that evaluate compute derivatives
//!explicitly or implicitly.
//!Derivatives will be computed via differencing over a 2x2 group of shader
//!invocations.
//!The `DerivativeGroupQuadsNV` execution mode assembles shader invocations
//!into 2x2 groups, where each group has x and y coordinates of the local
//!invocation ID of the form (2m+{0,1}, 2n+{0,1}).
//!The `DerivativeGroupLinearNV` execution mode assembles shader invocations
//!into 2x2 groups, where each group has local invocation index values of the
//!form 4m+{0,1,2,3}.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_compute_shader_derivatives]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the
//!   VK_NV_compute_shader_derivatives extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]
//!# New constants
//! - [`NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME`]
//! - [`NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`
//!# Known issues & F.A.Q.
//!(1) Should we specify that the groups of four shader invocations used for
//!derivatives in a compute shader are the same groups of four invocations that
//!form a “quad” in shader subgroups? **RESOLVED** : Yes.
//!# Version history
//! - Revision 1, 2018-07-19 (Pat Brown)  - Initial draft
//!# Other information
//! * 2018-07-19
//! * No known IP claims.
//! * - This extension requires [`SPV_NV_compute_shader_derivatives`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_compute_shader_derivatives.html)
//!   - This extension provides API support for [`GL_NV_compute_shader_derivatives`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_compute_shader_derivatives.txt)
//! * - Pat Brown, NVIDIA
//!# Related
//! - [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]
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
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_compute_shader_derivatives");
///[VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) - Structure describing compute shader derivative features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_compute_shader_derivatives
///typedef struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           computeDerivativeGroupQuads;
///    VkBool32           computeDerivativeGroupLinear;
///} VkPhysicalDeviceComputeShaderDerivativesFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`compute_derivative_group_quads`] indicates that the implementation supports the
///   `ComputeDerivativeGroupQuadsNV` SPIR-V capability.
/// - [`compute_derivative_group_linear`] indicates that the implementation supports the
///   `ComputeDerivativeGroupLinearNV` SPIR-V capability.
///See [Quad shader scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-quad) for more information.If the `VkPhysicalDeviceComputeShaderDerivativesFeaturesNVfeatures`. structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///`VkPhysicalDeviceComputeShaderDerivativesFeaturesNVfeatures`.  **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`
///# Related
/// - [`nv_compute_shader_derivatives`]
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
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`compute_derivative_group_quads`] indicates that the implementation
    ///supports the `ComputeDerivativeGroupQuadsNV` SPIR-V capability.
    pub compute_derivative_group_quads: Bool32,
    ///[`compute_derivative_group_linear`] indicates that the implementation
    ///supports the `ComputeDerivativeGroupLinearNV` SPIR-V capability.
    pub compute_derivative_group_linear: Bool32,
}
impl<'lt> Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            compute_derivative_group_quads: 0,
            compute_derivative_group_linear: 0,
        }
    }
}
impl<'lt> PhysicalDeviceComputeShaderDerivativesFeaturesNV<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceComputeShaderDerivativesFeaturesNV<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::compute_derivative_group_quads`]
    pub fn compute_derivative_group_quads_raw(&self) -> Bool32 {
        self.compute_derivative_group_quads
    }
    ///Gets the raw value of [`Self::compute_derivative_group_linear`]
    pub fn compute_derivative_group_linear_raw(&self) -> Bool32 {
        self.compute_derivative_group_linear
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::compute_derivative_group_quads`]
    pub fn set_compute_derivative_group_quads_raw(&mut self, value: Bool32) -> &mut Self {
        self.compute_derivative_group_quads = value;
        self
    }
    ///Sets the raw value of [`Self::compute_derivative_group_linear`]
    pub fn set_compute_derivative_group_linear_raw(&mut self, value: Bool32) -> &mut Self {
        self.compute_derivative_group_linear = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::compute_derivative_group_quads`]
    pub fn with_compute_derivative_group_quads_raw(mut self, value: Bool32) -> Self {
        self.compute_derivative_group_quads = value;
        self
    }
    ///Sets the raw value of [`Self::compute_derivative_group_linear`]
    pub fn with_compute_derivative_group_linear_raw(mut self, value: Bool32) -> Self {
        self.compute_derivative_group_linear = value;
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
    ///Gets the value of [`Self::compute_derivative_group_quads`]
    pub fn compute_derivative_group_quads(&self) -> bool {
        unsafe { std::mem::transmute(self.compute_derivative_group_quads as u8) }
    }
    ///Gets the value of [`Self::compute_derivative_group_linear`]
    pub fn compute_derivative_group_linear(&self) -> bool {
        unsafe { std::mem::transmute(self.compute_derivative_group_linear as u8) }
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
    ///Gets a mutable reference to the value of [`Self::compute_derivative_group_quads`]
    pub fn compute_derivative_group_quads_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.compute_derivative_group_quads as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.compute_derivative_group_quads as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::compute_derivative_group_linear`]
    pub fn compute_derivative_group_linear_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.compute_derivative_group_linear as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.compute_derivative_group_linear as *mut Bool32)
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
    ///Sets the value of [`Self::compute_derivative_group_quads`]
    pub fn set_compute_derivative_group_quads(&mut self, value: bool) -> &mut Self {
        self.compute_derivative_group_quads = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::compute_derivative_group_linear`]
    pub fn set_compute_derivative_group_linear(&mut self, value: bool) -> &mut Self {
        self.compute_derivative_group_linear = value as u8 as u32;
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
    ///Sets the value of [`Self::compute_derivative_group_quads`]
    pub fn with_compute_derivative_group_quads(mut self, value: bool) -> Self {
        self.compute_derivative_group_quads = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::compute_derivative_group_linear`]
    pub fn with_compute_derivative_group_linear(mut self, value: bool) -> Self {
        self.compute_derivative_group_linear = value as u8 as u32;
        self
    }
}
