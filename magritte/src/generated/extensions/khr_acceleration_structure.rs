#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`COPY_ACCELERATION_STRUCTURE_MODE_CLONE`] creates a direct
///copy of the acceleration structure specified in `src` into the one
///specified by `dst`.
///The `dst` acceleration structure **must** have been created with the
///same parameters as `src`.
///If `src` contains references to other acceleration structures,
///`dst` will reference the same acceleration structures.
/// - [`COPY_ACCELERATION_STRUCTURE_MODE_COMPACT`] creates a more
///compact version of an acceleration structure `src` into `dst`.
///The acceleration structure `dst`**must** have been created with a size
///at least as large as that returned by
///[`CmdWriteAccelerationStructuresPropertiesKHR`]
///or [`WriteAccelerationStructuresPropertiesKHR`]
///after the build of the acceleration structure specified by `src`.
///If `src` contains references to other acceleration structures,
///`dst` will reference the same acceleration structures.
/// - [`COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE`] serializes the
///acceleration structure to a semi-opaque format which can be reloaded on
///a compatible implementation.
/// - [`COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE`] deserializes
///the semi-opaque serialization format in the buffer to the acceleration
///structure.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CopyAccelerationStructureModeKHR(i32);
impl const Default for CopyAccelerationStructureModeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for CopyAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("CopyAccelerationStructureModeKHR")
            .field(match *self {
                Self::COPY_ACCELERATION_STRUCTURE_MODE_CLONE => &"COPY_ACCELERATION_STRUCTURE_MODE_CLONE",
                Self::COPY_ACCELERATION_STRUCTURE_MODE_COMPACT => &"COPY_ACCELERATION_STRUCTURE_MODE_COMPACT",
                Self::COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE => &"COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE",
                Self::COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE => &"COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE",
                other => unreachable!("invalid value for `CopyAccelerationStructureModeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl CopyAccelerationStructureModeKHR {
    ///[`COPY_ACCELERATION_STRUCTURE_MODE_CLONE`] creates a direct
    ///copy of the acceleration structure specified in `src` into the one
    ///specified by `dst`.
    ///The `dst` acceleration structure **must** have been created with the
    ///same parameters as `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    pub const COPY_ACCELERATION_STRUCTURE_MODE_CLONE: Self = Self(0);
    ///[`COPY_ACCELERATION_STRUCTURE_MODE_COMPACT`] creates a more
    ///compact version of an acceleration structure `src` into `dst`.
    ///The acceleration structure `dst`**must** have been created with a size
    ///at least as large as that returned by
    ///[`CmdWriteAccelerationStructuresPropertiesKHR`]
    ///or [`WriteAccelerationStructuresPropertiesKHR`]
    ///after the build of the acceleration structure specified by `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    pub const COPY_ACCELERATION_STRUCTURE_MODE_COMPACT: Self = Self(1);
    ///[`COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE`] serializes the
    ///acceleration structure to a semi-opaque format which can be reloaded on
    ///a compatible implementation.
    pub const COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE: Self = Self(2);
    ///[`COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE`] deserializes
    ///the semi-opaque serialization format in the buffer to the acceleration
    ///structure.
    pub const COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE: Self = Self(3);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV: Self = Self::COPY_ACCELERATION_STRUCTURE_MODE_CLONE;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV: Self = Self::COPY_ACCELERATION_STRUCTURE_MODE_COMPACT;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`BUILD_ACCELERATION_STRUCTURE_MODE_BUILD`] specifies that the
///destination acceleration structure will be built using the specified
///geometries.
/// - [`BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE`] specifies that the
///destination acceleration structure will be built using data in a source
///acceleration structure, updated by the specified geometries.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BuildAccelerationStructureModeKHR(i32);
impl const Default for BuildAccelerationStructureModeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for BuildAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("BuildAccelerationStructureModeKHR")
            .field(match *self {
                Self::BUILD_ACCELERATION_STRUCTURE_MODE_BUILD => &"BUILD_ACCELERATION_STRUCTURE_MODE_BUILD",
                Self::BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE => &"BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE",
                other => unreachable!("invalid value for `BuildAccelerationStructureModeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl BuildAccelerationStructureModeKHR {
    ///[`BUILD_ACCELERATION_STRUCTURE_MODE_BUILD`] specifies that the
    ///destination acceleration structure will be built using the specified
    ///geometries.
    pub const BUILD_ACCELERATION_STRUCTURE_MODE_BUILD: Self = Self(0);
    ///[`BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE`] specifies that the
    ///destination acceleration structure will be built using data in a source
    ///acceleration structure, updated by the specified geometries.
    pub const BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL`] is a top-level
///acceleration structure containing instance data referring to
///bottom-level acceleration structures.
/// - [`ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL`] is a bottom-level
///acceleration structure containing the AABBs or geometry to be
///intersected.
/// - [`ACCELERATION_STRUCTURE_TYPE_GENERIC`] is an acceleration
///structure whose type is determined at build time used for special
///circumstances.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureTypeKHR(i32);
impl const Default for AccelerationStructureTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureTypeKHR")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL => &"ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL",
                Self::ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL => &"ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL",
                Self::ACCELERATION_STRUCTURE_TYPE_GENERIC => &"ACCELERATION_STRUCTURE_TYPE_GENERIC",
                other => unreachable!("invalid value for `AccelerationStructureTypeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl AccelerationStructureTypeKHR {
    ///[`ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL`] is a top-level
    ///acceleration structure containing instance data referring to
    ///bottom-level acceleration structures.
    pub const ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL`] is a bottom-level
    ///acceleration structure containing the AABBs or geometry to be
    ///intersected.
    pub const ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL: Self = Self(1);
    ///[`ACCELERATION_STRUCTURE_TYPE_GENERIC`] is an acceleration
    ///structure whose type is determined at build time used for special
    ///circumstances.
    pub const ACCELERATION_STRUCTURE_TYPE_GENERIC: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV: Self = Self::ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV: Self = Self::ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`GEOMETRY_TYPE_TRIANGLES`] specifies a geometry type
///consisting of triangles.
/// - [`GEOMETRY_TYPE_AABBS`] specifies a geometry type consisting of
///axis-aligned bounding boxes.
/// - [`GEOMETRY_TYPE_INSTANCES`] specifies a geometry type
///consisting of acceleration structure instances.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct GeometryTypeKHR(i32);
impl const Default for GeometryTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for GeometryTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("GeometryTypeKHR")
            .field(match *self {
                Self::GEOMETRY_TYPE_TRIANGLES => &"GEOMETRY_TYPE_TRIANGLES",
                Self::GEOMETRY_TYPE_AABBS => &"GEOMETRY_TYPE_AABBS",
                Self::GEOMETRY_TYPE_INSTANCES => &"GEOMETRY_TYPE_INSTANCES",
                other => unreachable!("invalid value for `GeometryTypeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl GeometryTypeKHR {
    ///[`GEOMETRY_TYPE_TRIANGLES`] specifies a geometry type
    ///consisting of triangles.
    pub const GEOMETRY_TYPE_TRIANGLES: Self = Self(0);
    ///[`GEOMETRY_TYPE_AABBS`] specifies a geometry type consisting of
    ///axis-aligned bounding boxes.
    pub const GEOMETRY_TYPE_AABBS: Self = Self(1);
    ///[`GEOMETRY_TYPE_INSTANCES`] specifies a geometry type
    ///consisting of acceleration structure instances.
    pub const GEOMETRY_TYPE_INSTANCES: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const GEOMETRY_TYPE_TRIANGLES_NV: Self = Self::GEOMETRY_TYPE_TRIANGLES;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing`]
    pub const GEOMETRY_TYPE_AABBS_NV: Self = Self::GEOMETRY_TYPE_AABBS;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`ACCELERATION_STRUCTURE_BUILD_TYPE_HOST`] requests the memory
///requirement for operations performed by the host.
/// - [`ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE`] requests the
///memory requirement for operations performed by the device.
/// - [`ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE`] requests
///the memory requirement for operations performed by either the host, or
///the device.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildTypeKHR(i32);
impl const Default for AccelerationStructureBuildTypeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureBuildTypeKHR")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_BUILD_TYPE_HOST => &"ACCELERATION_STRUCTURE_BUILD_TYPE_HOST",
                Self::ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE => &"ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE",
                Self::ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE => {
                    &"ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE"
                },
                other => unreachable!("invalid value for `AccelerationStructureBuildTypeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl AccelerationStructureBuildTypeKHR {
    ///[`ACCELERATION_STRUCTURE_BUILD_TYPE_HOST`] requests the memory
    ///requirement for operations performed by the host.
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE`] requests the
    ///memory requirement for operations performed by the device.
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE: Self = Self(1);
    ///[`ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE`] requests
    ///the memory requirement for operations performed by either the host, or
    ///the device.
    pub const ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE`] if the
///`pVersionData` version acceleration structure is compatible with
///`device`.
/// - [`ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE`] if the
///`pVersionData` version acceleration structure is not compatible with
///`device`.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureCompatibilityKHR(i32);
impl const Default for AccelerationStructureCompatibilityKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for AccelerationStructureCompatibilityKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("AccelerationStructureCompatibilityKHR")
            .field(match *self {
                Self::ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE => {
                    &"ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE"
                },
                Self::ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE => {
                    &"ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE"
                },
                other => unreachable!("invalid value for `AccelerationStructureCompatibilityKHR`: {:?}", other),
            })
            .finish()
    }
}
impl AccelerationStructureCompatibilityKHR {
    ///[`ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE`] if the
    ///`pVersionData` version acceleration structure is compatible with
    ///`device`.
    pub const ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE: Self = Self(0);
    ///[`ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE`] if the
    ///`pVersionData` version acceleration structure is not compatible with
    ///`device`.
    pub const ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
