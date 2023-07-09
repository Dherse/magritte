pub use crate::common::extensions::khr_acceleration_structure::{
    AabbPositionsKHR, AccelerationStructureBuildRangeInfoKHR, AccelerationStructureBuildTypeKHR,
    AccelerationStructureCompatibilityKHR, AccelerationStructureCreateFlagBitsKHR, AccelerationStructureCreateFlagsKHR,
    AccelerationStructureInstanceKHR, AccelerationStructureTypeKHR, BuildAccelerationStructureFlagBitsKHR,
    BuildAccelerationStructureFlagsKHR, BuildAccelerationStructureModeKHR, CopyAccelerationStructureModeKHR,
    GeometryFlagBitsKHR, GeometryFlagsKHR, GeometryInstanceFlagBitsKHR, GeometryInstanceFlagsKHR, GeometryTypeKHR,
    TransformMatrixKHR, KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME, KHR_ACCELERATION_STRUCTURE_SPEC_VERSION,
};
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV;
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
use crate::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV;
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{Buffer, DeviceAddress, DeviceSize, Format, IndexType, StructureType, UUID_SIZE},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    #[doc(alias = "pAccelerationStructures")]
    pub acceleration_structures: SmallVec<[AccelerationStructureKHR; 8]>,
}
impl WriteDescriptorSetAccelerationStructureKHR {
    ///Get a reference to the `acceleration_structures` field.
    pub fn acceleration_structures(&self) -> &SmallVec<[AccelerationStructureKHR; 8]> {
        &self.acceleration_structures
    }
    ///Get a mutable reference to the `acceleration_structures` field.
    pub fn acceleration_structures_mut(&mut self) -> &mut SmallVec<[AccelerationStructureKHR; 8]> {
        &mut self.acceleration_structures
    }
    ///Sets the `acceleration_structures` field.
    pub fn set_acceleration_structures(
        &mut self,
        acceleration_structures: SmallVec<[AccelerationStructureKHR; 8]>,
    ) -> &mut Self {
        self.acceleration_structures = acceleration_structures;
        self
    }
    ///Sets the `acceleration_structures` field in a builder way.
    pub fn with_acceleration_structures(
        mut self,
        acceleration_structures: SmallVec<[AccelerationStructureKHR; 8]>,
    ) -> Self {
        self.acceleration_structures = acceleration_structures;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for WriteDescriptorSetAccelerationStructureKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::WriteDescriptorSetAccelerationStructureKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_acceleration_structures = self.acceleration_structures.len() as u32;
        let acceleration_structures = bump
            .alloc_slice_fill_iter(
                self.acceleration_structures
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::khr_acceleration_structure::WriteDescriptorSetAccelerationStructureKHR {
            s_type: StructureType::WriteDescriptorSetAccelerationStructureKhr,
            p_next: std::ptr::null(),
            acceleration_structure_count: len_acceleration_structures,
            acceleration_structures: acceleration_structures,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for WriteDescriptorSetAccelerationStructureKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let acceleration_structures_len = value.acceleration_structure_count;
        let mut acceleration_structures = SmallVec::with_capacity(acceleration_structures_len as usize);
        for i in 0..acceleration_structures_len {
            acceleration_structures.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structures.add(i as usize).read(),
            ));
        }
        Self {
            acceleration_structures: acceleration_structures,
        }
    }
}
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: bool,
    #[doc(alias = "accelerationStructureCaptureReplay")]
    pub acceleration_structure_capture_replay: bool,
    #[doc(alias = "accelerationStructureIndirectBuild")]
    pub acceleration_structure_indirect_build: bool,
    #[doc(alias = "accelerationStructureHostCommands")]
    pub acceleration_structure_host_commands: bool,
    #[doc(alias = "descriptorBindingAccelerationStructureUpdateAfterBind")]
    pub descriptor_binding_acceleration_structure_update_after_bind: bool,
}
impl PhysicalDeviceAccelerationStructureFeaturesKHR {
    ///Get a reference to the `acceleration_structure` field.
    pub fn acceleration_structure(&self) -> &bool {
        &self.acceleration_structure
    }
    ///Get a reference to the `acceleration_structure_capture_replay` field.
    pub fn acceleration_structure_capture_replay(&self) -> &bool {
        &self.acceleration_structure_capture_replay
    }
    ///Get a reference to the `acceleration_structure_indirect_build` field.
    pub fn acceleration_structure_indirect_build(&self) -> &bool {
        &self.acceleration_structure_indirect_build
    }
    ///Get a reference to the `acceleration_structure_host_commands` field.
    pub fn acceleration_structure_host_commands(&self) -> &bool {
        &self.acceleration_structure_host_commands
    }
    ///Get a reference to the `descriptor_binding_acceleration_structure_update_after_bind` field.
    pub fn descriptor_binding_acceleration_structure_update_after_bind(&self) -> &bool {
        &self.descriptor_binding_acceleration_structure_update_after_bind
    }
    ///Get a mutable reference to the `acceleration_structure` field.
    pub fn acceleration_structure_mut(&mut self) -> &mut bool {
        &mut self.acceleration_structure
    }
    ///Get a mutable reference to the `acceleration_structure_capture_replay` field.
    pub fn acceleration_structure_capture_replay_mut(&mut self) -> &mut bool {
        &mut self.acceleration_structure_capture_replay
    }
    ///Get a mutable reference to the `acceleration_structure_indirect_build` field.
    pub fn acceleration_structure_indirect_build_mut(&mut self) -> &mut bool {
        &mut self.acceleration_structure_indirect_build
    }
    ///Get a mutable reference to the `acceleration_structure_host_commands` field.
    pub fn acceleration_structure_host_commands_mut(&mut self) -> &mut bool {
        &mut self.acceleration_structure_host_commands
    }
    ///Get a mutable reference to the `descriptor_binding_acceleration_structure_update_after_bind`
    /// field.
    pub fn descriptor_binding_acceleration_structure_update_after_bind_mut(&mut self) -> &mut bool {
        &mut self.descriptor_binding_acceleration_structure_update_after_bind
    }
    ///Sets the `acceleration_structure` field.
    pub fn set_acceleration_structure(&mut self, acceleration_structure: bool) -> &mut Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
    ///Sets the `acceleration_structure_capture_replay` field.
    pub fn set_acceleration_structure_capture_replay(
        &mut self,
        acceleration_structure_capture_replay: bool,
    ) -> &mut Self {
        self.acceleration_structure_capture_replay = acceleration_structure_capture_replay;
        self
    }
    ///Sets the `acceleration_structure_indirect_build` field.
    pub fn set_acceleration_structure_indirect_build(
        &mut self,
        acceleration_structure_indirect_build: bool,
    ) -> &mut Self {
        self.acceleration_structure_indirect_build = acceleration_structure_indirect_build;
        self
    }
    ///Sets the `acceleration_structure_host_commands` field.
    pub fn set_acceleration_structure_host_commands(
        &mut self,
        acceleration_structure_host_commands: bool,
    ) -> &mut Self {
        self.acceleration_structure_host_commands = acceleration_structure_host_commands;
        self
    }
    ///Sets the `descriptor_binding_acceleration_structure_update_after_bind` field.
    pub fn set_descriptor_binding_acceleration_structure_update_after_bind(
        &mut self,
        descriptor_binding_acceleration_structure_update_after_bind: bool,
    ) -> &mut Self {
        self.descriptor_binding_acceleration_structure_update_after_bind =
            descriptor_binding_acceleration_structure_update_after_bind;
        self
    }
    ///Sets the `acceleration_structure` field in a builder way.
    pub fn with_acceleration_structure(mut self, acceleration_structure: bool) -> Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
    ///Sets the `acceleration_structure_capture_replay` field in a builder way.
    pub fn with_acceleration_structure_capture_replay(mut self, acceleration_structure_capture_replay: bool) -> Self {
        self.acceleration_structure_capture_replay = acceleration_structure_capture_replay;
        self
    }
    ///Sets the `acceleration_structure_indirect_build` field in a builder way.
    pub fn with_acceleration_structure_indirect_build(mut self, acceleration_structure_indirect_build: bool) -> Self {
        self.acceleration_structure_indirect_build = acceleration_structure_indirect_build;
        self
    }
    ///Sets the `acceleration_structure_host_commands` field in a builder way.
    pub fn with_acceleration_structure_host_commands(mut self, acceleration_structure_host_commands: bool) -> Self {
        self.acceleration_structure_host_commands = acceleration_structure_host_commands;
        self
    }
    ///Sets the `descriptor_binding_acceleration_structure_update_after_bind` field in a builder
    /// way.
    pub fn with_descriptor_binding_acceleration_structure_update_after_bind(
        mut self,
        descriptor_binding_acceleration_structure_update_after_bind: bool,
    ) -> Self {
        self.descriptor_binding_acceleration_structure_update_after_bind =
            descriptor_binding_acceleration_structure_update_after_bind;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceAccelerationStructureFeaturesKHR {
    type LowLevel =
        crate::native::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructureFeaturesKHR {
            s_type: StructureType::PhysicalDeviceAccelerationStructureFeaturesKhr,
            p_next: std::ptr::null_mut(),
            acceleration_structure: self.acceleration_structure.into_low_level(context, bump),
            acceleration_structure_capture_replay: self
                .acceleration_structure_capture_replay
                .into_low_level(context, bump),
            acceleration_structure_indirect_build: self
                .acceleration_structure_indirect_build
                .into_low_level(context, bump),
            acceleration_structure_host_commands: self
                .acceleration_structure_host_commands
                .into_low_level(context, bump),
            descriptor_binding_acceleration_structure_update_after_bind: self
                .descriptor_binding_acceleration_structure_update_after_bind
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceAccelerationStructureFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            acceleration_structure: crate::conv::FromLowLevel::from_low_level(context, value.acceleration_structure),
            acceleration_structure_capture_replay: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_capture_replay,
            ),
            acceleration_structure_indirect_build: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_indirect_build,
            ),
            acceleration_structure_host_commands: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_host_commands,
            ),
            descriptor_binding_acceleration_structure_update_after_bind: crate::conv::FromLowLevel::from_low_level(
                context,
                value.descriptor_binding_acceleration_structure_update_after_bind,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    #[doc(alias = "maxGeometryCount")]
    pub max_geometry_count: u64,
    #[doc(alias = "maxInstanceCount")]
    pub max_instance_count: u64,
    #[doc(alias = "maxPrimitiveCount")]
    pub max_primitive_count: u64,
    #[doc(alias = "maxPerStageDescriptorAccelerationStructures")]
    pub max_per_stage_descriptor_acceleration_structures: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindAccelerationStructures")]
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    #[doc(alias = "maxDescriptorSetAccelerationStructures")]
    pub max_descriptor_set_acceleration_structures: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindAccelerationStructures")]
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    #[doc(alias = "minAccelerationStructureScratchOffsetAlignment")]
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}
impl PhysicalDeviceAccelerationStructurePropertiesKHR {
    ///Get a reference to the `max_geometry_count` field.
    pub fn max_geometry_count(&self) -> u64 {
        self.max_geometry_count
    }
    ///Get a reference to the `max_instance_count` field.
    pub fn max_instance_count(&self) -> u64 {
        self.max_instance_count
    }
    ///Get a reference to the `max_primitive_count` field.
    pub fn max_primitive_count(&self) -> u64 {
        self.max_primitive_count
    }
    ///Get a reference to the `max_per_stage_descriptor_acceleration_structures` field.
    pub fn max_per_stage_descriptor_acceleration_structures(&self) -> u32 {
        self.max_per_stage_descriptor_acceleration_structures
    }
    ///Get a reference to the `max_per_stage_descriptor_update_after_bind_acceleration_structures`
    /// field.
    pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_acceleration_structures
    }
    ///Get a reference to the `max_descriptor_set_acceleration_structures` field.
    pub fn max_descriptor_set_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_acceleration_structures
    }
    ///Get a reference to the `max_descriptor_set_update_after_bind_acceleration_structures` field.
    pub fn max_descriptor_set_update_after_bind_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_acceleration_structures
    }
    ///Get a reference to the `min_acceleration_structure_scratch_offset_alignment` field.
    pub fn min_acceleration_structure_scratch_offset_alignment(&self) -> u32 {
        self.min_acceleration_structure_scratch_offset_alignment
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceAccelerationStructurePropertiesKHR {
    type LowLevel =
        crate::native::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::PhysicalDeviceAccelerationStructurePropertiesKHR {
            s_type: StructureType::PhysicalDeviceAccelerationStructurePropertiesKhr,
            p_next: std::ptr::null_mut(),
            max_geometry_count: self.max_geometry_count.into_low_level(context, bump),
            max_instance_count: self.max_instance_count.into_low_level(context, bump),
            max_primitive_count: self.max_primitive_count.into_low_level(context, bump),
            max_per_stage_descriptor_acceleration_structures: self
                .max_per_stage_descriptor_acceleration_structures
                .into_low_level(context, bump),
            max_per_stage_descriptor_update_after_bind_acceleration_structures: self
                .max_per_stage_descriptor_update_after_bind_acceleration_structures
                .into_low_level(context, bump),
            max_descriptor_set_acceleration_structures: self
                .max_descriptor_set_acceleration_structures
                .into_low_level(context, bump),
            max_descriptor_set_update_after_bind_acceleration_structures: self
                .max_descriptor_set_update_after_bind_acceleration_structures
                .into_low_level(context, bump),
            min_acceleration_structure_scratch_offset_alignment: self
                .min_acceleration_structure_scratch_offset_alignment
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceAccelerationStructurePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_geometry_count: crate::conv::FromLowLevel::from_low_level(context, value.max_geometry_count),
            max_instance_count: crate::conv::FromLowLevel::from_low_level(context, value.max_instance_count),
            max_primitive_count: crate::conv::FromLowLevel::from_low_level(context, value.max_primitive_count),
            max_per_stage_descriptor_acceleration_structures: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_per_stage_descriptor_acceleration_structures,
            ),
            max_per_stage_descriptor_update_after_bind_acceleration_structures:
                crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.max_per_stage_descriptor_update_after_bind_acceleration_structures,
                ),
            max_descriptor_set_acceleration_structures: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_acceleration_structures,
            ),
            max_descriptor_set_update_after_bind_acceleration_structures: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_descriptor_set_update_after_bind_acceleration_structures,
            ),
            min_acceleration_structure_scratch_offset_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_acceleration_structure_scratch_offset_alignment,
            ),
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[AccelerationStructureGeometryTrianglesDataKHRExtension; 1]>,
    #[doc(alias = "vertexFormat")]
    pub vertex_format: Format,
    #[doc(alias = "vertexData")]
    pub vertex_data: DeviceOrHostAddressConstKHR,
    #[doc(alias = "vertexStride")]
    pub vertex_stride: DeviceSize,
    #[doc(alias = "maxVertex")]
    pub max_vertex: u32,
    #[doc(alias = "indexType")]
    pub index_type: IndexType,
    #[doc(alias = "indexData")]
    pub index_data: DeviceOrHostAddressConstKHR,
    #[doc(alias = "transformData")]
    pub transform_data: DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryTrianglesDataKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<AccelerationStructureGeometryTrianglesDataKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[AccelerationStructureGeometryTrianglesDataKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `vertex_format` field.
    pub fn vertex_format(&self) -> Format {
        self.vertex_format
    }
    ///Get a reference to the `vertex_data` field.
    pub fn vertex_data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.vertex_data
    }
    ///Get a reference to the `vertex_stride` field.
    pub fn vertex_stride(&self) -> &DeviceSize {
        &self.vertex_stride
    }
    ///Get a reference to the `max_vertex` field.
    pub fn max_vertex(&self) -> u32 {
        self.max_vertex
    }
    ///Get a reference to the `index_type` field.
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Get a reference to the `index_data` field.
    pub fn index_data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.index_data
    }
    ///Get a reference to the `transform_data` field.
    pub fn transform_data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.transform_data
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[AccelerationStructureGeometryTrianglesDataKHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `vertex_format` field.
    pub fn vertex_format_mut(&mut self) -> &mut Format {
        &mut self.vertex_format
    }
    ///Get a mutable reference to the `vertex_data` field.
    pub fn vertex_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.vertex_data
    }
    ///Get a mutable reference to the `vertex_stride` field.
    pub fn vertex_stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_stride
    }
    ///Get a mutable reference to the `max_vertex` field.
    pub fn max_vertex_mut(&mut self) -> &mut u32 {
        &mut self.max_vertex
    }
    ///Get a mutable reference to the `index_type` field.
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Get a mutable reference to the `index_data` field.
    pub fn index_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.index_data
    }
    ///Get a mutable reference to the `transform_data` field.
    pub fn transform_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.transform_data
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(
        &mut self,
        extensions: SmallVec<[AccelerationStructureGeometryTrianglesDataKHRExtension; 1]>,
    ) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `vertex_format` field.
    pub fn set_vertex_format(&mut self, vertex_format: Format) -> &mut Self {
        self.vertex_format = vertex_format;
        self
    }
    ///Sets the `vertex_data` field.
    pub fn set_vertex_data(&mut self, vertex_data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.vertex_data = vertex_data;
        self
    }
    ///Sets the `vertex_stride` field.
    pub fn set_vertex_stride(&mut self, vertex_stride: DeviceSize) -> &mut Self {
        self.vertex_stride = vertex_stride;
        self
    }
    ///Sets the `max_vertex` field.
    pub fn set_max_vertex(&mut self, max_vertex: u32) -> &mut Self {
        self.max_vertex = max_vertex;
        self
    }
    ///Sets the `index_type` field.
    pub fn set_index_type(&mut self, index_type: IndexType) -> &mut Self {
        self.index_type = index_type;
        self
    }
    ///Sets the `index_data` field.
    pub fn set_index_data(&mut self, index_data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.index_data = index_data;
        self
    }
    ///Sets the `transform_data` field.
    pub fn set_transform_data(&mut self, transform_data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.transform_data = transform_data;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(
        mut self,
        extensions: SmallVec<[AccelerationStructureGeometryTrianglesDataKHRExtension; 1]>,
    ) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `vertex_format` field in a builder way.
    pub fn with_vertex_format(mut self, vertex_format: Format) -> Self {
        self.vertex_format = vertex_format;
        self
    }
    ///Sets the `vertex_data` field in a builder way.
    pub fn with_vertex_data(mut self, vertex_data: DeviceOrHostAddressConstKHR) -> Self {
        self.vertex_data = vertex_data;
        self
    }
    ///Sets the `vertex_stride` field in a builder way.
    pub fn with_vertex_stride(mut self, vertex_stride: DeviceSize) -> Self {
        self.vertex_stride = vertex_stride;
        self
    }
    ///Sets the `max_vertex` field in a builder way.
    pub fn with_max_vertex(mut self, max_vertex: u32) -> Self {
        self.max_vertex = max_vertex;
        self
    }
    ///Sets the `index_type` field in a builder way.
    pub fn with_index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }
    ///Sets the `index_data` field in a builder way.
    pub fn with_index_data(mut self, index_data: DeviceOrHostAddressConstKHR) -> Self {
        self.index_data = index_data;
        self
    }
    ///Sets the `transform_data` field in a builder way.
    pub fn with_transform_data(mut self, transform_data: DeviceOrHostAddressConstKHR) -> Self {
        self.transform_data = transform_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryTrianglesDataKHR {
    type LowLevel =
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryTrianglesDataKHR {
            s_type: StructureType::AccelerationStructureGeometryTrianglesDataKhr,
            p_next: next,
            vertex_format: self.vertex_format.into_low_level(context, bump),
            vertex_data: self.vertex_data.into_low_level(context, bump),
            vertex_stride: self.vertex_stride.into_low_level(context, bump),
            max_vertex: self.max_vertex.into_low_level(context, bump),
            index_type: self.index_type.into_low_level(context, bump),
            index_data: self.index_data.into_low_level(context, bump),
            transform_data: self.transform_data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryTrianglesDataKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            vertex_format: crate::conv::FromLowLevel::from_low_level(context, value.vertex_format),
            vertex_data: crate::conv::FromLowLevel::from_low_level(context, value.vertex_data),
            vertex_stride: crate::conv::FromLowLevel::from_low_level(context, value.vertex_stride),
            max_vertex: crate::conv::FromLowLevel::from_low_level(context, value.max_vertex),
            index_type: crate::conv::FromLowLevel::from_low_level(context, value.index_type),
            index_data: crate::conv::FromLowLevel::from_low_level(context, value.index_data),
            transform_data: crate::conv::FromLowLevel::from_low_level(context, value.transform_data),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`AccelerationStructureGeometryTrianglesDataKHR`]
pub enum AccelerationStructureGeometryTrianglesDataKHRExtension {
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    ///Contains a type [`AccelerationStructureGeometryMotionTrianglesDataNV`] for extending
    /// [`AccelerationStructureGeometryTrianglesDataKHR`]
    AccelerationStructureGeometryMotionTrianglesDataNV(AccelerationStructureGeometryMotionTrianglesDataNV),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryTrianglesDataKHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { # [cfg (feature = "VK_NV_ray_tracing_motion_blur")] Self :: AccelerationStructureGeometryMotionTrianglesDataNV (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: nv_ray_tracing_motion_blur :: AccelerationStructureGeometryMotionTrianglesDataNV) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryTrianglesDataKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_NV_ray_tracing_motion_blur")] crate :: native :: vulkan1_0 :: StructureType :: AccelerationStructureGeometryMotionTrianglesDataNv => Self :: AccelerationStructureGeometryMotionTrianglesDataNV (AccelerationStructureGeometryMotionTrianglesDataNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_ray_tracing_motion_blur :: AccelerationStructureGeometryMotionTrianglesDataNV > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (AccelerationStructureGeometryTrianglesDataKHR)) }
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl From<AccelerationStructureGeometryMotionTrianglesDataNV>
    for AccelerationStructureGeometryTrianglesDataKHRExtension
{
    fn from(ext: AccelerationStructureGeometryMotionTrianglesDataNV) -> Self {
        Self::AccelerationStructureGeometryMotionTrianglesDataNV(ext)
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl TryInto<AccelerationStructureGeometryMotionTrianglesDataNV>
    for AccelerationStructureGeometryTrianglesDataKHRExtension
{
    type Error = AccelerationStructureGeometryTrianglesDataKHRExtension;
    fn try_into(self) -> Result<AccelerationStructureGeometryMotionTrianglesDataNV, Self::Error> {
        match self {
            Self::AccelerationStructureGeometryMotionTrianglesDataNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: DeviceSize,
}
impl AccelerationStructureGeometryAabbsDataKHR {
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.data
    }
    ///Get a reference to the `stride` field.
    pub fn stride(&self) -> &DeviceSize {
        &self.stride
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.data
    }
    ///Get a mutable reference to the `stride` field.
    pub fn stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.stride
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `stride` field.
    pub fn set_stride(&mut self, stride: DeviceSize) -> &mut Self {
        self.stride = stride;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: DeviceOrHostAddressConstKHR) -> Self {
        self.data = data;
        self
    }
    ///Sets the `stride` field in a builder way.
    pub fn with_stride(mut self, stride: DeviceSize) -> Self {
        self.stride = stride;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryAabbsDataKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryAabbsDataKHR {
            s_type: StructureType::AccelerationStructureGeometryAabbsDataKhr,
            p_next: std::ptr::null(),
            data: self.data.into_low_level(context, bump),
            stride: self.stride.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryAabbsDataKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            data: crate::conv::FromLowLevel::from_low_level(context, value.data),
            stride: crate::conv::FromLowLevel::from_low_level(context, value.stride),
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    #[doc(alias = "arrayOfPointers")]
    pub array_of_pointers: bool,
    pub data: DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryInstancesDataKHR {
    ///Get a reference to the `array_of_pointers` field.
    pub fn array_of_pointers(&self) -> &bool {
        &self.array_of_pointers
    }
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.data
    }
    ///Get a mutable reference to the `array_of_pointers` field.
    pub fn array_of_pointers_mut(&mut self) -> &mut bool {
        &mut self.array_of_pointers
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.data
    }
    ///Sets the `array_of_pointers` field.
    pub fn set_array_of_pointers(&mut self, array_of_pointers: bool) -> &mut Self {
        self.array_of_pointers = array_of_pointers;
        self
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `array_of_pointers` field in a builder way.
    pub fn with_array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.array_of_pointers = array_of_pointers;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: DeviceOrHostAddressConstKHR) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryInstancesDataKHR {
    type LowLevel =
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryInstancesDataKHR {
            s_type: StructureType::AccelerationStructureGeometryInstancesDataKhr,
            p_next: std::ptr::null(),
            array_of_pointers: self.array_of_pointers.into_low_level(context, bump),
            data: self.data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryInstancesDataKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            array_of_pointers: crate::conv::FromLowLevel::from_low_level(context, value.array_of_pointers),
            data: crate::conv::FromLowLevel::from_low_level(context, value.data),
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureGeometryKHR {
    #[doc(alias = "geometryType")]
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}
impl AccelerationStructureGeometryKHR {
    ///Get a reference to the `geometry_type` field.
    pub fn geometry_type(&self) -> GeometryTypeKHR {
        self.geometry_type
    }
    ///Get a reference to the `geometry` field.
    pub fn geometry(&self) -> &AccelerationStructureGeometryDataKHR {
        &self.geometry
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> GeometryFlagsKHR {
        self.flags
    }
    ///Get a mutable reference to the `geometry_type` field.
    pub fn geometry_type_mut(&mut self) -> &mut GeometryTypeKHR {
        &mut self.geometry_type
    }
    ///Get a mutable reference to the `geometry` field.
    pub fn geometry_mut(&mut self) -> &mut AccelerationStructureGeometryDataKHR {
        &mut self.geometry
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut GeometryFlagsKHR {
        &mut self.flags
    }
    ///Sets the `geometry_type` field.
    pub fn set_geometry_type(&mut self, geometry_type: GeometryTypeKHR) -> &mut Self {
        self.geometry_type = geometry_type;
        self
    }
    ///Sets the `geometry` field.
    pub fn set_geometry(&mut self, geometry: AccelerationStructureGeometryDataKHR) -> &mut Self {
        self.geometry = geometry;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: GeometryFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `geometry_type` field in a builder way.
    pub fn with_geometry_type(mut self, geometry_type: GeometryTypeKHR) -> Self {
        self.geometry_type = geometry_type;
        self
    }
    ///Sets the `geometry` field in a builder way.
    pub fn with_geometry(mut self, geometry: AccelerationStructureGeometryDataKHR) -> Self {
        self.geometry = geometry;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: GeometryFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR {
            s_type: StructureType::AccelerationStructureGeometryKhr,
            p_next: std::ptr::null(),
            geometry_type: self.geometry_type.into_low_level(context, bump),
            geometry: self.geometry.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            geometry_type: crate::conv::FromLowLevel::from_low_level(context, value.geometry_type),
            geometry: crate::conv::FromLowLevel::from_low_level(context, value.geometry),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    #[doc(alias = "type")]
    pub type_: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub mode: BuildAccelerationStructureModeKHR,
    #[doc(alias = "srcAccelerationStructure")]
    pub src_acceleration_structure: Option<AccelerationStructureKHR>,
    #[doc(alias = "dstAccelerationStructure")]
    pub dst_acceleration_structure: Option<AccelerationStructureKHR>,
    #[doc(alias = "pGeometries")]
    pub geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>,
    #[doc(alias = "ppGeometries")]
    pub pp_geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>,
    #[doc(alias = "scratchData")]
    pub scratch_data: DeviceOrHostAddressKHR,
}
impl AccelerationStructureBuildGeometryInfoKHR {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> AccelerationStructureTypeKHR {
        self.type_
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> BuildAccelerationStructureFlagsKHR {
        self.flags
    }
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> BuildAccelerationStructureModeKHR {
        self.mode
    }
    ///Get a reference to the `src_acceleration_structure` field.
    pub fn src_acceleration_structure(&self) -> &Option<AccelerationStructureKHR> {
        &self.src_acceleration_structure
    }
    ///Get a reference to the `dst_acceleration_structure` field.
    pub fn dst_acceleration_structure(&self) -> &Option<AccelerationStructureKHR> {
        &self.dst_acceleration_structure
    }
    ///Get a reference to the `geometries` field.
    pub fn geometries(&self) -> &SmallVec<[AccelerationStructureGeometryKHR; 8]> {
        &self.geometries
    }
    ///Get a reference to the `pp_geometries` field.
    pub fn pp_geometries(&self) -> &SmallVec<[AccelerationStructureGeometryKHR; 8]> {
        &self.pp_geometries
    }
    ///Get a reference to the `scratch_data` field.
    pub fn scratch_data(&self) -> &DeviceOrHostAddressKHR {
        &self.scratch_data
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut AccelerationStructureTypeKHR {
        &mut self.type_
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut BuildAccelerationStructureFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut BuildAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Get a mutable reference to the `src_acceleration_structure` field.
    pub fn src_acceleration_structure_mut(&mut self) -> &mut Option<AccelerationStructureKHR> {
        &mut self.src_acceleration_structure
    }
    ///Get a mutable reference to the `dst_acceleration_structure` field.
    pub fn dst_acceleration_structure_mut(&mut self) -> &mut Option<AccelerationStructureKHR> {
        &mut self.dst_acceleration_structure
    }
    ///Get a mutable reference to the `geometries` field.
    pub fn geometries_mut(&mut self) -> &mut SmallVec<[AccelerationStructureGeometryKHR; 8]> {
        &mut self.geometries
    }
    ///Get a mutable reference to the `pp_geometries` field.
    pub fn pp_geometries_mut(&mut self) -> &mut SmallVec<[AccelerationStructureGeometryKHR; 8]> {
        &mut self.pp_geometries
    }
    ///Get a mutable reference to the `scratch_data` field.
    pub fn scratch_data_mut(&mut self) -> &mut DeviceOrHostAddressKHR {
        &mut self.scratch_data
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: AccelerationStructureTypeKHR) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: BuildAccelerationStructureFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: BuildAccelerationStructureModeKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `src_acceleration_structure` field.
    pub fn set_src_acceleration_structure(
        &mut self,
        src_acceleration_structure: Option<AccelerationStructureKHR>,
    ) -> &mut Self {
        self.src_acceleration_structure = src_acceleration_structure;
        self
    }
    ///Sets the `dst_acceleration_structure` field.
    pub fn set_dst_acceleration_structure(
        &mut self,
        dst_acceleration_structure: Option<AccelerationStructureKHR>,
    ) -> &mut Self {
        self.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    ///Sets the `geometries` field.
    pub fn set_geometries(&mut self, geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>) -> &mut Self {
        self.geometries = geometries;
        self
    }
    ///Sets the `pp_geometries` field.
    pub fn set_pp_geometries(&mut self, pp_geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>) -> &mut Self {
        self.pp_geometries = pp_geometries;
        self
    }
    ///Sets the `scratch_data` field.
    pub fn set_scratch_data(&mut self, scratch_data: DeviceOrHostAddressKHR) -> &mut Self {
        self.scratch_data = scratch_data;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: AccelerationStructureTypeKHR) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: BuildAccelerationStructureFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: BuildAccelerationStructureModeKHR) -> Self {
        self.mode = mode;
        self
    }
    ///Sets the `src_acceleration_structure` field in a builder way.
    pub fn with_src_acceleration_structure(
        mut self,
        src_acceleration_structure: Option<AccelerationStructureKHR>,
    ) -> Self {
        self.src_acceleration_structure = src_acceleration_structure;
        self
    }
    ///Sets the `dst_acceleration_structure` field in a builder way.
    pub fn with_dst_acceleration_structure(
        mut self,
        dst_acceleration_structure: Option<AccelerationStructureKHR>,
    ) -> Self {
        self.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    ///Sets the `geometries` field in a builder way.
    pub fn with_geometries(mut self, geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>) -> Self {
        self.geometries = geometries;
        self
    }
    ///Sets the `pp_geometries` field in a builder way.
    pub fn with_pp_geometries(mut self, pp_geometries: SmallVec<[AccelerationStructureGeometryKHR; 8]>) -> Self {
        self.pp_geometries = pp_geometries;
        self
    }
    ///Sets the `scratch_data` field in a builder way.
    pub fn with_scratch_data(mut self, scratch_data: DeviceOrHostAddressKHR) -> Self {
        self.scratch_data = scratch_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureBuildGeometryInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_geometries = self.geometries.len() as u32;
        let geometries = bump
            .alloc_slice_fill_iter(self.geometries.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let pp_geometries = bump
            .alloc_slice_fill_iter(self.pp_geometries.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildGeometryInfoKHR {
            s_type: StructureType::AccelerationStructureBuildGeometryInfoKhr,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            mode: self.mode.into_low_level(context, bump),
            src_acceleration_structure: self
                .src_acceleration_structure
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            dst_acceleration_structure: self
                .dst_acceleration_structure
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            geometry_count: len_geometries,
            geometries: geometries,
            pp_geometries: pp_geometries,
            scratch_data: self.scratch_data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureBuildGeometryInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let geometries_len = value.geometry_count;
        let mut geometries = SmallVec::with_capacity(geometries_len as usize);
        for i in 0..geometries_len {
            geometries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.geometries.add(i as usize).read(),
            ));
        }
        let pp_geometries_len = value.geometry_count;
        let mut pp_geometries = SmallVec::with_capacity(pp_geometries_len as usize);
        for i in 0..pp_geometries_len {
            pp_geometries.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.pp_geometries.add(i as usize).read(),
            ));
        }
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
            src_acceleration_structure: if value.src_acceleration_structure
                == crate::native::extensions::khr_acceleration_structure::AccelerationStructureKHR::null()
            {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.src_acceleration_structure,
                ))
            },
            dst_acceleration_structure: if value.dst_acceleration_structure
                == crate::native::extensions::khr_acceleration_structure::AccelerationStructureKHR::null()
            {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(
                    context,
                    value.dst_acceleration_structure,
                ))
            },
            geometries: geometries,
            pp_geometries: pp_geometries,
            scratch_data: crate::conv::FromLowLevel::from_low_level(context, value.scratch_data),
        }
    }
}
impl AccelerationStructureBuildRangeInfoKHR {
    ///Get a reference to the `primitive_count` field.
    pub fn primitive_count(&self) -> u32 {
        self.primitive_count
    }
    ///Get a reference to the `primitive_offset` field.
    pub fn primitive_offset(&self) -> u32 {
        self.primitive_offset
    }
    ///Get a reference to the `first_vertex` field.
    pub fn first_vertex(&self) -> u32 {
        self.first_vertex
    }
    ///Get a reference to the `transform_offset` field.
    pub fn transform_offset(&self) -> u32 {
        self.transform_offset
    }
    ///Get a mutable reference to the `primitive_count` field.
    pub fn primitive_count_mut(&mut self) -> &mut u32 {
        &mut self.primitive_count
    }
    ///Get a mutable reference to the `primitive_offset` field.
    pub fn primitive_offset_mut(&mut self) -> &mut u32 {
        &mut self.primitive_offset
    }
    ///Get a mutable reference to the `first_vertex` field.
    pub fn first_vertex_mut(&mut self) -> &mut u32 {
        &mut self.first_vertex
    }
    ///Get a mutable reference to the `transform_offset` field.
    pub fn transform_offset_mut(&mut self) -> &mut u32 {
        &mut self.transform_offset
    }
    ///Sets the `primitive_count` field.
    pub fn set_primitive_count(&mut self, primitive_count: u32) -> &mut Self {
        self.primitive_count = primitive_count;
        self
    }
    ///Sets the `primitive_offset` field.
    pub fn set_primitive_offset(&mut self, primitive_offset: u32) -> &mut Self {
        self.primitive_offset = primitive_offset;
        self
    }
    ///Sets the `first_vertex` field.
    pub fn set_first_vertex(&mut self, first_vertex: u32) -> &mut Self {
        self.first_vertex = first_vertex;
        self
    }
    ///Sets the `transform_offset` field.
    pub fn set_transform_offset(&mut self, transform_offset: u32) -> &mut Self {
        self.transform_offset = transform_offset;
        self
    }
    ///Sets the `primitive_count` field in a builder way.
    pub fn with_primitive_count(mut self, primitive_count: u32) -> Self {
        self.primitive_count = primitive_count;
        self
    }
    ///Sets the `primitive_offset` field in a builder way.
    pub fn with_primitive_offset(mut self, primitive_offset: u32) -> Self {
        self.primitive_offset = primitive_offset;
        self
    }
    ///Sets the `first_vertex` field in a builder way.
    pub fn with_first_vertex(mut self, first_vertex: u32) -> Self {
        self.first_vertex = first_vertex;
        self
    }
    ///Sets the `transform_offset` field in a builder way.
    pub fn with_transform_offset(mut self, transform_offset: u32) -> Self {
        self.transform_offset = transform_offset;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureBuildRangeInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildRangeInfoKHR {
            primitive_count: self.primitive_count.into_low_level(context, bump),
            primitive_offset: self.primitive_offset.into_low_level(context, bump),
            first_vertex: self.first_vertex.into_low_level(context, bump),
            transform_offset: self.transform_offset.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureBuildRangeInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            primitive_count: crate::conv::FromLowLevel::from_low_level(context, value.primitive_count),
            primitive_offset: crate::conv::FromLowLevel::from_low_level(context, value.primitive_offset),
            first_vertex: crate::conv::FromLowLevel::from_low_level(context, value.first_vertex),
            transform_offset: crate::conv::FromLowLevel::from_low_level(context, value.transform_offset),
        }
    }
}
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureCreateInfoKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[AccelerationStructureCreateInfoKHRExtension; 1]>,
    #[doc(alias = "createFlags")]
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    #[doc(alias = "type")]
    pub type_: AccelerationStructureTypeKHR,
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
}
impl AccelerationStructureCreateInfoKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<AccelerationStructureCreateInfoKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[AccelerationStructureCreateInfoKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `create_flags` field.
    pub fn create_flags(&self) -> AccelerationStructureCreateFlagsKHR {
        self.create_flags
    }
    ///Get a reference to the `buffer` field.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    ///Get a reference to the `offset` field.
    pub fn offset(&self) -> &DeviceSize {
        &self.offset
    }
    ///Get a reference to the `size` field.
    pub fn size(&self) -> &DeviceSize {
        &self.size
    }
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> AccelerationStructureTypeKHR {
        self.type_
    }
    ///Get a reference to the `device_address` field.
    pub fn device_address(&self) -> &DeviceAddress {
        &self.device_address
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[AccelerationStructureCreateInfoKHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `create_flags` field.
    pub fn create_flags_mut(&mut self) -> &mut AccelerationStructureCreateFlagsKHR {
        &mut self.create_flags
    }
    ///Get a mutable reference to the `buffer` field.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Get a mutable reference to the `offset` field.
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Get a mutable reference to the `size` field.
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut AccelerationStructureTypeKHR {
        &mut self.type_
    }
    ///Get a mutable reference to the `device_address` field.
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(
        &mut self,
        extensions: SmallVec<[AccelerationStructureCreateInfoKHRExtension; 1]>,
    ) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `create_flags` field.
    pub fn set_create_flags(&mut self, create_flags: AccelerationStructureCreateFlagsKHR) -> &mut Self {
        self.create_flags = create_flags;
        self
    }
    ///Sets the `buffer` field.
    pub fn set_buffer(&mut self, buffer: Buffer) -> &mut Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field.
    pub fn set_offset(&mut self, offset: DeviceSize) -> &mut Self {
        self.offset = offset;
        self
    }
    ///Sets the `size` field.
    pub fn set_size(&mut self, size: DeviceSize) -> &mut Self {
        self.size = size;
        self
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: AccelerationStructureTypeKHR) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `device_address` field.
    pub fn set_device_address(&mut self, device_address: DeviceAddress) -> &mut Self {
        self.device_address = device_address;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[AccelerationStructureCreateInfoKHRExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `create_flags` field in a builder way.
    pub fn with_create_flags(mut self, create_flags: AccelerationStructureCreateFlagsKHR) -> Self {
        self.create_flags = create_flags;
        self
    }
    ///Sets the `buffer` field in a builder way.
    pub fn with_buffer(mut self, buffer: Buffer) -> Self {
        self.buffer = buffer;
        self
    }
    ///Sets the `offset` field in a builder way.
    pub fn with_offset(mut self, offset: DeviceSize) -> Self {
        self.offset = offset;
        self
    }
    ///Sets the `size` field in a builder way.
    pub fn with_size(mut self, size: DeviceSize) -> Self {
        self.size = size;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: AccelerationStructureTypeKHR) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `device_address` field in a builder way.
    pub fn with_device_address(mut self, device_address: DeviceAddress) -> Self {
        self.device_address = device_address;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureCreateInfoKHR {
            s_type: StructureType::AccelerationStructureCreateInfoKhr,
            p_next: next,
            create_flags: self.create_flags.into_low_level(context, bump),
            buffer: self.buffer.into_low_level(context, bump),
            offset: self.offset.into_low_level(context, bump),
            size: self.size.into_low_level(context, bump),
            type_: self.type_.into_low_level(context, bump),
            device_address: self.device_address.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut next = value.p_next;
        let mut extensions = SmallVec::new();
        while !next.is_null() {
            extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
            next = std::ptr::read(next).next;
        }
        Self {
            extensions: extensions,
            create_flags: crate::conv::FromLowLevel::from_low_level(context, value.create_flags),
            buffer: crate::conv::FromLowLevel::from_low_level(context, value.buffer),
            offset: crate::conv::FromLowLevel::from_low_level(context, value.offset),
            size: crate::conv::FromLowLevel::from_low_level(context, value.size),
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            device_address: crate::conv::FromLowLevel::from_low_level(context, value.device_address),
        }
    }
}
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`AccelerationStructureCreateInfoKHR`]
pub enum AccelerationStructureCreateInfoKHRExtension {
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    ///Contains a type [`AccelerationStructureMotionInfoNV`] for extending
    /// [`AccelerationStructureCreateInfoKHR`]
    AccelerationStructureMotionInfoNV(AccelerationStructureMotionInfoNV),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCreateInfoKHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            Self::AccelerationStructureMotionInfoNV(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCreateInfoKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_NV_ray_tracing_motion_blur")] crate :: native :: vulkan1_0 :: StructureType :: AccelerationStructureMotionInfoNv => Self :: AccelerationStructureMotionInfoNV (AccelerationStructureMotionInfoNV :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: nv_ray_tracing_motion_blur :: AccelerationStructureMotionInfoNV > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (AccelerationStructureCreateInfoKHR)) }
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl From<AccelerationStructureMotionInfoNV> for AccelerationStructureCreateInfoKHRExtension {
    fn from(ext: AccelerationStructureMotionInfoNV) -> Self {
        Self::AccelerationStructureMotionInfoNV(ext)
    }
}
#[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
impl TryInto<AccelerationStructureMotionInfoNV> for AccelerationStructureCreateInfoKHRExtension {
    type Error = AccelerationStructureCreateInfoKHRExtension;
    fn try_into(self) -> Result<AccelerationStructureMotionInfoNV, Self::Error> {
        match self {
            Self::AccelerationStructureMotionInfoNV(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl AabbPositionsKHR {
    ///Get a reference to the `min_x` field.
    pub fn min_x(&self) -> f32 {
        self.min_x
    }
    ///Get a reference to the `min_y` field.
    pub fn min_y(&self) -> f32 {
        self.min_y
    }
    ///Get a reference to the `min_z` field.
    pub fn min_z(&self) -> f32 {
        self.min_z
    }
    ///Get a reference to the `max_x` field.
    pub fn max_x(&self) -> f32 {
        self.max_x
    }
    ///Get a reference to the `max_y` field.
    pub fn max_y(&self) -> f32 {
        self.max_y
    }
    ///Get a reference to the `max_z` field.
    pub fn max_z(&self) -> f32 {
        self.max_z
    }
    ///Get a mutable reference to the `min_x` field.
    pub fn min_x_mut(&mut self) -> &mut f32 {
        &mut self.min_x
    }
    ///Get a mutable reference to the `min_y` field.
    pub fn min_y_mut(&mut self) -> &mut f32 {
        &mut self.min_y
    }
    ///Get a mutable reference to the `min_z` field.
    pub fn min_z_mut(&mut self) -> &mut f32 {
        &mut self.min_z
    }
    ///Get a mutable reference to the `max_x` field.
    pub fn max_x_mut(&mut self) -> &mut f32 {
        &mut self.max_x
    }
    ///Get a mutable reference to the `max_y` field.
    pub fn max_y_mut(&mut self) -> &mut f32 {
        &mut self.max_y
    }
    ///Get a mutable reference to the `max_z` field.
    pub fn max_z_mut(&mut self) -> &mut f32 {
        &mut self.max_z
    }
    ///Sets the `min_x` field.
    pub fn set_min_x(&mut self, min_x: f32) -> &mut Self {
        self.min_x = min_x;
        self
    }
    ///Sets the `min_y` field.
    pub fn set_min_y(&mut self, min_y: f32) -> &mut Self {
        self.min_y = min_y;
        self
    }
    ///Sets the `min_z` field.
    pub fn set_min_z(&mut self, min_z: f32) -> &mut Self {
        self.min_z = min_z;
        self
    }
    ///Sets the `max_x` field.
    pub fn set_max_x(&mut self, max_x: f32) -> &mut Self {
        self.max_x = max_x;
        self
    }
    ///Sets the `max_y` field.
    pub fn set_max_y(&mut self, max_y: f32) -> &mut Self {
        self.max_y = max_y;
        self
    }
    ///Sets the `max_z` field.
    pub fn set_max_z(&mut self, max_z: f32) -> &mut Self {
        self.max_z = max_z;
        self
    }
    ///Sets the `min_x` field in a builder way.
    pub fn with_min_x(mut self, min_x: f32) -> Self {
        self.min_x = min_x;
        self
    }
    ///Sets the `min_y` field in a builder way.
    pub fn with_min_y(mut self, min_y: f32) -> Self {
        self.min_y = min_y;
        self
    }
    ///Sets the `min_z` field in a builder way.
    pub fn with_min_z(mut self, min_z: f32) -> Self {
        self.min_z = min_z;
        self
    }
    ///Sets the `max_x` field in a builder way.
    pub fn with_max_x(mut self, max_x: f32) -> Self {
        self.max_x = max_x;
        self
    }
    ///Sets the `max_y` field in a builder way.
    pub fn with_max_y(mut self, max_y: f32) -> Self {
        self.max_y = max_y;
        self
    }
    ///Sets the `max_z` field in a builder way.
    pub fn with_max_z(mut self, max_z: f32) -> Self {
        self.max_z = max_z;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AabbPositionsKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AabbPositionsKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AabbPositionsKHR {
            min_x: self.min_x.into_low_level(context, bump),
            min_y: self.min_y.into_low_level(context, bump),
            min_z: self.min_z.into_low_level(context, bump),
            max_x: self.max_x.into_low_level(context, bump),
            max_y: self.max_y.into_low_level(context, bump),
            max_z: self.max_z.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AabbPositionsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_x: crate::conv::FromLowLevel::from_low_level(context, value.min_x),
            min_y: crate::conv::FromLowLevel::from_low_level(context, value.min_y),
            min_z: crate::conv::FromLowLevel::from_low_level(context, value.min_z),
            max_x: crate::conv::FromLowLevel::from_low_level(context, value.max_x),
            max_y: crate::conv::FromLowLevel::from_low_level(context, value.max_y),
            max_z: crate::conv::FromLowLevel::from_low_level(context, value.max_z),
        }
    }
}
impl TransformMatrixKHR {
    ///Get a reference to the `matrix` field.
    pub fn matrix(&self) -> [f32; 3 as usize] {
        self.matrix
    }
    ///Get a mutable reference to the `matrix` field.
    pub fn matrix_mut(&mut self) -> &mut [f32; 3 as usize] {
        &mut self.matrix
    }
    ///Sets the `matrix` field.
    pub fn set_matrix(&mut self, matrix: [f32; 3 as usize]) -> &mut Self {
        self.matrix = matrix;
        self
    }
    ///Sets the `matrix` field in a builder way.
    pub fn with_matrix(mut self, matrix: [f32; 3 as usize]) -> Self {
        self.matrix = matrix;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for TransformMatrixKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::TransformMatrixKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::TransformMatrixKHR {
            matrix: self.matrix.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for TransformMatrixKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            matrix: crate::conv::FromLowLevel::from_low_level(context, value.matrix),
        }
    }
}
impl AccelerationStructureInstanceKHR {
    ///Get a reference to the `transform` field.
    pub fn transform(&self) -> TransformMatrixKHR {
        self.transform
    }
    ///Get a reference to the `instance_custom_index` field.
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Get a reference to the `mask` field.
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Get a reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Get a reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Get a mutable reference to the `transform` field.
    pub fn transform_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform
    }
    ///Get a mutable reference to the `instance_custom_index` field.
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Get a mutable reference to the `mask` field.
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Get a mutable reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the `transform` field.
    pub fn set_transform(&mut self, transform: TransformMatrixKHR) -> &mut Self {
        self.transform = transform;
        self
    }
    ///Sets the `instance_custom_index` field.
    pub fn set_instance_custom_index(&mut self, instance_custom_index: u32) -> &mut Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field.
    pub fn set_mask(&mut self, mask: u32) -> &mut Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field.
    pub fn set_instance_shader_binding_table_record_offset(
        &mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> &mut Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: GeometryInstanceFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field.
    pub fn set_acceleration_structure_reference(&mut self, acceleration_structure_reference: u64) -> &mut Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
    ///Sets the `transform` field in a builder way.
    pub fn with_transform(mut self, transform: TransformMatrixKHR) -> Self {
        self.transform = transform;
        self
    }
    ///Sets the `instance_custom_index` field in a builder way.
    pub fn with_instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field in a builder way.
    pub fn with_mask(mut self, mask: u32) -> Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field in a builder way.
    pub fn with_instance_shader_binding_table_record_offset(
        mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field in a builder way.
    pub fn with_acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureInstanceKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureInstanceKHR {
            transform: self.transform.into_low_level(context, bump),
            instance_custom_index: self.instance_custom_index.into_low_level(context, bump),
            mask: self.mask.into_low_level(context, bump),
            instance_shader_binding_table_record_offset: self
                .instance_shader_binding_table_record_offset
                .into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            acceleration_structure_reference: self.acceleration_structure_reference.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureInstanceKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            transform: crate::conv::FromLowLevel::from_low_level(context, value.transform),
            instance_custom_index: crate::conv::FromLowLevel::from_low_level(context, value.instance_custom_index),
            mask: crate::conv::FromLowLevel::from_low_level(context, value.mask),
            instance_shader_binding_table_record_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.instance_shader_binding_table_record_offset,
            ),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            acceleration_structure_reference: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_reference,
            ),
        }
    }
}
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureKHR,
}
impl AccelerationStructureDeviceAddressInfoKHR {
    ///Get a reference to the `acceleration_structure` field.
    pub fn acceleration_structure(&self) -> &AccelerationStructureKHR {
        &self.acceleration_structure
    }
    ///Get a mutable reference to the `acceleration_structure` field.
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.acceleration_structure
    }
    ///Sets the `acceleration_structure` field.
    pub fn set_acceleration_structure(&mut self, acceleration_structure: AccelerationStructureKHR) -> &mut Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
    ///Sets the `acceleration_structure` field in a builder way.
    pub fn with_acceleration_structure(mut self, acceleration_structure: AccelerationStructureKHR) -> Self {
        self.acceleration_structure = acceleration_structure;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureDeviceAddressInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureDeviceAddressInfoKHR {
            s_type: StructureType::AccelerationStructureDeviceAddressInfoKhr,
            p_next: std::ptr::null(),
            acceleration_structure: self.acceleration_structure.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureDeviceAddressInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            acceleration_structure: crate::conv::FromLowLevel::from_low_level(context, value.acceleration_structure),
        }
    }
}
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureVersionInfoKHR {
    #[doc(alias = "pVersionData")]
    pub version_data: SmallVec<[u8; 8]>,
}
impl AccelerationStructureVersionInfoKHR {
    ///Get a reference to the `version_data` field.
    pub fn version_data(&self) -> &SmallVec<[u8; 8]> {
        &self.version_data
    }
    ///Get a mutable reference to the `version_data` field.
    pub fn version_data_mut(&mut self) -> &mut SmallVec<[u8; 8]> {
        &mut self.version_data
    }
    ///Sets the `version_data` field.
    pub fn set_version_data(&mut self, version_data: SmallVec<[u8; 8]>) -> &mut Self {
        self.version_data = version_data;
        self
    }
    ///Sets the `version_data` field in a builder way.
    pub fn with_version_data(mut self, version_data: SmallVec<[u8; 8]>) -> Self {
        self.version_data = version_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureVersionInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let version_data = bump
            .alloc_slice_fill_iter(self.version_data.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureVersionInfoKHR {
            s_type: StructureType::AccelerationStructureVersionInfoKhr,
            p_next: std::ptr::null(),
            version_data: version_data,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureVersionInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let mut version_data = SmallVec::with_capacity(2 * UUID_SIZE as usize);
        for i in 0..2 * UUID_SIZE {
            version_data.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.version_data.add(i as usize).read(),
            ));
        }
        Self {
            version_data: version_data,
        }
    }
}
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyAccelerationStructureInfoKHR {
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureInfoKHR {
    ///Get a reference to the `src` field.
    pub fn src(&self) -> &AccelerationStructureKHR {
        &self.src
    }
    ///Get a reference to the `dst` field.
    pub fn dst(&self) -> &AccelerationStructureKHR {
        &self.dst
    }
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Get a mutable reference to the `src` field.
    pub fn src_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.src
    }
    ///Get a mutable reference to the `dst` field.
    pub fn dst_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.dst
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the `src` field.
    pub fn set_src(&mut self, src: AccelerationStructureKHR) -> &mut Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field.
    pub fn set_dst(&mut self, dst: AccelerationStructureKHR) -> &mut Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: CopyAccelerationStructureModeKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `src` field in a builder way.
    pub fn with_src(mut self, src: AccelerationStructureKHR) -> Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field in a builder way.
    pub fn with_dst(mut self, dst: AccelerationStructureKHR) -> Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
        self.mode = mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyAccelerationStructureInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureInfoKHR {
            s_type: StructureType::CopyAccelerationStructureInfoKhr,
            p_next: std::ptr::null(),
            src: self.src.into_low_level(context, bump),
            dst: self.dst.into_low_level(context, bump),
            mode: self.mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyAccelerationStructureInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src: crate::conv::FromLowLevel::from_low_level(context, value.src),
            dst: crate::conv::FromLowLevel::from_low_level(context, value.dst),
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
        }
    }
}
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl CopyAccelerationStructureToMemoryInfoKHR {
    ///Get a reference to the `src` field.
    pub fn src(&self) -> &AccelerationStructureKHR {
        &self.src
    }
    ///Get a reference to the `dst` field.
    pub fn dst(&self) -> &DeviceOrHostAddressKHR {
        &self.dst
    }
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Get a mutable reference to the `src` field.
    pub fn src_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.src
    }
    ///Get a mutable reference to the `dst` field.
    pub fn dst_mut(&mut self) -> &mut DeviceOrHostAddressKHR {
        &mut self.dst
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the `src` field.
    pub fn set_src(&mut self, src: AccelerationStructureKHR) -> &mut Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field.
    pub fn set_dst(&mut self, dst: DeviceOrHostAddressKHR) -> &mut Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: CopyAccelerationStructureModeKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `src` field in a builder way.
    pub fn with_src(mut self, src: AccelerationStructureKHR) -> Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field in a builder way.
    pub fn with_dst(mut self, dst: DeviceOrHostAddressKHR) -> Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
        self.mode = mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyAccelerationStructureToMemoryInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::CopyAccelerationStructureToMemoryInfoKHR {
            s_type: StructureType::CopyAccelerationStructureToMemoryInfoKhr,
            p_next: std::ptr::null(),
            src: self.src.into_low_level(context, bump),
            dst: self.dst.into_low_level(context, bump),
            mode: self.mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyAccelerationStructureToMemoryInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src: crate::conv::FromLowLevel::from_low_level(context, value.src),
            dst: crate::conv::FromLowLevel::from_low_level(context, value.dst),
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
        }
    }
}
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl CopyMemoryToAccelerationStructureInfoKHR {
    ///Get a reference to the `src` field.
    pub fn src(&self) -> &DeviceOrHostAddressConstKHR {
        &self.src
    }
    ///Get a reference to the `dst` field.
    pub fn dst(&self) -> &AccelerationStructureKHR {
        &self.dst
    }
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Get a mutable reference to the `src` field.
    pub fn src_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.src
    }
    ///Get a mutable reference to the `dst` field.
    pub fn dst_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.dst
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the `src` field.
    pub fn set_src(&mut self, src: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field.
    pub fn set_dst(&mut self, dst: AccelerationStructureKHR) -> &mut Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: CopyAccelerationStructureModeKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `src` field in a builder way.
    pub fn with_src(mut self, src: DeviceOrHostAddressConstKHR) -> Self {
        self.src = src;
        self
    }
    ///Sets the `dst` field in a builder way.
    pub fn with_dst(mut self, dst: AccelerationStructureKHR) -> Self {
        self.dst = dst;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: CopyAccelerationStructureModeKHR) -> Self {
        self.mode = mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyMemoryToAccelerationStructureInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::CopyMemoryToAccelerationStructureInfoKHR {
            s_type: StructureType::CopyMemoryToAccelerationStructureInfoKhr,
            p_next: std::ptr::null(),
            src: self.src.into_low_level(context, bump),
            dst: self.dst.into_low_level(context, bump),
            mode: self.mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyMemoryToAccelerationStructureInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            src: crate::conv::FromLowLevel::from_low_level(context, value.src),
            dst: crate::conv::FromLowLevel::from_low_level(context, value.dst),
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
        }
    }
}
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureBuildSizesInfoKHR {
    #[doc(alias = "accelerationStructureSize")]
    pub acceleration_structure_size: DeviceSize,
    #[doc(alias = "updateScratchSize")]
    pub update_scratch_size: DeviceSize,
    #[doc(alias = "buildScratchSize")]
    pub build_scratch_size: DeviceSize,
}
impl AccelerationStructureBuildSizesInfoKHR {
    ///Get a reference to the `acceleration_structure_size` field.
    pub fn acceleration_structure_size(&self) -> &DeviceSize {
        &self.acceleration_structure_size
    }
    ///Get a reference to the `update_scratch_size` field.
    pub fn update_scratch_size(&self) -> &DeviceSize {
        &self.update_scratch_size
    }
    ///Get a reference to the `build_scratch_size` field.
    pub fn build_scratch_size(&self) -> &DeviceSize {
        &self.build_scratch_size
    }
    ///Get a mutable reference to the `acceleration_structure_size` field.
    pub fn acceleration_structure_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.acceleration_structure_size
    }
    ///Get a mutable reference to the `update_scratch_size` field.
    pub fn update_scratch_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.update_scratch_size
    }
    ///Get a mutable reference to the `build_scratch_size` field.
    pub fn build_scratch_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.build_scratch_size
    }
    ///Sets the `acceleration_structure_size` field.
    pub fn set_acceleration_structure_size(&mut self, acceleration_structure_size: DeviceSize) -> &mut Self {
        self.acceleration_structure_size = acceleration_structure_size;
        self
    }
    ///Sets the `update_scratch_size` field.
    pub fn set_update_scratch_size(&mut self, update_scratch_size: DeviceSize) -> &mut Self {
        self.update_scratch_size = update_scratch_size;
        self
    }
    ///Sets the `build_scratch_size` field.
    pub fn set_build_scratch_size(&mut self, build_scratch_size: DeviceSize) -> &mut Self {
        self.build_scratch_size = build_scratch_size;
        self
    }
    ///Sets the `acceleration_structure_size` field in a builder way.
    pub fn with_acceleration_structure_size(mut self, acceleration_structure_size: DeviceSize) -> Self {
        self.acceleration_structure_size = acceleration_structure_size;
        self
    }
    ///Sets the `update_scratch_size` field in a builder way.
    pub fn with_update_scratch_size(mut self, update_scratch_size: DeviceSize) -> Self {
        self.update_scratch_size = update_scratch_size;
        self
    }
    ///Sets the `build_scratch_size` field in a builder way.
    pub fn with_build_scratch_size(mut self, build_scratch_size: DeviceSize) -> Self {
        self.build_scratch_size = build_scratch_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureBuildSizesInfoKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureBuildSizesInfoKHR {
            s_type: StructureType::AccelerationStructureBuildSizesInfoKhr,
            p_next: std::ptr::null(),
            acceleration_structure_size: self.acceleration_structure_size.into_low_level(context, bump),
            update_scratch_size: self.update_scratch_size.into_low_level(context, bump),
            build_scratch_size: self.build_scratch_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureBuildSizesInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            acceleration_structure_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_size,
            ),
            update_scratch_size: crate::conv::FromLowLevel::from_low_level(context, value.update_scratch_size),
            build_scratch_size: crate::conv::FromLowLevel::from_low_level(context, value.build_scratch_size),
        }
    }
}
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceOrHostAddressKHR {
    DeviceAddress(DeviceAddress),
    HostAddress(std::ffi::c_void),
}
impl From<DeviceAddress> for DeviceOrHostAddressKHR {
    fn from(v: DeviceAddress) -> Self {
        Self::DeviceAddress(v)
    }
}
impl From<std::ffi::c_void> for DeviceOrHostAddressKHR {
    fn from(v: std::ffi::c_void) -> Self {
        Self::HostAddress(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceOrHostAddressKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::DeviceAddress(v) => crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR {
                device_address: (v.into_low_level(context, bump)),
            },
            Self::HostAddress(v) => crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR {
                host_address: (v.into_low_level(context, bump)),
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceOrHostAddressKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        _value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        unreachable!("cannot convert native union to high level if it does not have a selection");
    }
}
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeviceOrHostAddressConstKHR {
    DeviceAddress(DeviceAddress),
    HostAddress(std::ffi::c_void),
}
impl From<DeviceAddress> for DeviceOrHostAddressConstKHR {
    fn from(v: DeviceAddress) -> Self {
        Self::DeviceAddress(v)
    }
}
impl From<std::ffi::c_void> for DeviceOrHostAddressConstKHR {
    fn from(v: std::ffi::c_void) -> Self {
        Self::HostAddress(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceOrHostAddressConstKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::DeviceAddress(v) => {
                crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR {
                    device_address: (v.into_low_level(context, bump)),
                }
            },
            Self::HostAddress(v) => {
                crate::native::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR {
                    host_address: (v.into_low_level(context, bump)),
                }
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceOrHostAddressConstKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        _value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        unreachable!("cannot convert native union to high level if it does not have a selection");
    }
}
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccelerationStructureGeometryDataKHR {
    Triangles(AccelerationStructureGeometryTrianglesDataKHR),
    Aabbs(AccelerationStructureGeometryAabbsDataKHR),
    Instances(AccelerationStructureGeometryInstancesDataKHR),
}
impl From<AccelerationStructureGeometryTrianglesDataKHR> for AccelerationStructureGeometryDataKHR {
    fn from(v: AccelerationStructureGeometryTrianglesDataKHR) -> Self {
        Self::Triangles(v)
    }
}
impl From<AccelerationStructureGeometryAabbsDataKHR> for AccelerationStructureGeometryDataKHR {
    fn from(v: AccelerationStructureGeometryAabbsDataKHR) -> Self {
        Self::Aabbs(v)
    }
}
impl From<AccelerationStructureGeometryInstancesDataKHR> for AccelerationStructureGeometryDataKHR {
    fn from(v: AccelerationStructureGeometryInstancesDataKHR) -> Self {
        Self::Instances(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryDataKHR {
    type LowLevel = (
        GeometryTypeKHR,
        crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR,
    );
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::Triangles(v) => {
                crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR {
                    triangles: (v.into_low_level(context, bump)),
                }
            },
            Self::Aabbs(v) => {
                crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR {
                    aabbs: (v.into_low_level(context, bump)),
                }
            },
            Self::Instances(v) => {
                crate::native::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR {
                    instances: (v.into_low_level(context, bump)),
                }
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryDataKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        (variant, value): <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        match variant {
            TRIANGLES => Self::Triangles(value.triangles.into_high_level(context)),
            AABBS => Self::Aabbs(value.aabbs.into_high_level(context)),
            INSTANCES => Self::Instances(value.instances.into_high_level(context)),
        }
    }
}
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Debug)]
pub struct AccelerationStructureKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for AccelerationStructureKHR {
    fn clone(&self) -> Self {
        self.context.clone_acceleration_structure_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for AccelerationStructureKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for AccelerationStructureKHR {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for AccelerationStructureKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_acceleration_structure_khr(&self.id);
        }
    }
}
impl PartialEq for AccelerationStructureKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl AccelerationStructureKHR {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureKHR {
    type LowLevel = crate::native::extensions::khr_acceleration_structure::AccelerationStructureKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .acceleration_structure_khr()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .acceleration_structure_khr()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
