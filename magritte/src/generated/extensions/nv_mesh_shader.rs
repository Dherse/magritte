//![VK_NV_mesh_shader](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_mesh_shader.html) - device extension
//!# Description
//!This extension provides a new mechanism allowing applications to generate
//!collections of geometric primitives via programmable mesh shading.
//!It is an alternative to the existing programmable primitive shading
//!pipeline, which relied on generating input primitives by a fixed function
//!assembler as well as fixed function vertex fetch.There are new programmable shader types — the
//! task and mesh shader — to
//!generate these collections to be processed by fixed-function primitive
//!assembly and rasterization logic.
//!When task and mesh shaders are dispatched, they replace the core
//![pre-rasterization stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization),
//!including vertex array attribute fetching, vertex shader processing,
//!tessellation, and geometry shader processing.This extension also adds support for the following
//! SPIR-V extension in
//!Vulkan:
//! - [`SPV_NV_mesh_shader`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_mesh_shader.html)
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Christoph Kubisch [pixeljetstream](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_mesh_shader]
//!   @pixeljetstream%0A<<Here describe the issue or question you have about the VK_NV_mesh_shader
//!   extension>>)
//!# New functions & commands
//! - [`CmdDrawMeshTasksIndirectCountNV`]
//! - [`CmdDrawMeshTasksIndirectNV`]
//! - [`CmdDrawMeshTasksNV`]
//!# New structures
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMeshShaderFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMeshShaderPropertiesNV`]
//!# New constants
//! - [`NV_MESH_SHADER_EXTENSION_NAME`]
//! - [`NV_MESH_SHADER_SPEC_VERSION`]
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
//! - Extending [`ShaderStageFlagBits`]:  - `VK_SHADER_STAGE_MESH_BIT_NV`  -
//!   `VK_SHADER_STAGE_TASK_BIT_NV`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!0. How to name this extension? **RESOLVED** : VK_NV_mesh_shaderOther options considered:  -
//! VK_NV_mesh_shading  - VK_NV_programmable_mesh_shading  - VK_NV_primitive_group_shading  -
//! VK_NV_grouped_drawing
//!1. Do we need a new VkPrimitiveTopology? **RESOLVED** : No. We skip the InputAssembler stage.
//!2. Should we allow Instancing? **RESOLVED** : No. There is no fixed function input, other than
//! the IDs. However, allow offsetting with a “first” value.
//!3. Should we use existing vkCmdDraw or introduce new functions? **RESOLVED** : Introduce new
//! functions.New functions make it easier to separate from “programmable primitive shading”
//! chapter, less “dual use” language about existing functions having alternative behavior. The text
//! around the existing “draws” is heavily based around emitting vertices.
//!4. If new functions, how to name? **RESOLVED** : CmdDrawMeshTasks*Other options considered:  -
//! CmdDrawMeshed  - CmdDrawTasked  - CmdDrawGrouped
//!5. Should VK_SHADER_STAGE_ALL_GRAPHICS be updated to include the new stages? **RESOLVED** : No.
//! If an application were to be recompiled with headers that include additional shader stage bits
//! in VK_SHADER_STAGE_ALL_GRAPHICS, then the previously valid application would no longer be valid
//! on implementations that do not support mesh or task shaders. This means the change would not be
//! backwards compatible. It is too bad VkShaderStageFlagBits does not have a dedicated “all
//! supported graphics stages” bit like VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT, which would have avoided
//! this problem.
//!# Version History
//! - Revision 1, 2018-07-19 (Christoph Kubisch, Daniel Koch)  - Internal revisions
//!# Other info
//! * 2018-07-19
//! * - This extension requires [`SPV_NV_mesh_shader`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_mesh_shader.html)
//!   - This extension provides API support for [`GLSL_NV_mesh_shader`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_mesh_shader.txt)
//! * - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Piers Daniell, NVIDIA  -
//!   Pierre Boudier, NVIDIA
//!# Related
//! - [`DrawMeshTasksIndirectCommandNV`]
//! - [`PhysicalDeviceMeshShaderFeaturesNV`]
//! - [`PhysicalDeviceMeshShaderPropertiesNV`]
//! - [`CmdDrawMeshTasksIndirectCountNV`]
//! - [`CmdDrawMeshTasksIndirectNV`]
//! - [`CmdDrawMeshTasksNV`]
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
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_mesh_shader");
///[VkPhysicalDeviceMeshShaderFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderFeaturesNV.html) - Structure describing mesh shading features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMeshShaderFeaturesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           taskShader;
///    VkBool32           meshShader;
///} VkPhysicalDeviceMeshShaderFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`task_shader`] indicates whether the task shader stage is supported.
/// - [`mesh_shader`] indicates whether the mesh shader stage is supported.
///If the [`PhysicalDeviceMeshShaderFeaturesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMeshShaderFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
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
pub struct PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`task_shader`] indicates whether the task
    ///shader stage is supported.
    task_shader: Bool32,
    ///[`mesh_shader`] indicates whether the mesh
    ///shader stage is supported.
    mesh_shader: Bool32,
}
impl<'lt> Default for PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            task_shader: 0,
            mesh_shader: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMeshShaderFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::task_shader`]
    pub fn task_shader_raw(&self) -> Bool32 {
        self.task_shader
    }
    ///Gets the raw value of [`Self::mesh_shader`]
    pub fn mesh_shader_raw(&self) -> Bool32 {
        self.mesh_shader
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::task_shader`]
    pub fn set_task_shader_raw(&mut self, value: Bool32) -> &mut Self {
        self.task_shader = value;
        self
    }
    ///Sets the raw value of [`Self::mesh_shader`]
    pub fn set_mesh_shader_raw(&mut self, value: Bool32) -> &mut Self {
        self.mesh_shader = value;
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
    ///Gets the value of [`Self::task_shader`]
    pub fn task_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.task_shader as u8) }
    }
    ///Gets the value of [`Self::mesh_shader`]
    pub fn mesh_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.mesh_shader as u8) }
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
    ///Gets a mutable reference to the value of [`Self::task_shader`]
    pub fn task_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.task_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.task_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::mesh_shader`]
    pub fn mesh_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.mesh_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.mesh_shader as *mut Bool32)
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
    ///Sets the raw value of [`Self::task_shader`]
    pub fn set_task_shader(&mut self, value: bool) -> &mut Self {
        self.task_shader = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::mesh_shader`]
    pub fn set_mesh_shader(&mut self, value: bool) -> &mut Self {
        self.mesh_shader = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceMeshShaderPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMeshShaderPropertiesNV.html) - Structure describing mesh shading properties
