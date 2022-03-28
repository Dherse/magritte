use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map2");
///[VkPhysicalDeviceFragmentDensityMap2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2FeaturesEXT.html) - Structure describing additional fragment density map features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map2
///typedef struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMapDeferred;
///} VkPhysicalDeviceFragmentDensityMap2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map_deferred`] specifies whether the implementation supports deferred reads
///   of fragment density map image views. If this feature is not enabled,
///   `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` **must**  not be included in
///   [`ImageViewCreateInfo::flags`].
///If the [`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMap2FeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map2`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_density_map_deferred`]
    ///specifies whether the implementation supports deferred reads of fragment
    ///density map image views.
    ///If this feature is not enabled,
    ///`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` **must**
    ///not be included in [`ImageViewCreateInfo`]::`flags`.
    fragment_density_map_deferred: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMap2FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            fragment_density_map_deferred: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMap2FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::fragment_density_map_deferred`]
    pub fn fragment_density_map_deferred_raw(&self) -> Bool32 {
        self.fragment_density_map_deferred
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_deferred`]
    pub fn set_fragment_density_map_deferred_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_map_deferred = value;
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
    ///Gets the value of [`Self::fragment_density_map_deferred`]
    pub fn fragment_density_map_deferred(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_map_deferred as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_density_map_deferred`]
    pub fn fragment_density_map_deferred_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_map_deferred as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_map_deferred as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_deferred`]
    pub fn set_fragment_density_map_deferred(&mut self, value: bool) -> &mut Self {
        self.fragment_density_map_deferred = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceFragmentDensityMap2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMap2PropertiesEXT.html) - Structure describing additional fragment density map properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map2
///typedef struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           subsampledLoads;
///    VkBool32           subsampledCoarseReconstructionEarlyAccess;
///    uint32_t           maxSubsampledArrayLayers;
///    uint32_t           maxDescriptorSetSubsampledSamplers;
///} VkPhysicalDeviceFragmentDensityMap2PropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`subsampled_loads`] specifies if performing image data read with load operations on
///   subsampled attachments will be resampled to the fragment density of the render pass
/// - [`subsampled_coarse_reconstruction_early_access`] specifies if performing image data read with
///   samplers created with `flags` containing
///   `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` in fragment shader will trigger
///   additional reads during `VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
/// - [`max_subsampled_array_layers`] is the maximum number of [`ImageView`] array layers for usages
///   supporting subsampled samplers
/// - [`max_descriptor_set_subsampled_samplers`] is the maximum number of subsampled samplers that
///   **can**  be included in a [`PipelineLayout`]
///# Description
///If the [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map2`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMap2PropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`subsampled_loads`] specifies if performing
    ///image data read with load operations on subsampled attachments will be
    ///resampled to the fragment density of the render pass
    subsampled_loads: Bool32,
    ///[`subsampled_coarse_reconstruction_early_access`] specifies if performing
    ///image data read with samplers created with `flags` containing
    ///`VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT` in
    ///fragment shader will trigger additional reads during
    ///`VK_PIPELINE_STAGE_VERTEX_SHADER_BIT`
    subsampled_coarse_reconstruction_early_access: Bool32,
    ///[`max_subsampled_array_layers`] is
    ///the maximum number of [`ImageView`] array layers for usages
    ///supporting subsampled samplers
    max_subsampled_array_layers: u32,
    ///[`max_descriptor_set_subsampled_samplers`] is the maximum number of
    ///subsampled samplers that  **can**  be included in a [`PipelineLayout`]
    max_descriptor_set_subsampled_samplers: u32,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMap2PropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            subsampled_loads: 0,
            subsampled_coarse_reconstruction_early_access: 0,
            max_subsampled_array_layers: 0,
            max_descriptor_set_subsampled_samplers: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMap2PropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::subsampled_loads`]
    pub fn subsampled_loads_raw(&self) -> Bool32 {
        self.subsampled_loads
    }
    ///Gets the raw value of [`Self::subsampled_coarse_reconstruction_early_access`]
    pub fn subsampled_coarse_reconstruction_early_access_raw(&self) -> Bool32 {
        self.subsampled_coarse_reconstruction_early_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::subsampled_loads`]
    pub fn set_subsampled_loads_raw(&mut self, value: Bool32) -> &mut Self {
        self.subsampled_loads = value;
        self
    }
    ///Sets the raw value of [`Self::subsampled_coarse_reconstruction_early_access`]
    pub fn set_subsampled_coarse_reconstruction_early_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.subsampled_coarse_reconstruction_early_access = value;
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
    ///Gets the value of [`Self::subsampled_loads`]
    pub fn subsampled_loads(&self) -> bool {
        unsafe { std::mem::transmute(self.subsampled_loads as u8) }
    }
    ///Gets the value of [`Self::subsampled_coarse_reconstruction_early_access`]
    pub fn subsampled_coarse_reconstruction_early_access(&self) -> bool {
        unsafe { std::mem::transmute(self.subsampled_coarse_reconstruction_early_access as u8) }
    }
    ///Gets the value of [`Self::max_subsampled_array_layers`]
    pub fn max_subsampled_array_layers(&self) -> u32 {
        self.max_subsampled_array_layers
    }
    ///Gets the value of [`Self::max_descriptor_set_subsampled_samplers`]
    pub fn max_descriptor_set_subsampled_samplers(&self) -> u32 {
        self.max_descriptor_set_subsampled_samplers
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
    ///Gets a mutable reference to the value of [`Self::subsampled_loads`]
    pub fn subsampled_loads_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subsampled_loads as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subsampled_loads as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::subsampled_coarse_reconstruction_early_access`]
    pub fn subsampled_coarse_reconstruction_early_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subsampled_coarse_reconstruction_early_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subsampled_coarse_reconstruction_early_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_subsampled_array_layers`]
    pub fn max_subsampled_array_layers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_descriptor_set_subsampled_samplers`]
    pub fn max_descriptor_set_subsampled_samplers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::subsampled_loads`]
    pub fn set_subsampled_loads(&mut self, value: bool) -> &mut Self {
        self.subsampled_loads = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::subsampled_coarse_reconstruction_early_access`]
    pub fn set_subsampled_coarse_reconstruction_early_access(&mut self, value: bool) -> &mut Self {
        self.subsampled_coarse_reconstruction_early_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_subsampled_array_layers`]
    pub fn set_max_subsampled_array_layers(&mut self, value: u32) -> &mut Self {
        self.max_subsampled_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_subsampled_samplers`]
    pub fn set_max_descriptor_set_subsampled_samplers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_subsampled_samplers = value;
        self
    }
}
