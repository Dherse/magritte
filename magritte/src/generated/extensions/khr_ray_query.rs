//![VK_KHR_ray_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_query.html) - device extension
//!# Description
//!Rasterization has been the dominant method to produce interactive graphics,
//!but increasing performance of graphics hardware has made ray tracing a
//!viable option for interactive rendering.
//!Being able to integrate ray tracing with traditional rasterization makes it
//!easier for applications to incrementally add ray traced effects to existing
//!applications or to do hybrid approaches with rasterization for primary
//!visibility and ray tracing for secondary queries.Ray queries are available to all shader types,
//! including graphics, compute
//!and ray tracing pipelines.
//!Ray queries are not able to launch additional shaders, instead returning
//!traversal results to the calling shader.This extension adds support for the following SPIR-V
//! extension in Vulkan:
//! - `SPV_KHR_ray_query`
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_KHR_spirv_1_4`]`
//! - Requires `[`VK_KHR_acceleration_structure`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_ray_query]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_ray_query
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRayQueryFeaturesKHR`]
//!# New constants
//! - [`KHR_RAY_QUERY_EXTENSION_NAME`]
//! - [`KHR_RAY_QUERY_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
//!# Known issues & F.A.Q
//!(1) What are the changes between the public provisional (VK_KHR_ray_tracing
//!v8) release and the final (VK_KHR_acceleration_structure v11 /
//!VK_KHR_ray_query v1) release?
//! - refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation flexibility and
//!   decoupling ray query support from ray pipelines:  - `[`VK_KHR_acceleration_structure`]` (for
//!   acceleration structure operations)  - `[`VK_KHR_ray_tracing_pipeline`]` (for ray tracing
//!   pipeline and shader stages)  - `[`VK_KHR_ray_query`]` (for ray queries in existing shader
//!   stages)
//! - Update SPIRV capabilities to use `RayQueryKHR`
//! - extension is no longer provisional
//!# Version History
//! - Revision 1, 2020-11-12 (Mathieu Robart, Daniel Koch, Andrew Garrard)  - Decomposition of the
//!   specification, from VK_KHR_ray_tracing to VK_KHR_ray_query (#1918,!3912)  - update to use
//!   `RayQueryKHR` SPIR-V capability  - add numerical limits for ray parameters (#2235,!3960)  -
//!   relax formula for ray intersection candidate determination (#2322,!4080)  - restrict traces to
//!   TLAS (#2239,!4141)  - require `HitT` to be in ray interval for
//!   `OpRayQueryGenerateIntersectionKHR` (#2359,!4146)  - add ray query shader stages for AS read
//!   bit (#2407,!4203)
//!# Other info
//! * 2020-11-12
//! * - This extension requires [`SPV_KHR_ray_query`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_ray_query.html)
//!   - This extension provides API support for [`GLSL_EXT_ray_query`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_ray_query.txt)
//! * - Matthäus Chajdas, AMD  - Greg Grebe, AMD  - Nicolai Hähnle, AMD  - Tobias Hector, AMD  -
//!   Dave Oldcorn, AMD  - Skyler Saleh, AMD  - Mathieu Robart, Arm  - Marius Bjorge, Arm  - Tom
//!   Olson, Arm  - Sebastian Tafuri, EA  - Henrik Rydgard, Embark  - Juan Cañada, Epic Games  -
//!   Patrick Kelly, Epic Games  - Yuriy O’Donnell, Epic Games  - Michael Doggett, Facebook/Oculus
//!   - Andrew Garrard, Imagination  - Don Scorgie, Imagination  - Dae Kim, Imagination  - Joshua
//!   Barczak, Intel  - Slawek Grajewski, Intel  - Jeff Bolz, NVIDIA  - Pascal Gautron, NVIDIA  -
//!   Daniel Koch, NVIDIA  - Christoph Kubisch, NVIDIA  - Ashwin Lele, NVIDIA  - Robert Stepinski,
//!   NVIDIA  - Martin Stich, NVIDIA  - Nuno Subtil, NVIDIA  - Eric Werness, NVIDIA  - Jon Leech,
//!   Khronos  - Jeroen van Schijndel, OTOY  - Juul Joosten, OTOY  - Alex Bourd, Qualcomm  - Roman
//!   Larionov, Qualcomm  - David McAllister, Qualcomm  - Spencer Fricke, Samsung  - Lewis Gordon,
//!   Samsung  - Ralph Potter, Samsung  - Jasper Bekkers, Traverse Research  - Jesse Barker, Unity
//!   - Baldur Karlsson, Valve
//!# Related
//! - [`PhysicalDeviceRayQueryFeaturesKHR`]
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
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_query");
///[VkPhysicalDeviceRayQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html) - Structure describing the ray query features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRayQueryFeaturesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_query
///typedef struct VkPhysicalDeviceRayQueryFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rayQuery;
///} VkPhysicalDeviceRayQueryFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ray_query`] indicates whether the implementation supports ray query (`OpRayQueryProceedKHR`)
///   functionality.
///If the [`PhysicalDeviceRayQueryFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayQueryFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
///# Related
/// - [`VK_KHR_ray_query`]
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
pub struct PhysicalDeviceRayQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`ray_query`] indicates whether the
    ///implementation supports ray query (`OpRayQueryProceedKHR`)
    ///functionality.
    ray_query: Bool32,
}
impl<'lt> Default for PhysicalDeviceRayQueryFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            ray_query: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRayQueryFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::ray_query`]
    pub fn ray_query_raw(&self) -> Bool32 {
        self.ray_query
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::ray_query`]
    pub fn set_ray_query_raw(&mut self, value: Bool32) -> &mut Self {
        self.ray_query = value;
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
    ///Gets the value of [`Self::ray_query`]
    pub fn ray_query(&self) -> bool {
        unsafe { std::mem::transmute(self.ray_query as u8) }
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
    ///Gets a mutable reference to the value of [`Self::ray_query`]
    pub fn ray_query_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.ray_query as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.ray_query as *mut Bool32)
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
    ///Sets the raw value of [`Self::ray_query`]
    pub fn set_ray_query(&mut self, value: bool) -> &mut Self {
        self.ray_query = value as u8 as u32;
        self
    }
}