///# C Specifications
///The [`PhysicalDeviceMeshShaderPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkPhysicalDeviceMeshShaderPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxDrawMeshTasksCount;
///    uint32_t           maxTaskWorkGroupInvocations;
///    uint32_t           maxTaskWorkGroupSize[3];
///    uint32_t           maxTaskTotalMemorySize;
///    uint32_t           maxTaskOutputCount;
///    uint32_t           maxMeshWorkGroupInvocations;
///    uint32_t           maxMeshWorkGroupSize[3];
///    uint32_t           maxMeshTotalMemorySize;
///    uint32_t           maxMeshOutputVertices;
///    uint32_t           maxMeshOutputPrimitives;
///    uint32_t           maxMeshMultiviewViewCount;
///    uint32_t           meshOutputPerVertexGranularity;
///    uint32_t           meshOutputPerPrimitiveGranularity;
///} VkPhysicalDeviceMeshShaderPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_draw_mesh_tasks_count`] is the maximum number of local workgroups that  **can**  be launched by a single draw mesh tasks command. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
/// - [`max_task_work_group_invocations`] is the maximum total number of task     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
/// - [`max_task_work_group_size`][3] is the maximum size of a local task     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
/// - [`max_task_total_memory_size`] is the maximum number of bytes that the task shader can use in
///   total for shared and output memory combined.
/// - [`max_task_output_count`] is the maximum number of output tasks a single task shader workgroup
///   can emit.
/// - [`max_mesh_work_group_invocations`] is the maximum total number of mesh     shader invocations
///   in a single local workgroup.     The product of the X, Y, and Z sizes, as specified by the
///   `LocalSize` or `LocalSizeId`     execution mode in shader modules or by the object decorated
///   by the     `WorkgroupSize` decoration,  **must**  be less than or equal to this     limit.
/// - [`max_mesh_work_group_size`][3] is the maximum size of a local mesh     workgroup.     These
///   three values represent the maximum local workgroup size in the X,     Y, and Z dimensions,
///   respectively.     The `x`, `y`, and `z` sizes, as specified by the     `LocalSize` or
///   `LocalSizeId`     execution mode or by the object decorated by the `WorkgroupSize`
///   decoration in shader modules,  **must**  be less than or equal to the     corresponding limit.
/// - [`max_mesh_total_memory_size`] is the maximum number of bytes that the mesh shader can use in
///   total for shared and output memory combined.
/// - [`max_mesh_output_vertices`] is the maximum number of vertices a mesh shader output can store.
/// - [`max_mesh_output_primitives`] is the maximum number of primitives a mesh shader output can
///   store.
/// - [`max_mesh_multiview_view_count`] is the maximum number of multi-view views a mesh shader can
///   use.
/// - [`mesh_output_per_vertex_granularity`] is the granularity with which mesh vertex outputs are
///   allocated. The value can be used to compute the memory size used by the mesh shader, which
///   must be less than or equal to [`max_mesh_total_memory_size`].
/// - [`mesh_output_per_primitive_granularity`] is the granularity with which mesh outputs qualified
///   as per-primitive are allocated. The value can be used to compute the memory size used by the
///   mesh shader, which must be less than or equal to [`max_mesh_total_memory_size`].
///# Description
///If the [`PhysicalDeviceMeshShaderPropertiesNV`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV`
///# Related
/// - [`VK_NV_mesh_shader`]
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
pub struct PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_draw_mesh_tasks_count`] is the maximum number of local workgroups
    ///that  **can**  be launched by a single draw mesh tasks command.
    ///See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-mesh-shading).
    max_draw_mesh_tasks_count: u32,
    ///[`max_task_work_group_invocations`] is the maximum total number of task
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration,  **must**  be less than or equal to this
    ///    limit.
    max_task_work_group_invocations: u32,
    ///[`max_task_work_group_size`][3] is the maximum size of a local task
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules,  **must**  be less than or equal to the
    ///    corresponding limit.
    max_task_work_group_size: [u32; 3],
    ///[`max_task_total_memory_size`] is the maximum number of bytes that the
    ///task shader can use in total for shared and output memory combined.
    max_task_total_memory_size: u32,
    ///[`max_task_output_count`] is the maximum number of output tasks a single
    ///task shader workgroup can emit.
    max_task_output_count: u32,
    ///[`max_mesh_work_group_invocations`] is the maximum total number of mesh
    ///    shader invocations in a single local workgroup.
    ///    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode in shader modules or by the object decorated by the
    ///    `WorkgroupSize` decoration,  **must**  be less than or equal to this
    ///    limit.
    max_mesh_work_group_invocations: u32,
    ///[`max_mesh_work_group_size`][3] is the maximum size of a local mesh
    ///    workgroup.
    ///    These three values represent the maximum local workgroup size in the X,
    ///    Y, and Z dimensions, respectively.
    ///    The `x`, `y`, and `z` sizes, as specified by the
    ///    `LocalSize`
    ///or `LocalSizeId`
    ///    execution mode or by the object decorated by the `WorkgroupSize`
    ///    decoration in shader modules,  **must**  be less than or equal to the
    ///    corresponding limit.
    max_mesh_work_group_size: [u32; 3],
    ///[`max_mesh_total_memory_size`] is the maximum number of bytes that the
    ///mesh shader can use in total for shared and output memory combined.
    max_mesh_total_memory_size: u32,
    ///[`max_mesh_output_vertices`] is the maximum number of vertices a mesh
    ///shader output can store.
    max_mesh_output_vertices: u32,
    ///[`max_mesh_output_primitives`] is the maximum number of primitives a mesh
    ///shader output can store.
    max_mesh_output_primitives: u32,
    ///[`max_mesh_multiview_view_count`] is the maximum number of multi-view
    ///views a mesh shader can use.
    max_mesh_multiview_view_count: u32,
    ///[`mesh_output_per_vertex_granularity`] is the granularity with which mesh
    ///vertex outputs are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    mesh_output_per_vertex_granularity: u32,
    ///[`mesh_output_per_primitive_granularity`] is the granularity with which
    ///mesh outputs qualified as per-primitive are allocated.
    ///The value can be used to compute the memory size used by the mesh
    ///shader, which must be less than or equal to
    ///[`max_mesh_total_memory_size`].
    mesh_output_per_primitive_granularity: u32,
}
impl<'lt> Default for PhysicalDeviceMeshShaderPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_draw_mesh_tasks_count: 0,
            max_task_work_group_invocations: 0,
            max_task_work_group_size: [0; 3],
            max_task_total_memory_size: 0,
            max_task_output_count: 0,
            max_mesh_work_group_invocations: 0,
            max_mesh_work_group_size: [0; 3],
            max_mesh_total_memory_size: 0,
            max_mesh_output_vertices: 0,
            max_mesh_output_primitives: 0,
            max_mesh_multiview_view_count: 0,
            mesh_output_per_vertex_granularity: 0,
            mesh_output_per_primitive_granularity: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMeshShaderPropertiesNV<'lt> {
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
    ///Gets the value of [`Self::max_draw_mesh_tasks_count`]
    pub fn max_draw_mesh_tasks_count(&self) -> u32 {
        self.max_draw_mesh_tasks_count
    }
    ///Gets the value of [`Self::max_task_work_group_invocations`]
    pub fn max_task_work_group_invocations(&self) -> u32 {
        self.max_task_work_group_invocations
    }
    ///Gets the value of [`Self::max_task_work_group_size`]
    pub fn max_task_work_group_size(&self) -> &[u32; 3] {
        &getter
    }
    ///Gets the value of [`Self::max_task_total_memory_size`]
    pub fn max_task_total_memory_size(&self) -> u32 {
        self.max_task_total_memory_size
    }
    ///Gets the value of [`Self::max_task_output_count`]
    pub fn max_task_output_count(&self) -> u32 {
        self.max_task_output_count
    }
    ///Gets the value of [`Self::max_mesh_work_group_invocations`]
    pub fn max_mesh_work_group_invocations(&self) -> u32 {
        self.max_mesh_work_group_invocations
    }
    ///Gets the value of [`Self::max_mesh_work_group_size`]
    pub fn max_mesh_work_group_size(&self) -> &[u32; 3] {
        &getter
    }
    ///Gets the value of [`Self::max_mesh_total_memory_size`]
    pub fn max_mesh_total_memory_size(&self) -> u32 {
        self.max_mesh_total_memory_size
    }
    ///Gets the value of [`Self::max_mesh_output_vertices`]
    pub fn max_mesh_output_vertices(&self) -> u32 {
        self.max_mesh_output_vertices
    }
    ///Gets the value of [`Self::max_mesh_output_primitives`]
    pub fn max_mesh_output_primitives(&self) -> u32 {
        self.max_mesh_output_primitives
    }
    ///Gets the value of [`Self::max_mesh_multiview_view_count`]
    pub fn max_mesh_multiview_view_count(&self) -> u32 {
        self.max_mesh_multiview_view_count
    }
    ///Gets the value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn mesh_output_per_vertex_granularity(&self) -> u32 {
        self.mesh_output_per_vertex_granularity
    }
    ///Gets the value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn mesh_output_per_primitive_granularity(&self) -> u32 {
        self.mesh_output_per_primitive_granularity
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
    ///Gets a mutable reference to the value of [`Self::max_draw_mesh_tasks_count`]
    pub fn max_draw_mesh_tasks_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_task_work_group_invocations`]
    pub fn max_task_work_group_invocations_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_task_work_group_size`]
    pub fn max_task_work_group_size_mut(&mut self) -> &mut [u32; 3] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_task_total_memory_size`]
    pub fn max_task_total_memory_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_task_output_count`]
    pub fn max_task_output_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_work_group_invocations`]
    pub fn max_mesh_work_group_invocations_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_work_group_size`]
    pub fn max_mesh_work_group_size_mut(&mut self) -> &mut [u32; 3] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_total_memory_size`]
    pub fn max_mesh_total_memory_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_output_vertices`]
    pub fn max_mesh_output_vertices_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_output_primitives`]
    pub fn max_mesh_output_primitives_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_mesh_multiview_view_count`]
    pub fn max_mesh_multiview_view_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn mesh_output_per_vertex_granularity_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn mesh_output_per_primitive_granularity_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::max_draw_mesh_tasks_count`]
    pub fn set_max_draw_mesh_tasks_count(&mut self, value: u32) -> &mut Self {
        self.max_draw_mesh_tasks_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_task_work_group_invocations`]
    pub fn set_max_task_work_group_invocations(&mut self, value: u32) -> &mut Self {
        self.max_task_work_group_invocations = value;
        self
    }
    ///Sets the raw value of [`Self::max_task_work_group_size`]
    pub fn set_max_task_work_group_size(&mut self, value: [u32; 3]) -> &mut Self {
        self.max_task_work_group_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_task_total_memory_size`]
    pub fn set_max_task_total_memory_size(&mut self, value: u32) -> &mut Self {
        self.max_task_total_memory_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_task_output_count`]
    pub fn set_max_task_output_count(&mut self, value: u32) -> &mut Self {
        self.max_task_output_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_work_group_invocations`]
    pub fn set_max_mesh_work_group_invocations(&mut self, value: u32) -> &mut Self {
        self.max_mesh_work_group_invocations = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_work_group_size`]
    pub fn set_max_mesh_work_group_size(&mut self, value: [u32; 3]) -> &mut Self {
        self.max_mesh_work_group_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_total_memory_size`]
    pub fn set_max_mesh_total_memory_size(&mut self, value: u32) -> &mut Self {
        self.max_mesh_total_memory_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_output_vertices`]
    pub fn set_max_mesh_output_vertices(&mut self, value: u32) -> &mut Self {
        self.max_mesh_output_vertices = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_output_primitives`]
    pub fn set_max_mesh_output_primitives(&mut self, value: u32) -> &mut Self {
        self.max_mesh_output_primitives = value;
        self
    }
    ///Sets the raw value of [`Self::max_mesh_multiview_view_count`]
    pub fn set_max_mesh_multiview_view_count(&mut self, value: u32) -> &mut Self {
        self.max_mesh_multiview_view_count = value;
        self
    }
    ///Sets the raw value of [`Self::mesh_output_per_vertex_granularity`]
    pub fn set_mesh_output_per_vertex_granularity(&mut self, value: u32) -> &mut Self {
        self.mesh_output_per_vertex_granularity = value;
        self
    }
    ///Sets the raw value of [`Self::mesh_output_per_primitive_granularity`]
    pub fn set_mesh_output_per_primitive_granularity(&mut self, value: u32) -> &mut Self {
        self.mesh_output_per_primitive_granularity = value;
        self
    }
}
///[VkDrawMeshTasksIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDrawMeshTasksIndirectCommandNV.html) - Structure specifying a mesh tasks draw indirect command
///# C Specifications
///The [`DrawMeshTasksIndirectCommandNV`] structure is defined as:
///```c
///// Provided by VK_NV_mesh_shader
///typedef struct VkDrawMeshTasksIndirectCommandNV {
///    uint32_t    taskCount;
///    uint32_t    firstTask;
///} VkDrawMeshTasksIndirectCommandNV;
///```
///# Members
/// - [`task_count`] is the number of local workgroups to dispatch in the X dimension. Y and Z
///   dimension are implicitly set to one.
/// - [`first_task`] is the X component of the first workgroup ID.
///# Description
///The members of [`DrawMeshTasksIndirectCommandNV`] have the same meaning
///as the similarly named parameters of [`CmdDrawMeshTasksNV`].
///## Valid Usage
/// - [`task_count`] **must**  be less than or equal to
///   [`PhysicalDeviceMeshShaderPropertiesNV::max_draw_mesh_tasks_count`]
///# Related
/// - [`VK_NV_mesh_shader`]
/// - [`CmdDrawMeshTasksIndirectNV`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrawMeshTasksIndirectCommandNV {
    ///[`task_count`] is the number of local workgroups to dispatch in the X
    ///dimension.
    ///Y and Z dimension are implicitly set to one.
    task_count: u32,
    ///[`first_task`] is the X component of the first workgroup ID.
    first_task: u32,
}
impl Default for DrawMeshTasksIndirectCommandNV {
    fn default() -> Self {
        Self {
            task_count: 0,
            first_task: 0,
        }
    }
}
impl DrawMeshTasksIndirectCommandNV {
    ///Gets the value of [`Self::task_count`]
    pub fn task_count(&self) -> u32 {
        self.task_count
    }
    ///Gets the value of [`Self::first_task`]
    pub fn first_task(&self) -> u32 {
        self.first_task
    }
    ///Gets a mutable reference to the value of [`Self::task_count`]
    pub fn task_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::first_task`]
    pub fn first_task_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::task_count`]
    pub fn set_task_count(&mut self, value: u32) -> &mut Self {
        self.task_count = value;
        self
    }
    ///Sets the raw value of [`Self::first_task`]
    pub fn set_first_task(&mut self, value: u32) -> &mut Self {
        self.first_task = value;
        self
    }
}
