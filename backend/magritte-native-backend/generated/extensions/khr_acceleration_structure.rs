//!# [VK_KHR_acceleration_structure](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VK_KHR_acceleration_structure.md")]
use crate::{
    cstr,
    extensions::khr_deferred_host_operations::DeferredOperationKHR,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceAddress,
        DeviceSize, Format, IndexType, QueryPool, QueryType, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkWriteDescriptorSetAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkWriteDescriptorSetAccelerationStructureKHR.md")]
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureCount")]
    acceleration_structure_count: u32,
    #[doc(alias = "pAccelerationStructures")]
    acceleration_structures: *const AccelerationStructureKHR,
}
///# [VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkPhysicalDeviceAccelerationStructureFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "accelerationStructure")]
    acceleration_structure: Bool32,
    #[doc(alias = "accelerationStructureCaptureReplay")]
    acceleration_structure_capture_replay: Bool32,
    #[doc(alias = "accelerationStructureIndirectBuild")]
    acceleration_structure_indirect_build: Bool32,
    #[doc(alias = "accelerationStructureHostCommands")]
    acceleration_structure_host_commands: Bool32,
    #[doc(alias = "descriptorBindingAccelerationStructureUpdateAfterBind")]
    descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}
///# [VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkPhysicalDeviceAccelerationStructurePropertiesKHR.md")]
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxGeometryCount")]
    max_geometry_count: u64,
    #[doc(alias = "maxInstanceCount")]
    max_instance_count: u64,
    #[doc(alias = "maxPrimitiveCount")]
    max_primitive_count: u64,
    #[doc(alias = "maxPerStageDescriptorAccelerationStructures")]
    max_per_stage_descriptor_acceleration_structures: u32,
    #[doc(alias = "maxPerStageDescriptorUpdateAfterBindAccelerationStructures")]
    max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    #[doc(alias = "maxDescriptorSetAccelerationStructures")]
    max_descriptor_set_acceleration_structures: u32,
    #[doc(alias = "maxDescriptorSetUpdateAfterBindAccelerationStructures")]
    max_descriptor_set_update_after_bind_acceleration_structures: u32,
    #[doc(alias = "minAccelerationStructureScratchOffsetAlignment")]
    min_acceleration_structure_scratch_offset_alignment: u32,
}
///# [VkAccelerationStructureGeometryTrianglesDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureGeometryTrianglesDataKHR.md")]
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "vertexFormat")]
    vertex_format: Format,
    #[doc(alias = "vertexData")]
    vertex_data: DeviceOrHostAddressConstKHR,
    #[doc(alias = "vertexStride")]
    vertex_stride: DeviceSize,
    #[doc(alias = "maxVertex")]
    max_vertex: u32,
    #[doc(alias = "indexType")]
    index_type: IndexType,
    #[doc(alias = "indexData")]
    index_data: DeviceOrHostAddressConstKHR,
    #[doc(alias = "transformData")]
    transform_data: DeviceOrHostAddressConstKHR,
}
///# [VkAccelerationStructureGeometryAabbsDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureGeometryAabbsDataKHR.md")]
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    data: DeviceOrHostAddressConstKHR,
    stride: DeviceSize,
}
///# [VkAccelerationStructureGeometryInstancesDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureGeometryInstancesDataKHR.md")]
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "arrayOfPointers")]
    array_of_pointers: Bool32,
    data: DeviceOrHostAddressConstKHR,
}
///# [VkAccelerationStructureGeometryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureGeometryKHR.md")]
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "geometryType")]
    geometry_type: GeometryTypeKHR,
    geometry: AccelerationStructureGeometryDataKHR,
    flags: GeometryFlagsKHR,
}
///# [VkAccelerationStructureBuildGeometryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureBuildGeometryInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: AccelerationStructureTypeKHR,
    flags: BuildAccelerationStructureFlagsKHR,
    mode: BuildAccelerationStructureModeKHR,
    #[doc(alias = "srcAccelerationStructure")]
    src_acceleration_structure: AccelerationStructureKHR,
    #[doc(alias = "dstAccelerationStructure")]
    dst_acceleration_structure: AccelerationStructureKHR,
    #[doc(alias = "geometryCount")]
    geometry_count: u32,
    #[doc(alias = "pGeometries")]
    geometries: *const AccelerationStructureGeometryKHR,
    #[doc(alias = "ppGeometries")]
    pp_geometries: *const *const AccelerationStructureGeometryKHR,
    #[doc(alias = "scratchData")]
    scratch_data: DeviceOrHostAddressKHR,
}
///# [VkAccelerationStructureBuildRangeInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureBuildRangeInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    #[doc(alias = "primitiveCount")]
    primitive_count: u32,
    #[doc(alias = "primitiveOffset")]
    primitive_offset: u32,
    #[doc(alias = "firstVertex")]
    first_vertex: u32,
    #[doc(alias = "transformOffset")]
    transform_offset: u32,
}
///# [VkAccelerationStructureCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureCreateInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "createFlags")]
    create_flags: AccelerationStructureCreateFlagsKHR,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    #[doc(alias = "type")]
    type_: AccelerationStructureTypeKHR,
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
}
///# [VkAabbPositionsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAabbPositionsKHR.md")]
#[doc(alias = "VkAabbPositionsKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AabbPositionsKHR {
    #[doc(alias = "minX")]
    min_x: f32,
    #[doc(alias = "minY")]
    min_y: f32,
    #[doc(alias = "minZ")]
    min_z: f32,
    #[doc(alias = "maxX")]
    max_x: f32,
    #[doc(alias = "maxY")]
    max_y: f32,
    #[doc(alias = "maxZ")]
    max_z: f32,
}
///# [VkTransformMatrixKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkTransformMatrixKHR.md")]
#[doc(alias = "VkTransformMatrixKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TransformMatrixKHR {
    matrix: [f32; 3 as usize],
}
///# [VkAccelerationStructureInstanceKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureInstanceKHR.md")]
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    transform: TransformMatrixKHR,
    #[doc(alias = "instanceCustomIndex")]
    instance_custom_index: u32,
    mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    instance_shader_binding_table_record_offset: u32,
    flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    acceleration_structure_reference: u64,
}
///# [VkAccelerationStructureDeviceAddressInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureDeviceAddressInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructure")]
    acceleration_structure: AccelerationStructureKHR,
}
///# [VkAccelerationStructureVersionInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureVersionInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pVersionData")]
    version_data: *const u8,
}
///# [VkCopyAccelerationStructureInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkCopyAccelerationStructureInfoKHR.md")]
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    src: AccelerationStructureKHR,
    dst: AccelerationStructureKHR,
    mode: CopyAccelerationStructureModeKHR,
}
///# [VkCopyAccelerationStructureToMemoryInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkCopyAccelerationStructureToMemoryInfoKHR.md")]
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    src: AccelerationStructureKHR,
    dst: DeviceOrHostAddressKHR,
    mode: CopyAccelerationStructureModeKHR,
}
///# [VkCopyMemoryToAccelerationStructureInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkCopyMemoryToAccelerationStructureInfoKHR.md")]
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    src: DeviceOrHostAddressConstKHR,
    dst: AccelerationStructureKHR,
    mode: CopyAccelerationStructureModeKHR,
}
///# [VkAccelerationStructureBuildSizesInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureBuildSizesInfoKHR.md")]
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "accelerationStructureSize")]
    acceleration_structure_size: DeviceSize,
    #[doc(alias = "updateScratchSize")]
    update_scratch_size: DeviceSize,
    #[doc(alias = "buildScratchSize")]
    build_scratch_size: DeviceSize,
}
///# [VkDeviceOrHostAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkDeviceOrHostAddressKHR.md")]
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressKHR {
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
    #[doc(alias = "hostAddress")]
    host_address: *mut std::ffi::c_void,
}
///# [VkDeviceOrHostAddressConstKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkDeviceOrHostAddressConstKHR.md")]
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressConstKHR {
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
    #[doc(alias = "hostAddress")]
    host_address: *const std::ffi::c_void,
}
///# [VkAccelerationStructureGeometryDataKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureGeometryDataKHR.md")]
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureGeometryDataKHR {
    triangles: AccelerationStructureGeometryTrianglesDataKHR,
    aabbs: AccelerationStructureGeometryAabbsDataKHR,
    instances: AccelerationStructureGeometryInstancesDataKHR,
}
///# [VkAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureKHR.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkAccelerationStructureKHR")]
#[repr(transparent)]
pub struct AccelerationStructureKHR(u64);
impl AccelerationStructureKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for AccelerationStructureKHR {
    fn default() -> Self {
        Self::null()
    }
}
///# [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkGeometryFlagBitsKHR.md")]
#[doc(alias = "VkGeometryFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryFlagsKHR(u32);
impl GeometryFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::OPAQUE;
        }
        {
            all |= Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::OPAQUE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::NO_DUPLICATE_ANY_HIT_INVOCATION_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for GeometryFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for GeometryFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for GeometryFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for GeometryFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for GeometryFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from(bit: GeometryFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagBitsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(GeometryFlagsKHR::OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE))?;
                    }
                    if self.0.contains(GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NO_DUPLICATE_ANY_HIT_INVOCATION))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryFlagsKHR::OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NO_DUPLICATE_ANY_HIT_INVOCATION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkGeometryInstanceFlagBitsKHR.md")]
