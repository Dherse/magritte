use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_multi_draw");
///[VkMultiDrawInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawInfoEXT {
///    uint32_t    firstVertex;
///    uint32_t    vertexCount;
///} VkMultiDrawInfoEXT;
///```
///# Members
/// - [`first_vertex`] is the first vertex to draw.
/// - [`vertex_count`] is the number of vertices to draw.
///# Description
///The members of [`MultiDrawInfoEXT`] have the same meaning as the
///[`first_vertex`] and [`vertex_count`] parameters in [`CmdDraw`].
///# Related
/// - [`VK_EXT_multi_draw`]
/// - [`CmdDrawMultiEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    ///[`first_vertex`] is the first vertex to draw.
    first_vertex: u32,
    ///[`vertex_count`] is the number of vertices to draw.
    vertex_count: u32,
}
impl Default for MultiDrawInfoEXT {
    fn default() -> Self {
        Self {
            first_vertex: 0,
            vertex_count: 0,
        }
    }
}
impl MultiDrawInfoEXT {
    ///Gets the raw value of [`Self::first_vertex`]
    pub fn first_vertex_raw(&self) -> u32 {
        self.first_vertex
    }
    ///Gets the raw value of [`Self::vertex_count`]
    pub fn vertex_count_raw(&self) -> u32 {
        self.vertex_count
    }
    ///Sets the raw value of [`Self::first_vertex`]
    pub fn set_first_vertex_raw(&mut self, value: u32) -> &mut Self {
        self.first_vertex = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_count`]
    pub fn set_vertex_count_raw(&mut self, value: u32) -> &mut Self {
        self.vertex_count = value;
        self
    }
    ///Gets the value of [`Self::first_vertex`]
    pub fn first_vertex(&self) -> u32 {
        self.first_vertex
    }
    ///Gets the value of [`Self::vertex_count`]
    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
    ///Gets a mutable reference to the value of [`Self::first_vertex`]
    pub fn first_vertex_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::vertex_count`]
    pub fn vertex_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::first_vertex`]
    pub fn set_first_vertex(&mut self, value: u32) -> &mut Self {
        self.first_vertex = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_count`]
    pub fn set_vertex_count(&mut self, value: u32) -> &mut Self {
        self.vertex_count = value;
        self
    }
}
///[VkMultiDrawIndexedInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) - Structure specifying a multi-draw command
///# C Specifications
///The [`MultiDrawIndexedInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkMultiDrawIndexedInfoEXT {
///    uint32_t    firstIndex;
///    uint32_t    indexCount;
///    int32_t     vertexOffset;
///} VkMultiDrawIndexedInfoEXT;
///```
///# Members
/// - [`first_index`] is the first index to draw.
/// - [`index_count`] is the number of vertices to draw.
/// - [`vertex_offset`] is the value added to the vertex index before indexing into the vertex
///   buffer for indexed multidraws.
///# Description
///The [`first_index`], [`index_count`], and [`vertex_offset`] members of
///[`MultiDrawIndexedInfoEXT`] have the same meaning as the
///[`first_index`], [`index_count`], and [`vertex_offset`] parameters,
///respectively, of [`CmdDrawIndexed`].
///# Related
/// - [`VK_EXT_multi_draw`]
/// - [`CmdDrawMultiIndexedEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    ///[`first_index`] is the first index to draw.
    first_index: u32,
    ///[`index_count`] is the number of vertices to draw.
    index_count: u32,
    ///[`vertex_offset`] is the value added to the vertex index before
    ///indexing into the vertex buffer for indexed multidraws.
    vertex_offset: i32,
}
impl Default for MultiDrawIndexedInfoEXT {
    fn default() -> Self {
        Self {
            first_index: 0,
            index_count: 0,
            vertex_offset: 0,
        }
    }
}
impl MultiDrawIndexedInfoEXT {
    ///Gets the raw value of [`Self::first_index`]
    pub fn first_index_raw(&self) -> u32 {
        self.first_index
    }
    ///Gets the raw value of [`Self::index_count`]
    pub fn index_count_raw(&self) -> u32 {
        self.index_count
    }
    ///Gets the raw value of [`Self::vertex_offset`]
    pub fn vertex_offset_raw(&self) -> i32 {
        self.vertex_offset
    }
    ///Sets the raw value of [`Self::first_index`]
    pub fn set_first_index_raw(&mut self, value: u32) -> &mut Self {
        self.first_index = value;
        self
    }
    ///Sets the raw value of [`Self::index_count`]
    pub fn set_index_count_raw(&mut self, value: u32) -> &mut Self {
        self.index_count = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_offset`]
    pub fn set_vertex_offset_raw(&mut self, value: i32) -> &mut Self {
        self.vertex_offset = value;
        self
    }
    ///Gets the value of [`Self::first_index`]
    pub fn first_index(&self) -> u32 {
        self.first_index
    }
    ///Gets the value of [`Self::index_count`]
    pub fn index_count(&self) -> u32 {
        self.index_count
    }
    ///Gets the value of [`Self::vertex_offset`]
    pub fn vertex_offset(&self) -> i32 {
        self.vertex_offset
    }
    ///Gets a mutable reference to the value of [`Self::first_index`]
    pub fn first_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::index_count`]
    pub fn index_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::vertex_offset`]
    pub fn vertex_offset_mut(&mut self) -> &mut i32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::first_index`]
    pub fn set_first_index(&mut self, value: u32) -> &mut Self {
        self.first_index = value;
        self
    }
    ///Sets the raw value of [`Self::index_count`]
    pub fn set_index_count(&mut self, value: u32) -> &mut Self {
        self.index_count = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_offset`]
    pub fn set_vertex_offset(&mut self, value: i32) -> &mut Self {
        self.vertex_offset = value;
        self
    }
}
///[VkPhysicalDeviceMultiDrawPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) - Structure describing multidraw limits of an implementation
///# C Specifications
///The [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxMultiDrawCount;
///} VkPhysicalDeviceMultiDrawPropertiesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure
///describe the following features:
///# Description
/// - [`max_multi_draw_count`] indicates the maximum number of draw calls which **can** be batched
///   into a single multidraw.
///If the [`PhysicalDeviceMultiDrawPropertiesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_multi_draw`]
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
pub struct PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT`
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_multi_draw_count`] indicates the
    ///maximum number of draw calls which **can** be batched into a single
    ///multidraw.
    max_multi_draw_count: u32,
}
impl<'lt> Default for PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_multi_draw_count: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiDrawPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_multi_draw_count`]
    pub fn max_multi_draw_count_raw(&self) -> u32 {
        self.max_multi_draw_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_multi_draw_count`]
    pub fn set_max_multi_draw_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_multi_draw_count = value;
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
    ///Gets the value of [`Self::max_multi_draw_count`]
    pub fn max_multi_draw_count(&self) -> u32 {
        self.max_multi_draw_count
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
    ///Gets a mutable reference to the value of [`Self::max_multi_draw_count`]
    pub fn max_multi_draw_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::max_multi_draw_count`]
    pub fn set_max_multi_draw_count(&mut self, value: u32) -> &mut Self {
        self.max_multi_draw_count = value;
        self
    }
}
///[VkPhysicalDeviceMultiDrawFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) - Structure describing whether the implementation supports multi draw functionality
///# C Specifications
///The [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_multi_draw
///typedef struct VkPhysicalDeviceMultiDrawFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           multiDraw;
///} VkPhysicalDeviceMultiDrawFeaturesEXT;
///```
///# Members
///The members of the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure
///describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`multi_draw`] indicates that the implementation supports [`CmdDrawMultiEXT`] and
///   [`CmdDrawMultiIndexedEXT`].
///If the [`PhysicalDeviceMultiDrawFeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMultiDrawFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT`
///# Related
/// - [`VK_EXT_multi_draw`]
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
pub struct PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`multi_draw`] indicates that the implementation
    ///supports [`CmdDrawMultiEXT`] and [`CmdDrawMultiIndexedEXT`].
    multi_draw: Bool32,
}
impl<'lt> Default for PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            multi_draw: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiDrawFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::multi_draw`]
    pub fn multi_draw_raw(&self) -> Bool32 {
        self.multi_draw
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::multi_draw`]
    pub fn set_multi_draw_raw(&mut self, value: Bool32) -> &mut Self {
        self.multi_draw = value;
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
    ///Gets the value of [`Self::multi_draw`]
    pub fn multi_draw(&self) -> bool {
        unsafe { std::mem::transmute(self.multi_draw as u8) }
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
    ///Gets a mutable reference to the value of [`Self::multi_draw`]
    pub fn multi_draw_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multi_draw as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multi_draw as *mut Bool32)
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
    ///Sets the raw value of [`Self::multi_draw`]
    pub fn set_multi_draw(&mut self, value: bool) -> &mut Self {
        self.multi_draw = value as u8 as u32;
        self
    }
}
