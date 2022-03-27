use crate::{
    native::GgpStreamDescriptor,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_GGP_stream_descriptor_surface");
///[VkStreamDescriptorSurfaceCreateInfoGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateInfoGGP.html) - Structure specifying parameters of a newly created Google Games Platform stream surface object
///# C Specifications
///The [`StreamDescriptorSurfaceCreateInfoGGP`] structure is defined as:
///```c
///// Provided by VK_GGP_stream_descriptor_surface
///typedef struct VkStreamDescriptorSurfaceCreateInfoGGP {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    VkStreamDescriptorSurfaceCreateFlagsGGP    flags;
///    GgpStreamDescriptor                        streamDescriptor;
///} VkStreamDescriptorSurfaceCreateInfoGGP;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`stream_descriptor`] is a [`GgpStreamDescriptor`] referring to the GGP stream descriptor to
///   associate with the surface.
///# Description
///Valid Usage
/// - [`stream_descriptor`]**must** be a valid [`GgpStreamDescriptor`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_GGP_stream_descriptor_surface`]
/// - [`StreamDescriptorSurfaceCreateFlagsGGP`]
/// - [`StructureType`]
/// - [`CreateStreamDescriptorSurfaceGGP`]
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
pub struct StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: StreamDescriptorSurfaceCreateFlagsGGP,
    ///[`stream_descriptor`] is a [`GgpStreamDescriptor`] referring to the
    ///GGP stream descriptor to associate with the surface.
    stream_descriptor: GgpStreamDescriptor,
}
