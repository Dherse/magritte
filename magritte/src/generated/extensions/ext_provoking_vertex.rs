//![VK_EXT_provoking_vertex](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_provoking_vertex.html) - device extension
//!# Description
//!This extension allows changing the provoking vertex convention between
//!Vulkan’s default convention (first vertex) and OpenGL’s convention (last
//!vertex).This extension is intended for use by API-translation layers that implement
//!APIs like OpenGL on top of Vulkan, and need to match the source API’s
//!provoking vertex convention.
//!Applications using Vulkan directly should use Vulkan’s default convention.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jesse Hall [jessehall](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_provoking_vertex]
//!   @jessehall%0A<<Here describe the issue or question you have about the VK_EXT_provoking_vertex
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceProvokingVertexFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceProvokingVertexPropertiesEXT`]
//! - Extending [`PipelineRasterizationStateCreateInfo`]:  -
//!   [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
//!# New enums
//! - [`ProvokingVertexModeEXT`]
//!# New constants
//! - [`EXT_PROVOKING_VERTEX_EXTENSION_NAME`]
//! - [`EXT_PROVOKING_VERTEX_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!1) At what granularity should this state be set? **RESOLVED** : At pipeline bind, with an
//! optional per-render pass restriction.The most natural place to put this state is in the graphics
//! pipeline object.
//!Some implementations require it to be known when creating the pipeline, and
//!pipeline state is convenient for implementing OpenGL 3.2’s
//!glProvokingVertex, which can change the state between draw calls.
//!However, some implementations can only change it approximately render pass
//!granularity.
//!To accommodate both, provoking vertex will be pipeline state, but
//!implementations can require that only one mode is used within a render pass
//!instance; the render pass’s mode is chosen implicitly when the first
//!pipeline is bound.2) Does the provoking vertex mode affect the order that vertices are written
//!to transform feedback buffers? **RESOLVED** : Yes, to enable layered implementations of OpenGL
//! and D3D.All of OpenGL, OpenGL ES, and Direct3D 11 require that vertices are written
//!to transform feedback buffers such that flat-shaded attributes have the same
//!value when drawing the contents of the transform feedback buffer as they did
//!in the original drawing when the transform feedback buffer was written
//!(assuming the provoking vertex mode has not changed, in APIs that support
//!more than one mode).
//!# Version History
//! - Revision 1, (1c) 2021-02-22 (Jesse Hall)  - Added
//!   VkPhysicalDeviceProvokingVertexPropertiesEXT::
//!   transformFeedbackPreservesTriangleFanProvokingVertex to accommodate implementations that
//!   cannot change the transform feedback vertex order for triangle fans.
//! - Revision 1, (1b) 2020-06-14 (Jesse Hall)  - Added
//!   VkPhysicalDeviceProvokingVertexFeaturesEXT::transformFeedbackPreservesProvokingVertex and
//!   required that transform feedback write vertices so as to preserve the provoking vertex of each
//!   primitive.
//! - Revision 1, (1a) 2019-10-23 (Jesse Hall)  - Initial draft, based on a proposal by Alexis Hétu
//!# Other info
//! * 2021-02-22
//! * No known IP claims.
//! * - Alexis Hétu, Google  - Bill Licea-Kane, Qualcomm  - Daniel Koch, Nvidia  - Jamie Madill,
//!   Google  - Jan-Harald Fredriksen, Arm  - Jason Ekstrand, Intel  - Jeff Bolz, Nvidia  - Jeff
//!   Leger, Qualcomm  - Jesse Hall, Google  - Jörg Wagner, Arm  - Matthew Netsch, Qualcomm  - Mike
//!   Blumenkrantz, Valve  - Piers Daniell, Nvidia  - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceProvokingVertexFeaturesEXT`]
//! - [`PhysicalDeviceProvokingVertexPropertiesEXT`]
//! - [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
//! - [`ProvokingVertexModeEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_provoking_vertex");
///[VkProvokingVertexModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html) - Specify which vertex in a primitive is the provoking vertex
///# C Specifications
///Possible values of
///[`PipelineRasterizationProvokingVertexStateCreateInfoEXT::provoking_vertex_mode`]
///are:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef enum VkProvokingVertexModeEXT {
///    VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT = 0,
///    VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT = 1,
///} VkProvokingVertexModeEXT;
///```
///# Description
/// - [`ProvokingVertexModeFirstVertexExt`] specifies that the provoking vertex is the first
///   non-adjacency vertex in the list of vertices used by a primitive.
/// - [`ProvokingVertexModeLastVertexExt`] specifies that the provoking vertex is the last
///   non-adjacency vertex in the list of vertices used by a primitive.
///These modes are described more precisely in
///[Primitive Topologies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topologies).
///# Related
/// - [`VK_EXT_provoking_vertex`]
/// - [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkProvokingVertexModeEXT")]
#[doc(alias = "VkProvokingVertexModeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ProvokingVertexModeEXT {
    ///[`ProvokingVertexModeFirstVertexExt`] specifies that the
    ///provoking vertex is the first non-adjacency vertex in the list of
    ///vertices used by a primitive.
    ProvokingVertexModeFirstVertexExt = 0,
    ///[`ProvokingVertexModeLastVertexExt`] specifies that the
    ///provoking vertex is the last non-adjacency vertex in the list of
    ///vertices used by a primitive.
    ProvokingVertexModeLastVertexExt = 1,
}
impl const Default for ProvokingVertexModeEXT {
    fn default() -> Self {
        Self::ProvokingVertexModeFirstVertexExt
    }
}
impl ProvokingVertexModeEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceProvokingVertexFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) - Structure describing the provoking vertex features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           provokingVertexLast;
///    VkBool32           transformFeedbackPreservesProvokingVertex;
///} VkPhysicalDeviceProvokingVertexFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_last`] indicates whether the implementation supports the
///   `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`[`ProvokingVertexModeEXT`] for flat shading.
/// - [`transform_feedback_preserves_provoking_vertex`] indicates that the order of vertices within each primitive written by transform feedback will preserve the provoking vertex. This does not apply to triangle fan primitives when [`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex) is [`FALSE`]. [`transform_feedback_preserves_provoking_vertex`] **must**  be [`FALSE`] when the [`VK_EXT_transform_feedback`] extension is not supported.
///If the [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceProvokingVertexFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.When
/// [`PhysicalDeviceProvokingVertexFeaturesEXT`] is in the [`p_next`]
///chain of [`DeviceCreateInfo`] but the
///[transform feedback feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-transformFeedback) is not enabled,
///the value of [`transform_feedback_preserves_provoking_vertex`] is ignored.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`
///# Related
/// - [`VK_EXT_provoking_vertex`]
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
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`provoking_vertex_last`] indicates
    ///whether the implementation supports the
    ///`VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`[`ProvokingVertexModeEXT`] for flat shading.
    pub provoking_vertex_last: Bool32,
    ///[`transform_feedback_preserves_provoking_vertex`] indicates that the order
    ///of vertices within each primitive written by transform feedback will
    ///preserve the provoking vertex.
    ///This does not apply to triangle fan primitives when
    ///[`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex)
    ///is [`FALSE`].
    ///[`transform_feedback_preserves_provoking_vertex`] **must**  be [`FALSE`]
    ///when the [`VK_EXT_transform_feedback`] extension is not supported.
    pub transform_feedback_preserves_provoking_vertex: Bool32,
}
impl<'lt> Default for PhysicalDeviceProvokingVertexFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            provoking_vertex_last: 0,
            transform_feedback_preserves_provoking_vertex: 0,
        }
    }
}
impl<'lt> PhysicalDeviceProvokingVertexFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::provoking_vertex_last`]
    pub fn provoking_vertex_last_raw(&self) -> Bool32 {
        self.provoking_vertex_last
    }
    ///Gets the raw value of [`Self::transform_feedback_preserves_provoking_vertex`]
    pub fn transform_feedback_preserves_provoking_vertex_raw(&self) -> Bool32 {
        self.transform_feedback_preserves_provoking_vertex
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::provoking_vertex_last`]
    pub fn set_provoking_vertex_last_raw(&mut self, value: Bool32) -> &mut Self {
        self.provoking_vertex_last = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_preserves_provoking_vertex`]
    pub fn set_transform_feedback_preserves_provoking_vertex_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_preserves_provoking_vertex = value;
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
    ///Gets the value of [`Self::provoking_vertex_last`]
    pub fn provoking_vertex_last(&self) -> bool {
        unsafe { std::mem::transmute(self.provoking_vertex_last as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_preserves_provoking_vertex`]
    pub fn transform_feedback_preserves_provoking_vertex(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_preserves_provoking_vertex as u8) }
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
    ///Gets a mutable reference to the value of [`Self::provoking_vertex_last`]
    pub fn provoking_vertex_last_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.provoking_vertex_last as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.provoking_vertex_last as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_preserves_provoking_vertex`]
    pub fn transform_feedback_preserves_provoking_vertex_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_preserves_provoking_vertex as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_preserves_provoking_vertex as *mut Bool32)
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
    ///Sets the raw value of [`Self::provoking_vertex_last`]
    pub fn set_provoking_vertex_last(&mut self, value: bool) -> &mut Self {
        self.provoking_vertex_last = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_preserves_provoking_vertex`]
    pub fn set_transform_feedback_preserves_provoking_vertex(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_preserves_provoking_vertex = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceProvokingVertexPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html) - Structure describing provoking vertex properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProvokingVertexPropertiesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           provokingVertexModePerPipeline;
///    VkBool32           transformFeedbackPreservesTriangleFanProvokingVertex;
///} VkPhysicalDeviceProvokingVertexPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_mode_per_pipeline`] indicates whether the implementation supports graphics
///   pipelines with different provoking vertex modes within the same render pass instance.
/// - [`transform_feedback_preserves_triangle_fan_provoking_vertex`] indicates whether the
///   implementation can preserve the provoking vertex order when writing triangle fan vertices to
///   transform feedback.
///# Description
///If the [`PhysicalDeviceProvokingVertexPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_provoking_vertex`]
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
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`provoking_vertex_mode_per_pipeline`] indicates whether the
    ///implementation supports graphics pipelines with different provoking
    ///vertex modes within the same render pass instance.
    pub provoking_vertex_mode_per_pipeline: Bool32,
    ///[`transform_feedback_preserves_triangle_fan_provoking_vertex`] indicates
    ///whether the implementation can preserve the provoking vertex order when
    ///writing triangle fan vertices to transform feedback.
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}
impl<'lt> Default for PhysicalDeviceProvokingVertexPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            provoking_vertex_mode_per_pipeline: 0,
            transform_feedback_preserves_triangle_fan_provoking_vertex: 0,
        }
    }
}
impl<'lt> PhysicalDeviceProvokingVertexPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::provoking_vertex_mode_per_pipeline`]
    pub fn provoking_vertex_mode_per_pipeline_raw(&self) -> Bool32 {
        self.provoking_vertex_mode_per_pipeline
    }
    ///Gets the raw value of [`Self::transform_feedback_preserves_triangle_fan_provoking_vertex`]
    pub fn transform_feedback_preserves_triangle_fan_provoking_vertex_raw(&self) -> Bool32 {
        self.transform_feedback_preserves_triangle_fan_provoking_vertex
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::provoking_vertex_mode_per_pipeline`]
    pub fn set_provoking_vertex_mode_per_pipeline_raw(&mut self, value: Bool32) -> &mut Self {
        self.provoking_vertex_mode_per_pipeline = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_preserves_triangle_fan_provoking_vertex`]
    pub fn set_transform_feedback_preserves_triangle_fan_provoking_vertex_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_preserves_triangle_fan_provoking_vertex = value;
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
    ///Gets the value of [`Self::provoking_vertex_mode_per_pipeline`]
    pub fn provoking_vertex_mode_per_pipeline(&self) -> bool {
        unsafe { std::mem::transmute(self.provoking_vertex_mode_per_pipeline as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_preserves_triangle_fan_provoking_vertex`]
    pub fn transform_feedback_preserves_triangle_fan_provoking_vertex(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_preserves_triangle_fan_provoking_vertex as u8) }
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
    ///Gets a mutable reference to the value of [`Self::provoking_vertex_mode_per_pipeline`]
    pub fn provoking_vertex_mode_per_pipeline_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.provoking_vertex_mode_per_pipeline as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.provoking_vertex_mode_per_pipeline as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_preserves_triangle_fan_provoking_vertex`]
    pub fn transform_feedback_preserves_triangle_fan_provoking_vertex_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_preserves_triangle_fan_provoking_vertex as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_preserves_triangle_fan_provoking_vertex as *mut Bool32)
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
    ///Sets the raw value of [`Self::provoking_vertex_mode_per_pipeline`]
    pub fn set_provoking_vertex_mode_per_pipeline(&mut self, value: bool) -> &mut Self {
        self.provoking_vertex_mode_per_pipeline = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_preserves_triangle_fan_provoking_vertex`]
    pub fn set_transform_feedback_preserves_triangle_fan_provoking_vertex(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_preserves_triangle_fan_provoking_vertex = value as u8 as u32;
        self
    }
}
///[VkPipelineRasterizationProvokingVertexStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) - Structure specifying provoking vertex mode used by a graphics pipeline
///# C Specifications
///For a given primitive topology, the pipeline’s provoking vertex mode
///determines which vertex is the provoking vertex.
///To specify the provoking vertex mode, include a
///[`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure in
///the [`PipelineRasterizationStateCreateInfo`]::[`p_next`] chain when
///creating the pipeline.The [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure
///is defined as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkProvokingVertexModeEXT    provokingVertexMode;
///} VkPipelineRasterizationProvokingVertexStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_mode`] is a [`ProvokingVertexModeEXT`] value selecting the provoking vertex
///   mode.
///# Description
///If this struct is not provided when creating the pipeline, the pipeline will
///use the `VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT` mode.If the
///[provokingVertexModePerPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-provokingVertexModePerPipeline)
///limit is [`FALSE`], then all pipelines bound within a render pass
///instance  **must**  have the same [`provoking_vertex_mode`].
///## Valid Usage
/// - If [`provoking_vertex_mode`] is `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`, then the [provokingVertexLast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-provokingVertexLast)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
/// - [`provoking_vertex_mode`] **must**  be a valid [`ProvokingVertexModeEXT`] value
///# Related
/// - [`VK_EXT_provoking_vertex`]
/// - [`ProvokingVertexModeEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`provoking_vertex_mode`] is a [`ProvokingVertexModeEXT`] value
    ///selecting the provoking vertex mode.
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}
impl<'lt> Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            provoking_vertex_mode: Default::default(),
        }
    }
}
impl<'lt> PipelineRasterizationProvokingVertexStateCreateInfoEXT<'lt> {
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
    ///Gets the value of [`Self::provoking_vertex_mode`]
    pub fn provoking_vertex_mode(&self) -> ProvokingVertexModeEXT {
        self.provoking_vertex_mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::provoking_vertex_mode`]
    pub fn provoking_vertex_mode_mut(&mut self) -> &mut ProvokingVertexModeEXT {
        &mut self.provoking_vertex_mode
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
    ///Sets the raw value of [`Self::provoking_vertex_mode`]
    pub fn set_provoking_vertex_mode(
        &mut self,
        value: crate::extensions::ext_provoking_vertex::ProvokingVertexModeEXT,
    ) -> &mut Self {
        self.provoking_vertex_mode = value;
        self
    }
}
