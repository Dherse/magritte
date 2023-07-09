pub use crate::common::extensions::ext_image_drm_format_modifier::{
    DrmFormatModifierProperties2EXT, DrmFormatModifierPropertiesEXT, EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME,
    EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{FormatFeatureFlags, SharingMode, StructureType, SubresourceLayout},
    vulkan1_3::FormatFeatureFlags2,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkDrmFormatModifierPropertiesListEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DrmFormatModifierPropertiesListEXT {
    #[doc(alias = "pDrmFormatModifierProperties")]
    pub drm_format_modifier_properties: SmallVec<[DrmFormatModifierPropertiesEXT; 8]>,
}
impl DrmFormatModifierPropertiesListEXT {
    ///Get a reference to the `drm_format_modifier_properties` field.
    pub fn drm_format_modifier_properties(&self) -> &SmallVec<[DrmFormatModifierPropertiesEXT; 8]> {
        &self.drm_format_modifier_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DrmFormatModifierPropertiesListEXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_drm_format_modifier_properties = self.drm_format_modifier_properties.len() as u32;
        let drm_format_modifier_properties = bump
            .alloc_slice_fill_iter(
                self.drm_format_modifier_properties
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_mut_ptr()
            .cast();
        crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesListEXT {
            s_type: StructureType::DrmFormatModifierPropertiesListExt,
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: len_drm_format_modifier_properties,
            drm_format_modifier_properties: drm_format_modifier_properties,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DrmFormatModifierPropertiesListEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let drm_format_modifier_properties_len = value.drm_format_modifier_count;
        let mut drm_format_modifier_properties = SmallVec::with_capacity(drm_format_modifier_properties_len as usize);
        for i in 0..drm_format_modifier_properties_len {
            drm_format_modifier_properties.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_properties.add(i as usize).read(),
            ));
        }
        Self {
            drm_format_modifier_properties: drm_format_modifier_properties,
        }
    }
}
impl DrmFormatModifierPropertiesEXT {
    ///Get a reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Get a reference to the `drm_format_modifier_plane_count` field.
    pub fn drm_format_modifier_plane_count(&self) -> u32 {
        self.drm_format_modifier_plane_count
    }
    ///Get a reference to the `drm_format_modifier_tiling_features` field.
    pub fn drm_format_modifier_tiling_features(&self) -> FormatFeatureFlags {
        self.drm_format_modifier_tiling_features
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DrmFormatModifierPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesEXT {
            drm_format_modifier: self.drm_format_modifier.into_low_level(context, bump),
            drm_format_modifier_plane_count: self.drm_format_modifier_plane_count.into_low_level(context, bump),
            drm_format_modifier_tiling_features: self.drm_format_modifier_tiling_features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DrmFormatModifierPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            drm_format_modifier: crate::conv::FromLowLevel::from_low_level(context, value.drm_format_modifier),
            drm_format_modifier_plane_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_plane_count,
            ),
            drm_format_modifier_tiling_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_tiling_features,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceImageDrmFormatModifierInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXT {
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "sharingMode")]
    pub sharing_mode: SharingMode,
    #[doc(alias = "pQueueFamilyIndices")]
    pub queue_family_indices: SmallVec<[u32; 8]>,
}
impl PhysicalDeviceImageDrmFormatModifierInfoEXT {
    ///Get a reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Get a reference to the `sharing_mode` field.
    pub fn sharing_mode(&self) -> SharingMode {
        self.sharing_mode
    }
    ///Get a reference to the `queue_family_indices` field.
    pub fn queue_family_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.queue_family_indices
    }
    ///Get a mutable reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Get a mutable reference to the `sharing_mode` field.
    pub fn sharing_mode_mut(&mut self) -> &mut SharingMode {
        &mut self.sharing_mode
    }
    ///Get a mutable reference to the `queue_family_indices` field.
    pub fn queue_family_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.queue_family_indices
    }
    ///Sets the `drm_format_modifier` field.
    pub fn set_drm_format_modifier(&mut self, drm_format_modifier: u64) -> &mut Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    ///Sets the `sharing_mode` field.
    pub fn set_sharing_mode(&mut self, sharing_mode: SharingMode) -> &mut Self {
        self.sharing_mode = sharing_mode;
        self
    }
    ///Sets the `queue_family_indices` field.
    pub fn set_queue_family_indices(&mut self, queue_family_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.queue_family_indices = queue_family_indices;
        self
    }
    ///Sets the `drm_format_modifier` field in a builder way.
    pub fn with_drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    ///Sets the `sharing_mode` field in a builder way.
    pub fn with_sharing_mode(mut self, sharing_mode: SharingMode) -> Self {
        self.sharing_mode = sharing_mode;
        self
    }
    ///Sets the `queue_family_indices` field in a builder way.
    pub fn with_queue_family_indices(mut self, queue_family_indices: SmallVec<[u32; 8]>) -> Self {
        self.queue_family_indices = queue_family_indices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_queue_family_indices = self.queue_family_indices.len() as u32;
        let queue_family_indices = bump
            .alloc_slice_fill_iter(
                self.queue_family_indices
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_image_drm_format_modifier::PhysicalDeviceImageDrmFormatModifierInfoEXT {
            s_type: StructureType::PhysicalDeviceImageDrmFormatModifierInfoExt,
            p_next: std::ptr::null(),
            drm_format_modifier: self.drm_format_modifier.into_low_level(context, bump),
            sharing_mode: self.sharing_mode.into_low_level(context, bump),
            queue_family_index_count: len_queue_family_indices,
            queue_family_indices: queue_family_indices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceImageDrmFormatModifierInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let queue_family_indices_len = value.queue_family_index_count;
        let mut queue_family_indices = SmallVec::with_capacity(queue_family_indices_len as usize);
        for i in 0..queue_family_indices_len {
            queue_family_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.queue_family_indices.add(i as usize).read(),
            ));
        }
        Self {
            drm_format_modifier: crate::conv::FromLowLevel::from_low_level(context, value.drm_format_modifier),
            sharing_mode: crate::conv::FromLowLevel::from_low_level(context, value.sharing_mode),
            queue_family_indices: queue_family_indices,
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierListCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageDrmFormatModifierListCreateInfoEXT {
    #[doc(alias = "pDrmFormatModifiers")]
    pub drm_format_modifiers: SmallVec<[u64; 8]>,
}
impl ImageDrmFormatModifierListCreateInfoEXT {
    ///Get a reference to the `drm_format_modifiers` field.
    pub fn drm_format_modifiers(&self) -> &SmallVec<[u64; 8]> {
        &self.drm_format_modifiers
    }
    ///Get a mutable reference to the `drm_format_modifiers` field.
    pub fn drm_format_modifiers_mut(&mut self) -> &mut SmallVec<[u64; 8]> {
        &mut self.drm_format_modifiers
    }
    ///Sets the `drm_format_modifiers` field.
    pub fn set_drm_format_modifiers(&mut self, drm_format_modifiers: SmallVec<[u64; 8]>) -> &mut Self {
        self.drm_format_modifiers = drm_format_modifiers;
        self
    }
    ///Sets the `drm_format_modifiers` field in a builder way.
    pub fn with_drm_format_modifiers(mut self, drm_format_modifiers: SmallVec<[u64; 8]>) -> Self {
        self.drm_format_modifiers = drm_format_modifiers;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageDrmFormatModifierListCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierListCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_drm_format_modifiers = self.drm_format_modifiers.len() as u32;
        let drm_format_modifiers = bump
            .alloc_slice_fill_iter(
                self.drm_format_modifiers
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierListCreateInfoEXT {
            s_type: StructureType::ImageDrmFormatModifierListCreateInfoExt,
            p_next: std::ptr::null(),
            drm_format_modifier_count: len_drm_format_modifiers,
            drm_format_modifiers: drm_format_modifiers,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageDrmFormatModifierListCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let drm_format_modifiers_len = value.drm_format_modifier_count;
        let mut drm_format_modifiers = SmallVec::with_capacity(drm_format_modifiers_len as usize);
        for i in 0..drm_format_modifiers_len {
            drm_format_modifiers.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifiers.add(i as usize).read(),
            ));
        }
        Self {
            drm_format_modifiers: drm_format_modifiers,
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierExplicitCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXT {
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "pPlaneLayouts")]
    pub plane_layouts: SmallVec<[SubresourceLayout; 8]>,
}
impl ImageDrmFormatModifierExplicitCreateInfoEXT {
    ///Get a reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Get a reference to the `plane_layouts` field.
    pub fn plane_layouts(&self) -> &SmallVec<[SubresourceLayout; 8]> {
        &self.plane_layouts
    }
    ///Get a mutable reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier_mut(&mut self) -> &mut u64 {
        &mut self.drm_format_modifier
    }
    ///Get a mutable reference to the `plane_layouts` field.
    pub fn plane_layouts_mut(&mut self) -> &mut SmallVec<[SubresourceLayout; 8]> {
        &mut self.plane_layouts
    }
    ///Sets the `drm_format_modifier` field.
    pub fn set_drm_format_modifier(&mut self, drm_format_modifier: u64) -> &mut Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    ///Sets the `plane_layouts` field.
    pub fn set_plane_layouts(&mut self, plane_layouts: SmallVec<[SubresourceLayout; 8]>) -> &mut Self {
        self.plane_layouts = plane_layouts;
        self
    }
    ///Sets the `drm_format_modifier` field in a builder way.
    pub fn with_drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.drm_format_modifier = drm_format_modifier;
        self
    }
    ///Sets the `plane_layouts` field in a builder way.
    pub fn with_plane_layouts(mut self, plane_layouts: SmallVec<[SubresourceLayout; 8]>) -> Self {
        self.plane_layouts = plane_layouts;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageDrmFormatModifierExplicitCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierExplicitCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_plane_layouts = self.plane_layouts.len() as u32;
        let plane_layouts = bump
            .alloc_slice_fill_iter(self.plane_layouts.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierExplicitCreateInfoEXT {
            s_type: StructureType::ImageDrmFormatModifierExplicitCreateInfoExt,
            p_next: std::ptr::null(),
            drm_format_modifier: self.drm_format_modifier.into_low_level(context, bump),
            drm_format_modifier_plane_count: len_plane_layouts,
            plane_layouts: plane_layouts,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageDrmFormatModifierExplicitCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let plane_layouts_len = value.drm_format_modifier_plane_count;
        let mut plane_layouts = SmallVec::with_capacity(plane_layouts_len as usize);
        for i in 0..plane_layouts_len {
            plane_layouts.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.plane_layouts.add(i as usize).read(),
            ));
        }
        Self {
            drm_format_modifier: crate::conv::FromLowLevel::from_low_level(context, value.drm_format_modifier),
            plane_layouts: plane_layouts,
        }
    }
}
#[doc(alias = "VkImageDrmFormatModifierPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageDrmFormatModifierPropertiesEXT {
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
}
impl ImageDrmFormatModifierPropertiesEXT {
    ///Get a reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageDrmFormatModifierPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_image_drm_format_modifier::ImageDrmFormatModifierPropertiesEXT {
            s_type: StructureType::ImageDrmFormatModifierPropertiesExt,
            p_next: std::ptr::null_mut(),
            drm_format_modifier: self.drm_format_modifier.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageDrmFormatModifierPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            drm_format_modifier: crate::conv::FromLowLevel::from_low_level(context, value.drm_format_modifier),
        }
    }
}
#[doc(alias = "VkDrmFormatModifierPropertiesList2EXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DrmFormatModifierPropertiesList2EXT {
    #[doc(alias = "pDrmFormatModifierProperties")]
    pub drm_format_modifier_properties: SmallVec<[DrmFormatModifierProperties2EXT; 8]>,
}
impl DrmFormatModifierPropertiesList2EXT {
    ///Get a reference to the `drm_format_modifier_properties` field.
    pub fn drm_format_modifier_properties(&self) -> &SmallVec<[DrmFormatModifierProperties2EXT; 8]> {
        &self.drm_format_modifier_properties
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DrmFormatModifierPropertiesList2EXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_drm_format_modifier_properties = self.drm_format_modifier_properties.len() as u32;
        let drm_format_modifier_properties = bump
            .alloc_slice_fill_iter(
                self.drm_format_modifier_properties
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_mut_ptr()
            .cast();
        crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierPropertiesList2EXT {
            s_type: StructureType::DrmFormatModifierPropertiesList2Ext,
            p_next: std::ptr::null_mut(),
            drm_format_modifier_count: len_drm_format_modifier_properties,
            drm_format_modifier_properties: drm_format_modifier_properties,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DrmFormatModifierPropertiesList2EXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let drm_format_modifier_properties_len = value.drm_format_modifier_count;
        let mut drm_format_modifier_properties = SmallVec::with_capacity(drm_format_modifier_properties_len as usize);
        for i in 0..drm_format_modifier_properties_len {
            drm_format_modifier_properties.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_properties.add(i as usize).read(),
            ));
        }
        Self {
            drm_format_modifier_properties: drm_format_modifier_properties,
        }
    }
}
impl DrmFormatModifierProperties2EXT {
    ///Get a reference to the `drm_format_modifier` field.
    pub fn drm_format_modifier(&self) -> u64 {
        self.drm_format_modifier
    }
    ///Get a reference to the `drm_format_modifier_plane_count` field.
    pub fn drm_format_modifier_plane_count(&self) -> u32 {
        self.drm_format_modifier_plane_count
    }
    ///Get a reference to the `drm_format_modifier_tiling_features` field.
    pub fn drm_format_modifier_tiling_features(&self) -> FormatFeatureFlags2 {
        self.drm_format_modifier_tiling_features
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DrmFormatModifierProperties2EXT {
    type LowLevel = crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierProperties2EXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_image_drm_format_modifier::DrmFormatModifierProperties2EXT {
            drm_format_modifier: self.drm_format_modifier.into_low_level(context, bump),
            drm_format_modifier_plane_count: self.drm_format_modifier_plane_count.into_low_level(context, bump),
            drm_format_modifier_tiling_features: self.drm_format_modifier_tiling_features.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DrmFormatModifierProperties2EXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            drm_format_modifier: crate::conv::FromLowLevel::from_low_level(context, value.drm_format_modifier),
            drm_format_modifier_plane_count: crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_plane_count,
            ),
            drm_format_modifier_tiling_features: crate::conv::FromLowLevel::from_low_level(
                context,
                value.drm_format_modifier_tiling_features,
            ),
        }
    }
}
