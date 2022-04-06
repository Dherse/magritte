//![VK_NV_shader_image_footprint](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_image_footprint.html) - device extension
//!# Description
//!This extension adds Vulkan support for the
//![`SPV_NV_shader_image_footprint`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_image_footprint.html)
//!SPIR-V extension.
//!That SPIR-V extension provides a new instruction
//!`OpImageSampleFootprintNV` allowing shaders to determine the set of
//!texels that would be accessed by an equivalent filtered texture lookup.Instead of returning a
//! filtered texture value, the instruction returns a
//!structure that can be interpreted by shader code to determine the footprint
//!of a filtered texture lookup.
//!This structure includes integer values that identify a small neighborhood of
//!texels in the image being accessed and a bitfield that indicates which
//!texels in that neighborhood would be used.
//!The structure also includes a bitfield where each bit identifies whether any
//!texel in a small aligned block of texels would be fetched by the texture
//!lookup.
//!The size of each block is specified by an access *granularity* provided by
//!the shader.
//!The minimum granularity supported by this extension is 2x2 (for 2D textures)
//!and 2x2x2 (for 3D textures); the maximum granularity is 256x256 (for 2D
//!textures) or 64x32x32 (for 3D textures).
//!Each footprint query returns the footprint from a single texture level.
//!When using minification filters that combine accesses from multiple mipmap
//!levels, shaders must perform separate queries for the two levels accessed
//!(“fine” and “coarse”).
//!The footprint query also returns a flag indicating if the texture lookup
//!would access texels from only one mipmap level or from two neighboring
//!levels.This extension should be useful for multi-pass rendering operations that do
//!an initial expensive rendering pass to produce a first image that is then
//!used as a texture for a second pass.
//!If the second pass ends up accessing only portions of the first image (e.g.,
//!due to visbility), the work spent rendering the non-accessed portion of the
//!first image was wasted.
//!With this feature, an application can limit this waste using an initial pass
//!over the geometry in the second image that performs a footprint query for
//!each visible pixel to determine the set of pixels that it needs from the
//!first image.
//!This pass would accumulate an aggregate footprint of all visible pixels into
//!a separate “footprint image” using shader atomics.
//!Then, when rendering the first image, the application can kill all shading
//!work for pixels not in this aggregate footprint.This extension has a number of limitations.
//!The `OpImageSampleFootprintNV` instruction only supports for two- and
//!three-dimensional textures.
//!Footprint evaluation only supports the CLAMP_TO_EDGE wrap mode; results are
//!undefined for all other wrap modes.
//!Only a limited set of granularity values and that set does not support
//!separate coverage information for each texel in the original image.When using SPIR-V generated
//! from the OpenGL Shading Language, the new
//!instruction will be generated from code using the new
//!`textureFootprint*NV` built-in functions from the
//!`GL_NV_shader_texture_footprint` shading language extension.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shader_image_footprint]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the
//!   VK_NV_shader_image_footprint extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderImageFootprintFeaturesNV`]
//!# New constants
//! - [`NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME`]
//! - [`NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV`
//!# Known issues & F.A.Q
//!(1) The footprint returned by the SPIR-V instruction is a structure that
//!    includes an anchor, an offset, and a mask that represents a 8x8 or 4x4x4
//!    neighborhood of texel groups.
//!    But the bits of the mask are not stored in simple pitch order.
//!    Why is the footprint built this way? **RESOLVED** : We expect that applications using this
//! feature will want to use
//!a fixed granularity and accumulate coverage information from the returned
//!footprints into an aggregate “footprint image” that tracks the portions of
//!an image that would be needed by regular texture filtering.
//!If an application is using a two-dimensional image with 4x4 pixel
//!granularity, we expect that the footprint image will use 64-bit texels where
//!each bit in an 8x8 array of bits corresponds to coverage for a 4x4 block in
//!the original image.
//!Texel (0,0) in the footprint image would correspond to texels (0,0) through
//!(31,31) in the original image.In the usual case, the footprint for a single access will fully
//! contained in
//!a 32x32 aligned region of the original texture, which corresponds to a
//!single 64-bit texel in the footprint image.
//!In that case, the implementation will return an anchor coordinate pointing
//!at the single footprint image texel, an offset vector of (0,0), and a mask
//!whose bits are aligned with the bits in the footprint texel.
//!For this case, the shader can simply atomically OR the mask bits into the
//!contents of the footprint texel to accumulate footprint coverage.In the worst case, the
//! footprint for a single access spans multiple 32x32
//!aligned regions and may require updates to four separate footprint image
//!texels.
//!In this case, the implementation will return an anchor coordinate pointing
//!at the lower right footprint image texel and an offset will identify how
//!many “columns” and “rows” of the returned 8x8 mask correspond to
//!footprint texels to the left and above the anchor texel.
//!If the anchor is (2,3), the 64 bits of the returned mask are arranged
//!spatially as follows, where each 4x4 block is assigned a bit number that
//!matches its bit number in the footprint image texels:
//!```c
//!    +-------------------------+-------------------------+
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- 46 47 | 40 41 42 43 44 45 -- -- |
//!    | -- -- -- -- -- -- 54 55 | 48 49 50 51 52 53 -- -- |
//!    | -- -- -- -- -- -- 62 63 | 56 57 58 59 60 61 -- -- |
//!    +-------------------------+-------------------------+
//!    | -- -- -- -- -- -- 06 07 | 00 01 02 03 04 05 -- -- |
//!    | -- -- -- -- -- -- 14 15 | 08 09 10 11 12 13 -- -- |
//!    | -- -- -- -- -- -- 22 23 | 16 17 18 19 20 21 -- -- |
//!    | -- -- -- -- -- -- 30 31 | 24 25 26 27 28 29 -- -- |
//!    | -- -- -- -- -- -- 38 39 | 32 33 34 35 36 37 -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    | -- -- -- -- -- -- -- -- | -- -- -- -- -- -- -- -- |
//!    +-------------------------+-------------------------+
//!```
//!To accumulate coverage for each of the four footprint image texels, a shader
//!can AND the returned mask with simple masks derived from the x and y offset
//!values and then atomically OR the updated mask bits into the contents of the
//!corresponding footprint texel.
//!```c
//!    uint64_t returnedMask = (uint64_t(footprint.mask.x) | (uint64_t(footprint.mask.y) link:https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html# 32));
//!    uint64_t rightMask    = ((0xFF [^] footprint.offset.x) * 0x0101010101010101UL);
//!    uint64_t bottomMask   = 0xFFFFFFFFFFFFFFFFUL >> (8 * footprint.offset.y);
//!    uint64_t bottomRight  = returnedMask & bottomMask & rightMask;
//!    uint64_t bottomLeft   = returnedMask & bottomMask & (~rightMask);
//!    uint64_t topRight     = returnedMask & (~bottomMask) & rightMask;
//!    uint64_t topLeft      = returnedMask & (~bottomMask) & (~rightMask);
//!```
//!(2) What should an application do to ensure maximum performance when
//!accumulating footprints into an aggregate footprint image? **RESOLVED** : We expect that the
//! most common usage of this feature will be to
//!accumulate aggregate footprint coverage, as described in the previous issue.
//!Even if you ignore the anisotropic filtering case where the implementation
//!may return a granularity larger than that requested by the caller, each
//!shader invocation will need to use atomic functions to update up to four
//!footprint image texels for each level of detail accessed.
//!Having each active shader invocation perform multiple atomic operations can
//!be expensive, particularly when neighboring invocations will want to update
//!the same footprint image texels.Techniques can be used to reduce the number of atomic operations
//! performed
//!when accumulating coverage include:
//! - Have logic that detects returned footprints where all components of the returned offset vector
//!   are zero. In that case, the mask returned by the footprint function is guaranteed to be
//!   aligned with the footprint image texels and affects only a single footprint image texel.
//! - Have fragment shaders communicate using built-in functions from the
//!   [`VK_NV_shader_subgroup_partitioned`] extension or other shader subgroup extensions. If you
//!   have multiple invocations in a subgroup that need to update the same texel (x,y) in the
//!   footprint image, compute an aggregate footprint mask across all invocations in the subgroup
//!   updating that texel and have a single invocation perform an atomic operation using that
//!   aggregate mask.
//! - When the returned footprint spans multiple texels in the footprint image, each invocation need
//!   to perform four atomic operations. In the previous issue, we had an example that computed
//!   separate masks for “topLeft”, “topRight”, “bottomLeft”, and “bottomRight”. When the
//!   invocations in a subgroup have good locality, it might be the case the “top left” for some
//!   invocations might refer to footprint image texel (10,10), while neighbors might have their
//!   “top left” texels at (11,10), (10,11), and (11,11). If you compute separate masks for even/odd
//!   x and y values instead of left/right or top/bottom, the “odd/odd” mask for all invocations in
//!   the subgroup hold coverage for footprint image texel (11,11), which can be updated by a single
//!   atomic operation for the entire subgroup.
//!# Version History
//! - Revision 2, 2018-09-13 (Pat Brown)  - Add issue (2) with performance tips.
//! - Revision 1, 2018-08-12 (Pat Brown)  - Initial draft
//!# Other info
//! * 2018-09-13
//! * No known IP claims.
//! * - This extension requires [`SPV_NV_shader_image_footprint`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_image_footprint.html)
//!   - This extension provides API support for [`GL_NV_shader_texture_footprint`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shader_texture_footprint.txt)
//! * - Pat Brown, NVIDIA  - Chris Lentini, NVIDIA  - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderImageFootprintFeaturesNV`]
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
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION")]
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME")]
pub const NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_shader_image_footprint");
///[VkPhysicalDeviceShaderImageFootprintFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageFootprintFeaturesNV.html) - Structure describing shader image footprint features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderImageFootprintFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_shader_image_footprint
///typedef struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           imageFootprint;
///} VkPhysicalDeviceShaderImageFootprintFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_footprint`] specifies whether the implementation supports the `ImageFootprintNV`
///   SPIR-V capability.
///See [Texel Footprint Evaluation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-footprint) for more information.If the [`PhysicalDeviceShaderImageFootprintFeaturesNV`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderImageFootprintFeaturesNV`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV`
///# Related
/// - [`VK_NV_shader_image_footprint`]
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
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`image_footprint`] specifies whether the
    ///implementation supports the `ImageFootprintNV` SPIR-V capability.
    pub image_footprint: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderImageFootprintFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            image_footprint: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderImageFootprintFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::image_footprint`]
    pub fn image_footprint_raw(&self) -> Bool32 {
        self.image_footprint
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::image_footprint`]
    pub fn set_image_footprint_raw(mut self, value: Bool32) -> Self {
        self.image_footprint = value;
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
    ///Gets the value of [`Self::image_footprint`]
    pub fn image_footprint(&self) -> bool {
        unsafe { std::mem::transmute(self.image_footprint as u8) }
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
    ///Gets a mutable reference to the value of [`Self::image_footprint`]
    pub fn image_footprint_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.image_footprint as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.image_footprint as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::image_footprint`]
    pub fn set_image_footprint(mut self, value: bool) -> Self {
        self.image_footprint = value as u8 as u32;
        self
    }
}
