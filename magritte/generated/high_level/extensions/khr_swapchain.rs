pub use crate::common::extensions::khr_swapchain::{
    SwapchainCreateFlagBitsKHR, SwapchainCreateFlagsKHR, KHR_SWAPCHAIN_EXTENSION_NAME, KHR_SWAPCHAIN_SPEC_VERSION,
};
#[cfg(feature = "VK_AMD_display_native_hdr")]
use crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD;
#[cfg(feature = "VK_EXT_display_control")]
use crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT;
#[cfg(feature = "VK_GOOGLE_display_timing")]
use crate::extensions::google_display_timing::PresentTimesInfoGOOGLE;
#[cfg(feature = "VK_KHR_device_group")]
use crate::extensions::khr_device_group::DeviceGroupPresentInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]
use crate::extensions::khr_device_group::DeviceGroupSwapchainCreateInfoKHR;
#[cfg(feature = "VK_KHR_display_swapchain")]
use crate::extensions::khr_display_swapchain::DisplayPresentInfoKHR;
#[cfg(feature = "VK_KHR_incremental_present")]
use crate::extensions::khr_incremental_present::PresentRegionsKHR;
#[cfg(feature = "VK_KHR_present_id")]
use crate::extensions::khr_present_id::PresentIdKHR;
use crate::{
    context::{Container, Context, Error, ObjectId},
    extensions::khr_surface::{
        ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR,
    },
    vulkan1_0::{Extent2D, Format, ImageUsageFlags, Semaphore, SharingMode, StructureType, VulkanResultCodes},
    vulkan1_2::ImageFormatListCreateInfo,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapchainCreateInfoKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SwapchainCreateInfoKHRExtension; 1]>,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    #[doc(alias = "minImageCount")]
    pub min_image_count: u32,
    #[doc(alias = "imageFormat")]
    pub image_format: Format,
    #[doc(alias = "imageColorSpace")]
    pub image_color_space: ColorSpaceKHR,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent2D,
    #[doc(alias = "imageArrayLayers")]
    pub image_array_layers: u32,
    #[doc(alias = "imageUsage")]
    pub image_usage: ImageUsageFlags,
    #[doc(alias = "imageSharingMode")]
    pub image_sharing_mode: SharingMode,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: SmallVec<[u32; 8]>,
    #[doc(alias = "preTransform")]
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "compositeAlpha")]
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    #[doc(alias = "presentMode")]
    pub present_mode: PresentModeKHR,
    pub clipped: bool,
    #[doc(alias = "oldSwapchain")]
    pub old_swapchain: Option<SwapchainKHR>,
}
impl SwapchainCreateInfoKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SwapchainCreateInfoKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SwapchainCreateInfoKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> SwapchainCreateFlagsKHR {
        self.flags
    }
    ///Get a reference to the `surface` field.
    pub fn surface(&self) -> &SurfaceKHR {
        &self.surface
    }
    ///Get a reference to the `min_image_count` field.
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Get a reference to the `image_format` field.
    pub fn image_format(&self) -> Format {
        self.image_format
    }
    ///Get a reference to the `image_color_space` field.
    pub fn image_color_space(&self) -> ColorSpaceKHR {
        self.image_color_space
    }
    ///Get a reference to the `image_extent` field.
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Get a reference to the `image_array_layers` field.
    pub fn image_array_layers(&self) -> u32 {
        self.image_array_layers
    }
    ///Get a reference to the `image_usage` field.
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Get a reference to the `image_sharing_mode` field.
    pub fn image_sharing_mode(&self) -> SharingMode {
        self.image_sharing_mode
    }
    ///Get a reference to the `queue_family_indices` field.
    pub fn queue_family_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.queue_family_indices
    }
    ///Get a reference to the `pre_transform` field.
    pub fn pre_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.pre_transform
    }
    ///Get a reference to the `composite_alpha` field.
    pub fn composite_alpha(&self) -> CompositeAlphaFlagBitsKHR {
        self.composite_alpha
    }
    ///Get a reference to the `present_mode` field.
    pub fn present_mode(&self) -> PresentModeKHR {
        self.present_mode
    }
    ///Get a reference to the `clipped` field.
    pub fn clipped(&self) -> &bool {
        &self.clipped
    }
    ///Get a reference to the `old_swapchain` field.
    pub fn old_swapchain(&self) -> &Option<SwapchainKHR> {
        &self.old_swapchain
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[SwapchainCreateInfoKHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut SwapchainCreateFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `surface` field.
    pub fn surface_mut(&mut self) -> &mut SurfaceKHR {
        &mut self.surface
    }
    ///Get a mutable reference to the `min_image_count` field.
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut self.min_image_count
    }
    ///Get a mutable reference to the `image_format` field.
    pub fn image_format_mut(&mut self) -> &mut Format {
        &mut self.image_format
    }
    ///Get a mutable reference to the `image_color_space` field.
    pub fn image_color_space_mut(&mut self) -> &mut ColorSpaceKHR {
        &mut self.image_color_space
    }
    ///Get a mutable reference to the `image_extent` field.
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
    }
    ///Get a mutable reference to the `image_array_layers` field.
    pub fn image_array_layers_mut(&mut self) -> &mut u32 {
        &mut self.image_array_layers
    }
    ///Get a mutable reference to the `image_usage` field.
    pub fn image_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.image_usage
    }
    ///Get a mutable reference to the `image_sharing_mode` field.
    pub fn image_sharing_mode_mut(&mut self) -> &mut SharingMode {
        &mut self.image_sharing_mode
    }
    ///Get a mutable reference to the `queue_family_indices` field.
    pub fn queue_family_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.queue_family_indices
    }
    ///Get a mutable reference to the `pre_transform` field.
    pub fn pre_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.pre_transform
    }
    ///Get a mutable reference to the `composite_alpha` field.
    pub fn composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagBitsKHR {
        &mut self.composite_alpha
    }
    ///Get a mutable reference to the `present_mode` field.
    pub fn present_mode_mut(&mut self) -> &mut PresentModeKHR {
        &mut self.present_mode
    }
    ///Get a mutable reference to the `clipped` field.
    pub fn clipped_mut(&mut self) -> &mut bool {
        &mut self.clipped
    }
    ///Get a mutable reference to the `old_swapchain` field.
    pub fn old_swapchain_mut(&mut self) -> &mut Option<SwapchainKHR> {
        &mut self.old_swapchain
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[SwapchainCreateInfoKHRExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: SwapchainCreateFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `surface` field.
    pub fn set_surface(&mut self, surface: SurfaceKHR) -> &mut Self {
        self.surface = surface;
        self
    }
    ///Sets the `min_image_count` field.
    pub fn set_min_image_count(&mut self, min_image_count: u32) -> &mut Self {
        self.min_image_count = min_image_count;
        self
    }
    ///Sets the `image_format` field.
    pub fn set_image_format(&mut self, image_format: Format) -> &mut Self {
        self.image_format = image_format;
        self
    }
    ///Sets the `image_color_space` field.
    pub fn set_image_color_space(&mut self, image_color_space: ColorSpaceKHR) -> &mut Self {
        self.image_color_space = image_color_space;
        self
    }
    ///Sets the `image_extent` field.
    pub fn set_image_extent(&mut self, image_extent: Extent2D) -> &mut Self {
        self.image_extent = image_extent;
        self
    }
    ///Sets the `image_array_layers` field.
    pub fn set_image_array_layers(&mut self, image_array_layers: u32) -> &mut Self {
        self.image_array_layers = image_array_layers;
        self
    }
    ///Sets the `image_usage` field.
    pub fn set_image_usage(&mut self, image_usage: ImageUsageFlags) -> &mut Self {
        self.image_usage = image_usage;
        self
    }
    ///Sets the `image_sharing_mode` field.
    pub fn set_image_sharing_mode(&mut self, image_sharing_mode: SharingMode) -> &mut Self {
        self.image_sharing_mode = image_sharing_mode;
        self
    }
    ///Sets the `queue_family_indices` field.
    pub fn set_queue_family_indices(&mut self, queue_family_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.queue_family_indices = queue_family_indices;
        self
    }
    ///Sets the `pre_transform` field.
    pub fn set_pre_transform(&mut self, pre_transform: SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.pre_transform = pre_transform;
        self
    }
    ///Sets the `composite_alpha` field.
    pub fn set_composite_alpha(&mut self, composite_alpha: CompositeAlphaFlagBitsKHR) -> &mut Self {
        self.composite_alpha = composite_alpha;
        self
    }
    ///Sets the `present_mode` field.
    pub fn set_present_mode(&mut self, present_mode: PresentModeKHR) -> &mut Self {
        self.present_mode = present_mode;
        self
    }
    ///Sets the `clipped` field.
    pub fn set_clipped(&mut self, clipped: bool) -> &mut Self {
        self.clipped = clipped;
        self
    }
    ///Sets the `old_swapchain` field.
    pub fn set_old_swapchain(&mut self, old_swapchain: Option<SwapchainKHR>) -> &mut Self {
        self.old_swapchain = old_swapchain;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[SwapchainCreateInfoKHRExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: SwapchainCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `surface` field in a builder way.
    pub fn with_surface(mut self, surface: SurfaceKHR) -> Self {
        self.surface = surface;
        self
    }
    ///Sets the `min_image_count` field in a builder way.
    pub fn with_min_image_count(mut self, min_image_count: u32) -> Self {
        self.min_image_count = min_image_count;
        self
    }
    ///Sets the `image_format` field in a builder way.
    pub fn with_image_format(mut self, image_format: Format) -> Self {
        self.image_format = image_format;
        self
    }
    ///Sets the `image_color_space` field in a builder way.
    pub fn with_image_color_space(mut self, image_color_space: ColorSpaceKHR) -> Self {
        self.image_color_space = image_color_space;
        self
    }
    ///Sets the `image_extent` field in a builder way.
    pub fn with_image_extent(mut self, image_extent: Extent2D) -> Self {
        self.image_extent = image_extent;
        self
    }
    ///Sets the `image_array_layers` field in a builder way.
    pub fn with_image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.image_array_layers = image_array_layers;
        self
    }
    ///Sets the `image_usage` field in a builder way.
    pub fn with_image_usage(mut self, image_usage: ImageUsageFlags) -> Self {
        self.image_usage = image_usage;
        self
    }
    ///Sets the `image_sharing_mode` field in a builder way.
    pub fn with_image_sharing_mode(mut self, image_sharing_mode: SharingMode) -> Self {
        self.image_sharing_mode = image_sharing_mode;
        self
    }
    ///Sets the `queue_family_indices` field in a builder way.
    pub fn with_queue_family_indices(mut self, queue_family_indices: SmallVec<[u32; 8]>) -> Self {
        self.queue_family_indices = queue_family_indices;
        self
    }
    ///Sets the `pre_transform` field in a builder way.
    pub fn with_pre_transform(mut self, pre_transform: SurfaceTransformFlagBitsKHR) -> Self {
        self.pre_transform = pre_transform;
        self
    }
    ///Sets the `composite_alpha` field in a builder way.
    pub fn with_composite_alpha(mut self, composite_alpha: CompositeAlphaFlagBitsKHR) -> Self {
        self.composite_alpha = composite_alpha;
        self
    }
    ///Sets the `present_mode` field in a builder way.
    pub fn with_present_mode(mut self, present_mode: PresentModeKHR) -> Self {
        self.present_mode = present_mode;
        self
    }
    ///Sets the `clipped` field in a builder way.
    pub fn with_clipped(mut self, clipped: bool) -> Self {
        self.clipped = clipped;
        self
    }
    ///Sets the `old_swapchain` field in a builder way.
    pub fn with_old_swapchain(mut self, old_swapchain: Option<SwapchainKHR>) -> Self {
        self.old_swapchain = old_swapchain;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_swapchain::SwapchainCreateInfoKHR;
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
        let len_queue_family_indices = self.queue_family_indices.len() as u32;
        let queue_family_indices = bump
            .alloc_slice_fill_iter(
                self.queue_family_indices
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::khr_swapchain::SwapchainCreateInfoKHR {
            s_type: StructureType::SwapchainCreateInfoKhr,
            p_next: next,
            flags: self.flags.into_low_level(context, bump),
            surface: self.surface.into_low_level(context, bump),
            min_image_count: self.min_image_count.into_low_level(context, bump),
            image_format: self.image_format.into_low_level(context, bump),
            image_color_space: self.image_color_space.into_low_level(context, bump),
            image_extent: self.image_extent.into_low_level(context, bump),
            image_array_layers: self.image_array_layers.into_low_level(context, bump),
            image_usage: self.image_usage.into_low_level(context, bump),
            image_sharing_mode: self.image_sharing_mode.into_low_level(context, bump),
            queue_family_index_count: len_queue_family_indices,
            queue_family_indices: queue_family_indices,
            pre_transform: self.pre_transform.into_low_level(context, bump),
            composite_alpha: self.composite_alpha.into_low_level(context, bump),
            present_mode: self.present_mode.into_low_level(context, bump),
            clipped: self.clipped.into_low_level(context, bump),
            old_swapchain: self
                .old_swapchain
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainCreateInfoKHR {
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
        let queue_family_indices_len = value.queue_family_index_count;
        let mut queue_family_indices = SmallVec::with_capacity(queue_family_indices_len as usize);
        for i in 0..queue_family_indices_len {
            queue_family_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.queue_family_indices.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            surface: crate::conv::FromLowLevel::from_low_level(context, value.surface),
            min_image_count: crate::conv::FromLowLevel::from_low_level(context, value.min_image_count),
            image_format: crate::conv::FromLowLevel::from_low_level(context, value.image_format),
            image_color_space: crate::conv::FromLowLevel::from_low_level(context, value.image_color_space),
            image_extent: crate::conv::FromLowLevel::from_low_level(context, value.image_extent),
            image_array_layers: crate::conv::FromLowLevel::from_low_level(context, value.image_array_layers),
            image_usage: crate::conv::FromLowLevel::from_low_level(context, value.image_usage),
            image_sharing_mode: crate::conv::FromLowLevel::from_low_level(context, value.image_sharing_mode),
            queue_family_indices: queue_family_indices,
            pre_transform: crate::conv::FromLowLevel::from_low_level(context, value.pre_transform),
            composite_alpha: crate::conv::FromLowLevel::from_low_level(context, value.composite_alpha),
            present_mode: crate::conv::FromLowLevel::from_low_level(context, value.present_mode),
            clipped: crate::conv::FromLowLevel::from_low_level(context, value.clipped),
            old_swapchain: if value.old_swapchain == crate::native::extensions::khr_swapchain::SwapchainKHR::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.old_swapchain))
            },
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SwapchainCreateInfoKHR`]
pub enum SwapchainCreateInfoKHRExtension {
    #[cfg(feature = "VK_EXT_display_control")]
    ///Contains a type [`SwapchainCounterCreateInfoEXT`] for extending [`SwapchainCreateInfoKHR`]
    SwapchainCounterCreateInfoEXT(SwapchainCounterCreateInfoEXT),
    #[cfg(feature = "VK_KHR_device_group")]
    ///Contains a type [`DeviceGroupSwapchainCreateInfoKHR`] for extending
    /// [`SwapchainCreateInfoKHR`]
    DeviceGroupSwapchainCreateInfoKHR(DeviceGroupSwapchainCreateInfoKHR),
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    ///Contains a type [`SwapchainDisplayNativeHdrCreateInfoAMD`] for extending
    /// [`SwapchainCreateInfoKHR`]
    SwapchainDisplayNativeHdrCreateInfoAMD(SwapchainDisplayNativeHdrCreateInfoAMD),
    ///Contains a type [`ImageFormatListCreateInfo`] for extending [`SwapchainCreateInfoKHR`]
    ImageFormatListCreateInfo(ImageFormatListCreateInfo),
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    ///Contains a type [`SurfaceFullScreenExclusiveInfoEXT`] for extending
    /// [`SwapchainCreateInfoKHR`]
    SurfaceFullScreenExclusiveInfoEXT(SurfaceFullScreenExclusiveInfoEXT),
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    ///Contains a type [`SurfaceFullScreenExclusiveWin32InfoEXT`] for extending
    /// [`SwapchainCreateInfoKHR`]
    SurfaceFullScreenExclusiveWin32InfoEXT(SurfaceFullScreenExclusiveWin32InfoEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SwapchainCreateInfoKHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_EXT_display_control")]
            Self::SwapchainCounterCreateInfoEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_display_control::SwapchainCounterCreateInfoEXT)
                .cast(),
            #[cfg(feature = "VK_KHR_device_group")]
            Self::DeviceGroupSwapchainCreateInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_device_group::DeviceGroupSwapchainCreateInfoKHR)
                .cast(),
            #[cfg(feature = "VK_AMD_display_native_hdr")]
            Self::SwapchainDisplayNativeHdrCreateInfoAMD(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD)
                .cast(),
            Self::ImageFormatListCreateInfo(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::vulkan1_2::ImageFormatListCreateInfo)
                .cast(),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            Self::SurfaceFullScreenExclusiveInfoEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT)
                .cast(),
            #[cfg(feature = "VK_EXT_full_screen_exclusive")]
            Self::SurfaceFullScreenExclusiveWin32InfoEXT(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainCreateInfoKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_EXT_display_control")] crate :: native :: vulkan1_0 :: StructureType :: SwapchainCounterCreateInfoExt => Self :: SwapchainCounterCreateInfoEXT (SwapchainCounterCreateInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_display_control :: SwapchainCounterCreateInfoEXT > ()))) , # [cfg (feature = "VK_KHR_device_group")] crate :: native :: vulkan1_0 :: StructureType :: DeviceGroupSwapchainCreateInfoKhr => Self :: DeviceGroupSwapchainCreateInfoKHR (DeviceGroupSwapchainCreateInfoKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_device_group :: DeviceGroupSwapchainCreateInfoKHR > ()))) , # [cfg (feature = "VK_AMD_display_native_hdr")] crate :: native :: vulkan1_0 :: StructureType :: SwapchainDisplayNativeHdrCreateInfoAmd => Self :: SwapchainDisplayNativeHdrCreateInfoAMD (SwapchainDisplayNativeHdrCreateInfoAMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_display_native_hdr :: SwapchainDisplayNativeHdrCreateInfoAMD > ()))) , crate :: native :: vulkan1_0 :: StructureType :: ImageFormatListCreateInfo => Self :: ImageFormatListCreateInfo (ImageFormatListCreateInfo :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: vulkan1_2 :: ImageFormatListCreateInfo > ()))) , # [cfg (feature = "VK_EXT_full_screen_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceFullScreenExclusiveInfoExt => Self :: SurfaceFullScreenExclusiveInfoEXT (SurfaceFullScreenExclusiveInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceFullScreenExclusiveInfoEXT > ()))) , # [cfg (feature = "VK_EXT_full_screen_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceFullScreenExclusiveWin32InfoExt => Self :: SurfaceFullScreenExclusiveWin32InfoEXT (SurfaceFullScreenExclusiveWin32InfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceFullScreenExclusiveWin32InfoEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (SwapchainCreateInfoKHR)) }
    }
}
#[cfg(feature = "VK_EXT_display_control")]
impl From<SwapchainCounterCreateInfoEXT> for SwapchainCreateInfoKHRExtension {
    fn from(ext: SwapchainCounterCreateInfoEXT) -> Self {
        Self::SwapchainCounterCreateInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_display_control")]
impl TryInto<SwapchainCounterCreateInfoEXT> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<SwapchainCounterCreateInfoEXT, Self::Error> {
        match self {
            Self::SwapchainCounterCreateInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl From<DeviceGroupSwapchainCreateInfoKHR> for SwapchainCreateInfoKHRExtension {
    fn from(ext: DeviceGroupSwapchainCreateInfoKHR) -> Self {
        Self::DeviceGroupSwapchainCreateInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl TryInto<DeviceGroupSwapchainCreateInfoKHR> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<DeviceGroupSwapchainCreateInfoKHR, Self::Error> {
        match self {
            Self::DeviceGroupSwapchainCreateInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl From<SwapchainDisplayNativeHdrCreateInfoAMD> for SwapchainCreateInfoKHRExtension {
    fn from(ext: SwapchainDisplayNativeHdrCreateInfoAMD) -> Self {
        Self::SwapchainDisplayNativeHdrCreateInfoAMD(ext)
    }
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl TryInto<SwapchainDisplayNativeHdrCreateInfoAMD> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<SwapchainDisplayNativeHdrCreateInfoAMD, Self::Error> {
        match self {
            Self::SwapchainDisplayNativeHdrCreateInfoAMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
impl From<ImageFormatListCreateInfo> for SwapchainCreateInfoKHRExtension {
    fn from(ext: ImageFormatListCreateInfo) -> Self {
        Self::ImageFormatListCreateInfo(ext)
    }
}
impl TryInto<ImageFormatListCreateInfo> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<ImageFormatListCreateInfo, Self::Error> {
        match self {
            Self::ImageFormatListCreateInfo(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<SurfaceFullScreenExclusiveInfoEXT> for SwapchainCreateInfoKHRExtension {
    fn from(ext: SurfaceFullScreenExclusiveInfoEXT) -> Self {
        Self::SurfaceFullScreenExclusiveInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl TryInto<SurfaceFullScreenExclusiveInfoEXT> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<SurfaceFullScreenExclusiveInfoEXT, Self::Error> {
        match self {
            Self::SurfaceFullScreenExclusiveInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<SurfaceFullScreenExclusiveWin32InfoEXT> for SwapchainCreateInfoKHRExtension {
    fn from(ext: SurfaceFullScreenExclusiveWin32InfoEXT) -> Self {
        Self::SurfaceFullScreenExclusiveWin32InfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl TryInto<SurfaceFullScreenExclusiveWin32InfoEXT> for SwapchainCreateInfoKHRExtension {
    type Error = SwapchainCreateInfoKHRExtension;
    fn try_into(self) -> Result<SurfaceFullScreenExclusiveWin32InfoEXT, Self::Error> {
        match self {
            Self::SurfaceFullScreenExclusiveWin32InfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PresentInfoKHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PresentInfoKHRExtension; 1]>,
    #[doc(alias = "pWaitSemaphores")]
    pub wait_semaphores: SmallVec<[Semaphore; 8]>,
    #[doc(alias = "pSwapchains")]
    pub swapchains: SmallVec<[SwapchainKHR; 8]>,
    #[doc(alias = "pImageIndices")]
    pub image_indices: SmallVec<[u32; 8]>,
    #[doc(alias = "pResults")]
    pub results: SmallVec<[VulkanResultCodes; 8]>,
}
impl PresentInfoKHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PresentInfoKHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PresentInfoKHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `wait_semaphores` field.
    pub fn wait_semaphores(&self) -> &SmallVec<[Semaphore; 8]> {
        &self.wait_semaphores
    }
    ///Get a reference to the `swapchains` field.
    pub fn swapchains(&self) -> &SmallVec<[SwapchainKHR; 8]> {
        &self.swapchains
    }
    ///Get a reference to the `image_indices` field.
    pub fn image_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.image_indices
    }
    ///Get a reference to the `results` field.
    pub fn results(&self) -> &SmallVec<[VulkanResultCodes; 8]> {
        &self.results
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[PresentInfoKHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `wait_semaphores` field.
    pub fn wait_semaphores_mut(&mut self) -> &mut SmallVec<[Semaphore; 8]> {
        &mut self.wait_semaphores
    }
    ///Get a mutable reference to the `swapchains` field.
    pub fn swapchains_mut(&mut self) -> &mut SmallVec<[SwapchainKHR; 8]> {
        &mut self.swapchains
    }
    ///Get a mutable reference to the `image_indices` field.
    pub fn image_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.image_indices
    }
    ///Get a mutable reference to the `results` field.
    pub fn results_mut(&mut self) -> &mut SmallVec<[VulkanResultCodes; 8]> {
        &mut self.results
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[PresentInfoKHRExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `wait_semaphores` field.
    pub fn set_wait_semaphores(&mut self, wait_semaphores: SmallVec<[Semaphore; 8]>) -> &mut Self {
        self.wait_semaphores = wait_semaphores;
        self
    }
    ///Sets the `swapchains` field.
    pub fn set_swapchains(&mut self, swapchains: SmallVec<[SwapchainKHR; 8]>) -> &mut Self {
        self.swapchains = swapchains;
        self
    }
    ///Sets the `image_indices` field.
    pub fn set_image_indices(&mut self, image_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.image_indices = image_indices;
        self
    }
    ///Sets the `results` field.
    pub fn set_results(&mut self, results: SmallVec<[VulkanResultCodes; 8]>) -> &mut Self {
        self.results = results;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[PresentInfoKHRExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `wait_semaphores` field in a builder way.
    pub fn with_wait_semaphores(mut self, wait_semaphores: SmallVec<[Semaphore; 8]>) -> Self {
        self.wait_semaphores = wait_semaphores;
        self
    }
    ///Sets the `swapchains` field in a builder way.
    pub fn with_swapchains(mut self, swapchains: SmallVec<[SwapchainKHR; 8]>) -> Self {
        self.swapchains = swapchains;
        self
    }
    ///Sets the `image_indices` field in a builder way.
    pub fn with_image_indices(mut self, image_indices: SmallVec<[u32; 8]>) -> Self {
        self.image_indices = image_indices;
        self
    }
    ///Sets the `results` field in a builder way.
    pub fn with_results(mut self, results: SmallVec<[VulkanResultCodes; 8]>) -> Self {
        self.results = results;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentInfoKHR {
    type LowLevel = crate::native::extensions::khr_swapchain::PresentInfoKHR;
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
        let len_wait_semaphores = self.wait_semaphores.len() as u32;
        let wait_semaphores = bump
            .alloc_slice_fill_iter(self.wait_semaphores.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_swapchains = self.swapchains.len() as u32;
        let swapchains = bump
            .alloc_slice_fill_iter(self.swapchains.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let image_indices = bump
            .alloc_slice_fill_iter(self.image_indices.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let results = bump
            .alloc_slice_fill_iter(self.results.iter().map(|x| x.into_low_level(context, bump)))
            .as_mut_ptr()
            .cast();
        crate::native::extensions::khr_swapchain::PresentInfoKHR {
            s_type: StructureType::PresentInfoKhr,
            p_next: next,
            wait_semaphore_count: len_wait_semaphores,
            wait_semaphores: wait_semaphores,
            swapchain_count: len_swapchains,
            swapchains: swapchains,
            image_indices: image_indices,
            results: results,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentInfoKHR {
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
        let wait_semaphores_len = value.wait_semaphore_count;
        let mut wait_semaphores = SmallVec::with_capacity(wait_semaphores_len as usize);
        for i in 0..wait_semaphores_len {
            wait_semaphores.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.wait_semaphores.add(i as usize).read(),
            ));
        }
        let swapchains_len = value.swapchain_count;
        let mut swapchains = SmallVec::with_capacity(swapchains_len as usize);
        for i in 0..swapchains_len {
            swapchains.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.swapchains.add(i as usize).read(),
            ));
        }
        let image_indices_len = value.swapchain_count;
        let mut image_indices = SmallVec::with_capacity(image_indices_len as usize);
        for i in 0..image_indices_len {
            image_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.image_indices.add(i as usize).read(),
            ));
        }
        let results_len = value.swapchain_count;
        let mut results = SmallVec::with_capacity(results_len as usize);
        for i in 0..results_len {
            results.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.results.add(i as usize).read(),
            ));
        }
        Self {
            extensions: extensions,
            wait_semaphores: wait_semaphores,
            swapchains: swapchains,
            image_indices: image_indices,
            results: results,
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PresentInfoKHR`]
pub enum PresentInfoKHRExtension {
    #[cfg(feature = "VK_KHR_display_swapchain")]
    ///Contains a type [`DisplayPresentInfoKHR`] for extending [`PresentInfoKHR`]
    DisplayPresentInfoKHR(DisplayPresentInfoKHR),
    #[cfg(feature = "VK_KHR_incremental_present")]
    ///Contains a type [`PresentRegionsKHR`] for extending [`PresentInfoKHR`]
    PresentRegionsKHR(PresentRegionsKHR),
    #[cfg(feature = "VK_KHR_device_group")]
    ///Contains a type [`DeviceGroupPresentInfoKHR`] for extending [`PresentInfoKHR`]
    DeviceGroupPresentInfoKHR(DeviceGroupPresentInfoKHR),
    #[cfg(feature = "VK_KHR_present_id")]
    ///Contains a type [`PresentIdKHR`] for extending [`PresentInfoKHR`]
    PresentIdKHR(PresentIdKHR),
    #[cfg(feature = "VK_GOOGLE_display_timing")]
    ///Contains a type [`PresentTimesInfoGOOGLE`] for extending [`PresentInfoKHR`]
    PresentTimesInfoGOOGLE(PresentTimesInfoGOOGLE),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PresentInfoKHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            #[cfg(feature = "VK_KHR_display_swapchain")]
            Self::DisplayPresentInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_display_swapchain::DisplayPresentInfoKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_incremental_present")]
            Self::PresentRegionsKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_incremental_present::PresentRegionsKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_device_group")]
            Self::DeviceGroupPresentInfoKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_device_group::DeviceGroupPresentInfoKHR)
                .cast(),
            #[cfg(feature = "VK_KHR_present_id")]
            Self::PresentIdKHR(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::khr_present_id::PresentIdKHR)
                .cast(),
            #[cfg(feature = "VK_GOOGLE_display_timing")]
            Self::PresentTimesInfoGOOGLE(ext) => (bump.alloc(ext.into_low_level(context, bump))
                as *mut crate::native::extensions::google_display_timing::PresentTimesInfoGOOGLE)
                .cast(),
            other => unreachable!("unexpected variant {:?}", other),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PresentInfoKHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (*value).s_type {
            #[cfg(feature = "VK_KHR_display_swapchain")]
            crate::native::vulkan1_0::StructureType::DisplayPresentInfoKhr => {
                Self::DisplayPresentInfoKHR(DisplayPresentInfoKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_display_swapchain::DisplayPresentInfoKHR>(),
                    ),
                ))
            },
            #[cfg(feature = "VK_KHR_incremental_present")]
            crate::native::vulkan1_0::StructureType::PresentRegionsKhr => {
                Self::PresentRegionsKHR(PresentRegionsKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_incremental_present::PresentRegionsKHR>(),
                    ),
                ))
            },
            #[cfg(feature = "VK_KHR_device_group")]
            crate::native::vulkan1_0::StructureType::DeviceGroupPresentInfoKhr => {
                Self::DeviceGroupPresentInfoKHR(DeviceGroupPresentInfoKHR::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::khr_device_group::DeviceGroupPresentInfoKHR>(),
                    ),
                ))
            },
            #[cfg(feature = "VK_KHR_present_id")]
            crate::native::vulkan1_0::StructureType::PresentIdKhr => Self::PresentIdKHR(PresentIdKHR::from_low_level(
                context,
                std::ptr::read(value.cast::<crate::native::extensions::khr_present_id::PresentIdKHR>()),
            )),
            #[cfg(feature = "VK_GOOGLE_display_timing")]
            crate::native::vulkan1_0::StructureType::PresentTimesInfoGoogle => {
                Self::PresentTimesInfoGOOGLE(PresentTimesInfoGOOGLE::from_low_level(
                    context,
                    std::ptr::read(
                        value.cast::<crate::native::extensions::google_display_timing::PresentTimesInfoGOOGLE>(),
                    ),
                ))
            },
            other => panic!(
                "Structure type {:?} is not a member of {}",
                other,
                stringify!(PresentInfoKHR)
            ),
        }
    }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl From<DisplayPresentInfoKHR> for PresentInfoKHRExtension {
    fn from(ext: DisplayPresentInfoKHR) -> Self {
        Self::DisplayPresentInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
impl TryInto<DisplayPresentInfoKHR> for PresentInfoKHRExtension {
    type Error = PresentInfoKHRExtension;
    fn try_into(self) -> Result<DisplayPresentInfoKHR, Self::Error> {
        match self {
            Self::DisplayPresentInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl From<PresentRegionsKHR> for PresentInfoKHRExtension {
    fn from(ext: PresentRegionsKHR) -> Self {
        Self::PresentRegionsKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_incremental_present")]
impl TryInto<PresentRegionsKHR> for PresentInfoKHRExtension {
    type Error = PresentInfoKHRExtension;
    fn try_into(self) -> Result<PresentRegionsKHR, Self::Error> {
        match self {
            Self::PresentRegionsKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl From<DeviceGroupPresentInfoKHR> for PresentInfoKHRExtension {
    fn from(ext: DeviceGroupPresentInfoKHR) -> Self {
        Self::DeviceGroupPresentInfoKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_device_group")]
impl TryInto<DeviceGroupPresentInfoKHR> for PresentInfoKHRExtension {
    type Error = PresentInfoKHRExtension;
    fn try_into(self) -> Result<DeviceGroupPresentInfoKHR, Self::Error> {
        match self {
            Self::DeviceGroupPresentInfoKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_present_id")]
impl From<PresentIdKHR> for PresentInfoKHRExtension {
    fn from(ext: PresentIdKHR) -> Self {
        Self::PresentIdKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_present_id")]
impl TryInto<PresentIdKHR> for PresentInfoKHRExtension {
    type Error = PresentInfoKHRExtension;
    fn try_into(self) -> Result<PresentIdKHR, Self::Error> {
        match self {
            Self::PresentIdKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl From<PresentTimesInfoGOOGLE> for PresentInfoKHRExtension {
    fn from(ext: PresentTimesInfoGOOGLE) -> Self {
        Self::PresentTimesInfoGOOGLE(ext)
    }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
impl TryInto<PresentTimesInfoGOOGLE> for PresentInfoKHRExtension {
    type Error = PresentInfoKHRExtension;
    fn try_into(self) -> Result<PresentTimesInfoGOOGLE, Self::Error> {
        match self {
            Self::PresentTimesInfoGOOGLE(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSwapchainKHR")]
#[derive(Debug)]
pub struct SwapchainKHR {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for SwapchainKHR {
    fn clone(&self) -> Self {
        self.context.clone_swapchain_khr(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for SwapchainKHR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SwapchainKHR {
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
impl Drop for SwapchainKHR {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_swapchain_khr(&self.id);
        }
    }
}
impl PartialEq for SwapchainKHR {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl SwapchainKHR {
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
unsafe impl crate::conv::IntoLowLevel for SwapchainKHR {
    type LowLevel = crate::native::extensions::khr_swapchain::SwapchainKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context.swapchain_khr().get(&self.id).expect("unknwon handle").handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SwapchainKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.swapchain_khr().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
