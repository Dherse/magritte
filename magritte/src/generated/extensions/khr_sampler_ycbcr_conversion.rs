//![VK_KHR_sampler_ycbcr_conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_sampler_ycbcr_conversion.html) - device extension
//!# Description
//!The use of Y′C<sub>B</sub>C<sub>R</sub> sampler conversion is an area in 3D graphics not used by
//!most Vulkan developers.
//!It is mainly used for processing inputs from video decoders and cameras.
//!The use of the extension assumes basic knowledge of Y′C<sub>B</sub>C<sub>R</sub> concepts.This
//! extension provides the ability to perform specified color space
//!conversions during texture sampling operations for the Y′C<sub>B</sub>C<sub>R</sub> color space
//!natively.
//!It also adds a selection of multi-planar formats, image aspect plane, and
//!the ability to bind memory to the planes of an image collectively or
//!separately.
# ! [doc = concat ! ("# " , "Revision")]
//!14
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_maintenance1`]`
//! - Requires `[`khr_bind_memory2`]`
//! - Requires `[`khr_get_memory_requirements2`]`
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Deprecation State")]
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
# ! [doc = concat ! ("# " , "Contacts")]
//! - Andrew Garrard [fluppeteer](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_sampler_ycbcr_conversion]
//!   @fluppeteer%0A<<Here describe the issue or question you have about the
//!   VK_KHR_sampler_ycbcr_conversion extension>>)
# ! [doc = concat ! ("# " , "New object types")]
//! - [`SamplerYcbcrConversionKHR`]
# ! [doc = concat ! ("# " , "New commands")]
//! - [`create_sampler_ycbcr_conversion_khr`]
//! - [`destroy_sampler_ycbcr_conversion_khr`]
# ! [doc = concat ! ("# " , "New structures")]
//! - [`SamplerYcbcrConversionCreateInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImagePlaneMemoryInfoKHR`]
//! - Extending [`ImageFormatProperties2`]:  - [`SamplerYcbcrConversionImageFormatPropertiesKHR`]
//! - Extending [`ImageMemoryRequirementsInfo2`]:  - [`ImagePlaneMemoryRequirementsInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSamplerYcbcrConversionFeaturesKHR`]
//! - Extending [`SamplerCreateInfo`], [`ImageViewCreateInfo`]:  - [`SamplerYcbcrConversionInfoKHR`]
# ! [doc = concat ! ("# " , "New enums")]
//! - [`ChromaLocationKHR`]
//! - [`SamplerYcbcrModelConversionKHR`]
//! - [`SamplerYcbcrRangeKHR`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME`]
//! - [`KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION`]
//! - Extending [`ChromaLocation`]:  - `VK_CHROMA_LOCATION_COSITED_EVEN_KHR`  -
//!   `VK_CHROMA_LOCATION_MIDPOINT_KHR`
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT`
//! - Extending [`Format`]:  - `VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR`  -
//!   `VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR`  -
//!   `VK_FORMAT_B16G16R16G16_422_UNORM_KHR`  - `VK_FORMAT_B8G8R8G8_422_UNORM_KHR`  -
//!   `VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR`  -
//!   `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR`  -
//!   `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR`  -
//!   `VK_FORMAT_G16B16G16R16_422_UNORM_KHR`  - `VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR`  -
//!   `VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR`  - `VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR`  -
//!   `VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR`  - `VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR`
//!   - `VK_FORMAT_G8B8G8R8_422_UNORM_KHR`  - `VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR`  -
//!   `VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR`  - `VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR`  -
//!   `VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR`  - `VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR`  -
//!   `VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR`  - `VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR`
//!   - `VK_FORMAT_R10X6_UNORM_PACK16_KHR`  - `VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR`  -
//!   `VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR`  - `VK_FORMAT_R12X4_UNORM_PACK16_KHR`
//! - Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_DISJOINT_BIT_KHR`  - `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR`
//!   - `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR`  -
//!   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR`
//! - Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_PLANE_0_BIT_KHR`  -
//!   `VK_IMAGE_ASPECT_PLANE_1_BIT_KHR`  - `VK_IMAGE_ASPECT_PLANE_2_BIT_KHR`
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_DISJOINT_BIT_KHR`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR`
//! - Extending [`SamplerYcbcrModelConversion`]:  -
//!   `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR`  -
//!   `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR`  -
//!   `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR`  -
//!   `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR`  -
//!   `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR`
//! - Extending [`SamplerYcbcrRange`]:  - `VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR`  -
//!   `VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR`
//!If [`ext_debug_report`] is supported:
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT`
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2017-01-24 (Andrew Garrard)  - Initial draft
//! - Revision 2, 2017-01-25 (Andrew Garrard)  - After initial feedback
//! - Revision 3, 2017-01-27 (Andrew Garrard)  - Higher bit depth formats, renaming, swizzle
//! - Revision 4, 2017-02-22 (Andrew Garrard)  - Added query function, formats as RGB,
//!   clarifications
//! - Revision 5, 2017-04-?? (Andrew Garrard)  - Simplified query and removed output conversions
//! - Revision 6, 2017-04-24 (Andrew Garrard)  - Tidying, incorporated new image query, restored
//!   transfer functions
//! - Revision 7, 2017-04-25 (Andrew Garrard)  - Added cosited option/midpoint requirement for
//!   formats, “bypassConversion”
//! - Revision 8, 2017-04-25 (Andrew Garrard)  - Simplified further
//! - Revision 9, 2017-04-27 (Andrew Garrard)  - Disjoint no more
//! - Revision 10, 2017-04-28 (Andrew Garrard)  - Restored disjoint
//! - Revision 11, 2017-04-29 (Andrew Garrard)  - Now Ycbcr conversion, and KHR
//! - Revision 12, 2017-06-06 (Andrew Garrard)  - Added conversion to image view creation
//! - Revision 13, 2017-07-13 (Andrew Garrard)  - Allowed cosited-only chroma samples for formats
//! - Revision 14, 2017-08-11 (Andrew Garrard)  - Reflected quantization changes in BT.2100-1
//!# Other info
//! * 2017-08-11
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Andrew Garrard, Samsung Electronics  - Tobias Hector, Imagination Technologies  - James
//!   Jones, NVIDIA  - Daniel Koch, NVIDIA  - Daniel Rakos, AMD  - Romain Guy, Google  - Jesse Hall,
//!   Google  - Tom Cooksey, ARM Ltd  - Jeff Leger, Qualcomm Technologies, Inc  - Jan-Harald
//!   Fredriksen, ARM Ltd  - Jan Outters, Samsung Electronics  - Alon Or-bach, Samsung Electronics
//!   - Michael Worcester, Imagination Technologies  - Jeff Bolz, NVIDIA  - Tony Zlatinski, NVIDIA
//!   - Matthew Netsch, Qualcomm Technologies, Inc
//!# Related
//! - [`BindImagePlaneMemoryInfoKHR`]
//! - [`ChromaLocationKHR`]
//! - [`ImagePlaneMemoryRequirementsInfoKHR`]
//! - [`PhysicalDeviceSamplerYcbcrConversionFeaturesKHR`]
//! - [`SamplerYcbcrConversionCreateInfoKHR`]
//! - [`SamplerYcbcrConversionImageFormatPropertiesKHR`]
//! - [`SamplerYcbcrConversionInfoKHR`]
//! - [`SamplerYcbcrConversionKHR`]
//! - [`SamplerYcbcrModelConversionKHR`]
//! - [`SamplerYcbcrRangeKHR`]
//! - [`create_sampler_ycbcr_conversion_khr`]
//! - [`destroy_sampler_ycbcr_conversion_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::Device,
    vulkan1_1::{FNCreateSamplerYcbcrConversion, FNDestroySamplerYcbcrConversion},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_sampler_ycbcr_conversion");
