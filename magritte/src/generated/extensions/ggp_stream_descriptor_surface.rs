//![VK_GGP_stream_descriptor_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GGP_stream_descriptor_surface.html) - instance extension
//!# Description
//!The [`VK_GGP_stream_descriptor_surface`] extension is an instance extension.
//!It provides a mechanism to create a [`SurfaceKHR`] object (defined by
//!the `[`VK_KHR_surface`]` extension) that refers to a Google Games
//!Platform [`GgpStreamDescriptor`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jean-Francois Roy [jfroy](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GGP_stream_descriptor_surface]
//!   @jfroy%0A<<Here describe the issue or question you have about the
//!   VK_GGP_stream_descriptor_surface extension>>)
//!# New functions & commands
//! - [`CreateStreamDescriptorSurfaceGGP`]
//!# New structures
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//!# New bitmasks
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//!# New constants
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME`]
//! - [`GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
//!# Version History
//! - Revision 1, 2018-11-26 (Jean-Francois Roy)  - Initial revision.
//!# Other info
//! * 2019-01-28
//! * No known IP claims.
//! * - Jean-Francois Roy, Google  - Brad Grantham, Google  - Connor Smith, Google  - Cort Stratton,
//!   Google  - Hai Nguyen, Google  - Ian Elliott, Google  - Jesse Hall, Google  - Jim Ray, Google
//!   - Katherine Wu, Google  - Kaye Mason, Google  - Kuangye Guo, Google  - Mark Segal, Google  -
//!   Nicholas Vining, Google  - Paul Lalonde, Google  - Richard O’Grady, Google
//!# Related
//! - [`StreamDescriptorSurfaceCreateFlagsGGP`]
//! - [`StreamDescriptorSurfaceCreateInfoGGP`]
//! - [`CreateStreamDescriptorSurfaceGGP`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///[VkStreamDescriptorSurfaceCreateFlagsGGP](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStreamDescriptorSurfaceCreateFlagsGGP.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_GGP_stream_descriptor_surface
///typedef VkFlags VkStreamDescriptorSurfaceCreateFlagsGGP;
///```
///# Related
/// - [`VK_GGP_stream_descriptor_surface`]
/// - [`StreamDescriptorSurfaceCreateInfoGGP`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct StreamDescriptorSurfaceCreateFlagsGGP(u32);
impl const Default for StreamDescriptorSurfaceCreateFlagsGGP {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for StreamDescriptorSurfaceCreateFlagsGGP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(StreamDescriptorSurfaceCreateFlagsGGP))
            .field(&self.0)
            .finish()
    }
}
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
///## Valid Usage
/// - [`stream_descriptor`] **must**  be a valid [`GgpStreamDescriptor`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
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
#[doc(alias = "VkStreamDescriptorSurfaceCreateInfoGGP")]
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    ///[`stream_descriptor`] is a [`GgpStreamDescriptor`] referring to the
    ///GGP stream descriptor to associate with the surface.
    pub stream_descriptor: GgpStreamDescriptor,
}
impl<'lt> Default for StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            stream_descriptor: Default::default(),
        }
    }
}
impl<'lt> StreamDescriptorSurfaceCreateInfoGGP<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stream_descriptor`]
    pub fn stream_descriptor_raw(&self) -> &GgpStreamDescriptor {
        &self.stream_descriptor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stream_descriptor`]
    pub fn set_stream_descriptor_raw(&mut self, value: GgpStreamDescriptor) -> &mut Self {
        self.stream_descriptor = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> StreamDescriptorSurfaceCreateFlagsGGP {
        self.flags
    }
    ///Gets the value of [`Self::stream_descriptor`]
    pub fn stream_descriptor(&self) -> &GgpStreamDescriptor {
        &self.stream_descriptor
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut StreamDescriptorSurfaceCreateFlagsGGP {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::stream_descriptor`]
    pub fn stream_descriptor_mut(&mut self) -> &mut GgpStreamDescriptor {
        &mut self.stream_descriptor
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ggp_stream_descriptor_surface::StreamDescriptorSurfaceCreateFlagsGGP,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::stream_descriptor`]
    pub fn set_stream_descriptor(&mut self, value: crate::native::GgpStreamDescriptor) -> &mut Self {
        self.stream_descriptor = value;
        self
    }
}
