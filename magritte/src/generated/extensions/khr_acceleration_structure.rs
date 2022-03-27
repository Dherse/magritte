use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Buffer, DeviceAddress, DeviceSize, Format, IndexType, StructureType,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_acceleration_structure");
///[VkCopyAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) - Acceleration structure copy mode
///# C Specifications
///Possible values of `mode` specifying additional operations to perform
///during the copy, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkCopyAccelerationStructureModeKHR {
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR = 2,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR = 3,
///  // Provided by VK_NV_ray_tracing
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV = VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV =
/// VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR,
///} VkCopyAccelerationStructureModeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkCopyAccelerationStructureModeKHR VkCopyAccelerationStructureModeNV;
///```
///# Description
/// - [`CopyAccelerationStructureModeCloneKhr`] creates a direct copy of the acceleration structure
///   specified in `src` into the one specified by `dst`. The `dst` acceleration structure **must**
///   have been created with the same parameters as `src`. If `src` contains references to other
///   acceleration structures, `dst` will reference the same acceleration structures.
/// - [`CopyAccelerationStructureModeCompactKhr`] creates a more compact version of an acceleration
///   structure `src` into `dst`. The acceleration structure `dst`**must** have been created with a
///   size at least as large as that returned by [`CmdWriteAccelerationStructuresPropertiesKHR`] or
///   [`WriteAccelerationStructuresPropertiesKHR`] after the build of the acceleration structure
///   specified by `src`. If `src` contains references to other acceleration structures, `dst` will
///   reference the same acceleration structures.
/// - [`CopyAccelerationStructureModeSerializeKhr`] serializes the acceleration structure to a
///   semi-opaque format which can be reloaded on a compatible implementation.
/// - [`CopyAccelerationStructureModeDeserializeKhr`] deserializes the semi-opaque serialization
///   format in the buffer to the acceleration structure.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`CopyAccelerationStructureInfoKHR`]
/// - [`CopyAccelerationStructureToMemoryInfoKHR`]
/// - [`CopyMemoryToAccelerationStructureInfoKHR`]
/// - [`CmdCopyAccelerationStructureNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum CopyAccelerationStructureModeKHR {
    ///[`CopyAccelerationStructureModeCloneKhr`] creates a direct
    ///copy of the acceleration structure specified in `src` into the one
    ///specified by `dst`.
    ///The `dst` acceleration structure **must** have been created with the
    ///same parameters as `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    CopyAccelerationStructureModeCloneKhr = 0,
    ///[`CopyAccelerationStructureModeCompactKhr`] creates a more
    ///compact version of an acceleration structure `src` into `dst`.
    ///The acceleration structure `dst`**must** have been created with a size
    ///at least as large as that returned by
    ///[`CmdWriteAccelerationStructuresPropertiesKHR`]
    ///or [`WriteAccelerationStructuresPropertiesKHR`]
    ///after the build of the acceleration structure specified by `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    CopyAccelerationStructureModeCompactKhr = 1,
    ///[`CopyAccelerationStructureModeSerializeKhr`] serializes the
    ///acceleration structure to a semi-opaque format which can be reloaded on
    ///a compatible implementation.
    CopyAccelerationStructureModeSerializeKhr = 2,
    ///[`CopyAccelerationStructureModeDeserializeKhr`] deserializes
    ///the semi-opaque serialization format in the buffer to the acceleration
    ///structure.
    CopyAccelerationStructureModeDeserializeKhr = 3,
}
impl const Default for CopyAccelerationStructureModeKHR {
    fn default() -> Self {
        CopyAccelerationStructureModeCloneKhr
    }
}
impl CopyAccelerationStructureModeKHR {
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
///[VkBuildAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html) - Enum specifying the type of build operation to perform
///# C Specifications
///The [`BuildAccelerationStructureModeKHR`] enumeration is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkBuildAccelerationStructureModeKHR {
///    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0,
///    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1,
///} VkBuildAccelerationStructureModeKHR;
///```
///# Description
/// - [`BuildAccelerationStructureModeBuildKhr`] specifies that the destination acceleration
///   structure will be built using the specified geometries.
/// - [`BuildAccelerationStructureModeUpdateKhr`] specifies that the destination acceleration
///   structure will be built using data in a source acceleration structure, updated by the
///   specified geometries.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum BuildAccelerationStructureModeKHR {
    ///[`BuildAccelerationStructureModeBuildKhr`] specifies that the
    ///destination acceleration structure will be built using the specified
    ///geometries.
    BuildAccelerationStructureModeBuildKhr = 0,
    ///[`BuildAccelerationStructureModeUpdateKhr`] specifies that the
    ///destination acceleration structure will be built using data in a source
    ///acceleration structure, updated by the specified geometries.
    BuildAccelerationStructureModeUpdateKhr = 1,
}
impl const Default for BuildAccelerationStructureModeKHR {
    fn default() -> Self {
        BuildAccelerationStructureModeBuildKhr
    }
}
impl BuildAccelerationStructureModeKHR {
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
///[VkAccelerationStructureTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html) - Type of acceleration structure
///# C Specifications
///Values which **can** be set in
///[`AccelerationStructureCreateInfoKHR::type_`]
///or
///[`AccelerationStructureInfoNV::type_`]
///specifying the type of acceleration structure, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureTypeKHR {
///    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
///    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV = VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV =
/// VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
///} VkAccelerationStructureTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAccelerationStructureTypeKHR VkAccelerationStructureTypeNV;
///```
///# Description
/// - [`AccelerationStructureTypeTopLevelKhr`] is a top-level acceleration structure containing
///   instance data referring to bottom-level acceleration structures.
/// - [`AccelerationStructureTypeBottomLevelKhr`] is a bottom-level acceleration structure
///   containing the AABBs or geometry to be intersected.
/// - [`AccelerationStructureTypeGenericKhr`] is an acceleration structure whose type is determined
///   at build time used for special circumstances.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`AccelerationStructureCreateInfoKHR`]
/// - [`AccelerationStructureInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum AccelerationStructureTypeKHR {
    ///[`AccelerationStructureTypeTopLevelKhr`] is a top-level
    ///acceleration structure containing instance data referring to
    ///bottom-level acceleration structures.
    AccelerationStructureTypeTopLevelKhr = 0,
    ///[`AccelerationStructureTypeBottomLevelKhr`] is a bottom-level
    ///acceleration structure containing the AABBs or geometry to be
    ///intersected.
    AccelerationStructureTypeBottomLevelKhr = 1,
    ///[`AccelerationStructureTypeGenericKhr`] is an acceleration
    ///structure whose type is determined at build time used for special
    ///circumstances.
    AccelerationStructureTypeGenericKhr = 2,
}
impl const Default for AccelerationStructureTypeKHR {
    fn default() -> Self {
        AccelerationStructureTypeTopLevelKhr
    }
}
impl AccelerationStructureTypeKHR {
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
///[VkGeometryTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html) - Enum specifying which type of geometry is provided
///# C Specifications
///Geometry types are specified by [`GeometryTypeKHR`], which takes values:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryTypeKHR {
///    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
///    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
///    VK_GEOMETRY_TYPE_INSTANCES_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_TYPE_TRIANGLES_NV = VK_GEOMETRY_TYPE_TRIANGLES_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_TYPE_AABBS_NV = VK_GEOMETRY_TYPE_AABBS_KHR,
///} VkGeometryTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryTypeKHR VkGeometryTypeNV;
///```
///# Description
/// - [`GeometryTypeTrianglesKhr`] specifies a geometry type consisting of triangles.
/// - [`GeometryTypeAabbsKhr`] specifies a geometry type consisting of axis-aligned bounding boxes.
/// - [`GeometryTypeInstancesKhr`] specifies a geometry type consisting of acceleration structure
///   instances.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureGeometryKHR`]
/// - [`GeometryNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum GeometryTypeKHR {
    ///[`GeometryTypeTrianglesKhr`] specifies a geometry type
    ///consisting of triangles.
    GeometryTypeTrianglesKhr = 0,
    ///[`GeometryTypeAabbsKhr`] specifies a geometry type consisting of
    ///axis-aligned bounding boxes.
    GeometryTypeAabbsKhr = 1,
    ///[`GeometryTypeInstancesKhr`] specifies a geometry type
    ///consisting of acceleration structure instances.
    GeometryTypeInstancesKhr = 2,
}
impl const Default for GeometryTypeKHR {
    fn default() -> Self {
        GeometryTypeTrianglesKhr
    }
}
impl GeometryTypeKHR {
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
///[VkAccelerationStructureBuildTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) - Acceleration structure build type
///# C Specifications
///Possible values of `buildType` in
///[`GetAccelerationStructureBuildSizesKHR`] are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureBuildTypeKHR {
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
///} VkAccelerationStructureBuildTypeKHR;
///```
///# Description
/// - [`AccelerationStructureBuildTypeHostKhr`] requests the memory requirement for operations
///   performed by the host.
/// - [`AccelerationStructureBuildTypeDeviceKhr`] requests the memory requirement for operations
///   performed by the device.
/// - [`AccelerationStructureBuildTypeHostOrDeviceKhr`] requests the memory requirement for
///   operations performed by either the host, or the device.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum AccelerationStructureBuildTypeKHR {
    ///[`AccelerationStructureBuildTypeHostKhr`] requests the memory
    ///requirement for operations performed by the host.
    AccelerationStructureBuildTypeHostKhr = 0,
    ///[`AccelerationStructureBuildTypeDeviceKhr`] requests the
    ///memory requirement for operations performed by the device.
    AccelerationStructureBuildTypeDeviceKhr = 1,
    ///[`AccelerationStructureBuildTypeHostOrDeviceKhr`] requests
    ///the memory requirement for operations performed by either the host, or
    ///the device.
    AccelerationStructureBuildTypeHostOrDeviceKhr = 2,
}
impl const Default for AccelerationStructureBuildTypeKHR {
    fn default() -> Self {
        AccelerationStructureBuildTypeHostKhr
    }
}
impl AccelerationStructureBuildTypeKHR {
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
///[VkAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html) - Acceleration structure compatibility
///# C Specifications
///Possible values of `pCompatibility` returned by
///[`GetDeviceAccelerationStructureCompatibilityKHR`] are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureCompatibilityKHR {
///    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR = 1,
///} VkAccelerationStructureCompatibilityKHR;
///```
///# Description
/// - [`AccelerationStructureCompatibilityCompatibleKhr`] if the `pVersionData` version acceleration
///   structure is compatible with `device`.
/// - [`AccelerationStructureCompatibilityIncompatibleKhr`] if the `pVersionData` version
///   acceleration structure is not compatible with `device`.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`GetDeviceAccelerationStructureCompatibilityKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum AccelerationStructureCompatibilityKHR {
    ///[`AccelerationStructureCompatibilityCompatibleKhr`] if the
    ///`pVersionData` version acceleration structure is compatible with
    ///`device`.
    AccelerationStructureCompatibilityCompatibleKhr = 0,
    ///[`AccelerationStructureCompatibilityIncompatibleKhr`] if the
    ///`pVersionData` version acceleration structure is not compatible with
    ///`device`.
    AccelerationStructureCompatibilityIncompatibleKhr = 1,
}
impl const Default for AccelerationStructureCompatibilityKHR {
    fn default() -> Self {
        AccelerationStructureCompatibilityCompatibleKhr
    }
}
impl AccelerationStructureCompatibilityKHR {
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
///[VkWriteDescriptorSetAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) - Structure specifying acceleration structure descriptor information
///# C Specifications
///The [`WriteDescriptorSetAccelerationStructureKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkWriteDescriptorSetAccelerationStructureKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    uint32_t                             accelerationStructureCount;
///    const VkAccelerationStructureKHR*    pAccelerationStructures;
///} VkWriteDescriptorSetAccelerationStructureKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure_count`] is the number of elements in [`p_acceleration_structures`].
/// - [`p_acceleration_structures`] is a pointer to an array of [`AccelerationStructureKHR`]
///   structures specifying the acceleration structures to update.
///# Description
///Valid Usage
/// - [`acceleration_structure_count`]**must** be equal to `descriptorCount` in the extended
///   structure
/// - Each acceleration structure in [`p_acceleration_structures`]**must** have been created with a
///   `type` of `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR` or
///   `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor)
///   feature is not enabled, each element of [`p_acceleration_structures`]**must** not be
///   [`crate::utils::Handle::null`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
/// - [`p_acceleration_structures`]**must** be a valid pointer to an array of
///   [`acceleration_structure_count`] valid or
///   [`crate::utils::Handle::null`][`AccelerationStructureKHR`] handles
/// - [`acceleration_structure_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`acceleration_structure_count`] is the number of elements in
    ///[`p_acceleration_structures`].
    acceleration_structure_count: u32,
    ///[`p_acceleration_structures`] is a pointer to an array of
    ///[`AccelerationStructureKHR`] structures specifying the acceleration
    ///structures to update.
    p_acceleration_structures: *mut AccelerationStructureKHR,
}
///[VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) - Structure describing the acceleration structure features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           accelerationStructure;
///    VkBool32           accelerationStructureCaptureReplay;
///    VkBool32           accelerationStructureIndirectBuild;
///    VkBool32           accelerationStructureHostCommands;
///    VkBool32           descriptorBindingAccelerationStructureUpdateAfterBind;
///} VkPhysicalDeviceAccelerationStructureFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure`] indicates whether the implementation supports the acceleration structure functionality. See [Acceleration Structures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
/// - [`acceleration_structure_capture_replay`] indicates whether the implementation supports saving
///   and reusing acceleration structure device addresses, e.g. for trace capture and replay.
/// - [`acceleration_structure_indirect_build`] indicates whether the implementation supports
///   indirect acceleration structure build commands, e.g.
///   [`CmdBuildAccelerationStructuresIndirectKHR`].
/// - [`acceleration_structure_host_commands`] indicates whether the implementation supports host
///   side acceleration structure commands, e.g. [`BuildAccelerationStructuresKHR`],
///   [`CopyAccelerationStructureKHR`], [`CopyAccelerationStructureToMemoryKHR`],
///   [`CopyMemoryToAccelerationStructureKHR`], [`WriteAccelerationStructuresPropertiesKHR`].
/// - [`descriptor_binding_acceleration_structure_update_after_bind`] indicates whether the
///   implementation supports updating acceleration structure descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`.
///If the [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceAccelerationStructureFeaturesKHR`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`
///# Related
/// - [`VK_KHR_acceleration_structure`]
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
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`acceleration_structure`] indicates
    ///whether the implementation supports the acceleration structure
    ///functionality.
    ///See [Acceleration Structures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
    acceleration_structure: Bool32,
    ///[`acceleration_structure_capture_replay`] indicates whether the
    ///implementation supports saving and reusing acceleration structure device
    ///addresses, e.g. for trace capture and replay.
    acceleration_structure_capture_replay: Bool32,
    ///[`acceleration_structure_indirect_build`] indicates whether the
    ///implementation supports indirect acceleration structure build commands,
    ///e.g. [`CmdBuildAccelerationStructuresIndirectKHR`].
    acceleration_structure_indirect_build: Bool32,
    ///[`acceleration_structure_host_commands`] indicates whether the
    ///implementation supports host side acceleration structure commands, e.g.
    ///[`BuildAccelerationStructuresKHR`],
    ///[`CopyAccelerationStructureKHR`],
    ///[`CopyAccelerationStructureToMemoryKHR`],
    ///[`CopyMemoryToAccelerationStructureKHR`],
    ///[`WriteAccelerationStructuresPropertiesKHR`].
    acceleration_structure_host_commands: Bool32,
    ///[`descriptor_binding_acceleration_structure_update_after_bind`] indicates
    ///whether the implementation supports updating acceleration structure
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`.
    descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}
///[VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) - Properties of the physical device for acceleration structure
///# C Specifications
///The [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           maxGeometryCount;
///    uint64_t           maxInstanceCount;
///    uint64_t           maxPrimitiveCount;
///    uint32_t           maxPerStageDescriptorAccelerationStructures;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindAccelerationStructures;
///    uint32_t           maxDescriptorSetAccelerationStructures;
///    uint32_t           maxDescriptorSetUpdateAfterBindAccelerationStructures;
///    uint32_t           minAccelerationStructureScratchOffsetAlignment;
///} VkPhysicalDeviceAccelerationStructurePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_geometry_count`] is the maximum number of geometries in the bottom level acceleration
///   structure.
/// - [`max_instance_count`] is the maximum number of instances in the top level acceleration
///   structure.
/// - [`max_primitive_count`] is the maximum number of triangles or AABBs in all geometries in the
///   bottom level acceleration structure.
/// - [`max_per_stage_descriptor_acceleration_structures`] is the maximum number of acceleration
///   structure bindings that **can** be accessible to a single shader stage in a pipeline layout.
///   Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
///   count against this limit. Only descriptor bindings in descriptor set layouts created without
///   the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this
///   limit.
/// - [`max_per_stage_descriptor_update_after_bind_acceleration_structures`] is similar to
///   [`max_per_stage_descriptor_acceleration_structures`] but counts descriptor bindings from
///   descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_acceleration_structures`] is the maximum number of acceleration structure
///   descriptors that **can** be included in descriptor bindings in a pipeline layout across all
///   pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type
///   of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this limit. Only descriptor
///   bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_descriptor_set_update_after_bind_acceleration_structures`] is similar to
///   [`max_descriptor_set_acceleration_structures`] but counts descriptor bindings from descriptor
///   sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`
///   bit set.
/// - [`min_acceleration_structure_scratch_offset_alignment`] is the minimum **required** alignment,
///   in bytes, for scratch data passed in to an acceleration structure build command. The value
///   **must** be a power of two.
///# Description
///Due to the fact that the geometry, instance, and primitive counts are
///specified at acceleration structure creation as 32-bit values,
///[[`max_geometry_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxGeometryCount),
///[[`max_instance_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInstanceCount), and
///[[`max_primitive_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxPrimitiveCount)**must** not exceed
///2<sup>32</sup>-1.If the [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is
/// included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure **must**
/// match those specified with the same
///name in [`PhysicalDeviceRayTracingPropertiesNV`].Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_acceleration_structure`]
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
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_geometry_count`] is the maximum number
    ///of geometries in the bottom level acceleration structure.
    max_geometry_count: u64,
    ///[`max_instance_count`] is the maximum number
    ///of instances in the top level acceleration structure.
    max_instance_count: u64,
    ///[`max_primitive_count`] is the maximum
    ///number of triangles or AABBs in all geometries in the bottom level
    ///acceleration structure.
    max_primitive_count: u64,
    ///[`max_per_stage_descriptor_acceleration_structures`] is the maximum number
    ///of acceleration structure bindings that **can** be accessible to a single
    ///shader stage in a pipeline layout.
    ///Descriptor bindings with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this
    ///limit.
    ///Only descriptor bindings in descriptor set layouts created without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
    ///count against this limit.
    max_per_stage_descriptor_acceleration_structures: u32,
    ///[`max_per_stage_descriptor_update_after_bind_acceleration_structures`] is
    ///similar to [`max_per_stage_descriptor_acceleration_structures`] but counts
    ///descriptor bindings from descriptor sets created with or without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit
    ///set.
    max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    ///[`max_descriptor_set_acceleration_structures`] is the maximum number of
    ///acceleration structure descriptors that **can** be included in descriptor
    ///bindings in a pipeline layout across all pipeline shader stages and
    ///descriptor set numbers.
    ///Descriptor bindings with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this
    ///limit.
    ///Only descriptor bindings in descriptor set layouts created without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
    ///count against this limit.
    max_descriptor_set_acceleration_structures: u32,
    ///[`max_descriptor_set_update_after_bind_acceleration_structures`] is similar
    ///to [`max_descriptor_set_acceleration_structures`] but counts descriptor
    ///bindings from descriptor sets created with or without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit
    ///set.
    max_descriptor_set_update_after_bind_acceleration_structures: u32,
    ///[`min_acceleration_structure_scratch_offset_alignment`] is the minimum
    ///**required** alignment, in bytes, for scratch data passed in to an
    ///acceleration structure build command.
    ///The value **must** be a power of two.
    min_acceleration_structure_scratch_offset_alignment: u32,
}
///[VkAccelerationStructureGeometryTrianglesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) - Structure specifying a triangle geometry in a bottom-level acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryTrianglesDataKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryTrianglesDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkFormat                         vertexFormat;
///    VkDeviceOrHostAddressConstKHR    vertexData;
///    VkDeviceSize                     vertexStride;
///    uint32_t                         maxVertex;
///    VkIndexType                      indexType;
///    VkDeviceOrHostAddressConstKHR    indexData;
///    VkDeviceOrHostAddressConstKHR    transformData;
///} VkAccelerationStructureGeometryTrianglesDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_format`] is the [`Format`] of each vertex element.
/// - [`vertex_data`] is a device or host address to memory containing vertex data for this
///   geometry.
/// - [`max_vertex`] is the highest index of a vertex that will be addressed by a build command
///   using this structure.
/// - [`vertex_stride`] is the stride in bytes between each vertex.
/// - [`index_type`] is the [`IndexType`] of each index element.
/// - [`index_data`] is a device or host address to memory containing index data for this geometry.
/// - [`transform_data`] is a device or host address to memory containing an optional reference to a
///   [`TransformMatrixKHR`] structure describing a transformation from the space in which the
///   vertices in this geometry are described to the space in which the acceleration structure is
///   defined.
///# Description
///Valid Usage
/// - [`vertex_stride`]**must** be a multiple of the size in bytes of the smallest component of
///   [`vertex_format`]
/// - [`vertex_stride`]**must** be less than or equal to 2<sup>32</sup>-1
/// - [`vertex_format`]**must** support the
///   `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR` in
///   [`FormatProperties::buffer_features`] as returned by [`GetPhysicalDeviceFormatProperties2`]
/// - [`index_type`]**must** be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or
///   `VK_INDEX_TYPE_NONE_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`AccelerationStructureGeometryMotionTrianglesDataNV`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`vertex_format`]**must** be a valid [`Format`] value
/// - [`index_type`]**must** be a valid [`IndexType`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`DeviceSize`]
/// - [`Format`]
/// - [`IndexType`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`vertex_format`] is the [`Format`] of each vertex element.
    vertex_format: Format,
    ///[`vertex_data`] is a device or host address to memory containing vertex
    ///data for this geometry.
    vertex_data: DeviceOrHostAddressConstKHR<'lt>,
    ///[`vertex_stride`] is the stride in bytes between each vertex.
    vertex_stride: DeviceSize,
    ///[`max_vertex`] is the highest index of a vertex that will be addressed
    ///by a build command using this structure.
    max_vertex: u32,
    ///[`index_type`] is the [`IndexType`] of each index element.
    index_type: IndexType,
    ///[`index_data`] is a device or host address to memory containing index
    ///data for this geometry.
    index_data: DeviceOrHostAddressConstKHR<'lt>,
    ///[`transform_data`] is a device or host address to memory containing an
    ///optional reference to a [`TransformMatrixKHR`] structure describing
    ///a transformation from the space in which the vertices in this geometry
    ///are described to the space in which the acceleration structure is
    ///defined.
    transform_data: DeviceOrHostAddressConstKHR<'lt>,
}
///[VkAccelerationStructureGeometryAabbsDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) - Structure specifying axis-aligned bounding box geometry in a bottom-level acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryAabbsDataKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryAabbsDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkDeviceOrHostAddressConstKHR    data;
///    VkDeviceSize                     stride;
///} VkAccelerationStructureGeometryAabbsDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`data`] is a device or host address to memory containing [`AabbPositionsKHR`] structures
///   containing position data for each axis-aligned bounding box in the geometry.
/// - [`stride`] is the stride in bytes between each entry in [`data`]. The stride **must** be a
///   multiple of `8`.
///# Description
///Valid Usage
/// - [`stride`]**must** be a multiple of `8`
/// - [`stride`]**must** be less than or equal to 2<sup>32</sup>-1
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`data`] is a device or host address to memory containing
    ///[`AabbPositionsKHR`] structures containing position data for each
    ///axis-aligned bounding box in the geometry.
    data: DeviceOrHostAddressConstKHR<'lt>,
    ///[`stride`] is the stride in bytes between each entry in [`data`].
    ///The stride **must** be a multiple of `8`.
    stride: DeviceSize,
}
///[VkAccelerationStructureGeometryInstancesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) - Structure specifying a geometry consisting of instances of other acceleration structures
///# C Specifications
///The [`AccelerationStructureGeometryInstancesDataKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryInstancesDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkBool32                         arrayOfPointers;
///    VkDeviceOrHostAddressConstKHR    data;
///} VkAccelerationStructureGeometryInstancesDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`array_of_pointers`] specifies whether [`data`] is used as an array of addresses or just an
///   array.
/// - [`data`] is either the address of an array of device or host addresses referencing individual [`AccelerationStructureInstanceKHR`] structures or packed motion instance information as described in [motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances) if [`array_of_pointers`] is [`TRUE`], or the address of an array of [`AccelerationStructureInstanceKHR`] or [`AccelerationStructureMotionInstanceNV`] structures. Addresses and [`AccelerationStructureInstanceKHR`] structures are tightly packed. [`AccelerationStructureMotionInstanceNV`] structures have a stride of 160 bytes.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`Bool32`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`array_of_pointers`] specifies whether [`data`] is used as an array
    ///of addresses or just an array.
    array_of_pointers: Bool32,
    ///[`data`] is either the address of an array of device or host addresses
    ///referencing individual [`AccelerationStructureInstanceKHR`]
    ///structures
    ///or packed motion instance information as described in
    ///[motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances)
    ///if [`array_of_pointers`] is [`TRUE`], or the address of an array of
    ///[`AccelerationStructureInstanceKHR`]
    ///or [`AccelerationStructureMotionInstanceNV`]
    ///structures.
    ///Addresses and [`AccelerationStructureInstanceKHR`] structures are
    ///tightly packed.
    ///[`AccelerationStructureMotionInstanceNV`] structures have a stride
    ///of 160 bytes.
    data: DeviceOrHostAddressConstKHR<'lt>,
}
///[VkAccelerationStructureGeometryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html) - Structure specifying geometries to be built into an acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryKHR {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    VkGeometryTypeKHR                         geometryType;
///    VkAccelerationStructureGeometryDataKHR    geometry;
///    VkGeometryFlagsKHR                        flags;
///} VkAccelerationStructureGeometryKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`geometry_type`] describes which type of geometry this [`AccelerationStructureGeometryKHR`]
///   refers to.
/// - [`geometry`] is a [`AccelerationStructureGeometryDataKHR`] union describing the geometry data
///   for the relevant geometry type.
/// - [`flags`] is a bitmask of [`GeometryFlagBitsKHR`] values describing additional properties of
///   how the geometry should be built.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`geometry_type`]**must** be a valid [`GeometryTypeKHR`] value
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the `triangles` member of
///   [`geometry`]**must** be a valid [`AccelerationStructureGeometryTrianglesDataKHR`] structure
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the `aabbs` member of
///   [`geometry`]**must** be a valid [`AccelerationStructureGeometryAabbsDataKHR`] structure
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the `instances` member of
///   [`geometry`]**must** be a valid [`AccelerationStructureGeometryInstancesDataKHR`] structure
/// - [`flags`]**must** be a valid combination of [`GeometryFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`GeometryFlagsKHR`]
/// - [`GeometryTypeKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`geometry_type`] describes which type of geometry this
    ///[`AccelerationStructureGeometryKHR`] refers to.
    geometry_type: GeometryTypeKHR,
    ///[`geometry`] is a [`AccelerationStructureGeometryDataKHR`] union
    ///describing the geometry data for the relevant geometry type.
    geometry: AccelerationStructureGeometryDataKHR<'lt>,
    ///[`flags`] is a bitmask of [`GeometryFlagBitsKHR`] values
    ///describing additional properties of how the geometry should be built.
    flags: GeometryFlagsKHR,
}
///[VkAccelerationStructureBuildGeometryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) - Structure specifying the geometry data used to build an acceleration structure
///# C Specifications
///The [`AccelerationStructureBuildGeometryInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildGeometryInfoKHR {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    VkAccelerationStructureTypeKHR                      type;
///    VkBuildAccelerationStructureFlagsKHR                flags;
///    VkBuildAccelerationStructureModeKHR                 mode;
///    VkAccelerationStructureKHR                          srcAccelerationStructure;
///    VkAccelerationStructureKHR                          dstAccelerationStructure;
///    uint32_t                                            geometryCount;
///    const VkAccelerationStructureGeometryKHR*           pGeometries;
///    const VkAccelerationStructureGeometryKHR* const*    ppGeometries;
///    VkDeviceOrHostAddressKHR                            scratchData;
///} VkAccelerationStructureBuildGeometryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration
///   structure being built.
/// - [`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsKHR`] specifying additional
///   parameters of the acceleration structure.
/// - [`mode`] is a [`BuildAccelerationStructureModeKHR`] value specifying the type of operation to
///   perform.
/// - [`src_acceleration_structure`] is a pointer to an existing acceleration structure that is to
///   be used to update the `dst` acceleration structure when [`mode`] is
///   `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`.
/// - [`dst_acceleration_structure`] is a pointer to the target acceleration structure for the
///   build.
/// - [`geometry_count`] specifies the number of geometries that will be built into
///   [`dst_acceleration_structure`].
/// - [`p_geometries`] is a pointer to an array of [`AccelerationStructureGeometryKHR`] structures.
/// - [`pp_geometries`] is a pointer to an array of pointers to [`AccelerationStructureGeometryKHR`]
///   structures.
/// - [`scratch_data`] is the device or host address to memory that will be used as scratch memory
///   for the build.
///# Description
///Only one of [`p_geometries`] or [`pp_geometries`]**can** be a valid pointer,
///the other **must** be `NULL`.
///Each element of the non-`NULL` array describes the data used to build each
///acceleration structure geometry.The index of each element of the [`p_geometries`] or
/// [`pp_geometries`]
///members of [`AccelerationStructureBuildGeometryInfoKHR`] is used as the
///*geometry index* during ray traversal.
///The geometry index is available in ray shaders via the
///[`RayGeometryIndexKHR`
///built-in](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-builtin-variables-raygeometryindex), and is [used to
///determine hit and intersection shaders executed during traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table-hit-shader-indexing).
///The geometry index is available to ray queries via the
///`OpRayQueryGetIntersectionGeometryIndexKHR` instruction.Setting
/// `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`]
///indicates that this build is a motion top level acceleration structure.
///A motion top level uses instances of format
///[`AccelerationStructureMotionInstanceNV`] if
///[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
///is [`FALSE`].If
///[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
///is [`TRUE`], the pointer for any given element of the array of instance
///pointers consists of 4 bits of
///[`AccelerationStructureMotionInstanceTypeNV`] in the low 4 bits of the
///pointer identifying the type of structure at the pointer.
///The device address accessed is the value in the array with the low 4 bits
///set to zero.
///The structure at the pointer is one of
///[`AccelerationStructureInstanceKHR`],
///[`AccelerationStructureMatrixMotionInstanceNV`] or
///[`AccelerationStructureSrtMotionInstanceNV`], depending on the type
///value encoded in the low 4 bits.A top level acceleration structure with either motion instances
/// or vertex
///motion in its instances **must** set
///`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`].Members
/// [`src_acceleration_structure`] and [`dst_acceleration_structure`]**may** be the same or
/// different for an update operation (when [`mode`] is
///`VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`).
///If they are the same, the update happens in-place.
///Otherwise, the target acceleration structure is updated and the source is
///not modified.Valid Usage
/// - [`type_`]**must** not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - Only one of [`p_geometries`] or [`pp_geometries`]**can** be a valid pointer, the other
///   **must** be `NULL`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, the `geometryType` member of
///   elements of either [`p_geometries`] or [`pp_geometries`]**must** be
///   `VK_GEOMETRY_TYPE_INSTANCES_KHR`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, [`geometry_count`]**must** be
///   `1`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` the `geometryType` member of
///   elements of either [`p_geometries`] or [`pp_geometries`]**must** not be
///   `VK_GEOMETRY_TYPE_INSTANCES_KHR`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then the `geometryType`
///   member of each geometry in either [`p_geometries`] or [`pp_geometries`]**must** be the same
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then
///   [`geometry_count`]**must** be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_geometry_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType`
///   member of either [`p_geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the
///   total number of AABBs in all geometries **must** be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType`
///   member of either [`p_geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`,
///   the total number of triangles in all geometries **must** be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
/// - If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR` bit set, then
///   it **must** not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR` bit set
/// - If [`dst_acceleration_structure`] was created with
///   `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in
///   [`AccelerationStructureCreateInfoKHR`]::[`flags`],
///   `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV`**must** be set in [`flags`]
/// - If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`],
///   [`dst_acceleration_structure`]**must** have been created with
///   `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in
///   [`AccelerationStructureCreateInfoKHR`]::[`flags`]
/// - If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`], [`type_`]**must** not
///   be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`AccelerationStructureTypeKHR`] value
/// - [`flags`]**must** be a valid combination of [`BuildAccelerationStructureFlagBitsKHR`] values
/// - If [`geometry_count`] is not `0`, and [`p_geometries`] is not `NULL`, [`p_geometries`]**must**
///   be a valid pointer to an array of [`geometry_count`] valid
///   [`AccelerationStructureGeometryKHR`] structures
/// - If [`geometry_count`] is not `0`, and [`pp_geometries`] is not `NULL`,
///   [`pp_geometries`]**must** be a valid pointer to an array of [`geometry_count`] valid pointers
///   to valid [`AccelerationStructureGeometryKHR`] structures
/// - Both of [`dst_acceleration_structure`], and [`src_acceleration_structure`] that are valid
///   handles of non-ignored parameters **must** have been created, allocated, or retrieved from the
///   same [`Device`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryKHR`]
/// - [`AccelerationStructureKHR`]
/// - [`AccelerationStructureTypeKHR`]
/// - [`BuildAccelerationStructureFlagsKHR`]
/// - [`BuildAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressKHR`]
/// - [`StructureType`]
/// - [`BuildAccelerationStructuresKHR`]
/// - [`CmdBuildAccelerationStructuresIndirectKHR`]
/// - [`CmdBuildAccelerationStructuresKHR`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`type_`] is a [`AccelerationStructureTypeKHR`] value specifying
    ///the type of acceleration structure being built.
    type_: AccelerationStructureTypeKHR,
    ///[`flags`] is a bitmask of
    ///[`BuildAccelerationStructureFlagBitsKHR`] specifying additional
    ///parameters of the acceleration structure.
    flags: BuildAccelerationStructureFlagsKHR,
    ///[`mode`] is a [`BuildAccelerationStructureModeKHR`] value
    ///specifying the type of operation to perform.
    mode: BuildAccelerationStructureModeKHR,
    ///[`src_acceleration_structure`] is a pointer to an existing acceleration
    ///structure that is to be used to update the `dst` acceleration
    ///structure when [`mode`] is
    ///`VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`.
    src_acceleration_structure: AccelerationStructureKHR,
    ///[`dst_acceleration_structure`] is a pointer to the target acceleration
    ///structure for the build.
    dst_acceleration_structure: AccelerationStructureKHR,
    ///[`geometry_count`] specifies the number of geometries that will be
    ///built into [`dst_acceleration_structure`].
    geometry_count: u32,
    ///[`p_geometries`] is a pointer to an array of
    ///[`AccelerationStructureGeometryKHR`] structures.
    p_geometries: *mut AccelerationStructureGeometryKHR<'lt>,
    ///[`pp_geometries`] is a pointer to an array of pointers to
    ///[`AccelerationStructureGeometryKHR`] structures.
    pp_geometries: *mut *mut AccelerationStructureGeometryKHR<'lt>,
    ///[`scratch_data`] is the device or host address to memory that will be
    ///used as scratch memory for the build.
    scratch_data: DeviceOrHostAddressKHR<'lt>,
}
///[VkAccelerationStructureBuildRangeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) - Structure specifying build offsets and counts for acceleration structure builds
///# C Specifications
///[`AccelerationStructureBuildRangeInfoKHR`] is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildRangeInfoKHR {
///    uint32_t    primitiveCount;
///    uint32_t    primitiveOffset;
///    uint32_t    firstVertex;
///    uint32_t    transformOffset;
///} VkAccelerationStructureBuildRangeInfoKHR;
///```
///# Members
/// - [`primitive_count`] defines the number of primitives for a corresponding acceleration
///   structure geometry.
/// - [`primitive_offset`] defines an offset in bytes into the memory where primitive data is
///   defined.
/// - [`first_vertex`] is the index of the first vertex to build from for triangle geometry.
/// - [`transform_offset`] defines an offset in bytes into the memory where a transform matrix is
///   defined.
///# Description
///The primitive count and primitive offset are interpreted differently
///depending on the [`GeometryTypeKHR`] used:
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, [`primitive_count`] is the number of
///   triangles to be built, where each triangle is treated as 3 vertices.  - If the geometry uses
///   indices, [`primitive_count`] × 3 indices are consumed from
///   [`AccelerationStructureGeometryTrianglesDataKHR::index_data`], starting at an offset of
///   [`primitive_offset`]. The value of [`first_vertex`] is added to the index values before
///   fetching vertices.  - If the geometry does not use indices, [`primitive_count`] × 3 vertices
///   are consumed from [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`], starting at
///   an offset of [`primitive_offset`] +
///   [`AccelerationStructureGeometryTrianglesDataKHR::vertex_stride`] × [`first_vertex`].  - If
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`] is not `NULL`, a single
///   [`TransformMatrixKHR`] structure is consumed from
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`], at an offset of
///   [`transform_offset`]. This matrix describes a transformation from the space in which the
///   vertices for all triangles in this geometry are described to the space in which the
///   acceleration structure is defined.
/// - For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, [`primitive_count`] is the number of
///   axis-aligned bounding boxes. [`primitive_count`][`AabbPositionsKHR`] structures are consumed
///   from [`AccelerationStructureGeometryAabbsDataKHR::data`], starting at an offset of
///   [`primitive_offset`].
/// - For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, [`primitive_count`] is the number of
///   acceleration structures. [`primitive_count`][`AccelerationStructureInstanceKHR`] or
///   [`AccelerationStructureMotionInstanceNV`] structures are consumed from
///   [`AccelerationStructureGeometryInstancesDataKHR::data`], starting at an offset of
///   [`primitive_offset`].
///Valid Usage
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry uses indices, the
///   offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryTrianglesDataKHR::index_data`]**must** be a multiple of the
///   element size of [`AccelerationStructureGeometryTrianglesDataKHR::index_type`]
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry does not use indices,
///   the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`]**must** be a multiple of the
///   component size of [`AccelerationStructureGeometryTrianglesDataKHR::vertex_format`]
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the offset [`transform_offset`] from
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`]**must** be a multiple of 16
/// - For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryAabbsDataKHR::data`]**must** be a multiple of 8
/// - For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryInstancesDataKHR::data`]**must** be a multiple of 16
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`BuildAccelerationStructuresKHR`]
/// - [`CmdBuildAccelerationStructuresKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    ///[`primitive_count`] defines the number of primitives for a
    ///corresponding acceleration structure geometry.
    primitive_count: u32,
    ///[`primitive_offset`] defines an offset in bytes into the memory where
    ///primitive data is defined.
    primitive_offset: u32,
    ///[`first_vertex`] is the index of the first vertex to build from for
    ///triangle geometry.
    first_vertex: u32,
    ///[`transform_offset`] defines an offset in bytes into the memory where a
    ///transform matrix is defined.
    transform_offset: u32,
}
///[VkAccelerationStructureCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) - Structure specifying the parameters of a newly created acceleration structure object
///# C Specifications
///The [`AccelerationStructureCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureCreateInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkAccelerationStructureCreateFlagsKHR    createFlags;
///    VkBuffer                                 buffer;
///    VkDeviceSize                             offset;
///    VkDeviceSize                             size;
///    VkAccelerationStructureTypeKHR           type;
///    VkDeviceAddress                          deviceAddress;
///} VkAccelerationStructureCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`create_flags`] is a bitmask of [`AccelerationStructureCreateFlagBitsKHR`] specifying
///   additional creation parameters of the acceleration structure.
/// - [`buffer`] is the buffer on which the acceleration structure will be stored.
/// - [`offset`] is an offset in bytes from the base address of the buffer at which the acceleration
///   structure will be stored, and **must** be a multiple of `256`.
/// - [`size`] is the size required for the acceleration structure.
/// - [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration
///   structure that will be created.
/// - [`device_address`] is the device address requested for the acceleration structure if the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay)
///   feature is being used.
///# Description
///If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not
/// zero, [`device_address`]**must** be an address
///retrieved from an identically created acceleration structure on the same
///implementation.
///The acceleration structure **must** also be placed on an identically created
///[`buffer`] and at the same [`offset`].Applications **should** avoid creating acceleration
/// structures with
///application-provided addresses and implementation-provided addresses in the
///same process, to reduce the likelihood of
///`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR` errors.Applications **should** create an
/// acceleration structure with a specific
///[`AccelerationStructureTypeKHR`] other than
///`VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`.If the acceleration structure will be the target of
/// a build operation, the
///required size for an acceleration structure **can** be queried with
///[`GetAccelerationStructureBuildSizesKHR`].
///If the acceleration structure is going to be the target of a compacting
///copy, [`CmdWriteAccelerationStructuresPropertiesKHR`] or
///[`WriteAccelerationStructuresPropertiesKHR`]**can** be used to obtain the
///compacted size required.If the acceleration structure will be the target of a build operation
/// with
///`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` it **must** include
///`VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` in `flags` and
///include [`AccelerationStructureMotionInfoNV`] as an extension structure
///in [`p_next`] with the number of instances as metadata for the object.Valid Usage
/// - If [`device_address`] is not zero, [`create_flags`]**must** include
///   `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`
/// - If [`create_flags`] includes
///   `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`,
///   [`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_capture_replay`]**
///   must** be [`TRUE`]
/// - [`buffer`]**must** have been created with a `usage` value containing
///   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR`
/// - [`buffer`]**must** not have been created with `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`
/// - The sum of [`offset`] and [`size`]**must** be less than the size of [`buffer`]
/// - [`offset`]**must** be a multiple of `256` bytes
/// - If `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` is set in    `flags` and [`type_`] is
///   `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, one member of the [`p_next`] chain **must** be
///   a pointer to a valid instance of [`AccelerationStructureMotionInfoNV`]
/// - If any geometry includes [`AccelerationStructureGeometryMotionTrianglesDataNV`] then
///   `flags`**must** contain `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`AccelerationStructureMotionInfoNV`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`create_flags`]**must** be a valid combination of [`AccelerationStructureCreateFlagBitsKHR`]
///   values
/// - [`buffer`]**must** be a valid [`Buffer`] handle
/// - [`type_`]**must** be a valid [`AccelerationStructureTypeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureCreateFlagsKHR`]
/// - [`AccelerationStructureTypeKHR`]
/// - [`Buffer`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`CreateAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`create_flags`] is a bitmask of
    ///[`AccelerationStructureCreateFlagBitsKHR`] specifying additional
    ///creation parameters of the acceleration structure.
    create_flags: AccelerationStructureCreateFlagsKHR,
    ///[`buffer`] is the buffer on which the acceleration structure will be
    ///stored.
    buffer: Buffer,
    ///[`offset`] is an offset in bytes from the base address of the buffer
    ///at which the acceleration structure will be stored, and **must** be a
    ///multiple of `256`.
    offset: DeviceSize,
    ///[`size`] is the size required for the acceleration structure.
    size: DeviceSize,
    ///[`type_`] is a [`AccelerationStructureTypeKHR`] value specifying
    ///the type of acceleration structure that will be created.
    type_: AccelerationStructureTypeKHR,
    ///[`device_address`] is the device address requested for the acceleration
    ///structure if the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay) feature is being used.
    device_address: DeviceAddress,
}
///[VkAabbPositionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html) - Structure specifying two opposing corners of an axis-aligned bounding box
///# C Specifications
///The [`AabbPositionsKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAabbPositionsKHR {
///    float    minX;
///    float    minY;
///    float    minZ;
///    float    maxX;
///    float    maxY;
///    float    maxZ;
///} VkAabbPositionsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAabbPositionsKHR VkAabbPositionsNV;
///```
///# Members
/// - [`min_x`] is the x position of one opposing corner of a bounding box.
/// - [`min_y`] is the y position of one opposing corner of a bounding box.
/// - [`min_z`] is the z position of one opposing corner of a bounding box.
/// - [`max_x`] is the x position of the other opposing corner of a bounding box.
/// - [`max_y`] is the y position of the other opposing corner of a bounding box.
/// - [`max_z`] is the z position of the other opposing corner of a bounding box.
///# Description
///Valid Usage
/// - [`min_x`]**must** be less than or equal to [`max_x`]
/// - [`min_y`]**must** be less than or equal to [`max_y`]
/// - [`min_z`]**must** be less than or equal to [`max_z`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AabbPositionsKHR {
    ///[`min_x`] is the x position of one opposing corner of a bounding box.
    min_x: f32,
    ///[`min_y`] is the y position of one opposing corner of a bounding box.
    min_y: f32,
    ///[`min_z`] is the z position of one opposing corner of a bounding box.
    min_z: f32,
    ///[`max_x`] is the x position of the other opposing corner of a bounding
    ///box.
    max_x: f32,
    ///[`max_y`] is the y position of the other opposing corner of a bounding
    ///box.
    max_y: f32,
    ///[`max_z`] is the z position of the other opposing corner of a bounding
    ///box.
    max_z: f32,
}
///[VkTransformMatrixKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html) - Structure specifying a 3x4 affine transformation matrix
///# C Specifications
///The [`TransformMatrixKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkTransformMatrixKHR {
///    float    matrix[3][4];
///} VkTransformMatrixKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkTransformMatrixKHR VkTransformMatrixNV;
///```
///# Members
/// - [`matrix`] is a 3x4 row-major affine transformation matrix.
///# Description
///Valid Usage
/// - The first three columns of [`matrix`]**must** define an invertible 3x3 matrix
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureInstanceKHR`]
/// - [`AccelerationStructureMatrixMotionInstanceNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TransformMatrixKHR {
    ///[`matrix`] is a 3x4 row-major affine transformation matrix.
    matrix: [f32; 3],
}
///[VkAccelerationStructureInstanceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html) - Structure specifying a single acceleration structure instance for building into an acceleration structure geometry
///# C Specifications
///*Acceleration structure instances***can** be built into top-level acceleration
///structures.
///Each acceleration structure instance is a separate entry in the top-level
///acceleration structure which includes all the geometry of a bottom-level
///acceleration structure at a transformed location.
///Multiple instances **can** point to the same bottom level acceleration
///structure.An acceleration structure instance is defined by the structure:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureInstanceKHR {
///    VkTransformMatrixKHR          transform;
///    uint32_t                      instanceCustomIndex:24;
///    uint32_t                      mask:8;
///    uint32_t                      instanceShaderBindingTableRecordOffset:24;
///    VkGeometryInstanceFlagsKHR    flags:8;
///    uint64_t                      accelerationStructureReference;
///} VkAccelerationStructureInstanceKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAccelerationStructureInstanceKHR VkAccelerationStructureInstanceNV;
///```
///# Members
/// - [`transform`] is a [`TransformMatrixKHR`] structure describing a transformation to be applied
///   to the acceleration structure.
/// - [`instance_custom_index`] is a 24-bit user-specified index value accessible to ray shaders in
///   the `InstanceCustomIndexKHR` built-in.
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance **may** only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`GetAccelerationStructureDeviceAddressKHR`] or
///   [`GetAccelerationStructureHandleNV`]      (used by device operations which reference
///   acceleration structures) or,  - a [`AccelerationStructureKHR`] object (used by host operations
///   which reference acceleration structures).
///# Description
///The C language specification does not define the ordering of bit-fields, but
///in practice, this struct produces the correct layout with existing
///compilers.
///The intended bit pattern is for the following:
/// - [`instance_custom_index`] and [`mask`] occupy the same memory as if a single `uint32_t` was
///   specified in their place  - [`instance_custom_index`] occupies the 24 least significant bits
///   of that memory  - [`mask`] occupies the 8 most significant bits of that memory
/// - [`instance_shader_binding_table_record_offset`] and [`flags`] occupy the same memory as if a
///   single `uint32_t` was specified in their place  -
///   [`instance_shader_binding_table_record_offset`] occupies the 24 least significant bits of that
///   memory  - [`flags`] occupies the 8 most significant bits of that memory
///If a compiler produces code that diverges from that pattern, applications
///**must** employ another method to set values according to the correct bit
///pattern.Valid Usage (Implicit)
/// - [`flags`]**must** be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureMotionInstanceDataNV`]
/// - [`GeometryInstanceFlagsKHR`]
/// - [`TransformMatrixKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    ///[`transform`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure.
    transform: TransformMatrixKHR,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance **may** only be hit if `Cull Mask & instance.mask != 0`
    mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`GetAccelerationStructureDeviceAddressKHR`] or [`GetAccelerationStructureHandleNV`]
    ///   (used by device operations which reference acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    acceleration_structure_reference: u64,
}
///[VkAccelerationStructureDeviceAddressInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) - Structure specifying the acceleration structure to query an address for
///# C Specifications
///The [`AccelerationStructureDeviceAddressInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureDeviceAddressInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkAccelerationStructureKHR    accelerationStructure;
///} VkAccelerationStructureDeviceAddressInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure`] specifies the acceleration structure whose address is being
///   queried.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`acceleration_structure`]**must** be a valid [`AccelerationStructureKHR`] handle
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`StructureType`]
/// - [`GetAccelerationStructureDeviceAddressKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`acceleration_structure`] specifies the acceleration structure whose
    ///address is being queried.
    acceleration_structure: AccelerationStructureKHR,
}
///[VkAccelerationStructureVersionInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) - Acceleration structure version information
///# C Specifications
///The [`AccelerationStructureVersionInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureVersionInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    const uint8_t*     pVersionData;
///} VkAccelerationStructureVersionInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_version_data`] is a pointer to the version header of an acceleration structure as defined
///   in [`CmdCopyAccelerationStructureToMemoryKHR`]
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`p_version_data`]**must** be a valid pointer to an array of <span class="katex"><span
///   class="katex-html" aria-hidden="true"><span class="base"><span
///   style="height:0.72777em;vertical-align:-0.08333em;" class="strut"></span><span
///   class="mord">2</span><span class="mspace"
///   style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span
///   class="mspace" style="margin-right:0.2222222222222222em;"></span></span><span
///   class="base"><span style="height:0.70625em;vertical-align:-0.09514em;"
///   class="strut"></span><span class="mord"><span class="mord mathtt">V</span><span class="mord
///   mathtt">K</span><span class="mord mathtt">_</span><span class="mord mathtt">U</span><span
///   class="mord mathtt">U</span><span class="mord mathtt">I</span><span class="mord
///   mathtt">D</span><span class="mord mathtt">_</span><span class="mord mathtt">S</span><span
///   class="mord mathtt">I</span><span class="mord mathtt">Z</span><span class="mord
///   mathtt">E</span></span></span></span></span>`uint8_t` values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`StructureType`]
/// - [`GetDeviceAccelerationStructureCompatibilityKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_version_data`] is a pointer to the version header of an acceleration
    ///structure as defined in [`CmdCopyAccelerationStructureToMemoryKHR`]
    p_version_data: *mut u8,
}
///[VkCopyAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) - Parameters for copying an acceleration structure
///# C Specifications
///The [`CopyAccelerationStructureInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyAccelerationStructureInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkAccelerationStructureKHR            src;
///    VkAccelerationStructureKHR            dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyAccelerationStructureInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the source acceleration structure for the copy.
/// - [`dst`] is the target acceleration structure for the copy.
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///Valid Usage
/// - [`mode`]**must** be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or
///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
/// - The source acceleration structure [`src`]**must** have been constructed prior to the execution
///   of this command
/// - If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`]**must** have been
///   constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the build
/// - The `buffer` used to create [`src`]**must** be bound to device memory
/// - The `buffer` used to create [`dst`]**must** be bound to device memory
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`src`]**must** be a valid [`AccelerationStructureKHR`] handle
/// - [`dst`]**must** be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`]**must** be a valid [`CopyAccelerationStructureModeKHR`] value
/// - Both of [`dst`], and [`src`]**must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`StructureType`]
/// - [`CmdCopyAccelerationStructureKHR`]
/// - [`CopyAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src`] is the source acceleration structure for the copy.
    src: AccelerationStructureKHR,
    ///[`dst`] is the target acceleration structure for the copy.
    dst: AccelerationStructureKHR,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    mode: CopyAccelerationStructureModeKHR,
}
///[VkCopyAccelerationStructureToMemoryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) - Parameters for serializing an acceleration structure
///# C Specifications
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyAccelerationStructureToMemoryInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkAccelerationStructureKHR            src;
///    VkDeviceOrHostAddressKHR              dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyAccelerationStructureToMemoryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the source acceleration structure for the copy
/// - [`dst`] is the device or host address to memory which is the target for the copy
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///Valid Usage
/// - The source acceleration structure [`src`]**must** have been constructed prior to the execution
///   of this command
/// - The memory pointed to by [`dst`]**must** be at least as large as the serialization size of
///   [`src`], as reported by [`WriteAccelerationStructuresPropertiesKHR`] or
///   [`CmdWriteAccelerationStructuresPropertiesKHR`] with a query type of
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
/// - [`mode`]**must** be `VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`src`]**must** be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`]**must** be a valid [`CopyAccelerationStructureModeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressKHR`]
/// - [`StructureType`]
/// - [`CmdCopyAccelerationStructureToMemoryKHR`]
/// - [`CopyAccelerationStructureToMemoryKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src`] is the source acceleration structure for the copy
    src: AccelerationStructureKHR,
    ///[`dst`] is the device or host address to memory which is the target
    ///for the copy
    dst: DeviceOrHostAddressKHR<'lt>,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    mode: CopyAccelerationStructureModeKHR,
}
///[VkCopyMemoryToAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) - Parameters for deserializing an acceleration structure
///# C Specifications
///The [`CopyMemoryToAccelerationStructureInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyMemoryToAccelerationStructureInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceOrHostAddressConstKHR         src;
///    VkAccelerationStructureKHR            dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyMemoryToAccelerationStructureInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the device or host address to memory containing the source data for the copy.
/// - [`dst`] is the target acceleration structure for the copy.
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///Valid Usage
/// - The source memory pointed to by [`src`]**must** contain data previously serialized using
///   [`CmdCopyAccelerationStructureToMemoryKHR`], potentially modified to relocate acceleration
///   structure references as described in that command
/// - [`mode`]**must** be `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR`
/// - The data in [`src`]**must** have a format compatible with the destination physical device as
///   returned by [`GetDeviceAccelerationStructureCompatibilityKHR`]
/// - [`dst`]**must** have been created with a `size` greater than or equal to that used to
///   serialize the data in [`src`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`dst`]**must** be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`]**must** be a valid [`CopyAccelerationStructureModeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`StructureType`]
/// - [`CmdCopyMemoryToAccelerationStructureKHR`]
/// - [`CopyMemoryToAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src`] is the device or host address to memory containing the source
    ///data for the copy.
    src: DeviceOrHostAddressConstKHR<'lt>,
    ///[`dst`] is the target acceleration structure for the copy.
    dst: AccelerationStructureKHR,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    mode: CopyAccelerationStructureModeKHR,
}
///[VkAccelerationStructureBuildSizesInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) - Structure specifying build sizes for an acceleration structure
///# C Specifications
///The [`AccelerationStructureBuildSizesInfoKHR`] structure describes the
///required build sizes for an acceleration structure and scratch buffers and
///is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildSizesInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceSize       accelerationStructureSize;
///    VkDeviceSize       updateScratchSize;
///    VkDeviceSize       buildScratchSize;
///} VkAccelerationStructureBuildSizesInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure_size`] is the size in bytes required in a
///   [`AccelerationStructureKHR`] for a build or update operation.
/// - [`update_scratch_size`] is the size in bytes required in a scratch buffer for an update
///   operation.
/// - [`build_scratch_size`] is the size in bytes required in a scratch buffer for a build
///   operation.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`acceleration_structure_size`] is the size in bytes required in a
    ///[`AccelerationStructureKHR`] for a build or update operation.
    acceleration_structure_size: DeviceSize,
    ///[`update_scratch_size`] is the size in bytes required in a scratch
    ///buffer for an update operation.
    update_scratch_size: DeviceSize,
    ///[`build_scratch_size`] is the size in bytes required in a scratch buffer
    ///for a build operation.
    build_scratch_size: DeviceSize,
}
