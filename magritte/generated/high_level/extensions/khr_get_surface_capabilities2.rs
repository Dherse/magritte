pub use crate::common::extensions::khr_get_surface_capabilities2::{
    KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME, KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION,
};
#[cfg(feature = "VK_AMD_display_native_hdr")]
use crate::extensions::amd_display_native_hdr::DisplayNativeHdrSurfaceCapabilitiesAMD;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::extensions::ext_full_screen_exclusive::SurfaceCapabilitiesFullScreenExclusiveEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT;
#[cfg(feature = "VK_KHR_shared_presentable_image")]
use crate::extensions::khr_shared_presentable_image::SharedPresentSurfaceCapabilitiesKHR;
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
use crate::extensions::khr_surface_protected_capabilities::SurfaceProtectedCapabilitiesKHR;
use crate::{
    context::Context,
    extensions::khr_surface::{SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR},
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceSurfaceInfo2KHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[PhysicalDeviceSurfaceInfo2KHRExtension; 1]>,
    pub surface: Option<SurfaceKHR>,
}
impl PhysicalDeviceSurfaceInfo2KHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<PhysicalDeviceSurfaceInfo2KHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[PhysicalDeviceSurfaceInfo2KHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `surface` field.
    pub fn surface(&self) -> &Option<SurfaceKHR> {
        &self.surface
    }
    ///Get a mutable reference to the `extensions` field.
    pub fn extensions_mut(&mut self) -> &mut SmallVec<[PhysicalDeviceSurfaceInfo2KHRExtension; 1]> {
        &mut self.extensions
    }
    ///Get a mutable reference to the `surface` field.
    pub fn surface_mut(&mut self) -> &mut Option<SurfaceKHR> {
        &mut self.surface
    }
    ///Sets the `extensions` field.
    pub fn set_extensions(&mut self, extensions: SmallVec<[PhysicalDeviceSurfaceInfo2KHRExtension; 1]>) -> &mut Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `surface` field.
    pub fn set_surface(&mut self, surface: Option<SurfaceKHR>) -> &mut Self {
        self.surface = surface;
        self
    }
    ///Sets the `extensions` field in a builder way.
    pub fn with_extensions(mut self, extensions: SmallVec<[PhysicalDeviceSurfaceInfo2KHRExtension; 1]>) -> Self {
        self.extensions = extensions;
        self
    }
    ///Sets the `surface` field in a builder way.
    pub fn with_surface(mut self, surface: Option<SurfaceKHR>) -> Self {
        self.surface = surface;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSurfaceInfo2KHR {
    type LowLevel = crate::native::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR;
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
        crate::native::extensions::khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR {
            s_type: StructureType::PhysicalDeviceSurfaceInfo2Khr,
            p_next: next,
            surface: self
                .surface
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSurfaceInfo2KHR {
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
        Self {
            extensions: extensions,
            surface: if value.surface == crate::native::extensions::khr_surface::SurfaceKHR::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.surface))
            },
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`PhysicalDeviceSurfaceInfo2KHR`]
pub enum PhysicalDeviceSurfaceInfo2KHRExtension {
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    ///Contains a type [`SurfaceFullScreenExclusiveInfoEXT`] for extending
    /// [`PhysicalDeviceSurfaceInfo2KHR`]
    SurfaceFullScreenExclusiveInfoEXT(SurfaceFullScreenExclusiveInfoEXT),
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    ///Contains a type [`SurfaceFullScreenExclusiveWin32InfoEXT`] for extending
    /// [`PhysicalDeviceSurfaceInfo2KHR`]
    SurfaceFullScreenExclusiveWin32InfoEXT(SurfaceFullScreenExclusiveWin32InfoEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceSurfaceInfo2KHRExtension {
    type LowLevel = *const crate::native::vulkan1_0::BaseInStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
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
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceSurfaceInfo2KHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_EXT_full_screen_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceFullScreenExclusiveInfoExt => Self :: SurfaceFullScreenExclusiveInfoEXT (SurfaceFullScreenExclusiveInfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceFullScreenExclusiveInfoEXT > ()))) , # [cfg (feature = "VK_EXT_full_screen_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceFullScreenExclusiveWin32InfoExt => Self :: SurfaceFullScreenExclusiveWin32InfoEXT (SurfaceFullScreenExclusiveWin32InfoEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceFullScreenExclusiveWin32InfoEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (PhysicalDeviceSurfaceInfo2KHR)) }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<SurfaceFullScreenExclusiveInfoEXT> for PhysicalDeviceSurfaceInfo2KHRExtension {
    fn from(ext: SurfaceFullScreenExclusiveInfoEXT) -> Self {
        Self::SurfaceFullScreenExclusiveInfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl TryInto<SurfaceFullScreenExclusiveInfoEXT> for PhysicalDeviceSurfaceInfo2KHRExtension {
    type Error = PhysicalDeviceSurfaceInfo2KHRExtension;
    fn try_into(self) -> Result<SurfaceFullScreenExclusiveInfoEXT, Self::Error> {
        match self {
            Self::SurfaceFullScreenExclusiveInfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<SurfaceFullScreenExclusiveWin32InfoEXT> for PhysicalDeviceSurfaceInfo2KHRExtension {
    fn from(ext: SurfaceFullScreenExclusiveWin32InfoEXT) -> Self {
        Self::SurfaceFullScreenExclusiveWin32InfoEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl TryInto<SurfaceFullScreenExclusiveWin32InfoEXT> for PhysicalDeviceSurfaceInfo2KHRExtension {
    type Error = PhysicalDeviceSurfaceInfo2KHRExtension;
    fn try_into(self) -> Result<SurfaceFullScreenExclusiveWin32InfoEXT, Self::Error> {
        match self {
            Self::SurfaceFullScreenExclusiveWin32InfoEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSurfaceCapabilities2KHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceCapabilities2KHR {
    #[doc(alias = "pNext")]
    pub extensions: SmallVec<[SurfaceCapabilities2KHRExtension; 1]>,
    #[doc(alias = "surfaceCapabilities")]
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}
impl SurfaceCapabilities2KHR {
    ///Adds an extension to the struct
    pub fn with_extension(mut self, ext: impl Into<SurfaceCapabilities2KHRExtension>) -> Self {
        self.extensions.push(ext.into());
        self
    }
    ///Get a reference to the `extensions` field.
    pub fn extensions(&self) -> &SmallVec<[SurfaceCapabilities2KHRExtension; 1]> {
        &self.extensions
    }
    ///Get a reference to the `surface_capabilities` field.
    pub fn surface_capabilities(&self) -> SurfaceCapabilitiesKHR {
        self.surface_capabilities
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceCapabilities2KHR {
    type LowLevel = crate::native::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let mut next = std::ptr::null_mut();
        let mut extensions = self.extensions.iter();
        while let Some(ext) = extensions.next() {
            let ext = ext.into_low_level(context, bump);
            (*ext).next = next;
            next = ext;
        }
        crate::native::extensions::khr_get_surface_capabilities2::SurfaceCapabilities2KHR {
            s_type: StructureType::SurfaceCapabilities2Khr,
            p_next: next,
            surface_capabilities: self.surface_capabilities.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceCapabilities2KHR {
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
        Self {
            extensions: extensions,
            surface_capabilities: crate::conv::FromLowLevel::from_low_level(context, value.surface_capabilities),
        }
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///Extensions for [`SurfaceCapabilities2KHR`]
pub enum SurfaceCapabilities2KHRExtension {
    #[cfg(feature = "VK_AMD_display_native_hdr")]
    ///Contains a type [`DisplayNativeHdrSurfaceCapabilitiesAMD`] for extending
    /// [`SurfaceCapabilities2KHR`]
    DisplayNativeHdrSurfaceCapabilitiesAMD(DisplayNativeHdrSurfaceCapabilitiesAMD),
    #[cfg(feature = "VK_KHR_shared_presentable_image")]
    ///Contains a type [`SharedPresentSurfaceCapabilitiesKHR`] for extending
    /// [`SurfaceCapabilities2KHR`]
    SharedPresentSurfaceCapabilitiesKHR(SharedPresentSurfaceCapabilitiesKHR),
    #[cfg(feature = "VK_KHR_surface_protected_capabilities")]
    ///Contains a type [`SurfaceProtectedCapabilitiesKHR`] for extending
    /// [`SurfaceCapabilities2KHR`]
    SurfaceProtectedCapabilitiesKHR(SurfaceProtectedCapabilitiesKHR),
    #[cfg(feature = "VK_EXT_full_screen_exclusive")]
    ///Contains a type [`SurfaceCapabilitiesFullScreenExclusiveEXT`] for extending
    /// [`SurfaceCapabilities2KHR`]
    SurfaceCapabilitiesFullScreenExclusiveEXT(SurfaceCapabilitiesFullScreenExclusiveEXT),
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceCapabilities2KHRExtension {
    type LowLevel = *mut crate::native::vulkan1_0::BaseOutStructure;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self { # [cfg (feature = "VK_AMD_display_native_hdr")] Self :: DisplayNativeHdrSurfaceCapabilitiesAMD (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: amd_display_native_hdr :: DisplayNativeHdrSurfaceCapabilitiesAMD) . cast () , # [cfg (feature = "VK_KHR_shared_presentable_image")] Self :: SharedPresentSurfaceCapabilitiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_shared_presentable_image :: SharedPresentSurfaceCapabilitiesKHR) . cast () , # [cfg (feature = "VK_KHR_surface_protected_capabilities")] Self :: SurfaceProtectedCapabilitiesKHR (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: khr_surface_protected_capabilities :: SurfaceProtectedCapabilitiesKHR) . cast () , # [cfg (feature = "VK_EXT_full_screen_exclusive")] Self :: SurfaceCapabilitiesFullScreenExclusiveEXT (ext) => (bump . alloc (ext . into_low_level (context , bump)) as * mut crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceCapabilitiesFullScreenExclusiveEXT) . cast () , other => unreachable ! ("unexpected variant {:?}" , other) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceCapabilities2KHRExtension {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        assert!(!value.is_null());
        match (* value) . s_type { # [cfg (feature = "VK_AMD_display_native_hdr")] crate :: native :: vulkan1_0 :: StructureType :: DisplayNativeHdrSurfaceCapabilitiesAmd => Self :: DisplayNativeHdrSurfaceCapabilitiesAMD (DisplayNativeHdrSurfaceCapabilitiesAMD :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: amd_display_native_hdr :: DisplayNativeHdrSurfaceCapabilitiesAMD > ()))) , # [cfg (feature = "VK_KHR_shared_presentable_image")] crate :: native :: vulkan1_0 :: StructureType :: SharedPresentSurfaceCapabilitiesKhr => Self :: SharedPresentSurfaceCapabilitiesKHR (SharedPresentSurfaceCapabilitiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_shared_presentable_image :: SharedPresentSurfaceCapabilitiesKHR > ()))) , # [cfg (feature = "VK_KHR_surface_protected_capabilities")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceProtectedCapabilitiesKhr => Self :: SurfaceProtectedCapabilitiesKHR (SurfaceProtectedCapabilitiesKHR :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: khr_surface_protected_capabilities :: SurfaceProtectedCapabilitiesKHR > ()))) , # [cfg (feature = "VK_EXT_full_screen_exclusive")] crate :: native :: vulkan1_0 :: StructureType :: SurfaceCapabilitiesFullScreenExclusiveExt => Self :: SurfaceCapabilitiesFullScreenExclusiveEXT (SurfaceCapabilitiesFullScreenExclusiveEXT :: from_low_level (context , std :: ptr :: read (value . cast :: < crate :: native :: extensions :: ext_full_screen_exclusive :: SurfaceCapabilitiesFullScreenExclusiveEXT > ()))) , other => panic ! ("Structure type {:?} is not a member of {}" , other , stringify ! (SurfaceCapabilities2KHR)) }
    }
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl From<DisplayNativeHdrSurfaceCapabilitiesAMD> for SurfaceCapabilities2KHRExtension {
    fn from(ext: DisplayNativeHdrSurfaceCapabilitiesAMD) -> Self {
        Self::DisplayNativeHdrSurfaceCapabilitiesAMD(ext)
    }
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
impl TryInto<DisplayNativeHdrSurfaceCapabilitiesAMD> for SurfaceCapabilities2KHRExtension {
    type Error = SurfaceCapabilities2KHRExtension;
    fn try_into(self) -> Result<DisplayNativeHdrSurfaceCapabilitiesAMD, Self::Error> {
        match self {
            Self::DisplayNativeHdrSurfaceCapabilitiesAMD(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl From<SharedPresentSurfaceCapabilitiesKHR> for SurfaceCapabilities2KHRExtension {
    fn from(ext: SharedPresentSurfaceCapabilitiesKHR) -> Self {
        Self::SharedPresentSurfaceCapabilitiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_shared_presentable_image")]
impl TryInto<SharedPresentSurfaceCapabilitiesKHR> for SurfaceCapabilities2KHRExtension {
    type Error = SurfaceCapabilities2KHRExtension;
    fn try_into(self) -> Result<SharedPresentSurfaceCapabilitiesKHR, Self::Error> {
        match self {
            Self::SharedPresentSurfaceCapabilitiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
impl From<SurfaceProtectedCapabilitiesKHR> for SurfaceCapabilities2KHRExtension {
    fn from(ext: SurfaceProtectedCapabilitiesKHR) -> Self {
        Self::SurfaceProtectedCapabilitiesKHR(ext)
    }
}
#[cfg(feature = "VK_KHR_surface_protected_capabilities")]
impl TryInto<SurfaceProtectedCapabilitiesKHR> for SurfaceCapabilities2KHRExtension {
    type Error = SurfaceCapabilities2KHRExtension;
    fn try_into(self) -> Result<SurfaceProtectedCapabilitiesKHR, Self::Error> {
        match self {
            Self::SurfaceProtectedCapabilitiesKHR(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl From<SurfaceCapabilitiesFullScreenExclusiveEXT> for SurfaceCapabilities2KHRExtension {
    fn from(ext: SurfaceCapabilitiesFullScreenExclusiveEXT) -> Self {
        Self::SurfaceCapabilitiesFullScreenExclusiveEXT(ext)
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
impl TryInto<SurfaceCapabilitiesFullScreenExclusiveEXT> for SurfaceCapabilities2KHRExtension {
    type Error = SurfaceCapabilities2KHRExtension;
    fn try_into(self) -> Result<SurfaceCapabilitiesFullScreenExclusiveEXT, Self::Error> {
        match self {
            Self::SurfaceCapabilitiesFullScreenExclusiveEXT(ext) => Ok(ext),
            other => Err(other),
        }
    }
}
#[doc(alias = "VkSurfaceFormat2KHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SurfaceFormat2KHR {
    #[doc(alias = "surfaceFormat")]
    pub surface_format: SurfaceFormatKHR,
}
impl SurfaceFormat2KHR {
    ///Get a reference to the `surface_format` field.
    pub fn surface_format(&self) -> SurfaceFormatKHR {
        self.surface_format
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SurfaceFormat2KHR {
    type LowLevel = crate::native::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_get_surface_capabilities2::SurfaceFormat2KHR {
            s_type: StructureType::SurfaceFormat2Khr,
            p_next: std::ptr::null_mut(),
            surface_format: self.surface_format.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SurfaceFormat2KHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            surface_format: crate::conv::FromLowLevel::from_low_level(context, value.surface_format),
        }
    }
}
