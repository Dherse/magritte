pub use crate::common::extensions::khr_acceleration_structure::{
    AabbPositionsKHR, AccelerationStructureBuildRangeInfoKHR, AccelerationStructureInstanceKHR, TransformMatrixKHR,
};
use crate::native::{
    extensions::khr_deferred_host_operations::DeferredOperationKHR,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceAddress,
        DeviceSize, Format, IndexType, QueryPool, QueryType, StructureType, VulkanResultCodes,
    },
};
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureCount")]
    pub acceleration_structure_count: u32,
    #[doc(alias = "pAccelerationStructures")]
    pub acceleration_structures: *const AccelerationStructureKHR,
}
impl Default for WriteDescriptorSetAccelerationStructureKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::WriteDescriptorSetAccelerationStructureKhr,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure_count: unsafe { std::mem::zeroed() },
            acceleration_structures: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: Bool32,
    #[doc(alias = "accelerationStructureCaptureReplay")]
    pub acceleration_structure_capture_replay: Bool32,
    #[doc(alias = "accelerationStructureIndirectBuild")]
    pub acceleration_structure_indirect_build: Bool32,
    #[doc(alias = "accelerationStructureHostCommands")]
    pub acceleration_structure_host_commands: Bool32,
    #[doc(alias = "descriptorBindingAccelerationStructureUpdateAfterBind")]
    pub descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}
