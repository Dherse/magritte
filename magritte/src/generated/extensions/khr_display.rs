use crate::{
    extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    vulkan1_0::{BaseInStructure, Bool32, Extent2D, Offset2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display");
///[VkDisplayPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPropertiesKHR.html) - Structure describing an available display device
///# C Specifications
///The [`DisplayPropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPropertiesKHR {
///    VkDisplayKHR                  display;
///    const char*                   displayName;
///    VkExtent2D                    physicalDimensions;
///    VkExtent2D                    physicalResolution;
///    VkSurfaceTransformFlagsKHR    supportedTransforms;
///    VkBool32                      planeReorderPossible;
///    VkBool32                      persistentContent;
///} VkDisplayPropertiesKHR;
///```
///# Members
/// - [`display`] is a handle that is used to refer to the display described here. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`display_name`] is `NULL` or a pointer to a null-terminated UTF-8 string containing the name
///   of the display. Generally, this will be the name provided by the display’s EDID. If `NULL`, no
///   suitable name is available. If not `NULL`, the string pointed to **must** remain accessible
///   and unmodified as long as [`display`] is valid.
/// - [`physical_dimensions`] describes the physical width and height of the visible portion of the
///   display, in millimeters.
/// - [`physical_resolution`] describes the physical, native, or preferred resolution of the
///   display.
///# Description
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] describing which
///   transforms are supported by this display.
/// - [`plane_reorder_possible`] tells whether the planes on this display **can** have their z order
///   changed. If this is [`TRUE`], the application **can** re-arrange the planes on this display in
///   any order relative to each other.
/// - [`persistent_content`] tells whether the display supports self-refresh/internal buffering. If
///   this is true, the application **can** submit persistent present operations on swapchains
///   created against this display.
///# Related
/// - [`VK_KHR_display`]
/// - [`Bool32`]
/// - [`DisplayKHR`]
/// - [`DisplayProperties2KHR`]
/// - [`Extent2D`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceDisplayPropertiesKHR`]
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
pub struct DisplayPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`display`] is a handle that is used to refer to the display described
    ///here.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    display: DisplayKHR,
    ///[`display_name`] is `NULL` or a pointer to a null-terminated UTF-8
    ///string containing the name of the display.
    ///Generally, this will be the name provided by the display’s EDID.
    ///If `NULL`, no suitable name is available.
    ///If not `NULL`, the string pointed to **must** remain accessible and
    ///unmodified as long as [`display`] is valid.
    display_name: &'lt CStr,
    ///[`physical_dimensions`] describes the physical width and height of the
    ///visible portion of the display, in millimeters.
    physical_dimensions: Extent2D,
    ///[`physical_resolution`] describes the physical, native, or preferred
    ///resolution of the display.
    physical_resolution: Extent2D,
    ///No documentation found
    supported_transforms: SurfaceTransformFlagsKHR,
    ///No documentation found
    plane_reorder_possible: Bool32,
    ///No documentation found
    persistent_content: Bool32,
}
///[VkDisplayPlanePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlanePropertiesKHR.html) - Structure describing display plane properties
///# C Specifications
///The [`DisplayPlanePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlanePropertiesKHR {
///    VkDisplayKHR    currentDisplay;
///    uint32_t        currentStackIndex;
///} VkDisplayPlanePropertiesKHR;
///```
///# Members
/// - [`current_display`] is the handle of the display the plane is currently associated with. If
///   the plane is not currently attached to any displays, this will be
///   [`crate::utils::Handle::null`].
/// - [`current_stack_index`] is the current z-order of the plane. This will be between 0 and the
///   value returned by [`GetPhysicalDeviceDisplayPlanePropertiesKHR`] in `pPropertyCount`.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayKHR`]
/// - [`DisplayPlaneProperties2KHR`]
/// - [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]
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
pub struct DisplayPlanePropertiesKHR {
    ///[`current_display`] is the handle of the display the plane is currently
    ///associated with.
    ///If the plane is not currently attached to any displays, this will be
    ///[`crate::utils::Handle::null`].
    current_display: DisplayKHR,
    ///[`current_stack_index`] is the current z-order of the plane.
    ///This will be between 0 and the value returned by
    ///[`GetPhysicalDeviceDisplayPlanePropertiesKHR`] in
    ///`pPropertyCount`.
    current_stack_index: u32,
}
///[VkDisplayModeParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeParametersKHR.html) - Structure describing display parameters associated with a display mode
///# C Specifications
///The [`DisplayModeParametersKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeParametersKHR {
///    VkExtent2D    visibleRegion;
///    uint32_t      refreshRate;
///} VkDisplayModeParametersKHR;
///```
///# Members
/// - [`visible_region`] is the 2D extents of the visible region.
/// - [`refresh_rate`] is a `uint32_t` that is the number of times the display is refreshed each
///   second multiplied by 1000.
///# Description
///Valid Usage
/// - The `width` member of [`visible_region`]**must** be greater than `0`
/// - The `height` member of [`visible_region`]**must** be greater than `0`
/// - [`refresh_rate`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateInfoKHR`]
/// - [`DisplayModePropertiesKHR`]
/// - [`Extent2D`]
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
pub struct DisplayModeParametersKHR {
    ///[`visible_region`] is the 2D extents of the visible region.
    visible_region: Extent2D,
    ///[`refresh_rate`] is a `uint32_t` that is the number of times the
    ///display is refreshed each second multiplied by 1000.
    refresh_rate: u32,
}
///[VkDisplayModePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModePropertiesKHR.html) - Structure describing display mode properties
///# C Specifications
///The [`DisplayModePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModePropertiesKHR {
///    VkDisplayModeKHR              displayMode;
///    VkDisplayModeParametersKHR    parameters;
///} VkDisplayModePropertiesKHR;
///```
///# Members
/// - [`display_mode`] is a handle to the display mode described in this structure. This handle will
///   be valid for the lifetime of the Vulkan instance.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   associated with [`display_mode`].
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`DisplayModeProperties2KHR`]
/// - [`GetDisplayModePropertiesKHR`]
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
pub struct DisplayModePropertiesKHR {
    ///[`display_mode`] is a handle to the display mode described in this
    ///structure.
    ///This handle will be valid for the lifetime of the Vulkan instance.
    display_mode: DisplayModeKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters associated with [`display_mode`].
    parameters: DisplayModeParametersKHR,
}
///[VkDisplayModeCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayModeCreateInfoKHR.html) - Structure specifying parameters of a newly created display mode object
///# C Specifications
///The [`DisplayModeCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayModeCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkDisplayModeCreateFlagsKHR    flags;
///    VkDisplayModeParametersKHR     parameters;
///} VkDisplayModeCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and **must** be zero.
/// - [`parameters`] is a [`DisplayModeParametersKHR`] structure describing the display parameters
///   to use in creating the new mode. If the parameters are not compatible with the specified
///   display, the implementation **must** return `VK_ERROR_INITIALIZATION_FAILED`.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`parameters`]**must** be a valid [`DisplayModeParametersKHR`] structure
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeCreateFlagsKHR`]
/// - [`DisplayModeParametersKHR`]
/// - [`StructureType`]
/// - [`CreateDisplayModeKHR`]
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
pub struct DisplayModeCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and **must** be zero.
    flags: DisplayModeCreateFlagsKHR,
    ///[`parameters`] is a [`DisplayModeParametersKHR`] structure
    ///describing the display parameters to use in creating the new mode.
    ///If the parameters are not compatible with the specified display, the
    ///implementation **must** return `VK_ERROR_INITIALIZATION_FAILED`.
    parameters: DisplayModeParametersKHR,
}
///[VkDisplayPlaneCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPlaneCapabilitiesKHR.html) - Structure describing capabilities of a mode and plane combination
///# C Specifications
///The [`DisplayPlaneCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplayPlaneCapabilitiesKHR {
///    VkDisplayPlaneAlphaFlagsKHR    supportedAlpha;
///    VkOffset2D                     minSrcPosition;
///    VkOffset2D                     maxSrcPosition;
///    VkExtent2D                     minSrcExtent;
///    VkExtent2D                     maxSrcExtent;
///    VkOffset2D                     minDstPosition;
///    VkOffset2D                     maxDstPosition;
///    VkExtent2D                     minDstExtent;
///    VkExtent2D                     maxDstExtent;
///} VkDisplayPlaneCapabilitiesKHR;
///```
///# Members
/// - [`supported_alpha`] is a bitmask of [`DisplayPlaneAlphaFlagBitsKHR`] describing the supported
///   alpha blending modes.
/// - [`min_src_position`] is the minimum source rectangle offset supported by this plane using the
///   specified mode.
/// - [`max_src_position`] is the maximum source rectangle offset supported by this plane using the
///   specified mode. The `x` and `y` components of [`max_src_position`]**must** each be greater
///   than or equal to the `x` and `y` components of [`min_src_position`], respectively.
/// - [`min_src_extent`] is the minimum source rectangle size supported by this plane using the
///   specified mode.
/// - [`max_src_extent`] is the maximum source rectangle size supported by this plane using the
///   specified mode.
/// - [`min_dst_position`], [`max_dst_position`], [`min_dst_extent`], [`max_dst_extent`] all have
///   similar semantics to their corresponding `*Src*` equivalents, but apply to the output region
///   within the mode rather than the input region within the source image. Unlike the `*Src*`
///   offsets, [`min_dst_position`] and [`max_dst_position`]**may** contain negative values.
///# Description
///The minimum and maximum position and extent fields describe the
///implementation limits, if any, as they apply to the specified display mode
///and plane.
///Vendors **may** support displaying a subset of a swapchain’s presentable images
///on the specified display plane.
///This is expressed by returning [`min_src_position`], [`max_src_position`],
///[`min_src_extent`], and [`max_src_extent`] values that indicate a range of
///possible positions and sizes which **may** be used to specify the region within
///the presentable images that source pixels will be read from when creating a
///swapchain on the specified display mode and plane.Vendors **may** also support mapping the
/// presentable images’ content to a
///subset or superset of the visible region in the specified display mode.
///This is expressed by returning [`min_dst_position`], [`max_dst_position`],
///[`min_dst_extent`] and [`max_dst_extent`] values that indicate a range of
///possible positions and sizes which **may** be used to describe the region
///within the display mode that the source pixels will be mapped to.Other vendors **may** support
/// only a 1-1 mapping between pixels in the
///presentable images and the display mode.
///This **may** be indicated by returning (0,0) for [`min_src_position`],
///[`max_src_position`], [`min_dst_position`], and [`max_dst_position`], and
///(display mode width, display mode height) for [`min_src_extent`],
///[`max_src_extent`], [`min_dst_extent`], and [`max_dst_extent`].The value
/// [`supported_alpha`]**must** contain at least one valid
///[`DisplayPlaneAlphaFlagBitsKHR`] bit.These values indicate the limits of the implementation’s
/// individual fields.
///Not all combinations of values within the offset and extent ranges returned
///in [`DisplayPlaneCapabilitiesKHR`] are guaranteed to be supported.
///Presentation requests specifying unsupported combinations **may** fail.
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayPlaneAlphaFlagsKHR`]
/// - [`DisplayPlaneCapabilities2KHR`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`GetDisplayPlaneCapabilitiesKHR`]
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
pub struct DisplayPlaneCapabilitiesKHR {
    ///[`supported_alpha`] is a bitmask of
    ///[`DisplayPlaneAlphaFlagBitsKHR`] describing the supported alpha
    ///blending modes.
    supported_alpha: DisplayPlaneAlphaFlagsKHR,
    ///[`min_src_position`] is the minimum source rectangle offset supported by
    ///this plane using the specified mode.
    min_src_position: Offset2D,
    ///[`max_src_position`] is the maximum source rectangle offset supported by
    ///this plane using the specified mode.
    ///The `x` and `y` components of [`max_src_position`]**must** each be
    ///greater than or equal to the `x` and `y` components of
    ///[`min_src_position`], respectively.
    max_src_position: Offset2D,
    ///[`min_src_extent`] is the minimum source rectangle size supported by
    ///this plane using the specified mode.
    min_src_extent: Extent2D,
    ///[`max_src_extent`] is the maximum source rectangle size supported by
    ///this plane using the specified mode.
    max_src_extent: Extent2D,
    ///[`min_dst_position`], [`max_dst_position`], [`min_dst_extent`],
    ///[`max_dst_extent`] all have similar semantics to their corresponding
    ///`*Src*` equivalents, but apply to the output region within the mode
    ///rather than the input region within the source image.
    ///Unlike the `*Src*` offsets, [`min_dst_position`] and
    ///[`max_dst_position`]**may** contain negative values.
    min_dst_position: Offset2D,
    ///No documentation found
    max_dst_position: Offset2D,
    ///No documentation found
    min_dst_extent: Extent2D,
    ///No documentation found
    max_dst_extent: Extent2D,
}
///[VkDisplaySurfaceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplaySurfaceCreateInfoKHR.html) - Structure specifying parameters of a newly created display plane surface object
///# C Specifications
///The [`DisplaySurfaceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display
///typedef struct VkDisplaySurfaceCreateInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkDisplaySurfaceCreateFlagsKHR    flags;
///    VkDisplayModeKHR                  displayMode;
///    uint32_t                          planeIndex;
///    uint32_t                          planeStackIndex;
///    VkSurfaceTransformFlagBitsKHR     transform;
///    float                             globalAlpha;
///    VkDisplayPlaneAlphaFlagBitsKHR    alphaMode;
///    VkExtent2D                        imageExtent;
///} VkDisplaySurfaceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use, and **must** be zero.
/// - [`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode to use when displaying
///   this surface.
/// - [`plane_index`] is the plane on which this surface appears.
/// - [`plane_stack_index`] is the z-order of the plane.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value specifying the transformation to
///   apply to images as part of the scanout operation.
/// - [`global_alpha`] is the global alpha value. This value is ignored if [`alpha_mode`] is not
///   `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
/// - [`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value specifying the type of alpha
///   blending to use.
/// - [`image_extent`] is the size of the presentable images to use with the surface.
///# Description
///Valid Usage
/// - [`plane_index`]**must** be less than the number of display planes supported by the device as
///   determined by calling [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]
/// - If the `planeReorderPossible` member of the [`DisplayPropertiesKHR`] structure returned by
///   [`GetPhysicalDeviceDisplayPropertiesKHR`] for the display corresponding to [`display_mode`] is
///   [`TRUE`] then [`plane_stack_index`]**must** be less than the number of display planes
///   supported by the device as determined by calling
///   [`GetPhysicalDeviceDisplayPlanePropertiesKHR`]; otherwise [`plane_stack_index`]**must** equal
///   the `currentStackIndex` member of [`DisplayPlanePropertiesKHR`] returned by
///   [`GetPhysicalDeviceDisplayPlanePropertiesKHR`] for the display plane corresponding to
///   [`display_mode`]
/// - If [`alpha_mode`] is `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR` then [`global_alpha`]**must** be
///   between `0` and `1`, inclusive
/// - [`alpha_mode`]**must** be one of the bits present in the `supportedAlpha` member of
///   [`DisplayPlaneCapabilitiesKHR`] for the display plane corresponding to [`display_mode`]
/// - The `width` and `height` members of [`image_extent`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_image_dimension_2_d`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`display_mode`]**must** be a valid [`DisplayModeKHR`] handle
/// - [`transform`]**must** be a valid [`SurfaceTransformFlagBitsKHR`] value
/// - [`alpha_mode`]**must** be a valid [`DisplayPlaneAlphaFlagBitsKHR`] value
///# Related
/// - [`VK_KHR_display`]
/// - [`DisplayModeKHR`]
/// - [`DisplayPlaneAlphaFlagBitsKHR`]
/// - [`DisplaySurfaceCreateFlagsKHR`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`CreateDisplayPlaneSurfaceKHR`]
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
pub struct DisplaySurfaceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use, and **must** be zero.
    flags: DisplaySurfaceCreateFlagsKHR,
    ///[`display_mode`] is a [`DisplayModeKHR`] handle specifying the mode
    ///to use when displaying this surface.
    display_mode: DisplayModeKHR,
    ///[`plane_index`] is the plane on which this surface appears.
    plane_index: u32,
    ///[`plane_stack_index`] is the z-order of the plane.
    plane_stack_index: u32,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///specifying the transformation to apply to images as part of the scanout
    ///operation.
    transform: SurfaceTransformFlagBitsKHR,
    ///[`global_alpha`] is the global alpha value.
    ///This value is ignored if [`alpha_mode`] is not
    ///`VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`.
    global_alpha: f32,
    ///[`alpha_mode`] is a [`DisplayPlaneAlphaFlagBitsKHR`] value
    ///specifying the type of alpha blending to use.
    alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    ///[`image_extent`] is the size of the presentable images to use with the
    ///surface.
    image_extent: Extent2D,
}