///The V-table of [`Device`] for functions from `VK_KHR_sampler_ycbcr_conversion`
pub struct DeviceKhrSamplerYcbcrConversionVTable {
    ///See [`FNCreateSamplerYcbcrConversion`] for more information.
    pub create_sampler_ycbcr_conversion_khr: FNCreateSamplerYcbcrConversion,
    ///See [`FNDestroySamplerYcbcrConversion`] for more information.
    pub destroy_sampler_ycbcr_conversion_khr: FNDestroySamplerYcbcrConversion,
}
impl DeviceKhrSamplerYcbcrConversionVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            create_sampler_ycbcr_conversion_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateSamplerYcbcrConversionKHR").as_ptr(),
                ))
            },
            destroy_sampler_ycbcr_conversion_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroySamplerYcbcrConversionKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::create_sampler_ycbcr_conversion_khr`]. See [`FNCreateSamplerYcbcrConversion`]
    /// for more information.
    pub fn create_sampler_ycbcr_conversion_khr(&self) -> FNCreateSamplerYcbcrConversion {
        self.create_sampler_ycbcr_conversion_khr
    }
    ///Gets [`Self::destroy_sampler_ycbcr_conversion_khr`]. See [`FNDestroySamplerYcbcrConversion`]
    /// for more information.
    pub fn destroy_sampler_ycbcr_conversion_khr(&self) -> FNDestroySamplerYcbcrConversion {
        self.destroy_sampler_ycbcr_conversion_khr
    }
}