#[doc(alias = "VkGeometryInstanceFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryInstanceFlagsKHR(u32);
impl GeometryInstanceFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
    pub const TRIANGLE_FLIP_FACING: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
    pub const FORCE_OPAQUE: Self = Self(4);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
    pub const FORCE_NO_OPAQUE: Self = Self(8);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::TRIANGLE_FACING_CULL_DISABLE;
        }
        {
            all |= Self::TRIANGLE_FLIP_FACING;
        }
        {
            all |= Self::FORCE_OPAQUE;
        }
        {
            all |= Self::FORCE_NO_OPAQUE;
        }
        {
            all |= Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::TRIANGLE_CULL_DISABLE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::FORCE_OPAQUE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::FORCE_NO_OPAQUE_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for GeometryInstanceFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from(bit: GeometryInstanceFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryInstanceFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryInstanceFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FACING_CULL_DISABLE))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_FLIP_FACING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FLIP_FACING))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_OPAQUE))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_NO_OPAQUE))?;
                    }
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::TRIANGLE_FRONT_COUNTERCLOCKWISE)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FRONT_COUNTERCLOCKWISE))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_CULL_DISABLE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_CULL_DISABLE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::TRIANGLE_FRONT_COUNTERCLOCKWISE_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FRONT_COUNTERCLOCKWISE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_OPAQUE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_NO_OPAQUE_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryInstanceFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkBuildAccelerationStructureFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkBuildAccelerationStructureFlagBitsKHR.md")]
#[doc(alias = "VkBuildAccelerationStructureFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuildAccelerationStructureFlagsKHR(u32);
impl BuildAccelerationStructureFlagsKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
    pub const ALLOW_UPDATE: Self = Self(1);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
    pub const ALLOW_COMPACTION: Self = Self(2);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
    pub const PREFER_FAST_TRACE: Self = Self(4);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
    pub const PREFER_FAST_BUILD: Self = Self(8);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
    pub const LOW_MEMORY: Self = Self(16);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(32);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::ALLOW_UPDATE;
        }
        {
            all |= Self::ALLOW_COMPACTION;
        }
        {
            all |= Self::PREFER_FAST_TRACE;
        }
        {
            all |= Self::PREFER_FAST_BUILD;
        }
        {
            all |= Self::LOW_MEMORY;
        }
        #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
        {
            all |= Self::MOTION_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::ALLOW_UPDATE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::ALLOW_COMPACTION_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::PREFER_FAST_TRACE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::PREFER_FAST_BUILD_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::LOW_MEMORY_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for BuildAccelerationStructureFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from(bit: BuildAccelerationStructureFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(BuildAccelerationStructureFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == BuildAccelerationStructureFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_UPDATE))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_COMPACTION))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_TRACE))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_BUILD))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::LOW_MEMORY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOW_MEMORY))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::MOTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MOTION_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_UPDATE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_COMPACTION_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_TRACE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_BUILD_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::LOW_MEMORY_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOW_MEMORY_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(BuildAccelerationStructureFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureCreateFlagBitsKHR.md")]
#[doc(alias = "VkAccelerationStructureCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureCreateFlagsKHR(u32);
impl AccelerationStructureCreateFlagsKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        }
        #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
        {
            all |= Self::MOTION_NV;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for AccelerationStructureCreateFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from(bit: AccelerationStructureCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AccelerationStructureCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AccelerationStructureCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(AccelerationStructureCreateFlagsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
                    if self.0.contains(AccelerationStructureCreateFlagsKHR::MOTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MOTION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AccelerationStructureCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_acceleration_structure");
///# [VkGeometryInstanceFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkGeometryInstanceFlagBitsKHR.md")]
#[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct GeometryInstanceFlagBitsKHR(u32);
impl GeometryInstanceFlagBitsKHR {
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
    pub const TRIANGLE_FLIP_FACING: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
    pub const FORCE_OPAQUE: Self = Self(4);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
    pub const FORCE_NO_OPAQUE: Self = Self(8);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::TRIANGLE_FACING_CULL_DISABLE.bits() => Some(Self(x)),
            x if x == Self::TRIANGLE_FLIP_FACING.bits() => Some(Self(x)),
            x if x == Self::FORCE_OPAQUE.bits() => Some(Self(x)),
            x if x == Self::FORCE_NO_OPAQUE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkGeometryFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkGeometryFlagBitsKHR.md")]
#[doc(alias = "VkGeometryFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct GeometryFlagBitsKHR(u32);
impl GeometryFlagBitsKHR {
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::OPAQUE.bits() => Some(Self(x)),
            x if x == Self::NO_DUPLICATE_ANY_HIT_INVOCATION.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkBuildAccelerationStructureFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkBuildAccelerationStructureFlagBitsKHR.md")]
#[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct BuildAccelerationStructureFlagBitsKHR(u32);
impl BuildAccelerationStructureFlagBitsKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
    pub const ALLOW_UPDATE: Self = Self(1);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
    pub const ALLOW_COMPACTION: Self = Self(2);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
    pub const PREFER_FAST_TRACE: Self = Self(4);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
    pub const PREFER_FAST_BUILD: Self = Self(8);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
    pub const LOW_MEMORY: Self = Self(16);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(32);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::ALLOW_UPDATE.bits() => Some(Self(x)),
            x if x == Self::ALLOW_COMPACTION.bits() => Some(Self(x)),
            x if x == Self::PREFER_FAST_TRACE.bits() => Some(Self(x)),
            x if x == Self::PREFER_FAST_BUILD.bits() => Some(Self(x)),
            x if x == Self::LOW_MEMORY.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            x if x == Self::MOTION_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkAccelerationStructureCreateFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureCreateFlagBitsKHR.md")]
#[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureCreateFlagBitsKHR(u32);
impl AccelerationStructureCreateFlagBitsKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            x if x == Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            x if x == Self::MOTION_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkCopyAccelerationStructureModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkCopyAccelerationStructureModeKHR.md")]
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct CopyAccelerationStructureModeKHR(i32);
impl CopyAccelerationStructureModeKHR {
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR")]
    pub const CLONE: Self = Self(0);
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR")]
    pub const COMPACT: Self = Self(1);
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR")]
    pub const SERIALIZE: Self = Self(2);
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR")]
    pub const DESERIALIZE: Self = Self(3);
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const CLONE_NV: Self = Self::CLONE;
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const COMPACT_NV: Self = Self::COMPACT;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::CLONE.bits() => Some(Self(x)),
            x if x == Self::COMPACT.bits() => Some(Self(x)),
            x if x == Self::SERIALIZE.bits() => Some(Self(x)),
            x if x == Self::DESERIALIZE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkBuildAccelerationStructureModeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkBuildAccelerationStructureModeKHR.md")]
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct BuildAccelerationStructureModeKHR(i32);
impl BuildAccelerationStructureModeKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR")]
    pub const BUILD: Self = Self(0);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR")]
    pub const UPDATE: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::BUILD.bits() => Some(Self(x)),
            x if x == Self::UPDATE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkAccelerationStructureTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureTypeKHR.md")]
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureTypeKHR(i32);
impl AccelerationStructureTypeKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR")]
    pub const TOP_LEVEL: Self = Self(0);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR")]
    pub const BOTTOM_LEVEL: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR")]
    pub const GENERIC: Self = Self(2);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL;
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::TOP_LEVEL.bits() => Some(Self(x)),
            x if x == Self::BOTTOM_LEVEL.bits() => Some(Self(x)),
            x if x == Self::GENERIC.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkGeometryTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkGeometryTypeKHR.md")]
#[doc(alias = "VkGeometryTypeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct GeometryTypeKHR(i32);
impl GeometryTypeKHR {
    #[doc(alias = "VK_GEOMETRY_TYPE_TRIANGLES_KHR")]
    pub const TRIANGLES: Self = Self(0);
    #[doc(alias = "VK_GEOMETRY_TYPE_AABBS_KHR")]
    pub const AABBS: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_TYPE_INSTANCES_KHR")]
    pub const INSTANCES: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_TYPE_TRIANGLES_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLES_NV: Self = Self::TRIANGLES;
    #[doc(alias = "VK_GEOMETRY_TYPE_AABBS_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const AABBS_NV: Self = Self::AABBS;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::TRIANGLES.bits() => Some(Self(x)),
            x if x == Self::AABBS.bits() => Some(Self(x)),
            x if x == Self::INSTANCES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkAccelerationStructureBuildTypeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureBuildTypeKHR.md")]
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureBuildTypeKHR(i32);
impl AccelerationStructureBuildTypeKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR")]
    pub const HOST: Self = Self(0);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR")]
    pub const DEVICE: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR")]
    pub const HOST_OR_DEVICE: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::HOST.bits() => Some(Self(x)),
            x if x == Self::DEVICE.bits() => Some(Self(x)),
            x if x == Self::HOST_OR_DEVICE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/VkAccelerationStructureCompatibilityKHR.md")]
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureCompatibilityKHR(i32);
impl AccelerationStructureCompatibilityKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR")]
    pub const COMPATIBLE: Self = Self(0);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR")]
    pub const INCOMPATIBLE: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::COMPATIBLE.bits() => Some(Self(x)),
            x if x == Self::INCOMPATIBLE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkDestroyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkDestroyAccelerationStructureKHR.md")]
#[doc(alias = "vkDestroyAccelerationStructureKHR")]
pub type FNDestroyAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const AllocationCallbacks,
);
///# [vkCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCopyAccelerationStructureKHR.md")]
#[doc(alias = "vkCopyAccelerationStructureKHR")]
pub type FNCopyAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR,
) -> VulkanResultCodes;
///# [vkCopyAccelerationStructureToMemoryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyAccelerationStructureToMemoryKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCopyAccelerationStructureToMemoryKHR.md")]
#[doc(alias = "vkCopyAccelerationStructureToMemoryKHR")]
pub type FNCopyAccelerationStructureToMemoryKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> VulkanResultCodes;
///# [vkCopyMemoryToAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCopyMemoryToAccelerationStructureKHR.md")]
#[doc(alias = "vkCopyMemoryToAccelerationStructureKHR")]
pub type FNCopyMemoryToAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> VulkanResultCodes;
///# [vkWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWriteAccelerationStructuresPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkWriteAccelerationStructuresPropertiesKHR.md")]
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
///# [vkGetDeviceAccelerationStructureCompatibilityKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceAccelerationStructureCompatibilityKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkGetDeviceAccelerationStructureCompatibilityKHR.md")]
#[doc(alias = "vkGetDeviceAccelerationStructureCompatibilityKHR")]
pub type FNGetDeviceAccelerationStructureCompatibilityKhr = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
///# [vkCreateAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCreateAccelerationStructureKHR.md")]
#[doc(alias = "vkCreateAccelerationStructureKHR")]
pub type FNCreateAccelerationStructureKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> VulkanResultCodes;
///# [vkBuildAccelerationStructuresKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkBuildAccelerationStructuresKHR.md")]
#[doc(alias = "vkBuildAccelerationStructuresKHR")]
pub type FNBuildAccelerationStructuresKhr = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> VulkanResultCodes;
///# [vkGetAccelerationStructureDeviceAddressKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureDeviceAddressKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkGetAccelerationStructureDeviceAddressKHR.md")]
#[doc(alias = "vkGetAccelerationStructureDeviceAddressKHR")]
pub type FNGetAccelerationStructureDeviceAddressKhr = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureDeviceAddressInfoKHR,
) -> DeviceAddress;
///# [vkGetAccelerationStructureBuildSizesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkGetAccelerationStructureBuildSizesKHR.md")]
#[doc(alias = "vkGetAccelerationStructureBuildSizesKHR")]
pub type FNGetAccelerationStructureBuildSizesKhr = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
///# [vkCmdCopyAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdCopyAccelerationStructureKHR.md")]
#[doc(alias = "vkCmdCopyAccelerationStructureKHR")]
pub type FNCmdCopyAccelerationStructureKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyAccelerationStructureInfoKHR);
///# [vkCmdCopyAccelerationStructureToMemoryKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureToMemoryKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdCopyAccelerationStructureToMemoryKHR.md")]
#[doc(alias = "vkCmdCopyAccelerationStructureToMemoryKHR")]
pub type FNCmdCopyAccelerationStructureToMemoryKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyAccelerationStructureToMemoryInfoKHR);
///# [vkCmdCopyMemoryToAccelerationStructureKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdCopyMemoryToAccelerationStructureKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdCopyMemoryToAccelerationStructureKHR.md")]
#[doc(alias = "vkCmdCopyMemoryToAccelerationStructureKHR")]
pub type FNCmdCopyMemoryToAccelerationStructureKhr =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_info: *const CopyMemoryToAccelerationStructureInfoKHR);
///# [vkCmdWriteAccelerationStructuresPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdWriteAccelerationStructuresPropertiesKHR.md")]
#[doc(alias = "vkCmdWriteAccelerationStructuresPropertiesKHR")]
pub type FNCmdWriteAccelerationStructuresPropertiesKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
///# [vkCmdBuildAccelerationStructuresKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdBuildAccelerationStructuresKHR.md")]
#[doc(alias = "vkCmdBuildAccelerationStructuresKHR")]
pub type FNCmdBuildAccelerationStructuresKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
///# [vkCmdBuildAccelerationStructuresIndirectKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresIndirectKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_acceleration_structure/vkCmdBuildAccelerationStructuresIndirectKHR.md")]
#[doc(alias = "vkCmdBuildAccelerationStructuresIndirectKHR")]
pub type FNCmdBuildAccelerationStructuresIndirectKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR,
    p_indirect_device_addresses: *const DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