impl Default for PhysicalDeviceAccelerationStructureFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceAccelerationStructureFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure: unsafe { std::mem::zeroed() },
            acceleration_structure_capture_replay: unsafe { std::mem::zeroed() },
            acceleration_structure_indirect_build: unsafe { std::mem::zeroed() },
            acceleration_structure_host_commands: unsafe { std::mem::zeroed() },
            descriptor_binding_acceleration_structure_update_after_bind: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
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
impl Default for PhysicalDeviceAccelerationStructurePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceAccelerationStructurePropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            max_geometry_count: unsafe { std::mem::zeroed() },
            max_instance_count: unsafe { std::mem::zeroed() },
            max_primitive_count: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_acceleration_structures: unsafe { std::mem::zeroed() },
            max_per_stage_descriptor_update_after_bind_acceleration_structures: unsafe { std::mem::zeroed() },
            max_descriptor_set_acceleration_structures: unsafe { std::mem::zeroed() },
            max_descriptor_set_update_after_bind_acceleration_structures: unsafe { std::mem::zeroed() },
            min_acceleration_structure_scratch_offset_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
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
impl Default for AccelerationStructureGeometryTrianglesDataKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureGeometryTrianglesDataKhr,
            p_next: unsafe { std::mem::zeroed() },
            vertex_format: unsafe { std::mem::zeroed() },
            vertex_data: unsafe { std::mem::zeroed() },
            vertex_stride: unsafe { std::mem::zeroed() },
            max_vertex: unsafe { std::mem::zeroed() },
            index_type: unsafe { std::mem::zeroed() },
            index_data: unsafe { std::mem::zeroed() },
            transform_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub data: DeviceOrHostAddressConstKHR,
    pub stride: DeviceSize,
}
impl Default for AccelerationStructureGeometryAabbsDataKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureGeometryAabbsDataKhr,
            p_next: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
            stride: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "arrayOfPointers")]
    pub array_of_pointers: Bool32,
    pub data: DeviceOrHostAddressConstKHR,
}
impl Default for AccelerationStructureGeometryInstancesDataKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureGeometryInstancesDataKhr,
            p_next: unsafe { std::mem::zeroed() },
            array_of_pointers: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "geometryType")]
    pub geometry_type: GeometryTypeKHR,
    pub geometry: AccelerationStructureGeometryDataKHR,
    pub flags: GeometryFlagsKHR,
}
impl Default for AccelerationStructureGeometryKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureGeometryKhr,
            p_next: unsafe { std::mem::zeroed() },
            geometry_type: unsafe { std::mem::zeroed() },
            geometry: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: AccelerationStructureTypeKHR,
    pub flags: BuildAccelerationStructureFlagsKHR,
    pub mode: BuildAccelerationStructureModeKHR,
    #[doc(alias = "srcAccelerationStructure")]
    pub src_acceleration_structure: AccelerationStructureKHR,
    #[doc(alias = "dstAccelerationStructure")]
    pub dst_acceleration_structure: AccelerationStructureKHR,
    #[doc(alias = "geometryCount")]
    pub geometry_count: u32,
    #[doc(alias = "pGeometries")]
    pub geometries: *const AccelerationStructureGeometryKHR,
    #[doc(alias = "ppGeometries")]
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR,
    #[doc(alias = "scratchData")]
    pub scratch_data: DeviceOrHostAddressKHR,
}
impl Default for AccelerationStructureBuildGeometryInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureBuildGeometryInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
            src_acceleration_structure: unsafe { std::mem::zeroed() },
            dst_acceleration_structure: unsafe { std::mem::zeroed() },
            geometry_count: unsafe { std::mem::zeroed() },
            geometries: unsafe { std::mem::zeroed() },
            pp_geometries: unsafe { std::mem::zeroed() },
            scratch_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
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
impl Default for AccelerationStructureCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            create_flags: unsafe { std::mem::zeroed() },
            buffer: unsafe { std::mem::zeroed() },
            offset: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            device_address: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructure")]
    pub acceleration_structure: AccelerationStructureKHR,
}
impl Default for AccelerationStructureDeviceAddressInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureDeviceAddressInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pVersionData")]
    pub version_data: *const u8,
}
impl Default for AccelerationStructureVersionInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureVersionInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            version_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub src: AccelerationStructureKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyAccelerationStructureInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            src: unsafe { std::mem::zeroed() },
            dst: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub src: AccelerationStructureKHR,
    pub dst: DeviceOrHostAddressKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyAccelerationStructureToMemoryInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyAccelerationStructureToMemoryInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            src: unsafe { std::mem::zeroed() },
            dst: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub src: DeviceOrHostAddressConstKHR,
    pub dst: AccelerationStructureKHR,
    pub mode: CopyAccelerationStructureModeKHR,
}
impl Default for CopyMemoryToAccelerationStructureInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::CopyMemoryToAccelerationStructureInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            src: unsafe { std::mem::zeroed() },
            dst: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureSize")]
    pub acceleration_structure_size: DeviceSize,
    #[doc(alias = "updateScratchSize")]
    pub update_scratch_size: DeviceSize,
    #[doc(alias = "buildScratchSize")]
    pub build_scratch_size: DeviceSize,
}
impl Default for AccelerationStructureBuildSizesInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureBuildSizesInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            acceleration_structure_size: unsafe { std::mem::zeroed() },
            update_scratch_size: unsafe { std::mem::zeroed() },
            build_scratch_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressKHR {
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
    #[doc(alias = "hostAddress")]
    pub host_address: *mut std::ffi::c_void,
}
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressConstKHR {
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
    #[doc(alias = "hostAddress")]
    pub host_address: *const std::ffi::c_void,
}
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureGeometryDataKHR {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR,
    pub instances: AccelerationStructureGeometryInstancesDataKHR,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkAccelerationStructureKHR")]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);
impl AccelerationStructureKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for AccelerationStructureKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_acceleration_structure::{
    AccelerationStructureBuildTypeKHR, AccelerationStructureCompatibilityKHR, AccelerationStructureCreateFlagBitsKHR,
    AccelerationStructureCreateFlagsKHR, AccelerationStructureTypeKHR, BuildAccelerationStructureFlagBitsKHR,
    BuildAccelerationStructureFlagsKHR, BuildAccelerationStructureModeKHR, CopyAccelerationStructureModeKHR,
    GeometryFlagBitsKHR, GeometryFlagsKHR, GeometryInstanceFlagBitsKHR, GeometryInstanceFlagsKHR, GeometryTypeKHR,
    KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME, KHR_ACCELERATION_STRUCTURE_SPEC_VERSION,
};
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
pub type FNDestroyAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkCopyAccelerationStructureKHR")]
pub type FNCopyAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
pub type FNCopyAccelerationStructureToMemoryKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
pub type FNCopyMemoryToAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkWriteAccelerationStructuresPropertiesKHR")]
pub type FNWriteAccelerationStructuresPropertiesKhr = unsafe extern "system" fn(
    device: Device,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut std::ffi::c_void,
    stride: usize,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
pub type FNGetDeviceAccelerationStructureCompatibilityKhr = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[doc(alias = "vkCreateAccelerationStructureKHR")]
pub type FNCreateAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
pub type FNBuildAccelerationStructuresKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
pub type FNGetAccelerationStructureDeviceAddressKhr = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureDeviceAddressInfoKHR,
) -> DeviceAddress;
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
pub type FNGetAccelerationStructureBuildSizesKhr = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
pub type FNCmdCopyAccelerationStructureKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyAccelerationStructureInfoKHR);
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
pub type FNCmdCopyAccelerationStructureToMemoryKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyAccelerationStructureToMemoryInfoKHR);
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
pub type FNCmdCopyMemoryToAccelerationStructureKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyMemoryToAccelerationStructureInfoKHR);
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
pub type FNCmdWriteAccelerationStructuresPropertiesKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
pub type FNCmdBuildAccelerationStructuresKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
pub type FNCmdBuildAccelerationStructuresIndirectKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    p_indirect_device_addresses: *const DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
