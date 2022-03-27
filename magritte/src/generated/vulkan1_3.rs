use crate::{
    core::{MAX_DESCRIPTION_SIZE, MAX_EXTENSION_NAME_SIZE},
    vulkan1_0::{
        AttachmentLoadOp, AttachmentStoreOp, BaseInStructure, BaseOutStructure, Bool32, Buffer, BufferCreateInfo,
        ClearValue, CommandBuffer, DependencyFlags, DeviceSize, Extent3D, Filter, Format, Image, ImageAspectFlagBits,
        ImageCreateInfo, ImageLayout, ImageSubresourceLayers, ImageSubresourceRange, ImageView, Offset3D, Rect2D,
        SampleCountFlagBits, Semaphore, ShaderStageFlags, StructureType,
    },
    vulkan1_2::ResolveModeFlagBits,
};
use std::{ffi::c_void, marker::PhantomData, os::raw::c_char};
///[VkDevicePrivateDataCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDevicePrivateDataCreateInfo.html) - Reserve private data slots
///# C Specifications
///To reserve private data storage slots, add a
///[`DevicePrivateDataCreateInfo`] structure to the [`p_next`] chain of
///the [`DeviceCreateInfo`] structure.
///Reserving slots in this manner is not strictly necessary, but doing so **may**
///improve performance.
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkDevicePrivateDataCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           privateDataSlotRequestCount;
///} VkDevicePrivateDataCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_private_data
///typedef VkDevicePrivateDataCreateInfo VkDevicePrivateDataCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`private_data_slot_request_count`] is the amount of slots to reserve.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO`
///# Related
/// - [`VK_EXT_private_data`]
/// - [`crate::vulkan1_3`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct DevicePrivateDataCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`private_data_slot_request_count`] is the amount of slots to reserve.
    private_data_slot_request_count: u32,
}
impl<'lt> Default for DevicePrivateDataCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            private_data_slot_request_count: 0,
        }
    }
}
impl<'lt> DevicePrivateDataCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::private_data_slot_request_count`]
    pub fn private_data_slot_request_count_raw(&self) -> u32 {
        self.private_data_slot_request_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::private_data_slot_request_count`]
    pub fn set_private_data_slot_request_count_raw(&mut self, value: u32) -> &mut Self {
        self.private_data_slot_request_count = value;
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
    ///Gets the value of [`Self::private_data_slot_request_count`]
    pub fn private_data_slot_request_count(&self) -> u32 {
        self.private_data_slot_request_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::private_data_slot_request_count`]
    pub fn private_data_slot_request_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::private_data_slot_request_count`]
    pub fn set_private_data_slot_request_count(&mut self, value: u32) -> &mut Self {
        self.private_data_slot_request_count = value;
        self
    }
}
///[VkPrivateDataSlotCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlotCreateInfo.html) - Structure specifying the parameters of private data slot construction
///# C Specifications
///The [`PrivateDataSlotCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPrivateDataSlotCreateInfo {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkPrivateDataSlotCreateFlags    flags;
///} VkPrivateDataSlotCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_private_data
///typedef VkPrivateDataSlotCreateInfo VkPrivateDataSlotCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_EXT_private_data`]
/// - [`crate::vulkan1_3`]
/// - [`PrivateDataSlotCreateFlags`]
/// - [`StructureType`]
/// - [`CreatePrivateDataSlot`]
/// - [`CreatePrivateDataSlotEXT`]
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
#[repr(C)]
pub struct PrivateDataSlotCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PrivateDataSlotCreateFlags,
}
impl<'lt> Default for PrivateDataSlotCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> PrivateDataSlotCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    pub fn flags(&self) -> PrivateDataSlotCreateFlags {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PrivateDataSlotCreateFlags {
        &mut self.flags
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
    pub fn set_flags(&mut self, value: crate::vulkan1_3::PrivateDataSlotCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkPhysicalDevicePrivateDataFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrivateDataFeatures.html) - Structure specifying physical device support
///# C Specifications
///The [`PhysicalDevicePrivateDataFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDevicePrivateDataFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           privateData;
///} VkPhysicalDevicePrivateDataFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_private_data
///typedef VkPhysicalDevicePrivateDataFeatures VkPhysicalDevicePrivateDataFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`private_data`] indicates whether the implementation supports private data. See [Private Data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#private-data).
///If the [`PhysicalDevicePrivateDataFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePrivateDataFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES`
///# Related
/// - [`VK_EXT_private_data`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDevicePrivateDataFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`private_data`] indicates
    ///whether the implementation supports private data.
    ///See [Private Data](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#private-data).
    private_data: Bool32,
}
impl<'lt> Default for PhysicalDevicePrivateDataFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            private_data: 0,
        }
    }
}
impl<'lt> PhysicalDevicePrivateDataFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::private_data`]
    pub fn private_data_raw(&self) -> Bool32 {
        self.private_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::private_data`]
    pub fn set_private_data_raw(&mut self, value: Bool32) -> &mut Self {
        self.private_data = value;
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
    ///Gets the value of [`Self::private_data`]
    pub fn private_data(&self) -> bool {
        unsafe { std::mem::transmute(self.private_data as u8) }
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
    ///Gets a mutable reference to the value of [`Self::private_data`]
    pub fn private_data_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.private_data as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.private_data as *mut Bool32)
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
    ///Sets the raw value of [`Self::private_data`]
    pub fn set_private_data(&mut self, value: bool) -> &mut Self {
        self.private_data = value as u8 as u32;
        self
    }
}
///[VkDeviceBufferMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceBufferMemoryRequirements.html) - (None)
///# C Specifications
///The [`DeviceBufferMemoryRequirements`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkDeviceBufferMemoryRequirements {
///    VkStructureType              sType;
///    const void*                  pNext;
///    const VkBufferCreateInfo*    pCreateInfo;
///} VkDeviceBufferMemoryRequirements;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance4
///typedef VkDeviceBufferMemoryRequirements VkDeviceBufferMemoryRequirementsKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`create_info`] is a pointer to a [`BufferCreateInfo`] structure containing parameters
///   affecting creation of the buffer to query.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS`
/// - [`p_next`]**must** be `NULL`
/// - [`create_info`]**must** be a valid pointer to a valid [`BufferCreateInfo`] structure
///# Related
/// - [`VK_KHR_maintenance4`]
/// - [`crate::vulkan1_3`]
/// - [`BufferCreateInfo`]
/// - [`StructureType`]
/// - [`GetDeviceBufferMemoryRequirements`]
/// - [`GetDeviceBufferMemoryRequirementsKHR`]
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
#[repr(C)]
pub struct DeviceBufferMemoryRequirements<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`create_info`] is a pointer to a [`BufferCreateInfo`] structure
    ///containing parameters affecting creation of the buffer to query.
    create_info: *const BufferCreateInfo<'lt>,
}
impl<'lt> Default for DeviceBufferMemoryRequirements<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            create_info: std::ptr::null(),
        }
    }
}
impl<'lt> DeviceBufferMemoryRequirements<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::create_info`]
    pub fn create_info_raw(&self) -> *const BufferCreateInfo<'lt> {
        self.create_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::create_info`]
    pub fn set_create_info_raw(&mut self, value: *const BufferCreateInfo<'lt>) -> &mut Self {
        self.create_info = value;
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
    ///Gets the value of [`Self::create_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn create_info(&self) -> &BufferCreateInfo<'lt> {
        &*self.create_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
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
    ///Sets the raw value of [`Self::create_info`]
    pub fn set_create_info(&mut self, value: &'lt crate::vulkan1_0::BufferCreateInfo<'lt>) -> &mut Self {
        self.create_info = value as *const _;
        self
    }
}
///[VkDeviceImageMemoryRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceImageMemoryRequirements.html) - (None)
///# C Specifications
///The [`DeviceImageMemoryRequirements`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkDeviceImageMemoryRequirements {
///    VkStructureType             sType;
///    const void*                 pNext;
///    const VkImageCreateInfo*    pCreateInfo;
///    VkImageAspectFlagBits       planeAspect;
///} VkDeviceImageMemoryRequirements;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance4
///typedef VkDeviceImageMemoryRequirements VkDeviceImageMemoryRequirementsKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`create_info`] is a pointer to a [`ImageCreateInfo`] structure containing parameters
///   affecting creation of the image to query.
/// - [`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the aspect corresponding to the
///   image plane to query. This parameter is ignored unless [`create_info`]`::tiling` is
///   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, or [`create_info`]`::flags` has
///   `VK_IMAGE_CREATE_DISJOINT_BIT` set.
///# Description
///Valid Usage
/// - The [`create_info`]::[`p_next`] chain **must** not contain a [`ImageSwapchainCreateInfoKHR`]
///   structure
/// - If [`create_info`]`::format` specifies a *multi-planar* format and [`create_info`]`::flags`
///   has `VK_IMAGE_CREATE_DISJOINT_BIT` set then [`plane_aspect`]**must** not be
///   `VK_IMAGE_ASPECT_NONE_KHR`
/// - If [`create_info`]`::flags` has `VK_IMAGE_CREATE_DISJOINT_BIT` set and if the
///   [`create_info`]`::tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then
///   [`plane_aspect`]**must** be a single valid *format plane* for the image (that is, for a
///   two-plane image [`plane_aspect`]**must** be `VK_IMAGE_ASPECT_PLANE_0_BIT` or
///   `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`]**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
/// - If [`create_info`]`::tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then
///   [`plane_aspect`]**must** be a single valid *memory plane* for the image (that is,
///   `aspectMask`**must** specify a plane index that is less than the
///   [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the
///   image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS`
/// - [`p_next`]**must** be `NULL`
/// - [`create_info`]**must** be a valid pointer to a valid [`ImageCreateInfo`] structure
/// - If [`plane_aspect`] is not `0`, [`plane_aspect`]**must** be a valid [`ImageAspectFlagBits`]
///   value
///# Related
/// - [`VK_KHR_maintenance4`]
/// - [`crate::vulkan1_3`]
/// - [`ImageAspectFlagBits`]
/// - [`ImageCreateInfo`]
/// - [`StructureType`]
/// - [`GetDeviceImageMemoryRequirements`]
/// - [`GetDeviceImageMemoryRequirementsKHR`]
/// - [`GetDeviceImageSparseMemoryRequirements`]
/// - [`GetDeviceImageSparseMemoryRequirementsKHR`]
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
#[repr(C)]
pub struct DeviceImageMemoryRequirements<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`create_info`] is a pointer to a [`ImageCreateInfo`] structure
    ///containing parameters affecting creation of the image to query.
    create_info: *const ImageCreateInfo<'lt>,
    ///[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
    ///aspect corresponding to the image plane to query.
    ///This parameter is ignored unless
    ///[`create_info`]::`tiling` is
    ///`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, or
    ///[`create_info`]::`flags` has `VK_IMAGE_CREATE_DISJOINT_BIT`
    ///set.
    plane_aspect: ImageAspectFlagBits,
}
impl<'lt> Default for DeviceImageMemoryRequirements<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            create_info: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
impl<'lt> DeviceImageMemoryRequirements<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::create_info`]
    pub fn create_info_raw(&self) -> *const ImageCreateInfo<'lt> {
        self.create_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::create_info`]
    pub fn set_create_info_raw(&mut self, value: *const ImageCreateInfo<'lt>) -> &mut Self {
        self.create_info = value;
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
    ///Gets the value of [`Self::create_info`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn create_info(&self) -> &ImageCreateInfo<'lt> {
        &*self.create_info
    }
    ///Gets the value of [`Self::plane_aspect`]
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::plane_aspect`]
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
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
    ///Sets the raw value of [`Self::create_info`]
    pub fn set_create_info(&mut self, value: &'lt crate::vulkan1_0::ImageCreateInfo<'lt>) -> &mut Self {
        self.create_info = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::plane_aspect`]
    pub fn set_plane_aspect(&mut self, value: crate::vulkan1_0::ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = value;
        self
    }
}
///[VkPhysicalDeviceInlineUniformBlockFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockFeatures.html) - Structure describing inline uniform block features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceInlineUniformBlockFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceInlineUniformBlockFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           inlineUniformBlock;
///    VkBool32           descriptorBindingInlineUniformBlockUpdateAfterBind;
///} VkPhysicalDeviceInlineUniformBlockFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_inline_uniform_block
///typedef VkPhysicalDeviceInlineUniformBlockFeatures
/// VkPhysicalDeviceInlineUniformBlockFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`inline_uniform_block`] indicates whether the implementation supports inline uniform block
///   descriptors. If this feature is not enabled, `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`**must**
///   not be used.
/// - [`descriptor_binding_inline_uniform_block_update_after_bind`] indicates whether the
///   implementation supports updating inline uniform block descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
///If the [`PhysicalDeviceInlineUniformBlockFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceInlineUniformBlockFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES`
///# Related
/// - [`VK_EXT_inline_uniform_block`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceInlineUniformBlockFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`inline_uniform_block`]
    ///indicates whether the implementation supports inline uniform block
    ///descriptors.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`**must** not be used.
    inline_uniform_block: Bool32,
    ///[`descriptor_binding_inline_uniform_block_update_after_bind`]
    ///indicates whether the implementation supports updating inline uniform
    ///block descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
    descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}
impl<'lt> Default for PhysicalDeviceInlineUniformBlockFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            inline_uniform_block: 0,
            descriptor_binding_inline_uniform_block_update_after_bind: 0,
        }
    }
}
impl<'lt> PhysicalDeviceInlineUniformBlockFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block_raw(&self) -> Bool32 {
        self.inline_uniform_block
    }
    ///Gets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::inline_uniform_block`]
    pub fn set_inline_uniform_block_raw(&mut self, value: Bool32) -> &mut Self {
        self.inline_uniform_block = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind = value;
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
    ///Gets the value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block(&self) -> bool {
        unsafe { std::mem::transmute(self.inline_uniform_block as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_inline_uniform_block_update_after_bind as u8) }
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
    ///Gets a mutable reference to the value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.inline_uniform_block as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.inline_uniform_block as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_inline_uniform_block_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_inline_uniform_block_update_after_bind as *mut Bool32)
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
    ///Sets the raw value of [`Self::inline_uniform_block`]
    pub fn set_inline_uniform_block(&mut self, value: bool) -> &mut Self {
        self.inline_uniform_block = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceInlineUniformBlockProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInlineUniformBlockProperties.html) - Structure describing inline uniform block properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceInlineUniformBlockProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceInlineUniformBlockProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxInlineUniformBlockSize;
///    uint32_t           maxPerStageDescriptorInlineUniformBlocks;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks;
///    uint32_t           maxDescriptorSetInlineUniformBlocks;
///    uint32_t           maxDescriptorSetUpdateAfterBindInlineUniformBlocks;
///} VkPhysicalDeviceInlineUniformBlockProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_inline_uniform_block
///typedef VkPhysicalDeviceInlineUniformBlockProperties
/// VkPhysicalDeviceInlineUniformBlockPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_inline_uniform_block_size`] is the maximum size in bytes of an [inline uniform block](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inlineuniformblock)
///   binding.
/// - `maxPerStageDescriptorInlineUniformBlock` is the maximum number of inline uniform block
///   bindings that **can** be accessible to a single shader stage in a pipeline layout. Descriptor
///   bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against
///   this limit. Only descriptor bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`] is similar to
///   [`max_per_stage_descriptor_inline_uniform_blocks`] but counts descriptor bindings from
///   descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_inline_uniform_blocks`] is the maximum number of inline uniform block
///   bindings that **can** be included in descriptor bindings in a pipeline layout across all
///   pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type
///   of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit. Only descriptor
///   bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_descriptor_set_update_after_bind_inline_uniform_blocks`] is similar to
///   [`max_descriptor_set_inline_uniform_blocks`] but counts descriptor bindings from descriptor
///   sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`
///   bit set.
///If the [`PhysicalDeviceInlineUniformBlockProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES`
///# Related
/// - [`VK_EXT_inline_uniform_block`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceInlineUniformBlockProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    max_inline_uniform_block_size: u32,
    ///No documentation found
    max_per_stage_descriptor_inline_uniform_blocks: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    ///No documentation found
    max_descriptor_set_inline_uniform_blocks: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
impl<'lt> Default for PhysicalDeviceInlineUniformBlockProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_inline_uniform_block_size: 0,
            max_per_stage_descriptor_inline_uniform_blocks: 0,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: 0,
            max_descriptor_set_inline_uniform_blocks: 0,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: 0,
        }
    }
}
impl<'lt> PhysicalDeviceInlineUniformBlockProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size_raw(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Gets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Gets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Gets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Gets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn set_max_inline_uniform_block_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_raw(
        &mut self,
        value: u32,
    ) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_update_after_bind_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks = value;
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
    ///Gets the value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
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
    ///Gets a mutable reference to the value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn set_max_inline_uniform_block_size(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_update_after_bind_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks = value;
        self
    }
}
///[VkWriteDescriptorSetInlineUniformBlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetInlineUniformBlock.html) - Structure specifying inline uniform block data
///# C Specifications
///If the `descriptorType` member of [`WriteDescriptorSet`] is
///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then the data to write to the
///descriptor set is specified through a
///[`WriteDescriptorSetInlineUniformBlock`] structure included in the
///[`p_next`] chain of [`WriteDescriptorSet`].The [`WriteDescriptorSetInlineUniformBlock`]
/// structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkWriteDescriptorSetInlineUniformBlock {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           dataSize;
///    const void*        pData;
///} VkWriteDescriptorSetInlineUniformBlock;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_inline_uniform_block
///typedef VkWriteDescriptorSetInlineUniformBlock VkWriteDescriptorSetInlineUniformBlockEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`data_size`] is the number of bytes of inline uniform block data pointed to by [`data`].
/// - [`data`] is a pointer to [`data_size`] number of bytes of data to write to the inline uniform
///   block.
///# Description
///Valid Usage
/// - [`data_size`]**must** be an integer multiple of `4`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK`
/// - [`data`]**must** be a valid pointer to an array of [`data_size`] bytes
/// - [`data_size`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_inline_uniform_block`]
/// - [`crate::vulkan1_3`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct WriteDescriptorSetInlineUniformBlock<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`data_size`] is the number of bytes of inline uniform block data
    ///pointed to by [`data`].
    data_size: u32,
    ///[`data`] is a pointer to [`data_size`] number of bytes of data to
    ///write to the inline uniform block.
    data: *const c_void,
}
impl<'lt> Default for WriteDescriptorSetInlineUniformBlock<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            data_size: 0,
            data: std::ptr::null(),
        }
    }
}
impl<'lt> WriteDescriptorSetInlineUniformBlock<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::data_size`]
    pub fn data_size_raw(&self) -> u32 {
        self.data_size
    }
    ///Gets the raw value of [`Self::data`]
    pub fn data_raw(&self) -> *const c_void {
        self.data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::data_size`]
    pub fn set_data_size_raw(&mut self, value: u32) -> &mut Self {
        self.data_size = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data_raw(&mut self, value: *const c_void) -> &mut Self {
        self.data = value;
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
    ///Gets the value of [`Self::data_size`]
    pub fn data_size(&self) -> u32 {
        self.data_size
    }
    ///Gets the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.data, self.data_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::data_size`]
    pub fn data_size_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::data_size`]
    pub fn set_data_size(&mut self, value: u32) -> &mut Self {
        self.data_size = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.data = value.as_ptr();
        self.data_size = len_;
        self
    }
}
///[VkDescriptorPoolInlineUniformBlockCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPoolInlineUniformBlockCreateInfo.html) - Structure specifying the maximum number of inline uniform block bindings of a newly created descriptor pool
///# C Specifications
///In order to be able to allocate descriptor sets having
///[inline uniform block](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-inlineuniformblock) bindings the
///descriptor pool **must** be created with specifying the inline uniform block
///binding capacity of the descriptor pool, in addition to the total inline
///uniform data capacity in bytes which is specified through a
///[`DescriptorPoolSize`] structure with a `descriptorType` value of
///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
///This **can** be done by adding a
///[`DescriptorPoolInlineUniformBlockCreateInfo`] structure to the
///[`p_next`] chain of [`DescriptorPoolCreateInfo`].The
/// [`DescriptorPoolInlineUniformBlockCreateInfo`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkDescriptorPoolInlineUniformBlockCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           maxInlineUniformBlockBindings;
///} VkDescriptorPoolInlineUniformBlockCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_inline_uniform_block
///typedef VkDescriptorPoolInlineUniformBlockCreateInfo
/// VkDescriptorPoolInlineUniformBlockCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_inline_uniform_block_bindings`] is the number of inline uniform block bindings to
///   allocate.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO`
///# Related
/// - [`VK_EXT_inline_uniform_block`]
/// - [`crate::vulkan1_3`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct DescriptorPoolInlineUniformBlockCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`max_inline_uniform_block_bindings`] is the number of inline uniform
    ///block bindings to allocate.
    max_inline_uniform_block_bindings: u32,
}
impl<'lt> Default for DescriptorPoolInlineUniformBlockCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            max_inline_uniform_block_bindings: 0,
        }
    }
}
impl<'lt> DescriptorPoolInlineUniformBlockCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::max_inline_uniform_block_bindings`]
    pub fn max_inline_uniform_block_bindings_raw(&self) -> u32 {
        self.max_inline_uniform_block_bindings
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_block_bindings`]
    pub fn set_max_inline_uniform_block_bindings_raw(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_bindings = value;
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
    ///Gets the value of [`Self::max_inline_uniform_block_bindings`]
    pub fn max_inline_uniform_block_bindings(&self) -> u32 {
        self.max_inline_uniform_block_bindings
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::max_inline_uniform_block_bindings`]
    pub fn max_inline_uniform_block_bindings_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::max_inline_uniform_block_bindings`]
    pub fn set_max_inline_uniform_block_bindings(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_bindings = value;
        self
    }
}
///[VkPhysicalDeviceMaintenance4Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Features.html) - Structure describing whether the implementation supports maintenance4 functionality
///# C Specifications
///The [`PhysicalDeviceMaintenance4Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceMaintenance4Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           maintenance4;
///} VkPhysicalDeviceMaintenance4Features;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance4
///typedef VkPhysicalDeviceMaintenance4Features VkPhysicalDeviceMaintenance4FeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`maintenance_4`] indicates that the implementation supports the following:  - The application
///   **may** destroy a [`PipelineLayout`] object immediately after using it to create another
///   object.  - `LocalSizeId`**can** be used as an alternative to `LocalSize` to specify the local
///   workgroup size with specialization constants.  - Images created with identical creation
///   parameters will always have the same alignment requirements.  - The size memory requirement of
///   a buffer or image is never greater than that of another buffer or image created with a greater
///   or equal size.  - Push constants do not have to be initialized before they are dynamically
///   accessed.  - The interface matching rules allow a larger output vector to match with a smaller
///   input vector, with additional values being discarded.
///If the [`PhysicalDeviceMaintenance4Features`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMaintenance4Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`
///# Related
/// - [`VK_KHR_maintenance4`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceMaintenance4Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES`
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`maintenance_4`] indicates
    ///that the implementation supports the following:
    /// - The application **may** destroy a [`PipelineLayout`] object immediately after using it to
    ///   create another object.
    /// - `LocalSizeId`**can** be used as an alternative to `LocalSize` to specify the local
    ///   workgroup size with specialization constants.
    /// - Images created with identical creation parameters will always have the same alignment
    ///   requirements.
    /// - The size memory requirement of a buffer or image is never greater than that of another
    ///   buffer or image created with a greater or equal size.
    /// - Push constants do not have to be initialized before they are dynamically accessed.
    /// - The interface matching rules allow a larger output vector to match with a smaller input
    ///   vector, with additional values being discarded.
    maintenance_4: Bool32,
}
impl<'lt> Default for PhysicalDeviceMaintenance4Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            maintenance_4: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMaintenance4Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::maintenance_4`]
    pub fn maintenance_4_raw(&self) -> Bool32 {
        self.maintenance_4
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::maintenance_4`]
    pub fn set_maintenance_4_raw(&mut self, value: Bool32) -> &mut Self {
        self.maintenance_4 = value;
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
    ///Gets the value of [`Self::maintenance_4`]
    pub fn maintenance_4(&self) -> bool {
        unsafe { std::mem::transmute(self.maintenance_4 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::maintenance_4`]
    pub fn maintenance_4_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.maintenance_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.maintenance_4 as *mut Bool32)
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
    ///Sets the raw value of [`Self::maintenance_4`]
    pub fn set_maintenance_4(&mut self, value: bool) -> &mut Self {
        self.maintenance_4 = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceMaintenance4Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance4Properties.html) - Structure describing various implementation-defined properties introduced with VK_KHR_maintenance4
///# C Specifications
///The [`PhysicalDeviceMaintenance4Properties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceMaintenance4Properties {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       maxBufferSize;
///} VkPhysicalDeviceMaintenance4Properties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance4
///typedef VkPhysicalDeviceMaintenance4Properties VkPhysicalDeviceMaintenance4PropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_buffer_size`] is the maximum size [`Buffer`] that **can** be created.
///If the [`PhysicalDeviceMaintenance4Properties`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES`
///# Related
/// - [`VK_KHR_maintenance4`]
/// - [`crate::vulkan1_3`]
/// - [`DeviceSize`]
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
pub struct PhysicalDeviceMaintenance4Properties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    max_buffer_size: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceMaintenance4Properties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_buffer_size: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceMaintenance4Properties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::max_buffer_size`]
    pub fn max_buffer_size(&self) -> DeviceSize {
        self.max_buffer_size
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
    ///Gets a mutable reference to the value of [`Self::max_buffer_size`]
    pub fn max_buffer_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.max_buffer_size
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
    ///Sets the raw value of [`Self::max_buffer_size`]
    pub fn set_max_buffer_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.max_buffer_size = value;
        self
    }
}
///[VkPhysicalDeviceTextureCompressionASTCHDRFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTextureCompressionASTCHDRFeatures.html) - Structure describing ASTC HDR features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTextureCompressionAstchdrFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           textureCompressionASTC_HDR;
///} VkPhysicalDeviceTextureCompressionASTCHDRFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_texture_compression_astc_hdr
///typedef VkPhysicalDeviceTextureCompressionASTCHDRFeatures
/// VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR compressed texture
///   formats are supported. If this feature is enabled, then the
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features **must** be supported in
///   `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK` To query for additional properties, or if the feature is
///   not enabled, [`GetPhysicalDeviceFormatProperties`] and
///   [`GetPhysicalDeviceImageFormatProperties`]**can** be used to check for supported properties of
///   individual formats as normal.
///If the [`PhysicalDeviceTextureCompressionAstchdrFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceTextureCompressionAstchdrFeatures`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES`
///# Related
/// - [`VK_EXT_texture_compression_astc_hdr`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceTextureCompressionAstchdrFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR
    ///compressed texture formats are supported.
    ///If this feature is enabled, then the
    ///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
    ///`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
    ///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features **must**
    ///be supported in `optimalTilingFeatures` for the following formats:
    /// - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`
    ///To query for additional properties, or if the feature is not enabled,
    ///[`GetPhysicalDeviceFormatProperties`] and
    ///[`GetPhysicalDeviceImageFormatProperties`]**can** be used to check for
    ///supported properties of individual formats as normal.
    texture_compression_astc_hdr: Bool32,
}
impl<'lt> Default for PhysicalDeviceTextureCompressionAstchdrFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            texture_compression_astc_hdr: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTextureCompressionAstchdrFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr_raw(&self) -> Bool32 {
        self.texture_compression_astc_hdr
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn set_texture_compression_astc_hdr_raw(&mut self, value: Bool32) -> &mut Self {
        self.texture_compression_astc_hdr = value;
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
    ///Gets the value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr(&self) -> bool {
        unsafe { std::mem::transmute(self.texture_compression_astc_hdr as u8) }
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
    ///Gets a mutable reference to the value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.texture_compression_astc_hdr as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.texture_compression_astc_hdr as *mut Bool32)
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
    ///Sets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn set_texture_compression_astc_hdr(&mut self, value: bool) -> &mut Self {
        self.texture_compression_astc_hdr = value as u8 as u32;
        self
    }
}
///[VkPipelineCreationFeedback](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedback.html) - Feedback about the creation of a pipeline or pipeline stage
///# C Specifications
///The [`PipelineCreationFeedback`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPipelineCreationFeedback {
///    VkPipelineCreationFeedbackFlags    flags;
///    uint64_t                           duration;
///} VkPipelineCreationFeedback;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_pipeline_creation_feedback
///typedef VkPipelineCreationFeedback VkPipelineCreationFeedbackEXT;
///```
///# Members
/// - [`flags`] is a bitmask of [`PipelineCreationFeedbackFlagBits`] providing feedback about the
///   creation of a pipeline or of a pipeline stage.
/// - [`duration`] is the duration spent creating a pipeline or pipeline stage in nanoseconds.
///# Description
///If the `VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT` is not set in
///[`flags`], an implementation **must** not set any other bits in [`flags`],
///and the values of all other [`PipelineCreationFeedback`] data members
///are undefined.
///# Related
/// - [`VK_EXT_pipeline_creation_feedback`]
/// - [`crate::vulkan1_3`]
/// - [`PipelineCreationFeedbackCreateInfo`]
/// - [`PipelineCreationFeedbackFlagBits`]
/// - [`PipelineCreationFeedbackFlags`]
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
pub struct PipelineCreationFeedback {
    ///[`flags`] is a bitmask of [`PipelineCreationFeedbackFlagBits`]
    ///providing feedback about the creation of a pipeline or of a pipeline
    ///stage.
    flags: PipelineCreationFeedbackFlags,
    ///[`duration`] is the duration spent creating a pipeline or pipeline
    ///stage in nanoseconds.
    duration: u64,
}
impl Default for PipelineCreationFeedback {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            duration: 0,
        }
    }
}
impl PipelineCreationFeedback {
    ///Gets the raw value of [`Self::duration`]
    pub fn duration_raw(&self) -> u64 {
        self.duration
    }
    ///Sets the raw value of [`Self::duration`]
    pub fn set_duration_raw(&mut self, value: u64) -> &mut Self {
        self.duration = value;
        self
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineCreationFeedbackFlags {
        self.flags
    }
    ///Gets the value of [`Self::duration`]
    pub fn duration(&self) -> u64 {
        self.duration
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineCreationFeedbackFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::duration`]
    pub fn duration_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_3::PipelineCreationFeedbackFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::duration`]
    pub fn set_duration(&mut self, value: u64) -> &mut Self {
        self.duration = value;
        self
    }
}
///[VkPipelineCreationFeedbackCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCreationFeedbackCreateInfo.html) - Request for feedback about the creation of a pipeline
///# C Specifications
///Feedback about the creation of a particular pipeline object **can** be obtained
///by adding a [`PipelineCreationFeedbackCreateInfo`] structure to the
///[`p_next`] chain of [`GraphicsPipelineCreateInfo`],
///[`RayTracingPipelineCreateInfoKHR`],
///[`RayTracingPipelineCreateInfoNV`],
///or [`ComputePipelineCreateInfo`].
///The [`PipelineCreationFeedbackCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPipelineCreationFeedbackCreateInfo {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkPipelineCreationFeedback*    pPipelineCreationFeedback;
///    uint32_t                       pipelineStageCreationFeedbackCount;
///    VkPipelineCreationFeedback*    pPipelineStageCreationFeedbacks;
///} VkPipelineCreationFeedbackCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_pipeline_creation_feedback
///typedef VkPipelineCreationFeedbackCreateInfo VkPipelineCreationFeedbackCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_creation_feedback`] is a pointer to a [`PipelineCreationFeedback`] structure.
/// - [`pipeline_stage_creation_feedback_count`] is the number of elements in
///   [`pipeline_stage_creation_feedbacks`].
/// - [`pipeline_stage_creation_feedbacks`] is a pointer to an array of
///   [`pipeline_stage_creation_feedback_count`][`PipelineCreationFeedback`] structures.
///# Description
///An implementation **should** write pipeline creation feedback to
///[`pipeline_creation_feedback`] and **may** write pipeline stage creation
///feedback to [`pipeline_stage_creation_feedbacks`].
///An implementation **must** set or clear the
///`VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT` in
///[`PipelineCreationFeedback::flags`] for
///[`pipeline_creation_feedback`] and every element of
///[`pipeline_stage_creation_feedbacks`].When chained to
///[`RayTracingPipelineCreateInfoKHR`],
///[`RayTracingPipelineCreateInfoNV`],
///or
///[`GraphicsPipelineCreateInfo`], the `i` element of
///[`pipeline_stage_creation_feedbacks`] corresponds to the `i` element of
///[`RayTracingPipelineCreateInfoKHR::stages`],
///[`RayTracingPipelineCreateInfoNV::stages`],
///or
///[`GraphicsPipelineCreateInfo::stages`].
///When chained to [`ComputePipelineCreateInfo`], the first element of
///[`pipeline_stage_creation_feedbacks`] corresponds to
///[`ComputePipelineCreateInfo::stage`].Valid Usage
/// - When chained to [`GraphicsPipelineCreateInfo`],
///   [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`]**must** equal
///   [`GraphicsPipelineCreateInfo::stage_count`]
/// - When chained to [`ComputePipelineCreateInfo`],
///   [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`]**must** equal 1
/// - When chained to [`RayTracingPipelineCreateInfoKHR`],
///   [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`]**must** equal
///   [`RayTracingPipelineCreateInfoKHR::stage_count`]
/// - When chained to [`RayTracingPipelineCreateInfoNV`],
///   [`PipelineCreationFeedback`]::[`pipeline_stage_creation_feedback_count`]**must** equal
///   [`RayTracingPipelineCreateInfoNV::stage_count`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO`
/// - [`pipeline_creation_feedback`]**must** be a valid pointer to a [`PipelineCreationFeedback`]
///   structure
/// - [`pipeline_stage_creation_feedbacks`]**must** be a valid pointer to an array of
///   [`pipeline_stage_creation_feedback_count`][`PipelineCreationFeedback`] structures
/// - [`pipeline_stage_creation_feedback_count`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_pipeline_creation_feedback`]
/// - [`crate::vulkan1_3`]
/// - [`ComputePipelineCreateInfo`]
/// - [`GraphicsPipelineCreateInfo`]
/// - [`PipelineCreationFeedback`]
/// - [`RayTracingPipelineCreateInfoKHR`]
/// - [`RayTracingPipelineCreateInfoNV`]
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
pub struct PipelineCreationFeedbackCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`pipeline_creation_feedback`] is a pointer to a
    ///[`PipelineCreationFeedback`] structure.
    pipeline_creation_feedback: *mut PipelineCreationFeedback,
    ///[`pipeline_stage_creation_feedback_count`] is the number of elements in
    ///[`pipeline_stage_creation_feedbacks`].
    pipeline_stage_creation_feedback_count: u32,
    ///[`pipeline_stage_creation_feedbacks`] is a pointer to an array of
    ///[`pipeline_stage_creation_feedback_count`][`PipelineCreationFeedback`] structures.
    pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}
impl<'lt> Default for PipelineCreationFeedbackCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            pipeline_creation_feedback: std::ptr::null_mut(),
            pipeline_stage_creation_feedback_count: 0,
            pipeline_stage_creation_feedbacks: std::ptr::null_mut(),
        }
    }
}
impl<'lt> PipelineCreationFeedbackCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::pipeline_creation_feedback`]
    pub fn pipeline_creation_feedback_raw(&self) -> &*mut PipelineCreationFeedback {
        &self.pipeline_creation_feedback
    }
    ///Gets the raw value of [`Self::pipeline_stage_creation_feedback_count`]
    pub fn pipeline_stage_creation_feedback_count_raw(&self) -> u32 {
        self.pipeline_stage_creation_feedback_count
    }
    ///Gets the raw value of [`Self::pipeline_stage_creation_feedbacks`]
    pub fn pipeline_stage_creation_feedbacks_raw(&self) -> &*mut PipelineCreationFeedback {
        &self.pipeline_stage_creation_feedbacks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_creation_feedback`]
    pub fn set_pipeline_creation_feedback_raw(&mut self, value: *mut PipelineCreationFeedback) -> &mut Self {
        self.pipeline_creation_feedback = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_stage_creation_feedback_count`]
    pub fn set_pipeline_stage_creation_feedback_count_raw(&mut self, value: u32) -> &mut Self {
        self.pipeline_stage_creation_feedback_count = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_stage_creation_feedbacks`]
    pub fn set_pipeline_stage_creation_feedbacks_raw(&mut self, value: *mut PipelineCreationFeedback) -> &mut Self {
        self.pipeline_stage_creation_feedbacks = value;
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
    ///Gets the value of [`Self::pipeline_creation_feedback`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pipeline_creation_feedback(&self) -> &PipelineCreationFeedback {
        &*self.pipeline_creation_feedback
    }
    ///Gets the value of [`Self::pipeline_stage_creation_feedback_count`]
    pub fn pipeline_stage_creation_feedback_count(&self) -> u32 {
        self.pipeline_stage_creation_feedback_count
    }
    ///Gets the value of [`Self::pipeline_stage_creation_feedbacks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pipeline_stage_creation_feedbacks(&self) -> &[PipelineCreationFeedback] {
        std::slice::from_raw_parts(
            self.pipeline_stage_creation_feedbacks,
            self.pipeline_stage_creation_feedback_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_creation_feedback`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pipeline_creation_feedback_mut(&mut self) -> &mut PipelineCreationFeedback {
        &mut *self.pipeline_creation_feedback
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_stage_creation_feedback_count`]
    pub fn pipeline_stage_creation_feedback_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_stage_creation_feedbacks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pipeline_stage_creation_feedbacks_mut(&mut self) -> &mut [PipelineCreationFeedback] {
        std::slice::from_raw_parts_mut(
            self.pipeline_stage_creation_feedbacks,
            self.pipeline_stage_creation_feedback_count as usize,
        )
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
    ///Sets the raw value of [`Self::pipeline_creation_feedback`]
    pub fn set_pipeline_creation_feedback(
        &mut self,
        value: &'lt mut crate::vulkan1_3::PipelineCreationFeedback,
    ) -> &mut Self {
        self.pipeline_creation_feedback = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::pipeline_stage_creation_feedback_count`]
    pub fn set_pipeline_stage_creation_feedback_count(&mut self, value: u32) -> &mut Self {
        self.pipeline_stage_creation_feedback_count = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_stage_creation_feedbacks`]
    pub fn set_pipeline_stage_creation_feedbacks(
        &mut self,
        value: &'lt mut [crate::vulkan1_3::PipelineCreationFeedback],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pipeline_stage_creation_feedbacks = value.as_mut_ptr();
        self.pipeline_stage_creation_feedback_count = len_;
        self
    }
}
///[VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures.html) - Structure describing the shader demote to helper invocations features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`] structure
///is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderDemoteToHelperInvocation;
///} VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_shader_demote_to_helper_invocation
///typedef VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures
/// VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_demote_to_helper_invocation`] indicates whether the implementation supports the
///   SPIR-V `DemoteToHelperInvocationEXT` capability.
///If the [`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderDemoteToHelperInvocationFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES`
///# Related
/// - [`VK_EXT_shader_demote_to_helper_invocation`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_demote_to_helper_invocation`] indicates whether the
    ///implementation supports the SPIR-V `DemoteToHelperInvocationEXT`
    ///capability.
    shader_demote_to_helper_invocation: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_demote_to_helper_invocation: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation_raw(&self) -> Bool32 {
        self.shader_demote_to_helper_invocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn set_shader_demote_to_helper_invocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_demote_to_helper_invocation = value;
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
    ///Gets the value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_demote_to_helper_invocation as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_demote_to_helper_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_demote_to_helper_invocation as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn set_shader_demote_to_helper_invocation(&mut self, value: bool) -> &mut Self {
        self.shader_demote_to_helper_invocation = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceTexelBufferAlignmentProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentProperties.html) - Structure describing the texel buffer alignment requirements supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTexelBufferAlignmentProperties`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceTexelBufferAlignmentProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       storageTexelBufferOffsetAlignmentBytes;
///    VkBool32           storageTexelBufferOffsetSingleTexelAlignment;
///    VkDeviceSize       uniformTexelBufferOffsetAlignmentBytes;
///    VkBool32           uniformTexelBufferOffsetSingleTexelAlignment;
///} VkPhysicalDeviceTexelBufferAlignmentProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_texel_buffer_alignment
///typedef VkPhysicalDeviceTexelBufferAlignmentProperties
/// VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`storage_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a
///   storage texel buffer of any format. The value **must** be a power of two.
/// - [`storage_texel_buffer_offset_single_texel_alignment`] indicates whether single texel
///   alignment is sufficient for a storage texel buffer of any format. The value **must** be a
///   power of two.
/// - [`uniform_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a
///   uniform texel buffer of any format. The value **must** be a power of two.
/// - [`uniform_texel_buffer_offset_single_texel_alignment`] indicates whether single texel
///   alignment is sufficient for a uniform texel buffer of any format. The value **must** be a
///   power of two.
///If the [`PhysicalDeviceTexelBufferAlignmentProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.If the single texel alignment property is
/// [`FALSE`], then the buffer
///view’s offset **must** be aligned to the corresponding byte alignment value.
///If the single texel alignment property is [`TRUE`], then the buffer
///view’s offset **must** be aligned to the lesser of the corresponding byte
///alignment value or the size of a single texel, based on
///[`BufferViewCreateInfo::format`].
///If the size of a single texel is a multiple of three bytes, then the size of
///a single component of the format is used instead.These limits **must** not advertise a larger
/// alignment than the
///[required](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-required) maximum minimum value of
///[`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`], for any
///format that supports use as a texel buffer.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES`
///# Related
/// - [`VK_EXT_texel_buffer_alignment`]
/// - [`crate::vulkan1_3`]
/// - [`Bool32`]
/// - [`DeviceSize`]
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
pub struct PhysicalDeviceTexelBufferAlignmentProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    ///No documentation found
    storage_texel_buffer_offset_single_texel_alignment: Bool32,
    ///No documentation found
    uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    ///No documentation found
    uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}
impl<'lt> Default for PhysicalDeviceTexelBufferAlignmentProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            storage_texel_buffer_offset_alignment_bytes: Default::default(),
            storage_texel_buffer_offset_single_texel_alignment: 0,
            uniform_texel_buffer_offset_alignment_bytes: Default::default(),
            uniform_texel_buffer_offset_single_texel_alignment: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTexelBufferAlignmentProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment_raw(&self) -> Bool32 {
        self.storage_texel_buffer_offset_single_texel_alignment
    }
    ///Gets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment_raw(&self) -> Bool32 {
        self.uniform_texel_buffer_offset_single_texel_alignment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn set_storage_texel_buffer_offset_single_texel_alignment_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_texel_buffer_offset_single_texel_alignment = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn set_uniform_texel_buffer_offset_single_texel_alignment_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_texel_buffer_offset_single_texel_alignment = value;
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
    ///Gets the value of [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn storage_texel_buffer_offset_alignment_bytes(&self) -> DeviceSize {
        self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Gets the value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_texel_buffer_offset_single_texel_alignment as u8) }
    }
    ///Gets the value of [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn uniform_texel_buffer_offset_alignment_bytes(&self) -> DeviceSize {
        self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Gets the value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_texel_buffer_offset_single_texel_alignment as u8) }
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
    ///Gets a mutable reference to the value of
    /// [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn storage_texel_buffer_offset_alignment_bytes_mut(&mut self) -> &mut DeviceSize {
        &mut self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Gets a mutable reference to the value of
    /// [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn uniform_texel_buffer_offset_alignment_bytes_mut(&mut self) -> &mut DeviceSize {
        &mut self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Gets a mutable reference to the value of
    /// [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_texel_buffer_offset_single_texel_alignment as *mut Bool32)
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
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn set_storage_texel_buffer_offset_alignment_bytes(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.storage_texel_buffer_offset_alignment_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn set_storage_texel_buffer_offset_single_texel_alignment(&mut self, value: bool) -> &mut Self {
        self.storage_texel_buffer_offset_single_texel_alignment = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn set_uniform_texel_buffer_offset_alignment_bytes(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.uniform_texel_buffer_offset_alignment_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn set_uniform_texel_buffer_offset_single_texel_alignment(&mut self, value: bool) -> &mut Self {
        self.uniform_texel_buffer_offset_single_texel_alignment = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceSubgroupSizeControlFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeatures.html) - Structure describing the subgroup size control features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceSubgroupSizeControlFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceSubgroupSizeControlFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           subgroupSizeControl;
///    VkBool32           computeFullSubgroups;
///} VkPhysicalDeviceSubgroupSizeControlFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_subgroup_size_control
///typedef VkPhysicalDeviceSubgroupSizeControlFeatures
/// VkPhysicalDeviceSubgroupSizeControlFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`subgroup_size_control`] indicates whether the implementation supports controlling shader
///   subgroup sizes via the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag
///   and the [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure.
/// - [`compute_full_subgroups`] indicates whether the implementation supports requiring full
///   subgroups in compute shaders via the
///   `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag.
///If the [`PhysicalDeviceSubgroupSizeControlFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSubgroupSizeControlFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES`
///# Related
/// - [`VK_EXT_subgroup_size_control`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceSubgroupSizeControlFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`subgroup_size_control`] indicates whether the implementation supports
    ///controlling shader subgroup sizes via the
    ///`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`
    ///flag and the [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`]
    ///structure.
    subgroup_size_control: Bool32,
    ///[`compute_full_subgroups`] indicates whether the implementation supports
    ///requiring full subgroups in compute shaders via the
    ///`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag.
    compute_full_subgroups: Bool32,
}
impl<'lt> Default for PhysicalDeviceSubgroupSizeControlFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            subgroup_size_control: 0,
            compute_full_subgroups: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubgroupSizeControlFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control_raw(&self) -> Bool32 {
        self.subgroup_size_control
    }
    ///Gets the raw value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups_raw(&self) -> Bool32 {
        self.compute_full_subgroups
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_size_control`]
    pub fn set_subgroup_size_control_raw(&mut self, value: Bool32) -> &mut Self {
        self.subgroup_size_control = value;
        self
    }
    ///Sets the raw value of [`Self::compute_full_subgroups`]
    pub fn set_compute_full_subgroups_raw(&mut self, value: Bool32) -> &mut Self {
        self.compute_full_subgroups = value;
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
    ///Gets the value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control(&self) -> bool {
        unsafe { std::mem::transmute(self.subgroup_size_control as u8) }
    }
    ///Gets the value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups(&self) -> bool {
        unsafe { std::mem::transmute(self.compute_full_subgroups as u8) }
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
    ///Gets a mutable reference to the value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subgroup_size_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subgroup_size_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.compute_full_subgroups as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.compute_full_subgroups as *mut Bool32)
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
    ///Sets the raw value of [`Self::subgroup_size_control`]
    pub fn set_subgroup_size_control(&mut self, value: bool) -> &mut Self {
        self.subgroup_size_control = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::compute_full_subgroups`]
    pub fn set_compute_full_subgroups(&mut self, value: bool) -> &mut Self {
        self.compute_full_subgroups = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceSubgroupSizeControlProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlProperties.html) - Structure describing the control subgroup size properties of an implementation
///# C Specifications
///The [`PhysicalDeviceSubgroupSizeControlProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceSubgroupSizeControlProperties {
///    VkStructureType       sType;
///    void*                 pNext;
///    uint32_t              minSubgroupSize;
///    uint32_t              maxSubgroupSize;
///    uint32_t              maxComputeWorkgroupSubgroups;
///    VkShaderStageFlags    requiredSubgroupSizeStages;
///} VkPhysicalDeviceSubgroupSizeControlProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_subgroup_size_control
///typedef VkPhysicalDeviceSubgroupSizeControlProperties
/// VkPhysicalDeviceSubgroupSizeControlPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`min_subgroup_size`] is the minimum subgroup size supported by this device. [`min_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`min_subgroup_size`] is a power-of-two. [`min_subgroup_size`] is less than or equal to [`max_subgroup_size`]. [`min_subgroup_size`] is less than or equal to [subgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subgroup-size).
/// - [`max_subgroup_size`] is the maximum subgroup size supported by this device. [`max_subgroup_size`] is at least one if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`max_subgroup_size`] is a power-of-two. [`max_subgroup_size`] is greater than or equal to [`min_subgroup_size`]. [`max_subgroup_size`] is greater than or equal to [subgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-subgroup-size).
/// - [`max_compute_workgroup_subgroups`] is the maximum number of subgroups supported by the
///   implementation within a workgroup.
/// - [`required_subgroup_size_stages`] is a bitfield of what shader stages support having a
///   required subgroup size specified.
///If the [`PhysicalDeviceSubgroupSizeControlProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.If
/// [`PhysicalDeviceSubgroupProperties::supported_operations`]
///includes [`VK_SUBGROUP_FEATURE_QUAD_BIT`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-subgroup-quad),
///[`min_subgroup_size`]**must** be greater than or equal to 4.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES`
///# Related
/// - [`VK_EXT_subgroup_size_control`]
/// - [`crate::vulkan1_3`]
/// - [`ShaderStageFlags`]
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
pub struct PhysicalDeviceSubgroupSizeControlProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    min_subgroup_size: u32,
    ///No documentation found
    max_subgroup_size: u32,
    ///No documentation found
    max_compute_workgroup_subgroups: u32,
    ///No documentation found
    required_subgroup_size_stages: ShaderStageFlags,
}
impl<'lt> Default for PhysicalDeviceSubgroupSizeControlProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            min_subgroup_size: 0,
            max_subgroup_size: 0,
            max_compute_workgroup_subgroups: 0,
            required_subgroup_size_stages: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceSubgroupSizeControlProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size_raw(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Gets the raw value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size_raw(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Gets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups_raw(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_subgroup_size`]
    pub fn set_min_subgroup_size_raw(&mut self, value: u32) -> &mut Self {
        self.min_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_subgroup_size`]
    pub fn set_max_subgroup_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn set_max_compute_workgroup_subgroups_raw(&mut self, value: u32) -> &mut Self {
        self.max_compute_workgroup_subgroups = value;
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
    ///Gets the value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Gets the value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Gets the value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Gets the value of [`Self::required_subgroup_size_stages`]
    pub fn required_subgroup_size_stages(&self) -> ShaderStageFlags {
        self.required_subgroup_size_stages
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
    ///Gets a mutable reference to the value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::required_subgroup_size_stages`]
    pub fn required_subgroup_size_stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.required_subgroup_size_stages
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
    ///Sets the raw value of [`Self::min_subgroup_size`]
    pub fn set_min_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.min_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_subgroup_size`]
    pub fn set_max_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.max_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn set_max_compute_workgroup_subgroups(&mut self, value: u32) -> &mut Self {
        self.max_compute_workgroup_subgroups = value;
        self
    }
    ///Sets the raw value of [`Self::required_subgroup_size_stages`]
    pub fn set_required_subgroup_size_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.required_subgroup_size_stages = value;
        self
    }
}
///[VkPipelineShaderStageRequiredSubgroupSizeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfo.html) - Structure specifying the required subgroup size of a newly created pipeline shader stage
///# C Specifications
///The [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           requiredSubgroupSize;
///} VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_subgroup_size_control
///typedef VkPipelineShaderStageRequiredSubgroupSizeCreateInfo
/// VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`required_subgroup_size`] is an unsigned integer value specifying the required subgroup size
///   for the newly created pipeline shader stage.
///# Description
///If a [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure is
///included in the [`p_next`] chain of [`PipelineShaderStageCreateInfo`],
///it specifies that the pipeline shader stage being compiled has a required
///subgroup size.Valid Usage
/// - [`required_subgroup_size`]**must** be a power-of-two integer
/// - [`required_subgroup_size`]**must** be greater or equal to [minSubgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-minSubgroupSize)
/// - [`required_subgroup_size`]**must** be less than or equal to [maxSubgroupSize](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxSubgroupSize)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO`
///# Related
/// - [`VK_EXT_subgroup_size_control`]
/// - [`crate::vulkan1_3`]
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
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`required_subgroup_size`] is an
    ///unsigned integer value specifying the required subgroup size for the
    ///newly created pipeline shader stage.
    required_subgroup_size: u32,
}
impl<'lt> Default for PipelineShaderStageRequiredSubgroupSizeCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            required_subgroup_size: 0,
        }
    }
}
impl<'lt> PipelineShaderStageRequiredSubgroupSizeCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::required_subgroup_size`]
    pub fn required_subgroup_size_raw(&self) -> u32 {
        self.required_subgroup_size
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::required_subgroup_size`]
    pub fn set_required_subgroup_size_raw(&mut self, value: u32) -> &mut Self {
        self.required_subgroup_size = value;
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
    ///Gets the value of [`Self::required_subgroup_size`]
    pub fn required_subgroup_size(&self) -> u32 {
        self.required_subgroup_size
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
    ///Gets a mutable reference to the value of [`Self::required_subgroup_size`]
    pub fn required_subgroup_size_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::required_subgroup_size`]
    pub fn set_required_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.required_subgroup_size = value;
        self
    }
}
///[VkPhysicalDevicePipelineCreationCacheControlFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineCreationCacheControlFeatures.html) - Structure describing whether pipeline cache control can be supported by an implementation
///# C Specifications
///The [`PhysicalDevicePipelineCreationCacheControlFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pipelineCreationCacheControl;
///} VkPhysicalDevicePipelineCreationCacheControlFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_pipeline_creation_cache_control
///typedef VkPhysicalDevicePipelineCreationCacheControlFeatures
/// VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`pipeline_creation_cache_control`] indicates that the implementation supports:  - The
///   following **can** be used in `Vk*PipelineCreateInfo`::`flags`:   -
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   -
///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`   - The following **can** be used in
///   [`PipelineCacheCreateInfo::flags`]:   - `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`
///If the [`PhysicalDevicePipelineCreationCacheControlFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePipelineCreationCacheControlFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES`
///# Related
/// - [`VK_EXT_pipeline_creation_cache_control`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDevicePipelineCreationCacheControlFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`pipeline_creation_cache_control`] indicates that the implementation
    ///supports:
    /// - The following **can** be used in `Vk*PipelineCreateInfo`::`flags`:   -
    ///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   -
    ///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
    /// - The following **can** be used in [`PipelineCacheCreateInfo`]::`flags`:   -
    ///   `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`
    pipeline_creation_cache_control: Bool32,
}
impl<'lt> Default for PhysicalDevicePipelineCreationCacheControlFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            pipeline_creation_cache_control: 0,
        }
    }
}
impl<'lt> PhysicalDevicePipelineCreationCacheControlFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control_raw(&self) -> Bool32 {
        self.pipeline_creation_cache_control
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn set_pipeline_creation_cache_control_raw(&mut self, value: Bool32) -> &mut Self {
        self.pipeline_creation_cache_control = value;
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
    ///Gets the value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control(&self) -> bool {
        unsafe { std::mem::transmute(self.pipeline_creation_cache_control as u8) }
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
    ///Gets a mutable reference to the value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pipeline_creation_cache_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pipeline_creation_cache_control as *mut Bool32)
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
    ///Sets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn set_pipeline_creation_cache_control(&mut self, value: bool) -> &mut Self {
        self.pipeline_creation_cache_control = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceVulkan13Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Features.html) - Structure describing the Vulkan 1.3 features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceVulkan13Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceVulkan13Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           robustImageAccess;
///    VkBool32           inlineUniformBlock;
///    VkBool32           descriptorBindingInlineUniformBlockUpdateAfterBind;
///    VkBool32           pipelineCreationCacheControl;
///    VkBool32           privateData;
///    VkBool32           shaderDemoteToHelperInvocation;
///    VkBool32           shaderTerminateInvocation;
///    VkBool32           subgroupSizeControl;
///    VkBool32           computeFullSubgroups;
///    VkBool32           synchronization2;
///    VkBool32           textureCompressionASTC_HDR;
///    VkBool32           shaderZeroInitializeWorkgroupMemory;
///    VkBool32           dynamicRendering;
///    VkBool32           shaderIntegerDotProduct;
///    VkBool32           maintenance4;
///} VkPhysicalDeviceVulkan13Features;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`robust_image_access`] indicates whether image accesses are tightly bounds-checked against
///   the dimensions of the image view. [Invalid texels]() resulting from out of bounds image loads
///   will be replaced as described in [Texel Replacement](), with either (0,0,1) or (0,0,0) values
///   inserted for missing G, B, or A components based on the format.
/// - [`inline_uniform_block`] indicates whether the implementation supports inline uniform block
///   descriptors. If this feature is not enabled, `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`**must**
///   not be used.
/// - [`descriptor_binding_inline_uniform_block_update_after_bind`] indicates whether the
///   implementation supports updating inline uniform block descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
/// - [`pipeline_creation_cache_control`] indicates that the implementation supports:  - The
///   following **can** be used in `Vk*PipelineCreateInfo`::`flags`:   -
///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   -
///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`   - The following **can** be used in
///   [`PipelineCacheCreateInfo::flags`]:   - `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`
/// - [`private_data`] indicates whether the implementation supports private data. See [Private
///   Data]().
/// - [`shader_demote_to_helper_invocation`] indicates whether the implementation supports the
///   SPIR-V `DemoteToHelperInvocationEXT` capability.
/// - [`shader_terminate_invocation`] specifies whether the implementation supports SPIR-V modules
///   that use the `SPV_KHR_terminate_invocation` extension.
/// - [`subgroup_size_control`] indicates whether the implementation supports controlling shader
///   subgroup sizes via the `VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT` flag
///   and the [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`] structure.
/// - [`compute_full_subgroups`] indicates whether the implementation supports requiring full
///   subgroups in compute shaders via the
///   `VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag.
/// - [`synchronization_2`] indicates whether the implementation supports the new set of
///   synchronization commands introduced in `[`VK_KHR_synchronization2`]`.
/// - [`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR compressed texture
///   formats are supported. If this feature is enabled, then the
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`, `VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features **must** be supported in
///   `optimalTilingFeatures` for the following formats:  - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`  - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`  -
///   `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK` To query for additional properties, or if the feature is
///   not enabled, [`GetPhysicalDeviceFormatProperties`] and
///   [`GetPhysicalDeviceImageFormatProperties`]**can** be used to check for supported properties of
///   individual formats as normal.
/// - [`shader_zero_initialize_workgroup_memory`] specifies whether the implementation supports
///   initializing a variable in Workgroup storage class.
/// - [`dynamic_rendering`] specifies that the implementation supports dynamic render pass instances
///   using the [`CmdBeginRendering`] command.
/// - [`shader_integer_dot_product`] specifies whether shader modules **can** declare the
///   `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`, `DotProductInput4x8BitPackedKHR` and
///   `DotProductKHR` capabilities.
/// - [`maintenance_4`] indicates that the implementation supports the following:  - The application
///   **may** destroy a [`PipelineLayout`] object immediately after using it to create another
///   object.  - `LocalSizeId`**can** be used as an alternative to `LocalSize` to specify the local
///   workgroup size with specialization constants.  - Images created with identical creation
///   parameters will always have the same alignment requirements.  - The size memory requirement of
///   a buffer or image is never greater than that of another buffer or image created with a greater
///   or equal size.  - Push constants do not have to be initialized before they are dynamically
///   accessed.  - The interface matching rules allow a larger output vector to match with a smaller
///   input vector, with additional values being discarded.
///If the [`PhysicalDeviceVulkan13Features`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkan13Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES`
///# Related
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceVulkan13Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`robust_image_access`]
    ///indicates whether image accesses are tightly bounds-checked against the
    ///dimensions of the image view.
    ///[Invalid texels]() resulting from out of
    ///bounds image loads will be replaced as described in
    ///[Texel Replacement](), with either
    ///(0,0,1) or (0,0,0) values inserted for missing G, B, or A
    ///components based on the format.
    robust_image_access: Bool32,
    ///[`inline_uniform_block`]
    ///indicates whether the implementation supports inline uniform block
    ///descriptors.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`**must** not be used.
    inline_uniform_block: Bool32,
    ///[`descriptor_binding_inline_uniform_block_update_after_bind`]
    ///indicates whether the implementation supports updating inline uniform
    ///block descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
    descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    ///[`pipeline_creation_cache_control`] indicates that the implementation
    ///supports:
    /// - The following **can** be used in `Vk*PipelineCreateInfo`::`flags`:   -
    ///   `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT`   -
    ///   `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT`
    /// - The following **can** be used in [`PipelineCacheCreateInfo`]::`flags`:   -
    ///   `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT`
    pipeline_creation_cache_control: Bool32,
    ///[`private_data`] indicates
    ///whether the implementation supports private data.
    ///See [Private Data]().
    private_data: Bool32,
    ///[`shader_demote_to_helper_invocation`] indicates whether the
    ///implementation supports the SPIR-V `DemoteToHelperInvocationEXT`
    ///capability.
    shader_demote_to_helper_invocation: Bool32,
    ///[`shader_terminate_invocation`] specifies whether the implementation
    ///supports SPIR-V modules that use the `SPV_KHR_terminate_invocation`
    ///extension.
    shader_terminate_invocation: Bool32,
    ///[`subgroup_size_control`] indicates whether the implementation supports
    ///controlling shader subgroup sizes via the
    ///`VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT`
    ///flag and the [`PipelineShaderStageRequiredSubgroupSizeCreateInfo`]
    ///structure.
    subgroup_size_control: Bool32,
    ///[`compute_full_subgroups`] indicates whether the implementation supports
    ///requiring full subgroups in compute shaders via the
    ///`VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT` flag.
    compute_full_subgroups: Bool32,
    ///[`synchronization_2`]
    ///indicates whether the implementation supports the new set of
    ///synchronization commands introduced in `[`VK_KHR_synchronization2`]`.
    synchronization_2: Bool32,
    ///[`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR
    ///compressed texture formats are supported.
    ///If this feature is enabled, then the
    ///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
    ///`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
    ///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features **must**
    ///be supported in `optimalTilingFeatures` for the following formats:
    /// - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`
    /// - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`
    ///To query for additional properties, or if the feature is not enabled,
    ///[`GetPhysicalDeviceFormatProperties`] and
    ///[`GetPhysicalDeviceImageFormatProperties`]**can** be used to check for
    ///supported properties of individual formats as normal.
    texture_compression_astc_hdr: Bool32,
    ///[`shader_zero_initialize_workgroup_memory`] specifies whether the
    ///implementation supports initializing a variable in Workgroup storage
    ///class.
    shader_zero_initialize_workgroup_memory: Bool32,
    ///[`dynamic_rendering`]
    ///specifies that the implementation supports dynamic render pass instances
    ///using the [`CmdBeginRendering`] command.
    dynamic_rendering: Bool32,
    ///[`shader_integer_dot_product`] specifies whether shader modules **can**
    ///declare the `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`,
    ///`DotProductInput4x8BitPackedKHR` and `DotProductKHR` capabilities.
    shader_integer_dot_product: Bool32,
    ///[`maintenance_4`] indicates
    ///that the implementation supports the following:
    /// - The application **may** destroy a [`PipelineLayout`] object immediately after using it to
    ///   create another object.
    /// - `LocalSizeId`**can** be used as an alternative to `LocalSize` to specify the local
    ///   workgroup size with specialization constants.
    /// - Images created with identical creation parameters will always have the same alignment
    ///   requirements.
    /// - The size memory requirement of a buffer or image is never greater than that of another
    ///   buffer or image created with a greater or equal size.
    /// - Push constants do not have to be initialized before they are dynamically accessed.
    /// - The interface matching rules allow a larger output vector to match with a smaller input
    ///   vector, with additional values being discarded.
    maintenance_4: Bool32,
}
impl<'lt> Default for PhysicalDeviceVulkan13Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            robust_image_access: 0,
            inline_uniform_block: 0,
            descriptor_binding_inline_uniform_block_update_after_bind: 0,
            pipeline_creation_cache_control: 0,
            private_data: 0,
            shader_demote_to_helper_invocation: 0,
            shader_terminate_invocation: 0,
            subgroup_size_control: 0,
            compute_full_subgroups: 0,
            synchronization_2: 0,
            texture_compression_astc_hdr: 0,
            shader_zero_initialize_workgroup_memory: 0,
            dynamic_rendering: 0,
            shader_integer_dot_product: 0,
            maintenance_4: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVulkan13Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::robust_image_access`]
    pub fn robust_image_access_raw(&self) -> Bool32 {
        self.robust_image_access
    }
    ///Gets the raw value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block_raw(&self) -> Bool32 {
        self.inline_uniform_block
    }
    ///Gets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_inline_uniform_block_update_after_bind
    }
    ///Gets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control_raw(&self) -> Bool32 {
        self.pipeline_creation_cache_control
    }
    ///Gets the raw value of [`Self::private_data`]
    pub fn private_data_raw(&self) -> Bool32 {
        self.private_data
    }
    ///Gets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation_raw(&self) -> Bool32 {
        self.shader_demote_to_helper_invocation
    }
    ///Gets the raw value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation_raw(&self) -> Bool32 {
        self.shader_terminate_invocation
    }
    ///Gets the raw value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control_raw(&self) -> Bool32 {
        self.subgroup_size_control
    }
    ///Gets the raw value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups_raw(&self) -> Bool32 {
        self.compute_full_subgroups
    }
    ///Gets the raw value of [`Self::synchronization_2`]
    pub fn synchronization_2_raw(&self) -> Bool32 {
        self.synchronization_2
    }
    ///Gets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr_raw(&self) -> Bool32 {
        self.texture_compression_astc_hdr
    }
    ///Gets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory_raw(&self) -> Bool32 {
        self.shader_zero_initialize_workgroup_memory
    }
    ///Gets the raw value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering_raw(&self) -> Bool32 {
        self.dynamic_rendering
    }
    ///Gets the raw value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product_raw(&self) -> Bool32 {
        self.shader_integer_dot_product
    }
    ///Gets the raw value of [`Self::maintenance_4`]
    pub fn maintenance_4_raw(&self) -> Bool32 {
        self.maintenance_4
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::robust_image_access`]
    pub fn set_robust_image_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_image_access = value;
        self
    }
    ///Sets the raw value of [`Self::inline_uniform_block`]
    pub fn set_inline_uniform_block_raw(&mut self, value: Bool32) -> &mut Self {
        self.inline_uniform_block = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn set_pipeline_creation_cache_control_raw(&mut self, value: Bool32) -> &mut Self {
        self.pipeline_creation_cache_control = value;
        self
    }
    ///Sets the raw value of [`Self::private_data`]
    pub fn set_private_data_raw(&mut self, value: Bool32) -> &mut Self {
        self.private_data = value;
        self
    }
    ///Sets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn set_shader_demote_to_helper_invocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_demote_to_helper_invocation = value;
        self
    }
    ///Sets the raw value of [`Self::shader_terminate_invocation`]
    pub fn set_shader_terminate_invocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_terminate_invocation = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_size_control`]
    pub fn set_subgroup_size_control_raw(&mut self, value: Bool32) -> &mut Self {
        self.subgroup_size_control = value;
        self
    }
    ///Sets the raw value of [`Self::compute_full_subgroups`]
    pub fn set_compute_full_subgroups_raw(&mut self, value: Bool32) -> &mut Self {
        self.compute_full_subgroups = value;
        self
    }
    ///Sets the raw value of [`Self::synchronization_2`]
    pub fn set_synchronization_2_raw(&mut self, value: Bool32) -> &mut Self {
        self.synchronization_2 = value;
        self
    }
    ///Sets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn set_texture_compression_astc_hdr_raw(&mut self, value: Bool32) -> &mut Self {
        self.texture_compression_astc_hdr = value;
        self
    }
    ///Sets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn set_shader_zero_initialize_workgroup_memory_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = value;
        self
    }
    ///Sets the raw value of [`Self::dynamic_rendering`]
    pub fn set_dynamic_rendering_raw(&mut self, value: Bool32) -> &mut Self {
        self.dynamic_rendering = value;
        self
    }
    ///Sets the raw value of [`Self::shader_integer_dot_product`]
    pub fn set_shader_integer_dot_product_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_integer_dot_product = value;
        self
    }
    ///Sets the raw value of [`Self::maintenance_4`]
    pub fn set_maintenance_4_raw(&mut self, value: Bool32) -> &mut Self {
        self.maintenance_4 = value;
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
    ///Gets the value of [`Self::robust_image_access`]
    pub fn robust_image_access(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_image_access as u8) }
    }
    ///Gets the value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block(&self) -> bool {
        unsafe { std::mem::transmute(self.inline_uniform_block as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_inline_uniform_block_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control(&self) -> bool {
        unsafe { std::mem::transmute(self.pipeline_creation_cache_control as u8) }
    }
    ///Gets the value of [`Self::private_data`]
    pub fn private_data(&self) -> bool {
        unsafe { std::mem::transmute(self.private_data as u8) }
    }
    ///Gets the value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_demote_to_helper_invocation as u8) }
    }
    ///Gets the value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_terminate_invocation as u8) }
    }
    ///Gets the value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control(&self) -> bool {
        unsafe { std::mem::transmute(self.subgroup_size_control as u8) }
    }
    ///Gets the value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups(&self) -> bool {
        unsafe { std::mem::transmute(self.compute_full_subgroups as u8) }
    }
    ///Gets the value of [`Self::synchronization_2`]
    pub fn synchronization_2(&self) -> bool {
        unsafe { std::mem::transmute(self.synchronization_2 as u8) }
    }
    ///Gets the value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr(&self) -> bool {
        unsafe { std::mem::transmute(self.texture_compression_astc_hdr as u8) }
    }
    ///Gets the value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_zero_initialize_workgroup_memory as u8) }
    }
    ///Gets the value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.dynamic_rendering as u8) }
    }
    ///Gets the value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_integer_dot_product as u8) }
    }
    ///Gets the value of [`Self::maintenance_4`]
    pub fn maintenance_4(&self) -> bool {
        unsafe { std::mem::transmute(self.maintenance_4 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::robust_image_access`]
    pub fn robust_image_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_image_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_image_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::inline_uniform_block`]
    pub fn inline_uniform_block_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.inline_uniform_block as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.inline_uniform_block as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn descriptor_binding_inline_uniform_block_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_inline_uniform_block_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_inline_uniform_block_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_creation_cache_control`]
    pub fn pipeline_creation_cache_control_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pipeline_creation_cache_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pipeline_creation_cache_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::private_data`]
    pub fn private_data_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.private_data as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.private_data as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_demote_to_helper_invocation`]
    pub fn shader_demote_to_helper_invocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_demote_to_helper_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_demote_to_helper_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_terminate_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_terminate_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_size_control`]
    pub fn subgroup_size_control_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subgroup_size_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subgroup_size_control as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::compute_full_subgroups`]
    pub fn compute_full_subgroups_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.compute_full_subgroups as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.compute_full_subgroups as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::synchronization_2`]
    pub fn synchronization_2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.synchronization_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.synchronization_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::texture_compression_astc_hdr`]
    pub fn texture_compression_astc_hdr_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.texture_compression_astc_hdr as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.texture_compression_astc_hdr as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_zero_initialize_workgroup_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_zero_initialize_workgroup_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dynamic_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dynamic_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_integer_dot_product as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_integer_dot_product as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::maintenance_4`]
    pub fn maintenance_4_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.maintenance_4 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.maintenance_4 as *mut Bool32)
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
    ///Sets the raw value of [`Self::robust_image_access`]
    pub fn set_robust_image_access(&mut self, value: bool) -> &mut Self {
        self.robust_image_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::inline_uniform_block`]
    pub fn set_inline_uniform_block(&mut self, value: bool) -> &mut Self {
        self.inline_uniform_block = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_inline_uniform_block_update_after_bind`]
    pub fn set_descriptor_binding_inline_uniform_block_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_inline_uniform_block_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::pipeline_creation_cache_control`]
    pub fn set_pipeline_creation_cache_control(&mut self, value: bool) -> &mut Self {
        self.pipeline_creation_cache_control = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::private_data`]
    pub fn set_private_data(&mut self, value: bool) -> &mut Self {
        self.private_data = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_demote_to_helper_invocation`]
    pub fn set_shader_demote_to_helper_invocation(&mut self, value: bool) -> &mut Self {
        self.shader_demote_to_helper_invocation = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_terminate_invocation`]
    pub fn set_shader_terminate_invocation(&mut self, value: bool) -> &mut Self {
        self.shader_terminate_invocation = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::subgroup_size_control`]
    pub fn set_subgroup_size_control(&mut self, value: bool) -> &mut Self {
        self.subgroup_size_control = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::compute_full_subgroups`]
    pub fn set_compute_full_subgroups(&mut self, value: bool) -> &mut Self {
        self.compute_full_subgroups = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::synchronization_2`]
    pub fn set_synchronization_2(&mut self, value: bool) -> &mut Self {
        self.synchronization_2 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::texture_compression_astc_hdr`]
    pub fn set_texture_compression_astc_hdr(&mut self, value: bool) -> &mut Self {
        self.texture_compression_astc_hdr = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn set_shader_zero_initialize_workgroup_memory(&mut self, value: bool) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::dynamic_rendering`]
    pub fn set_dynamic_rendering(&mut self, value: bool) -> &mut Self {
        self.dynamic_rendering = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_integer_dot_product`]
    pub fn set_shader_integer_dot_product(&mut self, value: bool) -> &mut Self {
        self.shader_integer_dot_product = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::maintenance_4`]
    pub fn set_maintenance_4(&mut self, value: bool) -> &mut Self {
        self.maintenance_4 = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceVulkan13Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan13Properties.html) - Structure specifying physical device properties for functionality promoted to Vulkan 1.3
///# C Specifications
///The [`PhysicalDeviceVulkan13Properties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceVulkan13Properties {
///    VkStructureType       sType;
///    void*                 pNext;
///    uint32_t              minSubgroupSize;
///    uint32_t              maxSubgroupSize;
///    uint32_t              maxComputeWorkgroupSubgroups;
///    VkShaderStageFlags    requiredSubgroupSizeStages;
///    uint32_t              maxInlineUniformBlockSize;
///    uint32_t              maxPerStageDescriptorInlineUniformBlocks;
///    uint32_t              maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks;
///    uint32_t              maxDescriptorSetInlineUniformBlocks;
///    uint32_t              maxDescriptorSetUpdateAfterBindInlineUniformBlocks;
///    uint32_t              maxInlineUniformTotalSize;
///    VkBool32              integerDotProduct8BitUnsignedAccelerated;
///    VkBool32              integerDotProduct8BitSignedAccelerated;
///    VkBool32              integerDotProduct8BitMixedSignednessAccelerated;
///    VkBool32              integerDotProduct4x8BitPackedUnsignedAccelerated;
///    VkBool32              integerDotProduct4x8BitPackedSignedAccelerated;
///    VkBool32              integerDotProduct4x8BitPackedMixedSignednessAccelerated;
///    VkBool32              integerDotProduct16BitUnsignedAccelerated;
///    VkBool32              integerDotProduct16BitSignedAccelerated;
///    VkBool32              integerDotProduct16BitMixedSignednessAccelerated;
///    VkBool32              integerDotProduct32BitUnsignedAccelerated;
///    VkBool32              integerDotProduct32BitSignedAccelerated;
///    VkBool32              integerDotProduct32BitMixedSignednessAccelerated;
///    VkBool32              integerDotProduct64BitUnsignedAccelerated;
///    VkBool32              integerDotProduct64BitSignedAccelerated;
///    VkBool32              integerDotProduct64BitMixedSignednessAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating8BitUnsignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating8BitSignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated;
///    VkBool32
/// integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating16BitUnsignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating16BitSignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating32BitUnsignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating32BitSignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating64BitUnsignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating64BitSignedAccelerated;
///    VkBool32              integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated;
///    VkDeviceSize          storageTexelBufferOffsetAlignmentBytes;
///    VkBool32              storageTexelBufferOffsetSingleTexelAlignment;
///    VkDeviceSize          uniformTexelBufferOffsetAlignmentBytes;
///    VkBool32              uniformTexelBufferOffsetSingleTexelAlignment;
///    VkDeviceSize          maxBufferSize;
///} VkPhysicalDeviceVulkan13Properties;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`min_subgroup_size`] is the minimum subgroup size supported by this device.
///   [`min_subgroup_size`] is at least one if any of the physical device’s queues support
///   `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`min_subgroup_size`] is a power-of-two.
///   [`min_subgroup_size`] is less than or equal to [`max_subgroup_size`]. [`min_subgroup_size`] is
///   less than or equal to [subgroupSize]().
/// - [`max_subgroup_size`] is the maximum subgroup size supported by this device.
///   [`max_subgroup_size`] is at least one if any of the physical device’s queues support
///   `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`. [`max_subgroup_size`] is a power-of-two.
///   [`max_subgroup_size`] is greater than or equal to [`min_subgroup_size`]. [`max_subgroup_size`]
///   is greater than or equal to [subgroupSize]().
/// - [`max_compute_workgroup_subgroups`] is the maximum number of subgroups supported by the
///   implementation within a workgroup.
/// - [`required_subgroup_size_stages`] is a bitfield of what shader stages support having a
///   required subgroup size specified.
/// - [`max_inline_uniform_block_size`] is the maximum size in bytes of an [inline uniform block]()
///   binding.
/// - `maxPerStageDescriptorInlineUniformBlock` is the maximum number of inline uniform block
///   bindings that **can** be accessible to a single shader stage in a pipeline layout. Descriptor
///   bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against
///   this limit. Only descriptor bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`] is similar to
///   [`max_per_stage_descriptor_inline_uniform_blocks`] but counts descriptor bindings from
///   descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_inline_uniform_blocks`] is the maximum number of inline uniform block
///   bindings that **can** be included in descriptor bindings in a pipeline layout across all
///   pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type
///   of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit. Only descriptor
///   bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_descriptor_set_update_after_bind_inline_uniform_blocks`] is similar to
///   [`max_descriptor_set_inline_uniform_blocks`] but counts descriptor bindings from descriptor
///   sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`
///   bit set.
/// - [`max_inline_uniform_total_size`] is the maximum total size in bytes of all inline uniform
///   block bindings, across all pipeline shader stages and descriptor set numbers, that **can** be
///   included in a pipeline layout. Descriptor bindings with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` count against this limit.
/// - [`integer_dot_product_8_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 8-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_8_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 8-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_8_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 8-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V
///   instruction is accelerated [as defined below]().
/// - [`integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`] is a boolean that will be
///   [`TRUE`] if the support for 8-bit unsigned dot product operations from operands packed into
///   32-bit integers using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_4_x_8_bit_packed_signed_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 8-bit signed dot product operations from operands packed into 32-bit
///   integers using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`] is a boolean that will
///   be [`TRUE`] if the support for 8-bit mixed signedness dot product operations from operands
///   packed into 32-bit integers using the `OpSUDotKHR` SPIR-V instruction is accelerated [as
///   defined below]().
/// - [`integer_dot_product_16_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 16-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_16_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 16-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_16_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 16-bit mixed signedness dot product operations using the `OpSUDotKHR`
///   SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_32_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 32-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_32_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 32-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_32_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 32-bit mixed signedness dot product operations using the `OpSUDotKHR`
///   SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_64_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 64-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_64_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the
///   support for 64-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_64_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 64-bit mixed signedness dot product operations using the `OpSUDotKHR`
///   SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 8-bit unsigned accumulating saturating dot product
///   operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 8-bit signed accumulating saturating dot product
///   operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 8-bit mixed signedness accumulating
///   saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 8-bit unsigned accumulating saturating dot
///   product operations from operands packed into 32-bit integers using the `OpUDotAccSatKHR`
///   SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 8-bit signed accumulating saturating dot
///   product operations from operands packed into 32-bit integers using the `OpSDotAccSatKHR`
///   SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
///   is a boolean that will be [`TRUE`] if the support for 8-bit mixed signedness accumulating
///   saturating dot product operations from operands packed into 32-bit integers using the
///   `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 16-bit unsigned accumulating saturating dot product
///   operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 16-bit signed accumulating saturating dot product
///   operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 16-bit mixed signedness accumulating
///   saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 32-bit unsigned accumulating saturating dot product
///   operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 32-bit signed accumulating saturating dot product
///   operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 32-bit mixed signedness accumulating
///   saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 64-bit unsigned accumulating saturating dot product
///   operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`] is a boolean that
///   will be [`TRUE`] if the support for 64-bit signed accumulating saturating dot product
///   operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below]().
/// - [`integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`] is a
///   boolean that will be [`TRUE`] if the support for 64-bit mixed signedness accumulating
///   saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is
///   accelerated [as defined below]().
/// - [`storage_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a
///   storage texel buffer of any format. The value **must** be a power of two.
/// - [`storage_texel_buffer_offset_single_texel_alignment`] indicates whether single texel
///   alignment is sufficient for a storage texel buffer of any format. The value **must** be a
///   power of two.
/// - [`uniform_texel_buffer_offset_alignment_bytes`] is a byte alignment that is sufficient for a
///   uniform texel buffer of any format. The value **must** be a power of two.
/// - [`uniform_texel_buffer_offset_single_texel_alignment`] indicates whether single texel
///   alignment is sufficient for a uniform texel buffer of any format. The value **must** be a
///   power of two.
/// - [`max_buffer_size`] is the maximum size [`Buffer`] that **can** be created.
///If the [`PhysicalDeviceVulkan13Properties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties correspond to Vulkan 1.3
/// functionality.The members of [`PhysicalDeviceVulkan13Properties`]**must** have the same
///values as the corresponding members of
///[`PhysicalDeviceInlineUniformBlockProperties`] and
///[`PhysicalDeviceSubgroupSizeControlProperties`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES`
///# Related
/// - [`crate::vulkan1_3`]
/// - [`Bool32`]
/// - [`DeviceSize`]
/// - [`ShaderStageFlags`]
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
pub struct PhysicalDeviceVulkan13Properties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    min_subgroup_size: u32,
    ///No documentation found
    max_subgroup_size: u32,
    ///No documentation found
    max_compute_workgroup_subgroups: u32,
    ///No documentation found
    required_subgroup_size_stages: ShaderStageFlags,
    ///No documentation found
    max_inline_uniform_block_size: u32,
    ///No documentation found
    max_per_stage_descriptor_inline_uniform_blocks: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    ///No documentation found
    max_descriptor_set_inline_uniform_blocks: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    ///No documentation found
    max_inline_uniform_total_size: u32,
    ///No documentation found
    integer_dot_product_8_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_8_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_8_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    ///No documentation found
    storage_texel_buffer_offset_single_texel_alignment: Bool32,
    ///No documentation found
    uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    ///No documentation found
    uniform_texel_buffer_offset_single_texel_alignment: Bool32,
    ///No documentation found
    max_buffer_size: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceVulkan13Properties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            min_subgroup_size: 0,
            max_subgroup_size: 0,
            max_compute_workgroup_subgroups: 0,
            required_subgroup_size_stages: Default::default(),
            max_inline_uniform_block_size: 0,
            max_per_stage_descriptor_inline_uniform_blocks: 0,
            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: 0,
            max_descriptor_set_inline_uniform_blocks: 0,
            max_descriptor_set_update_after_bind_inline_uniform_blocks: 0,
            max_inline_uniform_total_size: 0,
            integer_dot_product_8_bit_unsigned_accelerated: 0,
            integer_dot_product_8_bit_signed_accelerated: 0,
            integer_dot_product_8_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_signed_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: 0,
            integer_dot_product_16_bit_unsigned_accelerated: 0,
            integer_dot_product_16_bit_signed_accelerated: 0,
            integer_dot_product_16_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_32_bit_unsigned_accelerated: 0,
            integer_dot_product_32_bit_signed_accelerated: 0,
            integer_dot_product_32_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_64_bit_unsigned_accelerated: 0,
            integer_dot_product_64_bit_signed_accelerated: 0,
            integer_dot_product_64_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: 0,
            storage_texel_buffer_offset_alignment_bytes: Default::default(),
            storage_texel_buffer_offset_single_texel_alignment: 0,
            uniform_texel_buffer_offset_alignment_bytes: Default::default(),
            uniform_texel_buffer_offset_single_texel_alignment: 0,
            max_buffer_size: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceVulkan13Properties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size_raw(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Gets the raw value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size_raw(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Gets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups_raw(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Gets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size_raw(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Gets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Gets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Gets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Gets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks_raw(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
    }
    ///Gets the raw value of [`Self::max_inline_uniform_total_size`]
    pub fn max_inline_uniform_total_size_raw(&self) -> u32 {
        self.max_inline_uniform_total_size
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &self,
    ) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment_raw(&self) -> Bool32 {
        self.storage_texel_buffer_offset_single_texel_alignment
    }
    ///Gets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment_raw(&self) -> Bool32 {
        self.uniform_texel_buffer_offset_single_texel_alignment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_subgroup_size`]
    pub fn set_min_subgroup_size_raw(&mut self, value: u32) -> &mut Self {
        self.min_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_subgroup_size`]
    pub fn set_max_subgroup_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn set_max_compute_workgroup_subgroups_raw(&mut self, value: u32) -> &mut Self {
        self.max_compute_workgroup_subgroups = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn set_max_inline_uniform_block_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_raw(
        &mut self,
        value: u32,
    ) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_update_after_bind_inline_uniform_blocks_raw(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_total_size`]
    pub fn set_max_inline_uniform_total_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_total_size = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_8_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_8_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_8_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_16_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_16_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_16_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_32_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_32_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_32_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_64_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_64_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_64_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn set_storage_texel_buffer_offset_single_texel_alignment_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_texel_buffer_offset_single_texel_alignment = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn set_uniform_texel_buffer_offset_single_texel_alignment_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_texel_buffer_offset_single_texel_alignment = value;
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
    ///Gets the value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size(&self) -> u32 {
        self.min_subgroup_size
    }
    ///Gets the value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size(&self) -> u32 {
        self.max_subgroup_size
    }
    ///Gets the value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups(&self) -> u32 {
        self.max_compute_workgroup_subgroups
    }
    ///Gets the value of [`Self::required_subgroup_size_stages`]
    pub fn required_subgroup_size_stages(&self) -> ShaderStageFlags {
        self.required_subgroup_size_stages
    }
    ///Gets the value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size(&self) -> u32 {
        self.max_inline_uniform_block_size
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks
    }
    ///Gets the value of [`Self::max_inline_uniform_total_size`]
    pub fn max_inline_uniform_total_size(&self) -> u32 {
        self.max_inline_uniform_total_size
    }
    ///Gets the value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn storage_texel_buffer_offset_alignment_bytes(&self) -> DeviceSize {
        self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Gets the value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_texel_buffer_offset_single_texel_alignment as u8) }
    }
    ///Gets the value of [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn uniform_texel_buffer_offset_alignment_bytes(&self) -> DeviceSize {
        self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Gets the value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_texel_buffer_offset_single_texel_alignment as u8) }
    }
    ///Gets the value of [`Self::max_buffer_size`]
    pub fn max_buffer_size(&self) -> DeviceSize {
        self.max_buffer_size
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
    ///Gets a mutable reference to the value of [`Self::min_subgroup_size`]
    pub fn min_subgroup_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_subgroup_size`]
    pub fn max_subgroup_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_compute_workgroup_subgroups`]
    pub fn max_compute_workgroup_subgroups_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::required_subgroup_size_stages`]
    pub fn required_subgroup_size_stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.required_subgroup_size_stages
    }
    ///Gets a mutable reference to the value of [`Self::max_inline_uniform_block_size`]
    pub fn max_inline_uniform_block_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn max_per_stage_descriptor_update_after_bind_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn max_descriptor_set_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn max_descriptor_set_update_after_bind_inline_uniform_blocks_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_inline_uniform_total_size`]
    pub fn max_inline_uniform_total_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_mut(
        &mut self,
    ) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_mut(
        &mut self,
    ) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn storage_texel_buffer_offset_alignment_bytes_mut(&mut self) -> &mut DeviceSize {
        &mut self.storage_texel_buffer_offset_alignment_bytes
    }
    ///Gets a mutable reference to the value of
    /// [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn storage_texel_buffer_offset_single_texel_alignment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn uniform_texel_buffer_offset_alignment_bytes_mut(&mut self) -> &mut DeviceSize {
        &mut self.uniform_texel_buffer_offset_alignment_bytes
    }
    ///Gets a mutable reference to the value of
    /// [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn uniform_texel_buffer_offset_single_texel_alignment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_texel_buffer_offset_single_texel_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_buffer_size`]
    pub fn max_buffer_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.max_buffer_size
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
    ///Sets the raw value of [`Self::min_subgroup_size`]
    pub fn set_min_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.min_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_subgroup_size`]
    pub fn set_max_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.max_subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_compute_workgroup_subgroups`]
    pub fn set_max_compute_workgroup_subgroups(&mut self, value: u32) -> &mut Self {
        self.max_compute_workgroup_subgroups = value;
        self
    }
    ///Sets the raw value of [`Self::required_subgroup_size_stages`]
    pub fn set_required_subgroup_size_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.required_subgroup_size_stages = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_block_size`]
    pub fn set_max_inline_uniform_block_size(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_block_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_per_stage_descriptor_update_after_bind_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_inline_uniform_blocks`]
    pub fn set_max_descriptor_set_update_after_bind_inline_uniform_blocks(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_inline_uniform_blocks = value;
        self
    }
    ///Sets the raw value of [`Self::max_inline_uniform_total_size`]
    pub fn set_max_inline_uniform_total_size(&mut self, value: u32) -> &mut Self {
        self.max_inline_uniform_total_size = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_8_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_8_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_8_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_16_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_16_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_16_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_32_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_32_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_32_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_64_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_64_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_64_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated =
            value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_alignment_bytes`]
    pub fn set_storage_texel_buffer_offset_alignment_bytes(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.storage_texel_buffer_offset_alignment_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::storage_texel_buffer_offset_single_texel_alignment`]
    pub fn set_storage_texel_buffer_offset_single_texel_alignment(&mut self, value: bool) -> &mut Self {
        self.storage_texel_buffer_offset_single_texel_alignment = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_alignment_bytes`]
    pub fn set_uniform_texel_buffer_offset_alignment_bytes(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.uniform_texel_buffer_offset_alignment_bytes = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_texel_buffer_offset_single_texel_alignment`]
    pub fn set_uniform_texel_buffer_offset_single_texel_alignment(&mut self, value: bool) -> &mut Self {
        self.uniform_texel_buffer_offset_single_texel_alignment = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_buffer_size`]
    pub fn set_max_buffer_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.max_buffer_size = value;
        self
    }
}
///[VkPhysicalDeviceToolProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceToolProperties.html) - Structure providing information about an active tool
///# C Specifications
///The [`PhysicalDeviceToolProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceToolProperties {
///    VkStructureType       sType;
///    void*                 pNext;
///    char                  name[VK_MAX_EXTENSION_NAME_SIZE];
///    char                  version[VK_MAX_EXTENSION_NAME_SIZE];
///    VkToolPurposeFlags    purposes;
///    char                  description[VK_MAX_DESCRIPTION_SIZE];
///    char                  layer[VK_MAX_EXTENSION_NAME_SIZE];
///} VkPhysicalDeviceToolProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_tooling_info
///typedef VkPhysicalDeviceToolProperties VkPhysicalDeviceToolPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`name`] is a null-terminated UTF-8 string containing the name of the tool.
/// - [`version`] is a null-terminated UTF-8 string containing the version of the tool.
/// - [`purposes`] is a bitmask of [`ToolPurposeFlagBits`] which is populated with purposes
///   supported by the tool.
/// - [`description`] is a null-terminated UTF-8 string containing a description of the tool.
/// - [`layer`] is a null-terminated UTF-8 string containing the name of the layer implementing the
///   tool, if the tool is implemented in a layer - otherwise it **may** be an empty string.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_EXT_tooling_info`]
/// - [`crate::vulkan1_3`]
/// - [`StructureType`]
/// - [`ToolPurposeFlags`]
/// - [`GetPhysicalDeviceToolProperties`]
/// - [`GetPhysicalDeviceToolPropertiesEXT`]
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
pub struct PhysicalDeviceToolProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`name`] is a null-terminated UTF-8 string containing the name of the
    ///tool.
    name: [c_schar; MAX_EXTENSION_NAME_SIZE],
    ///[`version`] is a null-terminated UTF-8 string containing the version
    ///of the tool.
    version: [c_schar; MAX_EXTENSION_NAME_SIZE],
    ///[`purposes`] is a bitmask of [`ToolPurposeFlagBits`] which is
    ///populated with purposes supported by the tool.
    purposes: ToolPurposeFlags,
    ///[`description`] is a null-terminated UTF-8 string containing a
    ///description of the tool.
    description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`layer`] is a null-terminated UTF-8 string containing the name of the
    ///layer implementing the tool, if the tool is implemented in a layer -
    ///otherwise it **may** be an empty string.
    layer: [c_schar; MAX_EXTENSION_NAME_SIZE],
}
impl<'lt> Default for PhysicalDeviceToolProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            name: [b'\0' as i8; MAX_EXTENSION_NAME_SIZE],
            version: [b'\0' as i8; MAX_EXTENSION_NAME_SIZE],
            purposes: Default::default(),
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            layer: [b'\0' as i8; MAX_EXTENSION_NAME_SIZE],
        }
    }
}
impl<'lt> PhysicalDeviceToolProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> [c_schar; MAX_EXTENSION_NAME_SIZE] {
        self.name
    }
    ///Gets the raw value of [`Self::version`]
    pub fn version_raw(&self) -> [c_schar; MAX_EXTENSION_NAME_SIZE] {
        self.version
    }
    ///Gets the raw value of [`Self::description`]
    pub fn description_raw(&self) -> [c_schar; MAX_DESCRIPTION_SIZE] {
        self.description
    }
    ///Gets the raw value of [`Self::layer`]
    pub fn layer_raw(&self) -> [c_schar; MAX_EXTENSION_NAME_SIZE] {
        self.layer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: [c_schar; MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::version`]
    pub fn set_version_raw(&mut self, value: [c_schar; MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.version = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description_raw(&mut self, value: [c_schar; MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer_raw(&mut self, value: [c_schar; MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.layer = value;
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
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_schar; MAX_EXTENSION_NAME_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::version`]
    pub fn version(&self) -> &[c_schar; MAX_EXTENSION_NAME_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::purposes`]
    pub fn purposes(&self) -> ToolPurposeFlags {
        self.purposes
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::layer`]
    pub fn layer(&self) -> &[c_schar; MAX_EXTENSION_NAME_SIZE] {
        &getter
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
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_schar; MAX_EXTENSION_NAME_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::version`]
    pub fn version_mut(&mut self) -> &mut [c_schar; MAX_EXTENSION_NAME_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::purposes`]
    pub fn purposes_mut(&mut self) -> &mut ToolPurposeFlags {
        &mut self.purposes
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::layer`]
    pub fn layer_mut(&mut self) -> &mut [c_schar; MAX_EXTENSION_NAME_SIZE] {
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::version`]
    pub fn set_version(&mut self, value: [std::os::raw::c_char; crate::core::MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.version = value;
        self
    }
    ///Sets the raw value of [`Self::purposes`]
    pub fn set_purposes(&mut self, value: crate::vulkan1_3::ToolPurposeFlags) -> &mut Self {
        self.purposes = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer(&mut self, value: [std::os::raw::c_char; crate::core::MAX_EXTENSION_NAME_SIZE]) -> &mut Self {
        self.layer = value;
        self
    }
}
///[VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures.html) - Structure describing support for zero initialization of workgroup memory by an implementation
///# C Specifications
///The [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderZeroInitializeWorkgroupMemory;
///} VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_zero_initialize_workgroup_memory
///typedef VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures
/// VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_zero_initialize_workgroup_memory`] specifies whether the implementation supports
///   initializing a variable in Workgroup storage class.
///If the [`PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES`
///# Related
/// - [`VK_KHR_zero_initialize_workgroup_memory`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_zero_initialize_workgroup_memory`] specifies whether the
    ///implementation supports initializing a variable in Workgroup storage
    ///class.
    shader_zero_initialize_workgroup_memory: Bool32,
}
impl<'lt> Default for PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_zero_initialize_workgroup_memory: 0,
        }
    }
}
impl<'lt> PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory_raw(&self) -> Bool32 {
        self.shader_zero_initialize_workgroup_memory
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn set_shader_zero_initialize_workgroup_memory_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = value;
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
    ///Gets the value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_zero_initialize_workgroup_memory as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn shader_zero_initialize_workgroup_memory_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_zero_initialize_workgroup_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_zero_initialize_workgroup_memory as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_zero_initialize_workgroup_memory`]
    pub fn set_shader_zero_initialize_workgroup_memory(&mut self, value: bool) -> &mut Self {
        self.shader_zero_initialize_workgroup_memory = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceImageRobustnessFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageRobustnessFeatures.html) - Structure describing the out-of-bounds behavior for an implementation
///# C Specifications
///The [`PhysicalDeviceImageRobustnessFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceImageRobustnessFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           robustImageAccess;
///} VkPhysicalDeviceImageRobustnessFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_image_robustness
///typedef VkPhysicalDeviceImageRobustnessFeatures VkPhysicalDeviceImageRobustnessFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`robust_image_access`] indicates whether image accesses are tightly bounds-checked against the dimensions of the image view. [Invalid texels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-input-validation) resulting from out of bounds image loads will be replaced as described in [Texel Replacement](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-replacement), with either (0,0,1) or (0,0,0) values inserted for missing G, B, or A components based on the format.
///If the [`PhysicalDeviceImageRobustnessFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceImageRobustnessFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES`
///# Related
/// - [`VK_EXT_image_robustness`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceImageRobustnessFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`robust_image_access`]
    ///indicates whether image accesses are tightly bounds-checked against the
    ///dimensions of the image view.
    ///[Invalid texels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-input-validation) resulting from out of
    ///bounds image loads will be replaced as described in
    ///[Texel Replacement](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-replacement), with either
    ///(0,0,1) or (0,0,0) values inserted for missing G, B, or A
    ///components based on the format.
    robust_image_access: Bool32,
}
impl<'lt> Default for PhysicalDeviceImageRobustnessFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            robust_image_access: 0,
        }
    }
}
impl<'lt> PhysicalDeviceImageRobustnessFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::robust_image_access`]
    pub fn robust_image_access_raw(&self) -> Bool32 {
        self.robust_image_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::robust_image_access`]
    pub fn set_robust_image_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_image_access = value;
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
    ///Gets the value of [`Self::robust_image_access`]
    pub fn robust_image_access(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_image_access as u8) }
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
    ///Gets a mutable reference to the value of [`Self::robust_image_access`]
    pub fn robust_image_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_image_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_image_access as *mut Bool32)
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
    ///Sets the raw value of [`Self::robust_image_access`]
    pub fn set_robust_image_access(&mut self, value: bool) -> &mut Self {
        self.robust_image_access = value as u8 as u32;
        self
    }
}
///[VkBufferCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCopy2.html) - Structure specifying a buffer copy operation
///# C Specifications
///The [`BufferCopy2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkBufferCopy2 {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceSize       srcOffset;
///    VkDeviceSize       dstOffset;
///    VkDeviceSize       size;
///} VkBufferCopy2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkBufferCopy2 VkBufferCopy2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_offset`] is the starting offset in bytes from the start of `srcBuffer`.
/// - [`dst_offset`] is the starting offset in bytes from the start of `dstBuffer`.
/// - [`size`] is the number of bytes to copy.
///# Description
///Valid Usage
/// - The [`size`]**must** be greater than `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_COPY_2`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`CopyBufferInfo2`]
/// - [`DeviceSize`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct BufferCopy2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_offset`] is the starting offset in bytes from the start of
    ///`srcBuffer`.
    src_offset: DeviceSize,
    ///[`dst_offset`] is the starting offset in bytes from the start of
    ///`dstBuffer`.
    dst_offset: DeviceSize,
    ///[`size`] is the number of bytes to copy.
    size: DeviceSize,
}
impl<'lt> Default for BufferCopy2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_offset: Default::default(),
            dst_offset: Default::default(),
            size: Default::default(),
        }
    }
}
impl<'lt> BufferCopy2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::src_offset`]
    pub fn src_offset(&self) -> DeviceSize {
        self.src_offset
    }
    ///Gets the value of [`Self::dst_offset`]
    pub fn dst_offset(&self) -> DeviceSize {
        self.dst_offset
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_offset`]
    pub fn src_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.src_offset
    }
    ///Gets a mutable reference to the value of [`Self::dst_offset`]
    pub fn dst_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.dst_offset
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
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
    ///Sets the raw value of [`Self::src_offset`]
    pub fn set_src_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.src_offset = value;
        self
    }
    ///Sets the raw value of [`Self::dst_offset`]
    pub fn set_dst_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.dst_offset = value;
        self
    }
    ///Sets the raw value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
}
///[VkImageCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageCopy2.html) - Structure specifying an image copy operation
///# C Specifications
///The [`ImageCopy2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkImageCopy2 {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkImageSubresourceLayers    srcSubresource;
///    VkOffset3D                  srcOffset;
///    VkImageSubresourceLayers    dstSubresource;
///    VkOffset3D                  dstOffset;
///    VkExtent3D                  extent;
///} VkImageCopy2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkImageCopy2 VkImageCopy2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_subresource`] and [`dst_subresource`] are [`ImageSubresourceLayers`] structures
///   specifying the image subresources of the images used for the source and destination image
///   data, respectively.
/// - [`src_offset`] and [`dst_offset`] select the initial `x`, `y`, and `z` offsets in texels of
///   the sub-regions of the source and destination image data.
/// - [`extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.
///# Description
///Valid Usage
/// - The number of slices of the [`extent`] (for 3D) or layers of the [`src_subresource`] (for
///   non-3D) **must** match the number of slices of the [`extent`] (for 3D) or layers of the
///   [`dst_subresource`] (for non-3D)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_COPY_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
/// - [`dst_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`CopyImageInfo2`]
/// - [`Extent3D`]
/// - [`ImageSubresourceLayers`]
/// - [`Offset3D`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct ImageCopy2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_subresource`] and [`dst_subresource`] are
    ///[`ImageSubresourceLayers`] structures specifying the image
    ///subresources of the images used for the source and destination image
    ///data, respectively.
    src_subresource: ImageSubresourceLayers,
    ///[`src_offset`] and [`dst_offset`] select the initial `x`, `y`,
    ///and `z` offsets in texels of the sub-regions of the source and
    ///destination image data.
    src_offset: Offset3D,
    ///No documentation found
    dst_subresource: ImageSubresourceLayers,
    ///No documentation found
    dst_offset: Offset3D,
    ///[`extent`] is the size in texels of the image to copy in `width`,
    ///`height` and `depth`.
    extent: Extent3D,
}
impl<'lt> Default for ImageCopy2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
impl<'lt> ImageCopy2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::src_subresource`]
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Gets the value of [`Self::src_offset`]
    pub fn src_offset(&self) -> Offset3D {
        self.src_offset
    }
    ///Gets the value of [`Self::dst_subresource`]
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Gets the value of [`Self::dst_offset`]
    pub fn dst_offset(&self) -> Offset3D {
        self.dst_offset
    }
    ///Gets the value of [`Self::extent`]
    pub fn extent(&self) -> Extent3D {
        self.extent
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_subresource`]
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Gets a mutable reference to the value of [`Self::src_offset`]
    pub fn src_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.src_offset
    }
    ///Gets a mutable reference to the value of [`Self::dst_subresource`]
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Gets a mutable reference to the value of [`Self::dst_offset`]
    pub fn dst_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.dst_offset
    }
    ///Gets a mutable reference to the value of [`Self::extent`]
    pub fn extent_mut(&mut self) -> &mut Extent3D {
        &mut self.extent
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
    ///Sets the raw value of [`Self::src_subresource`]
    pub fn set_src_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::src_offset`]
    pub fn set_src_offset(&mut self, value: crate::vulkan1_0::Offset3D) -> &mut Self {
        self.src_offset = value;
        self
    }
    ///Sets the raw value of [`Self::dst_subresource`]
    pub fn set_dst_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::dst_offset`]
    pub fn set_dst_offset(&mut self, value: crate::vulkan1_0::Offset3D) -> &mut Self {
        self.dst_offset = value;
        self
    }
    ///Sets the raw value of [`Self::extent`]
    pub fn set_extent(&mut self, value: crate::vulkan1_0::Extent3D) -> &mut Self {
        self.extent = value;
        self
    }
}
///[VkImageBlit2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageBlit2.html) - Structure specifying an image blit operation
///# C Specifications
///The [`ImageBlit2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkImageBlit2 {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkImageSubresourceLayers    srcSubresource;
///    VkOffset3D                  srcOffsets[2];
///    VkImageSubresourceLayers    dstSubresource;
///    VkOffset3D                  dstOffsets[2];
///} VkImageBlit2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkImageBlit2 VkImageBlit2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_subresource`] is the subresource to blit from.
/// - [`src_offsets`] is a pointer to an array of two [`Offset3D`] structures specifying the bounds
///   of the source region within [`src_subresource`].
/// - [`dst_subresource`] is the subresource to blit into.
/// - [`dst_offsets`] is a pointer to an array of two [`Offset3D`] structures specifying the bounds
///   of the destination region within [`dst_subresource`].
///# Description
///For each element of the `pRegions` array, a blit operation is performed
///for the specified source and destination regions.Valid Usage
/// - The `aspectMask` member of [`src_subresource`] and [`dst_subresource`]**must** match
/// - The `layerCount` member of [`src_subresource`] and [`dst_subresource`]**must** match
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_BLIT_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`CopyCommandTransformInfoQCOM`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`src_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
/// - [`dst_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`BlitImageInfo2`]
/// - [`ImageSubresourceLayers`]
/// - [`Offset3D`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct ImageBlit2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_subresource`] is the subresource to blit from.
    src_subresource: ImageSubresourceLayers,
    ///[`src_offsets`] is a pointer to an array of two [`Offset3D`]
    ///structures specifying the bounds of the source region within
    ///[`src_subresource`].
    src_offsets: [Offset3D; 2],
    ///[`dst_subresource`] is the subresource to blit into.
    dst_subresource: ImageSubresourceLayers,
    ///[`dst_offsets`] is a pointer to an array of two [`Offset3D`]
    ///structures specifying the bounds of the destination region within
    ///[`dst_subresource`].
    dst_offsets: [Offset3D; 2],
}
impl<'lt> Default for ImageBlit2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offsets: [Default::default(); 2],
            dst_subresource: Default::default(),
            dst_offsets: [Default::default(); 2],
        }
    }
}
impl<'lt> ImageBlit2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::src_subresource`]
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Gets the value of [`Self::src_offsets`]
    pub fn src_offsets(&self) -> &[Offset3D; 2] {
        &getter
    }
    ///Gets the value of [`Self::dst_subresource`]
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Gets the value of [`Self::dst_offsets`]
    pub fn dst_offsets(&self) -> &[Offset3D; 2] {
        &getter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_subresource`]
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Gets a mutable reference to the value of [`Self::src_offsets`]
    pub fn src_offsets_mut(&mut self) -> &mut [Offset3D; 2] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::dst_subresource`]
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Gets a mutable reference to the value of [`Self::dst_offsets`]
    pub fn dst_offsets_mut(&mut self) -> &mut [Offset3D; 2] {
        &mut getter
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
    ///Sets the raw value of [`Self::src_subresource`]
    pub fn set_src_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::src_offsets`]
    pub fn set_src_offsets(&mut self, value: [crate::vulkan1_0::Offset3D; 2]) -> &mut Self {
        self.src_offsets = value;
        self
    }
    ///Sets the raw value of [`Self::dst_subresource`]
    pub fn set_dst_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::dst_offsets`]
    pub fn set_dst_offsets(&mut self, value: [crate::vulkan1_0::Offset3D; 2]) -> &mut Self {
        self.dst_offsets = value;
        self
    }
}
///[VkBufferImageCopy2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferImageCopy2.html) - Structure specifying a buffer image copy operation
///# C Specifications
///For both [`CmdCopyBufferToImage2`] and [`CmdCopyImageToBuffer2`],
///each element of `pRegions` is a structure defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkBufferImageCopy2 {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkDeviceSize                bufferOffset;
///    uint32_t                    bufferRowLength;
///    uint32_t                    bufferImageHeight;
///    VkImageSubresourceLayers    imageSubresource;
///    VkOffset3D                  imageOffset;
///    VkExtent3D                  imageExtent;
///} VkBufferImageCopy2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkBufferImageCopy2 VkBufferImageCopy2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer_offset`] is the offset in bytes from the start of the buffer object where the image
///   data is copied from or to.
/// - [`buffer_row_length`] and [`buffer_image_height`] specify in texels a subregion of a larger
///   two- or three-dimensional image in buffer memory, and control the addressing calculations. If
///   either of these values is zero, that aspect of the buffer memory is considered to be tightly
///   packed according to the [`image_extent`].
/// - [`image_subresource`] is a [`ImageSubresourceLayers`] used to specify the specific image
///   subresources of the image used for the source or destination image data.
/// - [`image_offset`] selects the initial `x`, `y`, `z` offsets in texels of the sub-region of the
///   source or destination image data.
/// - [`image_extent`] is the size in texels of the image to copy in `width`, `height` and `depth`.
///# Description
///This structure is functionally identical to [`BufferImageCopy`], but
///adds [`s_type`] and [`p_next`] parameters, allowing it to be more easily
///extended.Valid Usage
/// - [`buffer_row_length`]**must** be `0`, or greater than or equal to the `width` member of
///   [`image_extent`]
/// - [`buffer_image_height`]**must** be `0`, or greater than or equal to the `height` member of
///   [`image_extent`]
/// - The `aspectMask` member of [`image_subresource`]**must** only have a single bit set
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`CopyCommandTransformInfoQCOM`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`image_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`CopyBufferToImageInfo2`]
/// - [`CopyImageToBufferInfo2`]
/// - [`DeviceSize`]
/// - [`Extent3D`]
/// - [`ImageSubresourceLayers`]
/// - [`Offset3D`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct BufferImageCopy2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`buffer_offset`] is the offset in bytes from the start of the buffer
    ///object where the image data is copied from or to.
    buffer_offset: DeviceSize,
    ///[`buffer_row_length`] and [`buffer_image_height`] specify in texels a
    ///subregion of a larger two- or three-dimensional image in buffer memory,
    ///and control the addressing calculations.
    ///If either of these values is zero, that aspect of the buffer memory is
    ///considered to be tightly packed according to the [`image_extent`].
    buffer_row_length: u32,
    ///No documentation found
    buffer_image_height: u32,
    ///[`image_subresource`] is a [`ImageSubresourceLayers`] used to
    ///specify the specific image subresources of the image used for the source
    ///or destination image data.
    image_subresource: ImageSubresourceLayers,
    ///[`image_offset`] selects the initial `x`, `y`, `z` offsets
    ///in texels of the sub-region of the source or destination image data.
    image_offset: Offset3D,
    ///[`image_extent`] is the size in texels of the image to copy in
    ///`width`, `height` and `depth`.
    image_extent: Extent3D,
}
impl<'lt> Default for BufferImageCopy2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            buffer_offset: Default::default(),
            buffer_row_length: 0,
            buffer_image_height: 0,
            image_subresource: Default::default(),
            image_offset: Default::default(),
            image_extent: Default::default(),
        }
    }
}
impl<'lt> BufferImageCopy2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::buffer_row_length`]
    pub fn buffer_row_length_raw(&self) -> u32 {
        self.buffer_row_length
    }
    ///Gets the raw value of [`Self::buffer_image_height`]
    pub fn buffer_image_height_raw(&self) -> u32 {
        self.buffer_image_height
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_row_length`]
    pub fn set_buffer_row_length_raw(&mut self, value: u32) -> &mut Self {
        self.buffer_row_length = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_image_height`]
    pub fn set_buffer_image_height_raw(&mut self, value: u32) -> &mut Self {
        self.buffer_image_height = value;
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
    ///Gets the value of [`Self::buffer_offset`]
    pub fn buffer_offset(&self) -> DeviceSize {
        self.buffer_offset
    }
    ///Gets the value of [`Self::buffer_row_length`]
    pub fn buffer_row_length(&self) -> u32 {
        self.buffer_row_length
    }
    ///Gets the value of [`Self::buffer_image_height`]
    pub fn buffer_image_height(&self) -> u32 {
        self.buffer_image_height
    }
    ///Gets the value of [`Self::image_subresource`]
    pub fn image_subresource(&self) -> ImageSubresourceLayers {
        self.image_subresource
    }
    ///Gets the value of [`Self::image_offset`]
    pub fn image_offset(&self) -> Offset3D {
        self.image_offset
    }
    ///Gets the value of [`Self::image_extent`]
    pub fn image_extent(&self) -> Extent3D {
        self.image_extent
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer_offset`]
    pub fn buffer_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.buffer_offset
    }
    ///Gets a mutable reference to the value of [`Self::buffer_row_length`]
    pub fn buffer_row_length_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::buffer_image_height`]
    pub fn buffer_image_height_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image_subresource`]
    pub fn image_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.image_subresource
    }
    ///Gets a mutable reference to the value of [`Self::image_offset`]
    pub fn image_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.image_offset
    }
    ///Gets a mutable reference to the value of [`Self::image_extent`]
    pub fn image_extent_mut(&mut self) -> &mut Extent3D {
        &mut self.image_extent
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
    ///Sets the raw value of [`Self::buffer_offset`]
    pub fn set_buffer_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.buffer_offset = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_row_length`]
    pub fn set_buffer_row_length(&mut self, value: u32) -> &mut Self {
        self.buffer_row_length = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_image_height`]
    pub fn set_buffer_image_height(&mut self, value: u32) -> &mut Self {
        self.buffer_image_height = value;
        self
    }
    ///Sets the raw value of [`Self::image_subresource`]
    pub fn set_image_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.image_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::image_offset`]
    pub fn set_image_offset(&mut self, value: crate::vulkan1_0::Offset3D) -> &mut Self {
        self.image_offset = value;
        self
    }
    ///Sets the raw value of [`Self::image_extent`]
    pub fn set_image_extent(&mut self, value: crate::vulkan1_0::Extent3D) -> &mut Self {
        self.image_extent = value;
        self
    }
}
///[VkImageResolve2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageResolve2.html) - Structure specifying an image resolve operation
///# C Specifications
///The [`ImageResolve2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkImageResolve2 {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkImageSubresourceLayers    srcSubresource;
///    VkOffset3D                  srcOffset;
///    VkImageSubresourceLayers    dstSubresource;
///    VkOffset3D                  dstOffset;
///    VkExtent3D                  extent;
///} VkImageResolve2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkImageResolve2 VkImageResolve2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_subresource`] and [`dst_subresource`] are [`ImageSubresourceLayers`] structures
///   specifying the image subresources of the images used for the source and destination image
///   data, respectively. Resolve of depth/stencil images is not supported.
/// - [`src_offset`] and [`dst_offset`] select the initial `x`, `y`, and `z` offsets in texels of
///   the sub-regions of the source and destination image data.
/// - [`extent`] is the size in texels of the source image to resolve in `width`, `height` and
///   `depth`.
///# Description
///Valid Usage
/// - The `aspectMask` member of [`src_subresource`] and [`dst_subresource`]**must** only contain
///   `VK_IMAGE_ASPECT_COLOR_BIT`
/// - The `layerCount` member of [`src_subresource`] and [`dst_subresource`]**must** match
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
/// - [`dst_subresource`]**must** be a valid [`ImageSubresourceLayers`] structure
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Extent3D`]
/// - [`ImageSubresourceLayers`]
/// - [`Offset3D`]
/// - [`ResolveImageInfo2`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct ImageResolve2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_subresource`] and [`dst_subresource`] are
    ///[`ImageSubresourceLayers`] structures specifying the image
    ///subresources of the images used for the source and destination image
    ///data, respectively.
    ///Resolve of depth/stencil images is not supported.
    src_subresource: ImageSubresourceLayers,
    ///[`src_offset`] and [`dst_offset`] select the initial `x`, `y`,
    ///and `z` offsets in texels of the sub-regions of the source and
    ///destination image data.
    src_offset: Offset3D,
    ///No documentation found
    dst_subresource: ImageSubresourceLayers,
    ///No documentation found
    dst_offset: Offset3D,
    ///[`extent`] is the size in texels of the source image to resolve in
    ///`width`, `height` and `depth`.
    extent: Extent3D,
}
impl<'lt> Default for ImageResolve2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
impl<'lt> ImageResolve2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::src_subresource`]
    pub fn src_subresource(&self) -> ImageSubresourceLayers {
        self.src_subresource
    }
    ///Gets the value of [`Self::src_offset`]
    pub fn src_offset(&self) -> Offset3D {
        self.src_offset
    }
    ///Gets the value of [`Self::dst_subresource`]
    pub fn dst_subresource(&self) -> ImageSubresourceLayers {
        self.dst_subresource
    }
    ///Gets the value of [`Self::dst_offset`]
    pub fn dst_offset(&self) -> Offset3D {
        self.dst_offset
    }
    ///Gets the value of [`Self::extent`]
    pub fn extent(&self) -> Extent3D {
        self.extent
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_subresource`]
    pub fn src_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.src_subresource
    }
    ///Gets a mutable reference to the value of [`Self::src_offset`]
    pub fn src_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.src_offset
    }
    ///Gets a mutable reference to the value of [`Self::dst_subresource`]
    pub fn dst_subresource_mut(&mut self) -> &mut ImageSubresourceLayers {
        &mut self.dst_subresource
    }
    ///Gets a mutable reference to the value of [`Self::dst_offset`]
    pub fn dst_offset_mut(&mut self) -> &mut Offset3D {
        &mut self.dst_offset
    }
    ///Gets a mutable reference to the value of [`Self::extent`]
    pub fn extent_mut(&mut self) -> &mut Extent3D {
        &mut self.extent
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
    ///Sets the raw value of [`Self::src_subresource`]
    pub fn set_src_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.src_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::src_offset`]
    pub fn set_src_offset(&mut self, value: crate::vulkan1_0::Offset3D) -> &mut Self {
        self.src_offset = value;
        self
    }
    ///Sets the raw value of [`Self::dst_subresource`]
    pub fn set_dst_subresource(&mut self, value: crate::vulkan1_0::ImageSubresourceLayers) -> &mut Self {
        self.dst_subresource = value;
        self
    }
    ///Sets the raw value of [`Self::dst_offset`]
    pub fn set_dst_offset(&mut self, value: crate::vulkan1_0::Offset3D) -> &mut Self {
        self.dst_offset = value;
        self
    }
    ///Sets the raw value of [`Self::extent`]
    pub fn set_extent(&mut self, value: crate::vulkan1_0::Extent3D) -> &mut Self {
        self.extent = value;
        self
    }
}
///[VkCopyBufferInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferInfo2.html) - Structure specifying parameters of a buffer copy command
///# C Specifications
///The [`CopyBufferInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCopyBufferInfo2 {
///    VkStructureType         sType;
///    const void*             pNext;
///    VkBuffer                srcBuffer;
///    VkBuffer                dstBuffer;
///    uint32_t                regionCount;
///    const VkBufferCopy2*    pRegions;
///} VkCopyBufferInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkCopyBufferInfo2 VkCopyBufferInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_buffer`] is the source buffer.
/// - [`dst_buffer`] is the destination buffer.
/// - [`region_count`] is the number of regions to copy.
/// - [`regions`] is a pointer to an array of [`BufferCopy2`] structures specifying the regions to
///   copy.
///# Description
///Members defined by this structure with the same name as parameters in
///[`CmdCopyBuffer`] have the identical effect to those parameters; the
///child structure [`BufferCopy2`] is a variant of [`BufferCopy`] which
///includes [`s_type`] and [`p_next`] parameters, allowing it to be extended.Valid Usage
/// - The `srcOffset` member of each element of [`regions`]**must** be less than the size of
///   [`src_buffer`]
/// - The `dstOffset` member of each element of [`regions`]**must** be less than the size of
///   [`dst_buffer`]
/// - The `size` member of each element of [`regions`]**must** be less than or equal to the size of
///   [`src_buffer`] minus `srcOffset`
/// - The `size` member of each element of [`regions`]**must** be less than or equal to the size of
///   [`dst_buffer`] minus `dstOffset`
/// - The union of the source regions, and the union of the destination regions, specified by the
///   elements of [`regions`], **must** not overlap in memory
/// - [`src_buffer`]**must** have been created with `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` usage flag
/// - If [`src_buffer`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_buffer`]**must** have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_buffer`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_buffer`]**must** be a valid [`Buffer`] handle
/// - [`dst_buffer`]**must** be a valid [`Buffer`] handle
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid [`BufferCopy2`]
///   structures
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_buffer`], and [`src_buffer`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Buffer`]
/// - [`BufferCopy2`]
/// - [`StructureType`]
/// - [`CmdCopyBuffer2`]
/// - [`CmdCopyBuffer2KHR`]
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
#[repr(C)]
pub struct CopyBufferInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_buffer`] is the source buffer.
    src_buffer: Buffer,
    ///[`dst_buffer`] is the destination buffer.
    dst_buffer: Buffer,
    ///[`region_count`] is the number of regions to copy.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`BufferCopy2`]
    ///structures specifying the regions to copy.
    regions: *const BufferCopy2<'lt>,
}
impl<'lt> Default for CopyBufferInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_buffer: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> CopyBufferInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const BufferCopy2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const BufferCopy2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_buffer`]
    pub fn src_buffer(&self) -> Buffer {
        self.src_buffer
    }
    ///Gets the value of [`Self::dst_buffer`]
    pub fn dst_buffer(&self) -> Buffer {
        self.dst_buffer
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[BufferCopy2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer`]
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Gets a mutable reference to the value of [`Self::dst_buffer`]
    pub fn dst_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_buffer
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::src_buffer`]
    pub fn set_src_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.src_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::dst_buffer`]
    pub fn set_dst_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.dst_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::BufferCopy2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
}
///[VkCopyImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageInfo2.html) - Structure specifying parameters of an image copy command
///# C Specifications
///The [`CopyImageInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCopyImageInfo2 {
///    VkStructureType        sType;
///    const void*            pNext;
///    VkImage                srcImage;
///    VkImageLayout          srcImageLayout;
///    VkImage                dstImage;
///    VkImageLayout          dstImageLayout;
///    uint32_t               regionCount;
///    const VkImageCopy2*    pRegions;
///} VkCopyImageInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkCopyImageInfo2 VkCopyImageInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_image`] is the source image.
/// - [`src_image_layout`] is the current layout of the source image subresource.
/// - [`dst_image`] is the destination image.
/// - [`dst_image_layout`] is the current layout of the destination image subresource.
/// - [`region_count`] is the number of regions to copy.
/// - [`regions`] is a pointer to an array of [`ImageCopy2`] structures specifying the regions to
///   copy.
///# Description
///Valid Usage
/// - The union of all source regions, and the union of all destination regions, specified by the
///   elements of [`regions`], **must** not overlap in memory
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`src_image`]**must** contain `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`
/// - [`src_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
/// - If [`src_image`] is non-sparse then the image or *disjoint* plane to be copied **must** be
///   bound completely and contiguously to a single [`DeviceMemory`] object
/// - [`src_image_layout`]**must** specify the layout of the image subresources of [`src_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`src_image_layout`]**must** be `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`dst_image`]**must** contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
/// - [`dst_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_image`] is non-sparse then the image or *disjoint* plane that is the destination of
///   the copy **must** be bound completely and contiguously to a single [`DeviceMemory`] object
/// - [`dst_image_layout`]**must** specify the layout of the image subresources of [`dst_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`dst_image_layout`]**must** be `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
/// - If the [`Format`] of each of [`src_image`] and [`dst_image`] is not a [*multi-planar format*](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   the [`Format`] of each of [`src_image`] and [`dst_image`]**must** be compatible, as defined [above](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-images-format-compatibility)
/// -    In a copy to or from a plane of a [multi-planar image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion), the [`Format`] of the image and plane **must** be compatible according to [the description of compatible planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes) for the plane being copied
/// - The sample count of [`src_image`] and [`dst_image`]**must** match
/// - The `srcSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `srcOffset` and `extent` members of each element of [`regions`]**must** respect the image
///   transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as
///   described in [`QueueFamilyProperties`]
/// - The `dstOffset` and `extent` members of each element of [`regions`]**must** respect the image
///   transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as
///   described in [`QueueFamilyProperties`]
/// - [`dst_image`] and [`src_image`]**must** not have been created with `flags` containing
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - If neither [`src_image`] nor [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   then for each element of [`regions`], `srcSubresource.aspectMask` and
///   `dstSubresource.aspectMask`**must** match
/// - If [`src_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   then for each element of [`regions`], `srcSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
/// - If [`src_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   then for each element of [`regions`], `srcSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
/// - If [`dst_image`] has a [`Format`] with [two planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   then for each element of [`regions`], `dstSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT` or `VK_IMAGE_ASPECT_PLANE_1_BIT`
/// - If [`dst_image`] has a [`Format`] with [three planes](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   then for each element of [`regions`], `dstSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
/// - If [`src_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   and the [`dst_image`] does not have a multi-planar image format, then for each element of
///   [`regions`], `dstSubresource.aspectMask`**must** be `VK_IMAGE_ASPECT_COLOR_BIT`
/// - If [`dst_image`] has a [multi-planar image format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///   and the [`src_image`] does not have a multi-planar image format, then for each element of
///   [`regions`], `srcSubresource.aspectMask`**must** be `VK_IMAGE_ASPECT_COLOR_BIT`
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`],
///   `srcSubresource.baseArrayLayer`**must** be `0` and `srcSubresource.layerCount`**must** be `1`
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element of [`regions`],
///   `dstSubresource.baseArrayLayer`**must** be `0` and `dstSubresource.layerCount`**must** be `1`
/// - For each element of [`regions`], `srcSubresource.aspectMask`**must** specify aspects present
///   in [`src_image`]
/// - For each element of [`regions`], `dstSubresource.aspectMask`**must** specify aspects present
///   in [`dst_image`]
/// - For each element of [`regions`], `srcOffset.x` and (`extent.width` +  `srcOffset.x`)**must**
///   both be greater than or equal to `0` and less than or equal to the width of the specified
///   `srcSubresource` of [`src_image`]
/// - For each element of [`regions`], `srcOffset.y` and (`extent.height` +  `srcOffset.y`)**must**
///   both be greater than or equal to `0` and less than or equal to the height of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `srcOffset.y`**must** be `0` and `extent.height`**must** be `1`
/// - For each element of [`regions`], `srcOffset.z` and (`extent.depth` +  `srcOffset.z`)**must**
///   both be greater than or equal to `0` and less than or equal to the depth of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `srcOffset.z`**must** be `0` and `extent.depth`**must** be `1`
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `dstOffset.z`**must** be `0` and `extent.depth`**must** be `1`
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`regions`],
///   `srcOffset.z`**must** be `0`
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, then for each element of [`regions`],
///   `dstOffset.z`**must** be `0`
/// - If [`src_image`] and [`dst_image`] are both of type `VK_IMAGE_TYPE_2D`, then for each element
///   of [`regions`], `extent.depth`**must** be `1`
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_2D`, and [`dst_image`] is of type
///   `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `extent.depth`**must** equal
///   `srcSubresource.layerCount`
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_2D`, and [`src_image`] is of type
///   `VK_IMAGE_TYPE_3D`, then for each element of [`regions`], `extent.depth`**must** equal
///   `dstSubresource.layerCount`
/// - For each element of [`regions`], `dstOffset.x` and (`extent.width` +  `dstOffset.x`)**must**
///   both be greater than or equal to `0` and less than or equal to the width of the specified
///   `dstSubresource` of [`dst_image`]
/// - For each element of [`regions`], `dstOffset.y` and (`extent.height` +  `dstOffset.y`)**must**
///   both be greater than or equal to `0` and less than or equal to the height of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `dstOffset.y`**must** be `0` and `extent.height`**must** be `1`
/// - For each element of [`regions`], `dstOffset.z` and (`extent.depth` +  `dstOffset.z`)**must**
///   both be greater than or equal to `0` and less than or equal to the depth of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], all members of `srcOffset`**must** be a multiple of the
///   corresponding dimensions of the compressed texel block
/// - If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.width`**must** be a multiple of the compressed
///   texel block width or (`extent.width` +  `srcOffset.x`)**must** equal the width of the
///   specified `srcSubresource` of [`src_image`]
/// - If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.height`**must** be a multiple of the compressed
///   texel block height or (`extent.height` +  `srcOffset.y`)**must** equal the height of the
///   specified `srcSubresource` of [`src_image`]
/// - If [`src_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.depth`**must** be a multiple of the compressed
///   texel block depth or (`extent.depth` +  `srcOffset.z`)**must** equal the depth of the
///   specified `srcSubresource` of [`src_image`]
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], all members of `dstOffset`**must** be a multiple of the
///   corresponding dimensions of the compressed texel block
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.width`**must** be a multiple of the compressed
///   texel block width or (`extent.width` +  `dstOffset.x`)**must** equal the width of the
///   specified `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.height`**must** be a multiple of the compressed
///   texel block height or (`extent.height` +  `dstOffset.y`)**must** equal the height of the
///   specified `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   then for each element of [`regions`], `extent.depth`**must** be a multiple of the compressed
///   texel block depth or (`extent.depth` +  `dstOffset.z`)**must** equal the depth of the
///   specified `dstSubresource` of [`dst_image`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_image`]**must** be a valid [`Image`] handle
/// - [`src_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`dst_image`]**must** be a valid [`Image`] handle
/// - [`dst_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid [`ImageCopy2`]
///   structures
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_image`], and [`src_image`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Image`]
/// - [`ImageCopy2`]
/// - [`ImageLayout`]
/// - [`StructureType`]
/// - [`CmdCopyImage2`]
/// - [`CmdCopyImage2KHR`]
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
#[repr(C)]
pub struct CopyImageInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_image`] is the source image.
    src_image: Image,
    ///[`src_image_layout`] is the current layout of the source image
    ///subresource.
    src_image_layout: ImageLayout,
    ///[`dst_image`] is the destination image.
    dst_image: Image,
    ///[`dst_image_layout`] is the current layout of the destination image
    ///subresource.
    dst_image_layout: ImageLayout,
    ///[`region_count`] is the number of regions to copy.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`ImageCopy2`] structures
    ///specifying the regions to copy.
    regions: *const ImageCopy2<'lt>,
}
impl<'lt> Default for CopyImageInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> CopyImageInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const ImageCopy2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const ImageCopy2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_image`]
    pub fn src_image(&self) -> Image {
        self.src_image
    }
    ///Gets the value of [`Self::src_image_layout`]
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Gets the value of [`Self::dst_image`]
    pub fn dst_image(&self) -> Image {
        self.dst_image
    }
    ///Gets the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[ImageCopy2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_image`]
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Gets a mutable reference to the value of [`Self::src_image_layout`]
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::dst_image`]
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Gets a mutable reference to the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::src_image`]
    pub fn set_src_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.src_image = value;
        self
    }
    ///Sets the raw value of [`Self::src_image_layout`]
    pub fn set_src_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.src_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image`]
    pub fn set_dst_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.dst_image = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image_layout`]
    pub fn set_dst_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.dst_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::ImageCopy2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
}
///[VkBlitImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlitImageInfo2.html) - Structure specifying parameters of blit image command
///# C Specifications
///The [`BlitImageInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkBlitImageInfo2 {
///    VkStructureType        sType;
///    const void*            pNext;
///    VkImage                srcImage;
///    VkImageLayout          srcImageLayout;
///    VkImage                dstImage;
///    VkImageLayout          dstImageLayout;
///    uint32_t               regionCount;
///    const VkImageBlit2*    pRegions;
///    VkFilter               filter;
///} VkBlitImageInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkBlitImageInfo2 VkBlitImageInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_image`] is the source image.
/// - [`src_image_layout`] is the layout of the source image subresources for the blit.
/// - [`dst_image`] is the destination image.
/// - [`dst_image_layout`] is the layout of the destination image subresources for the blit.
/// - [`region_count`] is the number of regions to blit.
/// - [`regions`] is a pointer to an array of [`ImageBlit2`] structures specifying the regions to
///   blit.
/// - [`filter`] is a [`Filter`] specifying the filter to apply if the blits require scaling.
///# Description
///Valid Usage
/// - The source region specified by each element of [`regions`]**must** be a region that is
///   contained within [`src_image`]
/// - The destination region specified by each element of [`regions`]**must** be a region that is
///   contained within [`dst_image`]
/// - The union of all destination regions, specified by the elements of [`regions`], **must** not
///   overlap in memory with any texel that **may** be sampled during the blit operation
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`src_image`]**must** contain `VK_FORMAT_FEATURE_BLIT_SRC_BIT`
/// - [`src_image`]**must** not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
/// - [`src_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
/// - If [`src_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`src_image_layout`]**must** specify the layout of the image subresources of [`src_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`src_image_layout`]**must** be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`,
///   `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`dst_image`]**must** contain `VK_FORMAT_FEATURE_BLIT_DST_BIT`
/// - [`dst_image`]**must** not use a [format that requires a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
/// - [`dst_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_image_layout`]**must** specify the layout of the image subresources of [`dst_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`dst_image_layout`]**must** be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`,
///   `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
/// - If either of [`src_image`] or [`dst_image`] was created with a signed integer [`Format`], the
///   other **must** also have been created with a signed integer [`Format`]
/// - If either of [`src_image`] or [`dst_image`] was created with an unsigned integer [`Format`],
///   the other **must** also have been created with an unsigned integer [`Format`]
/// - If either of [`src_image`] or [`dst_image`] was created with a depth/stencil format, the other
///   **must** have exactly the same format
/// - If [`src_image`] was created with a depth/stencil format, [`filter`]**must** be
///   `VK_FILTER_NEAREST`
/// - [`src_image`]**must** have been created with a `samples` value of `VK_SAMPLE_COUNT_1_BIT`
/// - [`dst_image`]**must** have been created with a `samples` value of `VK_SAMPLE_COUNT_1_BIT`
/// - If [`filter`] is `VK_FILTER_LINEAR`, then the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`src_image`]**must** contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If [`filter`] is `VK_FILTER_CUBIC_EXT`, then the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`src_image`]**must** contain `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - If [`filter`] is `VK_FILTER_CUBIC_EXT`, [`src_image`]**must** be of type `VK_IMAGE_TYPE_2D`
/// - The `srcSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`dst_image`] was created
/// - [`dst_image`] and [`src_image`]**must** not have been created with `flags` containing
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - If either [`src_image`] or [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, then for each element
///   of [`regions`], `srcSubresource.baseArrayLayer` and `dstSubresource.baseArrayLayer`**must**
///   each be `0`, and `srcSubresource.layerCount` and `dstSubresource.layerCount`**must** each be
///   `1`
/// - For each element of [`regions`], `srcSubresource.aspectMask`**must** specify aspects present
///   in [`src_image`]
/// - For each element of [`regions`], `dstSubresource.aspectMask`**must** specify aspects present
///   in [`dst_image`]
/// - For each element of [`regions`], `srcOffsets`[0].x and `srcOffsets`[1].x **must** both be
///   greater than or equal to `0` and less than or equal to the width of the specified
///   `srcSubresource` of [`src_image`]
/// - For each element of [`regions`], `srcOffsets`[0].y and `srcOffsets`[1].y **must** both be
///   greater than or equal to `0` and less than or equal to the height of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `srcOffsets`[0].y **must** be `0` and `srcOffsets`[1].y **must** be `1`
/// - For each element of [`regions`], `srcOffsets`[0].z and `srcOffsets`[1].z **must** both be
///   greater than or equal to `0` and less than or equal to the depth of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `srcOffsets`[0].z **must** be `0` and `srcOffsets`[1].z **must** be `1`
/// - For each element of [`regions`], `dstOffsets`[0].x and `dstOffsets`[1].x **must** both be
///   greater than or equal to `0` and less than or equal to the width of the specified
///   `dstSubresource` of [`dst_image`]
/// - For each element of [`regions`], `dstOffsets`[0].y and `dstOffsets`[1].y **must** both be
///   greater than or equal to `0` and less than or equal to the height of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `dstOffsets`[0].y **must** be `0` and `dstOffsets`[1].y **must** be `1`
/// - For each element of [`regions`], `dstOffsets`[0].z and `dstOffsets`[1].z **must** both be
///   greater than or equal to `0` and less than or equal to the depth of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `dstOffsets`[0].z **must** be `0` and `dstOffsets`[1].z **must** be `1`
/// - If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`]
///   chain, then [`src_image`] and [`dst_image`]**must** not be block-compressed images
/// - If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`]
///   chain, then [`src_image`]**must** be of type `VK_IMAGE_TYPE_2D`
/// -    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`]**must** not have a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_image`]**must** be a valid [`Image`] handle
/// - [`src_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`dst_image`]**must** be a valid [`Image`] handle
/// - [`dst_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid [`ImageBlit2`]
///   structures
/// - [`filter`]**must** be a valid [`Filter`] value
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_image`], and [`src_image`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Filter`]
/// - [`Image`]
/// - [`ImageBlit2`]
/// - [`ImageLayout`]
/// - [`StructureType`]
/// - [`CmdBlitImage2`]
/// - [`CmdBlitImage2KHR`]
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
#[repr(C)]
pub struct BlitImageInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_image`] is the source image.
    src_image: Image,
    ///[`src_image_layout`] is the layout of the source image subresources for
    ///the blit.
    src_image_layout: ImageLayout,
    ///[`dst_image`] is the destination image.
    dst_image: Image,
    ///[`dst_image_layout`] is the layout of the destination image subresources
    ///for the blit.
    dst_image_layout: ImageLayout,
    ///[`region_count`] is the number of regions to blit.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`ImageBlit2`] structures
    ///specifying the regions to blit.
    regions: *const ImageBlit2<'lt>,
    ///[`filter`] is a [`Filter`] specifying the filter to apply if the
    ///blits require scaling.
    filter: Filter,
}
impl<'lt> Default for BlitImageInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
            filter: Default::default(),
        }
    }
}
impl<'lt> BlitImageInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const ImageBlit2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const ImageBlit2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_image`]
    pub fn src_image(&self) -> Image {
        self.src_image
    }
    ///Gets the value of [`Self::src_image_layout`]
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Gets the value of [`Self::dst_image`]
    pub fn dst_image(&self) -> Image {
        self.dst_image
    }
    ///Gets the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[ImageBlit2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets the value of [`Self::filter`]
    pub fn filter(&self) -> Filter {
        self.filter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_image`]
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Gets a mutable reference to the value of [`Self::src_image_layout`]
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::dst_image`]
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Gets a mutable reference to the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::filter`]
    pub fn filter_mut(&mut self) -> &mut Filter {
        &mut self.filter
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
    ///Sets the raw value of [`Self::src_image`]
    pub fn set_src_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.src_image = value;
        self
    }
    ///Sets the raw value of [`Self::src_image_layout`]
    pub fn set_src_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.src_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image`]
    pub fn set_dst_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.dst_image = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image_layout`]
    pub fn set_dst_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.dst_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::ImageBlit2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
    ///Sets the raw value of [`Self::filter`]
    pub fn set_filter(&mut self, value: crate::vulkan1_0::Filter) -> &mut Self {
        self.filter = value;
        self
    }
}
///[VkCopyBufferToImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyBufferToImageInfo2.html) - Structure specifying parameters of a buffer to image copy command
///# C Specifications
///The [`CopyBufferToImageInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCopyBufferToImageInfo2 {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkBuffer                     srcBuffer;
///    VkImage                      dstImage;
///    VkImageLayout                dstImageLayout;
///    uint32_t                     regionCount;
///    const VkBufferImageCopy2*    pRegions;
///} VkCopyBufferToImageInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkCopyBufferToImageInfo2 VkCopyBufferToImageInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_buffer`] is the source buffer.
/// - [`dst_image`] is the destination image.
/// - [`dst_image_layout`] is the layout of the destination image subresources for the copy.
/// - [`region_count`] is the number of regions to copy.
/// - [`regions`] is a pointer to an array of [`BufferImageCopy2`] structures specifying the regions
///   to copy.
///# Description
///Valid Usage
/// - If the image region specified by each element of [`regions`] does not contain
///   [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, it **must** be a region that is
///   contained within the specified `imageSubresource` of [`dst_image`]
/// -    If the image region specified by each element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, the rotated destination region as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing)**must** be contained within [`dst_image`]
/// -    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`dst_image`]**must** not be a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#blocked-image)
/// - If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`]
///   chain, then [`dst_image`]**must** be of type `VK_IMAGE_TYPE_2D`
/// -    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`dst_image`]**must** not have a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///
/// -  [`src_buffer`]**must** be large enough to contain all buffer locations that are accessed according to [Buffer and Image Addressing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-buffers-images-addressing), for each element of [`regions`]
/// - The union of all source regions, and the union of all destination regions, specified by the
///   elements of [`regions`], **must** not overlap in memory
/// - [`src_buffer`]**must** have been created with `VK_BUFFER_USAGE_TRANSFER_SRC_BIT` usage flag
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`dst_image`]**must** contain `VK_FORMAT_FEATURE_TRANSFER_DST_BIT`
/// - If [`src_buffer`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_image`]**must** have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
/// - [`dst_image_layout`]**must** specify the layout of the image subresources of [`dst_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`dst_image_layout`]**must** be `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
/// - The `imageSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `imageSubresource.baseArrayLayer` +  `imageSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `imageOffset` and `imageExtent` members of each element of [`regions`]**must** respect the
///   image transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as
///   described in [`QueueFamilyProperties`]
/// - [`dst_image`]**must** not have been created with `flags` containing
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - If the queue family used to create the [`CommandPool`] which `commandBuffer` was allocated
///   from does not support `VK_QUEUE_GRAPHICS_BIT`, for each element of [`regions`], the
///   `aspectMask` member of `imageSubresource`**must** not be `VK_IMAGE_ASPECT_DEPTH_BIT` or
///   `VK_IMAGE_ASPECT_STENCIL_BIT`
/// - For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its
///   [`p_next`] chain, `imageOffset.x` and (`imageExtent.width` +  `imageOffset.x`)**must** both be
///   greater than or equal to `0` and less than or equal to the width of the specified
///   `imageSubresource` of [`dst_image`]
/// - For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its
///   [`p_next`] chain, `imageOffset.y` and (`imageExtent.height` +  `imageOffset.y`)**must** both
///   be greater than or equal to `0` and less than or equal to the height of the specified
///   `imageSubresource` of [`dst_image`]
///
/// - If [`dst_image`] does not have either a depth/stencil or a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `bufferOffset`**must** be a multiple of the format’s
///   texel block size
/// - If [`dst_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `bufferOffset`**must** be a multiple of the element size
///   of the compatible format for the format and the `aspectMask` of the `imageSubresource` as defined
///   in [[formats-compatible-planes]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes)
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `imageOffset.y`**must** be `0` and `imageExtent.height`**must** be `1`
/// - For each element of [`regions`], `imageOffset.z` and (`imageExtent.depth` +
///   `imageOffset.z`)**must** both be greater than or equal to `0` and less than or equal to the
///   depth of the specified `imageSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `imageOffset.z`**must** be `0` and `imageExtent.depth`**must** be `1`
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength`**must** be a multiple of the compressed
///   texel block width
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferImageHeight`**must** be a multiple of the compressed
///   texel block height
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], all members of `imageOffset`**must** be a multiple of the
///   corresponding dimensions of the compressed texel block
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferOffset`**must** be a multiple of the compressed texel
///   block size in bytes
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.width`**must** be a multiple of the compressed
///   texel block width or (`imageExtent.width` +  `imageOffset.x`)**must** equal the width of the
///   specified `imageSubresource` of [`dst_image`]
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.height`**must** be a multiple of the compressed
///   texel block height or (`imageExtent.height` +  `imageOffset.y`)**must** equal the height of
///   the specified `imageSubresource` of [`dst_image`]
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.depth`**must** be a multiple of the compressed
///   texel block depth or (`imageExtent.depth` +  `imageOffset.z`)**must** equal the depth of the
///   specified `imageSubresource` of [`dst_image`]
/// - For each element of [`regions`], `imageSubresource.aspectMask`**must** specify aspects present
///   in [`dst_image`]
/// - If [`dst_image`] has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `imageSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
///   (with `VK_IMAGE_ASPECT_PLANE_2_BIT` valid only for image formats with three planes)
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_3D`, for each element of [`regions`],
///   `imageSubresource.baseArrayLayer`**must** be `0` and `imageSubresource.layerCount`**must** be
///   `1`
/// - If [`dst_image`] is not a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength` multiplied by the texel block size of
///   [`dst_image`]**must** be less than or equal to 2<sup>31</sup>-1
/// - If [`dst_image`] is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength` divided by the compressed texel block width
///   and then multiplied by the texel block size of [`dst_image`]**must** be less than or equal to
///   2<sup>31</sup>-1
/// - If the queue family used to create the [`CommandPool`] which `commandBuffer` was allocated
///   from does not support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`, the `bufferOffset`
///   member of any element of [`regions`]**must** be a multiple of `4`
/// - If [`dst_image`] has a depth/stencil format, the `bufferOffset` member of any element of
///   [`regions`]**must** be a multiple of `4`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_buffer`]**must** be a valid [`Buffer`] handle
/// - [`dst_image`]**must** be a valid [`Image`] handle
/// - [`dst_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid
///   [`BufferImageCopy2`] structures
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_image`], and [`src_buffer`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Buffer`]
/// - [`BufferImageCopy2`]
/// - [`Image`]
/// - [`ImageLayout`]
/// - [`StructureType`]
/// - [`CmdCopyBufferToImage2`]
/// - [`CmdCopyBufferToImage2KHR`]
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
#[repr(C)]
pub struct CopyBufferToImageInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_buffer`] is the source buffer.
    src_buffer: Buffer,
    ///[`dst_image`] is the destination image.
    dst_image: Image,
    ///[`dst_image_layout`] is the layout of the destination image subresources
    ///for the copy.
    dst_image_layout: ImageLayout,
    ///[`region_count`] is the number of regions to copy.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`BufferImageCopy2`]
    ///structures specifying the regions to copy.
    regions: *const BufferImageCopy2<'lt>,
}
impl<'lt> Default for CopyBufferToImageInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> CopyBufferToImageInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const BufferImageCopy2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const BufferImageCopy2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_buffer`]
    pub fn src_buffer(&self) -> Buffer {
        self.src_buffer
    }
    ///Gets the value of [`Self::dst_image`]
    pub fn dst_image(&self) -> Image {
        self.dst_image
    }
    ///Gets the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[BufferImageCopy2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_buffer`]
    pub fn src_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.src_buffer
    }
    ///Gets a mutable reference to the value of [`Self::dst_image`]
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Gets a mutable reference to the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::src_buffer`]
    pub fn set_src_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.src_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image`]
    pub fn set_dst_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.dst_image = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image_layout`]
    pub fn set_dst_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.dst_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::BufferImageCopy2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
}
///[VkCopyImageToBufferInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyImageToBufferInfo2.html) - Structure specifying parameters of an image to buffer copy command
///# C Specifications
///The [`CopyImageToBufferInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCopyImageToBufferInfo2 {
///    VkStructureType              sType;
///    const void*                  pNext;
///    VkImage                      srcImage;
///    VkImageLayout                srcImageLayout;
///    VkBuffer                     dstBuffer;
///    uint32_t                     regionCount;
///    const VkBufferImageCopy2*    pRegions;
///} VkCopyImageToBufferInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkCopyImageToBufferInfo2 VkCopyImageToBufferInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_image`] is the source image.
/// - [`src_image_layout`] is the layout of the source image subresources for the copy.
/// - [`dst_buffer`] is the destination buffer.
/// - [`region_count`] is the number of regions to copy.
/// - [`regions`] is a pointer to an array of [`BufferImageCopy2`] structures specifying the regions
///   to copy.
///# Description
///Valid Usage
/// - If the image region specified by each element of [`regions`] does not contain
///   [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, it **must** be contained within the
///   specified `imageSubresource` of [`src_image`]
/// -    If the image region specified by each element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, the rotated source region as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#copies-buffers-images-rotation-addressing)**must** be contained within [`src_image`]
/// -    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`]**must** not be a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#blocked-image)
/// - If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`]
///   chain, then [`src_image`]**must** be of type `VK_IMAGE_TYPE_2D`
/// -    If any element of [`regions`] contains [`CopyCommandTransformInfoQCOM`] in its [`p_next`] chain, then [`src_image`]**must** not have a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///
/// -  [`dst_buffer`]**must** be large enough to contain all buffer locations that are accessed according to [Buffer and Image Addressing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#copies-buffers-images-addressing), for each element of [`regions`]
/// - The union of all source regions, and the union of all destination regions, specified by the
///   elements of [`regions`], **must** not overlap in memory
/// - [`src_image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT` usage flag
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`src_image`]**must** contain `VK_FORMAT_FEATURE_TRANSFER_SRC_BIT`
/// - If [`src_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_buffer`]**must** have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_buffer`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`src_image`]**must** have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
/// - [`src_image_layout`]**must** specify the layout of the image subresources of [`src_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`src_image_layout`]**must** be `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_GENERAL`, or `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
/// - The `imageSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
/// - The `imageSubresource.baseArrayLayer` +  `imageSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`src_image`] was created
/// - The `imageOffset` and `imageExtent` members of each element of [`regions`]**must** respect the
///   image transfer granularity requirements of `commandBuffer`’s command pool’s queue family, as
///   described in [`QueueFamilyProperties`]
/// - [`src_image`]**must** not have been created with `flags` containing
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its
///   [`p_next`] chain, `imageOffset.x` and (`imageExtent.width` +  `imageOffset.x`)**must** both be
///   greater than or equal to `0` and less than or equal to the width of the specified
///   `imageSubresource` of [`src_image`]
/// - For each element of [`regions`] not containing [`CopyCommandTransformInfoQCOM`] in its
///   [`p_next`] chain, `imageOffset.y` and (`imageExtent.height` +  `imageOffset.y`)**must** both
///   be greater than or equal to `0` and less than or equal to the height of the specified
///   `imageSubresource` of [`src_image`]
///
/// - If {imageparam} does not have either a depth/stencil or a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `bufferOffset`**must** be a multiple of the format’s
///   texel block size
/// - If {imageparam} has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `bufferOffset`**must** be a multiple of the element size
///   of the compatible format for the format and the `aspectMask` of the `imageSubresource` as defined
///   in [[formats-compatible-planes]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-compatible-planes)
/// - If {imageparam} is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `imageOffset.y`**must** be `0` and `imageExtent.height`**must** be `1`
/// - For each element of [`regions`], `imageOffset.z` and (`imageExtent.depth` +
///   `imageOffset.z`)**must** both be greater than or equal to `0` and less than or equal to the
///   depth of the specified `imageSubresource` of {imageparam}
/// - If {imageparam} is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `imageOffset.z`**must** be `0` and `imageExtent.depth`**must** be `1`
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength`**must** be a multiple of the compressed
///   texel block width
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferImageHeight`**must** be a multiple of the compressed
///   texel block height
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], all members of `imageOffset`**must** be a multiple of the
///   corresponding dimensions of the compressed texel block
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferOffset`**must** be a multiple of the compressed texel
///   block size in bytes
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.width`**must** be a multiple of the compressed
///   texel block width or (`imageExtent.width` +  `imageOffset.x`)**must** equal the width of the
///   specified `imageSubresource` of {imageparam}
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.height`**must** be a multiple of the compressed
///   texel block height or (`imageExtent.height` +  `imageOffset.y`)**must** equal the height of
///   the specified `imageSubresource` of {imageparam}
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `imageExtent.depth`**must** be a multiple of the compressed
///   texel block depth or (`imageExtent.depth` +  `imageOffset.z`)**must** equal the depth of the
///   specified `imageSubresource` of {imageparam}
/// - For each element of [`regions`], `imageSubresource.aspectMask`**must** specify aspects present
///   in {imageparam}
/// - If {imageparam} has a [multi-planar format](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion),
///   then for each element of [`regions`], `imageSubresource.aspectMask`**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, or `VK_IMAGE_ASPECT_PLANE_2_BIT`
///   (with `VK_IMAGE_ASPECT_PLANE_2_BIT` valid only for image formats with three planes)
/// - If {imageparam} is of type `VK_IMAGE_TYPE_3D`, for each element of [`regions`],
///   `imageSubresource.baseArrayLayer`**must** be `0` and `imageSubresource.layerCount`**must** be
///   `1`
/// - If {imageparam} is not a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength` multiplied by the texel block size of
///   {imageparam} **must** be less than or equal to 2<sup>31</sup>-1
/// - If {imageparam} is a [blocked image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#blocked-image),
///   for each element of [`regions`], `bufferRowLength` divided by the compressed texel block width
///   and then multiplied by the texel block size of {imageparam} **must** be less than or equal to
///   2<sup>31</sup>-1
/// - If the queue family used to create the [`CommandPool`] which `commandBuffer` was allocated
///   from does not support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`, the `bufferOffset`
///   member of any element of [`regions`]**must** be a multiple of `4`
/// - If {imageparam} has a depth/stencil format, the `bufferOffset` member of any element of
///   [`regions`]**must** be a multiple of `4`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_image`]**must** be a valid [`Image`] handle
/// - [`src_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`dst_buffer`]**must** be a valid [`Buffer`] handle
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid
///   [`BufferImageCopy2`] structures
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_buffer`], and [`src_image`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Buffer`]
/// - [`BufferImageCopy2`]
/// - [`Image`]
/// - [`ImageLayout`]
/// - [`StructureType`]
/// - [`CmdCopyImageToBuffer2`]
/// - [`CmdCopyImageToBuffer2KHR`]
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
#[repr(C)]
pub struct CopyImageToBufferInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_image`] is the source image.
    src_image: Image,
    ///[`src_image_layout`] is the layout of the source image subresources for
    ///the copy.
    src_image_layout: ImageLayout,
    ///[`dst_buffer`] is the destination buffer.
    dst_buffer: Buffer,
    ///[`region_count`] is the number of regions to copy.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`BufferImageCopy2`]
    ///structures specifying the regions to copy.
    regions: *const BufferImageCopy2<'lt>,
}
impl<'lt> Default for CopyImageToBufferInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_buffer: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> CopyImageToBufferInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const BufferImageCopy2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const BufferImageCopy2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_image`]
    pub fn src_image(&self) -> Image {
        self.src_image
    }
    ///Gets the value of [`Self::src_image_layout`]
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Gets the value of [`Self::dst_buffer`]
    pub fn dst_buffer(&self) -> Buffer {
        self.dst_buffer
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[BufferImageCopy2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_image`]
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Gets a mutable reference to the value of [`Self::src_image_layout`]
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::dst_buffer`]
    pub fn dst_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.dst_buffer
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::src_image`]
    pub fn set_src_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.src_image = value;
        self
    }
    ///Sets the raw value of [`Self::src_image_layout`]
    pub fn set_src_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.src_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::dst_buffer`]
    pub fn set_dst_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.dst_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::BufferImageCopy2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
}
///[VkResolveImageInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveImageInfo2.html) - Structure specifying parameters of resolve image command
///# C Specifications
///The [`ResolveImageInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkResolveImageInfo2 {
///    VkStructureType           sType;
///    const void*               pNext;
///    VkImage                   srcImage;
///    VkImageLayout             srcImageLayout;
///    VkImage                   dstImage;
///    VkImageLayout             dstImageLayout;
///    uint32_t                  regionCount;
///    const VkImageResolve2*    pRegions;
///} VkResolveImageInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_copy_commands2
///typedef VkResolveImageInfo2 VkResolveImageInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_image`] is the source image.
/// - [`src_image_layout`] is the layout of the source image subresources for the resolve.
/// - [`dst_image`] is the destination image.
/// - [`dst_image_layout`] is the layout of the destination image subresources for the resolve.
/// - [`region_count`] is the number of regions to resolve.
/// - [`regions`] is a pointer to an array of [`ImageResolve2`] structures specifying the regions to
///   resolve.
///# Description
///Valid Usage
/// - The union of all source regions, and the union of all destination regions, specified by the
///   elements of [`regions`], **must** not overlap in memory
/// - If [`src_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`src_image`]**must** have a sample count equal to any valid sample count value other than
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`dst_image`] is non-sparse then it **must** be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_image`]**must** have a sample count equal to `VK_SAMPLE_COUNT_1_BIT`
/// - [`src_image_layout`]**must** specify the layout of the image subresources of [`src_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`src_image_layout`]**must** be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`,
///   `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
/// - [`dst_image_layout`]**must** specify the layout of the image subresources of [`dst_image`]
///   specified in [`regions`] at the time this command is executed on a [`Device`]
/// - [`dst_image_layout`]**must** be `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`,
///   `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` or `VK_IMAGE_LAYOUT_GENERAL`
/// - The [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features)
///   of [`dst_image`]**must** contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// -    If the [`linearColorAttachment`]() feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, the [format features](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-image-format-features) of [`dst_image`]**must** contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// - [`src_image`] and [`dst_image`]**must** have been created with the same image format
/// - The `srcSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.mipLevel` member of each element of [`regions`]**must** be less than the
///   `mipLevels` specified in [`ImageCreateInfo`] when [`dst_image`] was created
/// - The `srcSubresource.baseArrayLayer` +  `srcSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`src_image`] was created
/// - The `dstSubresource.baseArrayLayer` +  `dstSubresource.layerCount` of each element of
///   [`regions`]**must** be less than or equal to the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`dst_image`] was created
/// - [`dst_image`] and [`src_image`]**must** not have been created with `flags` containing
///   `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - If either [`src_image`] or [`dst_image`] are of type `VK_IMAGE_TYPE_3D`, then for each element
///   of [`regions`], `srcSubresource.baseArrayLayer`**must** be `0` and
///   `srcSubresource.layerCount`**must** be `1`
/// - If either [`src_image`] or [`dst_image`] are of type `VK_IMAGE_TYPE_3D`, then for each element
///   of [`regions`], `dstSubresource.baseArrayLayer`**must** be `0` and
///   `dstSubresource.layerCount`**must** be `1`
/// - For each element of [`regions`], `srcOffset.x` and (`extent.width` +  `srcOffset.x`)**must**
///   both be greater than or equal to `0` and less than or equal to the width of the specified
///   `srcSubresource` of [`src_image`]
/// - For each element of [`regions`], `srcOffset.y` and (`extent.height` +  `srcOffset.y`)**must**
///   both be greater than or equal to `0` and less than or equal to the height of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `srcOffset.y`**must** be `0` and `extent.height`**must** be `1`
/// - For each element of [`regions`], `srcOffset.z` and (`extent.depth` +  `srcOffset.z`)**must**
///   both be greater than or equal to `0` and less than or equal to the depth of the specified
///   `srcSubresource` of [`src_image`]
/// - If [`src_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `srcOffset.z`**must** be `0` and `extent.depth`**must** be `1`
/// - For each element of [`regions`], `dstOffset.x` and (`extent.width` +  `dstOffset.x`)**must**
///   both be greater than or equal to `0` and less than or equal to the width of the specified
///   `dstSubresource` of [`dst_image`]
/// - For each element of [`regions`], `dstOffset.y` and (`extent.height` +  `dstOffset.y`)**must**
///   both be greater than or equal to `0` and less than or equal to the height of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D`, then for each element of [`regions`],
///   `dstOffset.y`**must** be `0` and `extent.height`**must** be `1`
/// - For each element of [`regions`], `dstOffset.z` and (`extent.depth` +  `dstOffset.z`)**must**
///   both be greater than or equal to `0` and less than or equal to the depth of the specified
///   `dstSubresource` of [`dst_image`]
/// - If [`dst_image`] is of type `VK_IMAGE_TYPE_1D` or `VK_IMAGE_TYPE_2D`, then for each element of
///   [`regions`], `dstOffset.z`**must** be `0` and `extent.depth`**must** be `1`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_image`]**must** be a valid [`Image`] handle
/// - [`src_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`dst_image`]**must** be a valid [`Image`] handle
/// - [`dst_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`regions`]**must** be a valid pointer to an array of [`region_count`] valid [`ImageResolve2`]
///   structures
/// - [`region_count`]**must** be greater than `0`
/// - Both of [`dst_image`], and [`src_image`]**must** have been created, allocated, or retrieved
///   from the same [`Device`]
///# Related
/// - [`VK_KHR_copy_commands2`]
/// - [`crate::vulkan1_3`]
/// - [`Image`]
/// - [`ImageLayout`]
/// - [`ImageResolve2`]
/// - [`StructureType`]
/// - [`CmdResolveImage2`]
/// - [`CmdResolveImage2KHR`]
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
#[repr(C)]
pub struct ResolveImageInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_image`] is the source image.
    src_image: Image,
    ///[`src_image_layout`] is the layout of the source image subresources for
    ///the resolve.
    src_image_layout: ImageLayout,
    ///[`dst_image`] is the destination image.
    dst_image: Image,
    ///[`dst_image_layout`] is the layout of the destination image subresources
    ///for the resolve.
    dst_image_layout: ImageLayout,
    ///[`region_count`] is the number of regions to resolve.
    region_count: u32,
    ///[`regions`] is a pointer to an array of [`ImageResolve2`]
    ///structures specifying the regions to resolve.
    regions: *const ImageResolve2<'lt>,
}
impl<'lt> Default for ResolveImageInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> ResolveImageInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::region_count`]
    pub fn region_count_raw(&self) -> u32 {
        self.region_count
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const ImageResolve2<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count_raw(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const ImageResolve2<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::src_image`]
    pub fn src_image(&self) -> Image {
        self.src_image
    }
    ///Gets the value of [`Self::src_image_layout`]
    pub fn src_image_layout(&self) -> ImageLayout {
        self.src_image_layout
    }
    ///Gets the value of [`Self::dst_image`]
    pub fn dst_image(&self) -> Image {
        self.dst_image
    }
    ///Gets the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout(&self) -> ImageLayout {
        self.dst_image_layout
    }
    ///Gets the value of [`Self::region_count`]
    pub fn region_count(&self) -> u32 {
        self.region_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[ImageResolve2<'lt>] {
        std::slice::from_raw_parts(self.regions, self.region_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_image`]
    pub fn src_image_mut(&mut self) -> &mut Image {
        &mut self.src_image
    }
    ///Gets a mutable reference to the value of [`Self::src_image_layout`]
    pub fn src_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.src_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::dst_image`]
    pub fn dst_image_mut(&mut self) -> &mut Image {
        &mut self.dst_image
    }
    ///Gets a mutable reference to the value of [`Self::dst_image_layout`]
    pub fn dst_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.dst_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::region_count`]
    pub fn region_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::src_image`]
    pub fn set_src_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.src_image = value;
        self
    }
    ///Sets the raw value of [`Self::src_image_layout`]
    pub fn set_src_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.src_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image`]
    pub fn set_dst_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.dst_image = value;
        self
    }
    ///Sets the raw value of [`Self::dst_image_layout`]
    pub fn set_dst_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.dst_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::region_count`]
    pub fn set_region_count(&mut self, value: u32) -> &mut Self {
        self.region_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(&mut self, value: &'lt [crate::vulkan1_3::ImageResolve2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.region_count = len_;
        self
    }
}
///[VkPhysicalDeviceShaderTerminateInvocationFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderTerminateInvocationFeatures.html) - Structure describing support for the SPIR-V code:SPV_KHR_terminate_invocation extension
///# C Specifications
///The [`PhysicalDeviceShaderTerminateInvocationFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderTerminateInvocation;
///} VkPhysicalDeviceShaderTerminateInvocationFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_terminate_invocation
///typedef VkPhysicalDeviceShaderTerminateInvocationFeatures
/// VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_terminate_invocation`] specifies whether the implementation supports SPIR-V modules
///   that use the `SPV_KHR_terminate_invocation` extension.
///If the [`PhysicalDeviceShaderTerminateInvocationFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderTerminateInvocationFeatures`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES`
///# Related
/// - [`VK_KHR_shader_terminate_invocation`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceShaderTerminateInvocationFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_terminate_invocation`] specifies whether the implementation
    ///supports SPIR-V modules that use the `SPV_KHR_terminate_invocation`
    ///extension.
    shader_terminate_invocation: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderTerminateInvocationFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_terminate_invocation: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderTerminateInvocationFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation_raw(&self) -> Bool32 {
        self.shader_terminate_invocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_terminate_invocation`]
    pub fn set_shader_terminate_invocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_terminate_invocation = value;
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
    ///Gets the value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_terminate_invocation as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_terminate_invocation`]
    pub fn shader_terminate_invocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_terminate_invocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_terminate_invocation as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_terminate_invocation`]
    pub fn set_shader_terminate_invocation(&mut self, value: bool) -> &mut Self {
        self.shader_terminate_invocation = value as u8 as u32;
        self
    }
}
///[VkMemoryBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryBarrier2.html) - Structure specifying a global memory barrier
///# C Specifications
///The [`MemoryBarrier2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkMemoryBarrier2 {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkPipelineStageFlags2    srcStageMask;
///    VkAccessFlags2           srcAccessMask;
///    VkPipelineStageFlags2    dstStageMask;
///    VkAccessFlags2           dstAccessMask;
///} VkMemoryBarrier2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkMemoryBarrier2 VkMemoryBarrier2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [first access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
/// - [`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [second access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
///# Description
///This structure defines a [memory
///dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory) affecting all device memory.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
///[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described by
///this structure include only operations and memory accesses specified by
///[`src_stage_mask`] and [`src_access_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
///and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described
///by this structure include only operations and memory accesses specified by
///[`dst_stage_mask`] and [`dst_access_mask`].Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`src_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`src_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`dst_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`dst_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2`
/// - [`src_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`src_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
/// - [`dst_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`dst_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`AccessFlags2`]
/// - [`DependencyInfo`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct MemoryBarrier2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    src_stage_mask: PipelineStageFlags2,
    ///[`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [first
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    src_access_mask: AccessFlags2,
    ///[`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    dst_stage_mask: PipelineStageFlags2,
    ///[`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [second
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    dst_access_mask: AccessFlags2,
}
impl<'lt> Default for MemoryBarrier2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
        }
    }
}
impl<'lt> MemoryBarrier2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Gets the value of [`Self::src_access_mask`]
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Gets the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Gets the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::src_access_mask`]
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
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
    ///Sets the raw value of [`Self::src_stage_mask`]
    pub fn set_src_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::src_access_mask`]
    pub fn set_src_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.src_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_stage_mask`]
    pub fn set_dst_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_access_mask`]
    pub fn set_dst_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.dst_access_mask = value;
        self
    }
}
///[VkImageMemoryBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryBarrier2.html) - Structure specifying an image memory barrier
///# C Specifications
///The [`ImageMemoryBarrier2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkImageMemoryBarrier2 {
///    VkStructureType            sType;
///    const void*                pNext;
///    VkPipelineStageFlags2      srcStageMask;
///    VkAccessFlags2             srcAccessMask;
///    VkPipelineStageFlags2      dstStageMask;
///    VkAccessFlags2             dstAccessMask;
///    VkImageLayout              oldLayout;
///    VkImageLayout              newLayout;
///    uint32_t                   srcQueueFamilyIndex;
///    uint32_t                   dstQueueFamilyIndex;
///    VkImage                    image;
///    VkImageSubresourceRange    subresourceRange;
///} VkImageMemoryBarrier2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkImageMemoryBarrier2 VkImageMemoryBarrier2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [first access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
/// - [`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [second access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
/// - [`old_layout`] is the old layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
/// - [`new_layout`] is the new layout in an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
/// - [`src_queue_family_index`] is the source queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
/// - [`dst_queue_family_index`] is the destination queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
/// - [`image`] is a handle to the image affected by this barrier.
/// - [`subresource_range`] describes the [image subresource range](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views)
///   within [`image`] that is affected by this barrier.
///# Description
///This structure defines a [memory
///dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory) limited to an image subresource range, and **can** define a
///[queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and
///[image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions) for
///that subresource range.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
///[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described by
///this structure include only operations and memory accesses specified by
///[`src_stage_mask`] and [`src_access_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
///and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described
///by this structure include only operations and memory accesses specified by
///[`dst_stage_mask`] and [`dst_access_mask`].Both [access scopes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) are
///limited to only memory accesses to [`image`] in the subresource range
///defined by [`subresource_range`].If [`image`] was created with `VK_SHARING_MODE_EXCLUSIVE`, and
///[`src_queue_family_index`] is not equal to [`dst_queue_family_index`], this
///memory barrier defines a [queue family
///transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
///When executed on a queue in the family identified by
///[`src_queue_family_index`], this barrier defines a
///[queue family release operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release)
///for the specified image subresource range, and the second synchronization
///and access scopes do not synchronize operations on that queue.
///When executed on a queue in the family identified by
///[`dst_queue_family_index`], this barrier defines a
///[queue family acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire)
///for the specified image subresource range, and the first synchronization and
///access scopes do not synchronize operations on that queue.A [queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) is
///also defined if the values are not equal, and either is one of the special
///queue family values reserved for external memory ownership transfers, as
///described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
///A [queue family release
///operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) is defined when [`dst_queue_family_index`] is one of those
///values, and a [queue family
///acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) is defined when [`src_queue_family_index`] is one of
///those values.If [`old_layout`] is not equal to [`new_layout`], then the memory barrier
///defines an [image layout
///transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions) for the specified image subresource range.
///If this memory barrier defines a [queue
///family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers), the layout transition is only executed once
///between the queues.If [`image`] has a multi-planar format and the image is *disjoint*, then
///including `VK_IMAGE_ASPECT_COLOR_BIT` in the `aspectMask` member of
///[`subresource_range`] is equivalent to including
///`VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and
///(for three-plane formats only) `VK_IMAGE_ASPECT_PLANE_2_BIT`.Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`src_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`src_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`dst_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`dst_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///
/// - `subresourceRange.baseMipLevel`**must** be less than the `mipLevels` specified in
///   [`ImageCreateInfo`] when [`image`] was created
/// - If `subresourceRange.levelCount` is not [`REMAINING_MIP_LEVELS`],
///   `subresourceRange.baseMipLevel` +  `subresourceRange.levelCount`**must** be less than or equal
///   to the `mipLevels` specified in [`ImageCreateInfo`] when [`image`] was created
/// - `subresourceRange.baseArrayLayer`**must** be less than the `arrayLayers` specified in
///   [`ImageCreateInfo`] when [`image`] was created
/// - If `subresourceRange.layerCount` is not [`REMAINING_ARRAY_LAYERS`],
///   `subresourceRange.baseArrayLayer` +  `subresourceRange.layerCount`**must** be less than or
///   equal to the `arrayLayers` specified in [`ImageCreateInfo`] when [`image`] was created
/// - If [`image`] is non-sparse then it **must** be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`
///   then [`image`]**must** have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_SAMPLED_BIT` or
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_SRC_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_TRANSFER_DST_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   [`old_layout`]**must** be `VK_IMAGE_LAYOUT_UNDEFINED` or the current layout of the image
///   subresources affected by the barrier
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   [`new_layout`]**must** not be `VK_IMAGE_LAYOUT_UNDEFINED` or `VK_IMAGE_LAYOUT_PREINITIALIZED`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` then [`image`]**must** have been
///   created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL` then [`image`]**must** have been
///   created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL` then
///   [`image`]**must** have been created with at least one of
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` then
///   [`image`]**must** have been created with at least one of
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`, `VK_IMAGE_USAGE_SAMPLED_BIT`, or
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` then
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` set
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL`,
///   [`image`]**must** have been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` or
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL`, [`image`]**must**
///   have been created with at least one of `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`,
///   `VK_IMAGE_USAGE_SAMPLED_BIT`, or `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
/// - If [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
///   or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-image-layout-transitions),
///   and [`old_layout`] or [`new_layout`] is
///   `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR` then [`image`]**must** have
///   been created with `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR` set
/// - If [`image`] has a single-plane color format or is not *disjoint*, then the `aspectMask`
///   member of [`subresource_range`]**must** be `VK_IMAGE_ASPECT_COLOR_BIT`
/// - If [`image`] has a multi-planar format and the image is *disjoint*, then the `aspectMask`
///   member of [`subresource_range`]**must** include either at least one of
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT`, and
///   `VK_IMAGE_ASPECT_PLANE_2_BIT`; or **must** include `VK_IMAGE_ASPECT_COLOR_BIT`
/// - If [`image`] has a multi-planar format with only two planes, then the `aspectMask` member of
///   [`subresource_range`]**must** not include `VK_IMAGE_ASPECT_PLANE_2_BIT`
/// - If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is enabled, then the `aspectMask` member of [`subresource_range`]**must** include
///   either or both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
/// - If [`image`] has a depth/stencil format with both depth and stencil and the [separateDepthStencilLayouts](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, then the `aspectMask` member of [`subresource_range`]**must** include
///   both `VK_IMAGE_ASPECT_DEPTH_BIT` and `VK_IMAGE_ASPECT_STENCIL_BIT`
/// -    If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], at least one **must** not be a special queue family reserved for external memory ownership transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
/// - If [`image`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`,
///   [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, and one of
///   [`src_queue_family_index`] and [`dst_queue_family_index`] is one of the special queue family
///   values reserved for external memory transfers, the other **must** be [`QUEUE_FAMILY_IGNORED`]
/// -    If [`image`] was created with a sharing mode of `VK_SHARING_MODE_EXCLUSIVE`, and [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, [`src_queue_family_index`] and [`dst_queue_family_index`]**must** both be valid queue families, or one of the special queue family values reserved for external memory transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
/// - If either [`src_stage_mask`] or [`dst_stage_mask`] includes `VK_PIPELINE_STAGE_2_HOST_BIT`,
///   [`src_queue_family_index`] and [`dst_queue_family_index`]**must** be equal
/// -    If [`src_stage_mask`] includes `VK_PIPELINE_STAGE_2_HOST_BIT`, and [`src_queue_family_index`] and [`dst_queue_family_index`] define a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) or [`old_layout`] and [`new_layout`] define an [image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions), [`old_layout`]**must** be one of `VK_IMAGE_LAYOUT_PREINITIALIZED`, `VK_IMAGE_LAYOUT_UNDEFINED`, or `VK_IMAGE_LAYOUT_GENERAL`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of [`SampleLocationsInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`src_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`src_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
/// - [`dst_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`dst_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
/// - [`old_layout`]**must** be a valid [`ImageLayout`] value
/// - [`new_layout`]**must** be a valid [`ImageLayout`] value
/// - [`image`]**must** be a valid [`Image`] handle
/// - [`subresource_range`]**must** be a valid [`ImageSubresourceRange`] structure
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`AccessFlags2`]
/// - [`DependencyInfo`]
/// - [`Image`]
/// - [`ImageLayout`]
/// - [`ImageSubresourceRange`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct ImageMemoryBarrier2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    src_stage_mask: PipelineStageFlags2,
    ///[`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [first
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    src_access_mask: AccessFlags2,
    ///[`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    dst_stage_mask: PipelineStageFlags2,
    ///[`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [second
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    dst_access_mask: AccessFlags2,
    ///[`old_layout`] is the old layout in an
    ///[image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
    old_layout: ImageLayout,
    ///[`new_layout`] is the new layout in an
    ///[image layout transition](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).
    new_layout: ImageLayout,
    ///[`src_queue_family_index`] is the source queue family for a
    ///[queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
    src_queue_family_index: u32,
    ///[`dst_queue_family_index`] is the destination queue family for a
    ///[queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
    dst_queue_family_index: u32,
    ///[`image`] is a handle to the image affected by this barrier.
    image: Image,
    ///[`subresource_range`] describes the [image
    ///subresource range](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) within [`image`] that is affected by this barrier.
    subresource_range: ImageSubresourceRange,
}
impl<'lt> Default for ImageMemoryBarrier2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            old_layout: Default::default(),
            new_layout: Default::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            image: Default::default(),
            subresource_range: Default::default(),
        }
    }
}
impl<'lt> ImageMemoryBarrier2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index_raw(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Gets the raw value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index_raw(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::src_queue_family_index`]
    pub fn set_src_queue_family_index_raw(&mut self, value: u32) -> &mut Self {
        self.src_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::dst_queue_family_index`]
    pub fn set_dst_queue_family_index_raw(&mut self, value: u32) -> &mut Self {
        self.dst_queue_family_index = value;
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
    ///Gets the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Gets the value of [`Self::src_access_mask`]
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Gets the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Gets the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Gets the value of [`Self::old_layout`]
    pub fn old_layout(&self) -> ImageLayout {
        self.old_layout
    }
    ///Gets the value of [`Self::new_layout`]
    pub fn new_layout(&self) -> ImageLayout {
        self.new_layout
    }
    ///Gets the value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Gets the value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets the value of [`Self::subresource_range`]
    pub fn subresource_range(&self) -> ImageSubresourceRange {
        self.subresource_range
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::src_access_mask`]
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::old_layout`]
    pub fn old_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.old_layout
    }
    ///Gets a mutable reference to the value of [`Self::new_layout`]
    pub fn new_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.new_layout
    }
    ///Gets a mutable reference to the value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Gets a mutable reference to the value of [`Self::subresource_range`]
    pub fn subresource_range_mut(&mut self) -> &mut ImageSubresourceRange {
        &mut self.subresource_range
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
    ///Sets the raw value of [`Self::src_stage_mask`]
    pub fn set_src_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::src_access_mask`]
    pub fn set_src_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.src_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_stage_mask`]
    pub fn set_dst_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_access_mask`]
    pub fn set_dst_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.dst_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::old_layout`]
    pub fn set_old_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.old_layout = value;
        self
    }
    ///Sets the raw value of [`Self::new_layout`]
    pub fn set_new_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.new_layout = value;
        self
    }
    ///Sets the raw value of [`Self::src_queue_family_index`]
    pub fn set_src_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.src_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::dst_queue_family_index`]
    pub fn set_dst_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.dst_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
    ///Sets the raw value of [`Self::subresource_range`]
    pub fn set_subresource_range(&mut self, value: crate::vulkan1_0::ImageSubresourceRange) -> &mut Self {
        self.subresource_range = value;
        self
    }
}
///[VkBufferMemoryBarrier2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryBarrier2.html) - Structure specifying a buffer memory barrier
///# C Specifications
///The [`BufferMemoryBarrier2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkBufferMemoryBarrier2 {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkPipelineStageFlags2    srcStageMask;
///    VkAccessFlags2           srcAccessMask;
///    VkPipelineStageFlags2    dstStageMask;
///    VkAccessFlags2           dstAccessMask;
///    uint32_t                 srcQueueFamilyIndex;
///    uint32_t                 dstQueueFamilyIndex;
///    VkBuffer                 buffer;
///    VkDeviceSize             offset;
///    VkDeviceSize             size;
///} VkBufferMemoryBarrier2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkBufferMemoryBarrier2 VkBufferMemoryBarrier2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [first access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
/// - [`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
/// - [`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be included in the [second access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
/// - [`src_queue_family_index`] is the source queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
/// - [`dst_queue_family_index`] is the destination queue family for a [queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
/// - [`buffer`] is a handle to the buffer whose backing memory is affected by the barrier.
/// - [`offset`] is an offset in bytes into the backing memory for [`buffer`]; this is relative to
///   the base offset as bound to the buffer (see [`BindBufferMemory`]).
/// - [`size`] is a size in bytes of the affected area of backing memory for [`buffer`], or
///   [`WHOLE_SIZE`] to use the range from [`offset`] to the end of the buffer.
///# Description
///This structure defines a [memory
///dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory) limited to a range of a buffer, and **can** define a
///[queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) for
///that range.The first [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes) and
///[access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described by
///this structure include only operations and memory accesses specified by
///[`src_stage_mask`] and [`src_access_mask`].The second [synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes)
///and [access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) described
///by this structure include only operations and memory accesses specified by
///[`dst_stage_mask`] and [`dst_access_mask`].Both [access scopes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes) are
///limited to only memory accesses to [`buffer`] in the range defined by
///[`offset`] and [`size`].If [`buffer`] was created with `VK_SHARING_MODE_EXCLUSIVE`, and
///[`src_queue_family_index`] is not equal to [`dst_queue_family_index`], this
///memory barrier defines a [queue family
///transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
///When executed on a queue in the family identified by
///[`src_queue_family_index`], this barrier defines a
///[queue family release operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release)
///for the specified buffer range, and the second synchronization and access
///scopes do not synchronize operations on that queue.
///When executed on a queue in the family identified by
///[`dst_queue_family_index`], this barrier defines a
///[queue family acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire)
///for the specified buffer range, and the first synchronization and access
///scopes do not synchronize operations on that queue.A [queue family transfer operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) is
///also defined if the values are not equal, and either is one of the special
///queue family values reserved for external memory ownership transfers, as
///described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
///A [queue family release
///operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-release) is defined when [`dst_queue_family_index`] is one of those
///values, and a [queue family
///acquire operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers-acquire) is defined when [`src_queue_family_index`] is one of
///those values.Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`src_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`src_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`src_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`src_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`src_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`src_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`src_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INDEX_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT`,
///   `VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT`,
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_UNIFORM_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_SAMPLED_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or one of the
///   `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT`,
///   `VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFER_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_COPY_BIT`, `VK_PIPELINE_STAGE_2_BLIT_BIT`,
///   `VK_PIPELINE_STAGE_2_RESOLVE_BIT`, `VK_PIPELINE_STAGE_2_CLEAR_BIT`,
///   `VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT`,
///   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_READ_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_HOST_WRITE_BIT`, [`dst_stage_mask`]**must**
///   include `VK_PIPELINE_STAGE_2_HOST_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT`,
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`, `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`,
///   `VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV` or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`,
///   [`dst_stage_mask`]**must** include
///   `VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT``VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT`, or
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`,
///   `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`, or one of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
///   or `VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT`
/// - If [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-rayQuery)
///   is not enabled and [`dst_access_mask`] includes
///   `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`, [`dst_stage_mask`]**must** not include any
///   of the `VK_PIPELINE_STAGE_*_SHADER_BIT` stages except
///   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_DECODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_DECODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_READ_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
/// - If [`dst_access_mask`] includes `VK_ACCESS_2_VIDEO_ENCODE_WRITE_BIT_KHR`,
///   [`dst_stage_mask`]**must** include `VK_PIPELINE_STAGE_2_VIDEO_ENCODE_BIT_KHR`
///
/// - [`offset`]**must** be less than the size of [`buffer`]
/// - If [`size`] is not equal to [`WHOLE_SIZE`], [`size`]**must** be greater than `0`
/// - If [`size`] is not equal to [`WHOLE_SIZE`], [`size`]**must** be less than or equal to than the
///   size of [`buffer`] minus [`offset`]
/// - If [`buffer`] is non-sparse then it **must** be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// -    If [`src_queue_family_index`] is not equal to [`dst_queue_family_index`], at least one **must** not be a special queue family reserved for external memory ownership transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
/// - If [`buffer`] was created with a sharing mode of `VK_SHARING_MODE_CONCURRENT`,
///   [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, and one of
///   [`src_queue_family_index`] and [`dst_queue_family_index`] is one of the special queue family
///   values reserved for external memory transfers, the other **must** be [`QUEUE_FAMILY_IGNORED`]
/// -    If [`buffer`] was created with a sharing mode of `VK_SHARING_MODE_EXCLUSIVE`, and [`src_queue_family_index`] and [`dst_queue_family_index`] are not equal, [`src_queue_family_index`] and [`dst_queue_family_index`]**must** both be valid queue families, or one of the special queue family values reserved for external memory transfers, as described in [[synchronization-queue-transfers]](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#synchronization-queue-transfers)
/// - If either [`src_stage_mask`] or [`dst_stage_mask`] includes `VK_PIPELINE_STAGE_2_HOST_BIT`,
///   [`src_queue_family_index`] and [`dst_queue_family_index`]**must** be equal
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2`
/// - [`p_next`]**must** be `NULL`
/// - [`src_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`src_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
/// - [`dst_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
/// - [`dst_access_mask`]**must** be a valid combination of [`AccessFlagBits2`] values
/// - [`buffer`]**must** be a valid [`Buffer`] handle
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`AccessFlags2`]
/// - [`Buffer`]
/// - [`DependencyInfo`]
/// - [`DeviceSize`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct BufferMemoryBarrier2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [first synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    src_stage_mask: PipelineStageFlags2,
    ///[`src_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [first
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    src_access_mask: AccessFlags2,
    ///[`dst_stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline
    ///stages to be included in the [second synchronization scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-scopes).
    dst_stage_mask: PipelineStageFlags2,
    ///[`dst_access_mask`] is a [`AccessFlags2`] mask of access flags to be
    ///included in the [second
    ///access scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-access-scopes).
    dst_access_mask: AccessFlags2,
    ///[`src_queue_family_index`] is the source queue family for a
    ///[queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
    src_queue_family_index: u32,
    ///[`dst_queue_family_index`] is the destination queue family for a
    ///[queue family ownership transfer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers).
    dst_queue_family_index: u32,
    ///[`buffer`] is a handle to the buffer whose backing memory is affected
    ///by the barrier.
    buffer: Buffer,
    ///[`offset`] is an offset in bytes into the backing memory for
    ///[`buffer`]; this is relative to the base offset as bound to the buffer
    ///(see [`BindBufferMemory`]).
    offset: DeviceSize,
    ///[`size`] is a size in bytes of the affected area of backing memory for
    ///[`buffer`], or [`WHOLE_SIZE`] to use the range from [`offset`]
    ///to the end of the buffer.
    size: DeviceSize,
}
impl<'lt> Default for BufferMemoryBarrier2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_stage_mask: Default::default(),
            dst_access_mask: Default::default(),
            src_queue_family_index: 0,
            dst_queue_family_index: 0,
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
        }
    }
}
impl<'lt> BufferMemoryBarrier2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index_raw(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Gets the raw value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index_raw(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::src_queue_family_index`]
    pub fn set_src_queue_family_index_raw(&mut self, value: u32) -> &mut Self {
        self.src_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::dst_queue_family_index`]
    pub fn set_dst_queue_family_index_raw(&mut self, value: u32) -> &mut Self {
        self.dst_queue_family_index = value;
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
    ///Gets the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask(&self) -> PipelineStageFlags2 {
        self.src_stage_mask
    }
    ///Gets the value of [`Self::src_access_mask`]
    pub fn src_access_mask(&self) -> AccessFlags2 {
        self.src_access_mask
    }
    ///Gets the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask(&self) -> PipelineStageFlags2 {
        self.dst_stage_mask
    }
    ///Gets the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask(&self) -> AccessFlags2 {
        self.dst_access_mask
    }
    ///Gets the value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index(&self) -> u32 {
        self.src_queue_family_index
    }
    ///Gets the value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index(&self) -> u32 {
        self.dst_queue_family_index
    }
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.src_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::src_access_mask`]
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.src_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.dst_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags2 {
        &mut self.dst_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::src_queue_family_index`]
    pub fn src_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::dst_queue_family_index`]
    pub fn dst_queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
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
    ///Sets the raw value of [`Self::src_stage_mask`]
    pub fn set_src_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.src_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::src_access_mask`]
    pub fn set_src_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.src_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_stage_mask`]
    pub fn set_dst_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.dst_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_access_mask`]
    pub fn set_dst_access_mask(&mut self, value: crate::vulkan1_3::AccessFlags2) -> &mut Self {
        self.dst_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::src_queue_family_index`]
    pub fn set_src_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.src_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::dst_queue_family_index`]
    pub fn set_dst_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.dst_queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
    ///Sets the raw value of [`Self::offset`]
    pub fn set_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.offset = value;
        self
    }
    ///Sets the raw value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
}
///[VkDependencyInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDependencyInfo.html) - Structure specifying dependency information for a synchronization command
///# C Specifications
///The [`DependencyInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkDependencyInfo {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkDependencyFlags                dependencyFlags;
///    uint32_t                         memoryBarrierCount;
///    const VkMemoryBarrier2*          pMemoryBarriers;
///    uint32_t                         bufferMemoryBarrierCount;
///    const VkBufferMemoryBarrier2*    pBufferMemoryBarriers;
///    uint32_t                         imageMemoryBarrierCount;
///    const VkImageMemoryBarrier2*     pImageMemoryBarriers;
///} VkDependencyInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkDependencyInfo VkDependencyInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`dependency_flags`] is a bitmask of [`DependencyFlagBits`] specifying how execution and
///   memory dependencies are formed.
/// - [`memory_barrier_count`] is the length of the [`memory_barriers`] array.
/// - [`memory_barriers`] is a pointer to an array of [`MemoryBarrier2`] structures defining memory
///   dependencies between any memory accesses.
/// - [`buffer_memory_barrier_count`] is the length of the [`buffer_memory_barriers`] array.
/// - [`buffer_memory_barriers`] is a pointer to an array of [`BufferMemoryBarrier2`] structures
///   defining memory dependencies between buffer ranges.
/// - [`image_memory_barrier_count`] is the length of the [`image_memory_barriers`] array.
/// - [`image_memory_barriers`] is a pointer to an array of [`ImageMemoryBarrier2`] structures
///   defining memory dependencies between image subresources.
///# Description
///This structure defines a set of [memory dependencies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory), as well as [queue
///family transfer operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-queue-transfers) and [image layout transitions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-image-layout-transitions).Each member of [`memory_barriers`], [`buffer_memory_barriers`], and
///[`image_memory_barriers`] defines a separate
///[memory dependency](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies-memory).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEPENDENCY_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`dependency_flags`]**must** be a valid combination of [`DependencyFlagBits`] values
/// - If [`memory_barrier_count`] is not `0`, [`memory_barriers`]**must** be a valid pointer to an
///   array of [`memory_barrier_count`] valid [`MemoryBarrier2`] structures
/// - If [`buffer_memory_barrier_count`] is not `0`, [`buffer_memory_barriers`]**must** be a valid
///   pointer to an array of [`buffer_memory_barrier_count`] valid [`BufferMemoryBarrier2`]
///   structures
/// - If [`image_memory_barrier_count`] is not `0`, [`image_memory_barriers`]**must** be a valid
///   pointer to an array of [`image_memory_barrier_count`] valid [`ImageMemoryBarrier2`] structures
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`BufferMemoryBarrier2`]
/// - [`DependencyFlags`]
/// - [`ImageMemoryBarrier2`]
/// - [`MemoryBarrier2`]
/// - [`StructureType`]
/// - [`CmdPipelineBarrier2`]
/// - [`CmdPipelineBarrier2KHR`]
/// - [`CmdSetEvent2`]
/// - [`CmdSetEvent2KHR`]
/// - [`CmdWaitEvents2`]
/// - [`CmdWaitEvents2KHR`]
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
#[repr(C)]
pub struct DependencyInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`dependency_flags`] is a bitmask of [`DependencyFlagBits`]
    ///specifying how execution and memory dependencies are formed.
    dependency_flags: DependencyFlags,
    ///[`memory_barrier_count`] is the length of the [`memory_barriers`]
    ///array.
    memory_barrier_count: u32,
    ///[`memory_barriers`] is a pointer to an array of [`MemoryBarrier2`]
    ///structures defining memory dependencies between any memory accesses.
    memory_barriers: *const MemoryBarrier2<'lt>,
    ///[`buffer_memory_barrier_count`] is the length of the
    ///[`buffer_memory_barriers`] array.
    buffer_memory_barrier_count: u32,
    ///[`buffer_memory_barriers`] is a pointer to an array of
    ///[`BufferMemoryBarrier2`] structures defining memory dependencies
    ///between buffer ranges.
    buffer_memory_barriers: *const BufferMemoryBarrier2<'lt>,
    ///[`image_memory_barrier_count`] is the length of the
    ///[`image_memory_barriers`] array.
    image_memory_barrier_count: u32,
    ///[`image_memory_barriers`] is a pointer to an array of
    ///[`ImageMemoryBarrier2`] structures defining memory dependencies
    ///between image subresources.
    image_memory_barriers: *const ImageMemoryBarrier2<'lt>,
}
impl<'lt> Default for DependencyInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            dependency_flags: Default::default(),
            memory_barrier_count: 0,
            memory_barriers: std::ptr::null(),
            buffer_memory_barrier_count: 0,
            buffer_memory_barriers: std::ptr::null(),
            image_memory_barrier_count: 0,
            image_memory_barriers: std::ptr::null(),
        }
    }
}
impl<'lt> DependencyInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::memory_barrier_count`]
    pub fn memory_barrier_count_raw(&self) -> u32 {
        self.memory_barrier_count
    }
    ///Gets the raw value of [`Self::memory_barriers`]
    pub fn memory_barriers_raw(&self) -> *const MemoryBarrier2<'lt> {
        self.memory_barriers
    }
    ///Gets the raw value of [`Self::buffer_memory_barrier_count`]
    pub fn buffer_memory_barrier_count_raw(&self) -> u32 {
        self.buffer_memory_barrier_count
    }
    ///Gets the raw value of [`Self::buffer_memory_barriers`]
    pub fn buffer_memory_barriers_raw(&self) -> *const BufferMemoryBarrier2<'lt> {
        self.buffer_memory_barriers
    }
    ///Gets the raw value of [`Self::image_memory_barrier_count`]
    pub fn image_memory_barrier_count_raw(&self) -> u32 {
        self.image_memory_barrier_count
    }
    ///Gets the raw value of [`Self::image_memory_barriers`]
    pub fn image_memory_barriers_raw(&self) -> *const ImageMemoryBarrier2<'lt> {
        self.image_memory_barriers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_barrier_count`]
    pub fn set_memory_barrier_count_raw(&mut self, value: u32) -> &mut Self {
        self.memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::memory_barriers`]
    pub fn set_memory_barriers_raw(&mut self, value: *const MemoryBarrier2<'lt>) -> &mut Self {
        self.memory_barriers = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_memory_barrier_count`]
    pub fn set_buffer_memory_barrier_count_raw(&mut self, value: u32) -> &mut Self {
        self.buffer_memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_memory_barriers`]
    pub fn set_buffer_memory_barriers_raw(&mut self, value: *const BufferMemoryBarrier2<'lt>) -> &mut Self {
        self.buffer_memory_barriers = value;
        self
    }
    ///Sets the raw value of [`Self::image_memory_barrier_count`]
    pub fn set_image_memory_barrier_count_raw(&mut self, value: u32) -> &mut Self {
        self.image_memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::image_memory_barriers`]
    pub fn set_image_memory_barriers_raw(&mut self, value: *const ImageMemoryBarrier2<'lt>) -> &mut Self {
        self.image_memory_barriers = value;
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
    ///Gets the value of [`Self::dependency_flags`]
    pub fn dependency_flags(&self) -> DependencyFlags {
        self.dependency_flags
    }
    ///Gets the value of [`Self::memory_barrier_count`]
    pub fn memory_barrier_count(&self) -> u32 {
        self.memory_barrier_count
    }
    ///Gets the value of [`Self::memory_barriers`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn memory_barriers(&self) -> &[MemoryBarrier2<'lt>] {
        std::slice::from_raw_parts(self.memory_barriers, self.memory_barrier_count as usize)
    }
    ///Gets the value of [`Self::buffer_memory_barrier_count`]
    pub fn buffer_memory_barrier_count(&self) -> u32 {
        self.buffer_memory_barrier_count
    }
    ///Gets the value of [`Self::buffer_memory_barriers`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn buffer_memory_barriers(&self) -> &[BufferMemoryBarrier2<'lt>] {
        std::slice::from_raw_parts(self.buffer_memory_barriers, self.buffer_memory_barrier_count as usize)
    }
    ///Gets the value of [`Self::image_memory_barrier_count`]
    pub fn image_memory_barrier_count(&self) -> u32 {
        self.image_memory_barrier_count
    }
    ///Gets the value of [`Self::image_memory_barriers`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn image_memory_barriers(&self) -> &[ImageMemoryBarrier2<'lt>] {
        std::slice::from_raw_parts(self.image_memory_barriers, self.image_memory_barrier_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dependency_flags`]
    pub fn dependency_flags_mut(&mut self) -> &mut DependencyFlags {
        &mut self.dependency_flags
    }
    ///Gets a mutable reference to the value of [`Self::memory_barrier_count`]
    pub fn memory_barrier_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::buffer_memory_barrier_count`]
    pub fn buffer_memory_barrier_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image_memory_barrier_count`]
    pub fn image_memory_barrier_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::dependency_flags`]
    pub fn set_dependency_flags(&mut self, value: crate::vulkan1_0::DependencyFlags) -> &mut Self {
        self.dependency_flags = value;
        self
    }
    ///Sets the raw value of [`Self::memory_barrier_count`]
    pub fn set_memory_barrier_count(&mut self, value: u32) -> &mut Self {
        self.memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::memory_barriers`]
    pub fn set_memory_barriers(&mut self, value: &'lt [crate::vulkan1_3::MemoryBarrier2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.memory_barriers = value.as_ptr();
        self.memory_barrier_count = len_;
        self
    }
    ///Sets the raw value of [`Self::buffer_memory_barrier_count`]
    pub fn set_buffer_memory_barrier_count(&mut self, value: u32) -> &mut Self {
        self.buffer_memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_memory_barriers`]
    pub fn set_buffer_memory_barriers(
        &mut self,
        value: &'lt [crate::vulkan1_3::BufferMemoryBarrier2<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.buffer_memory_barriers = value.as_ptr();
        self.buffer_memory_barrier_count = len_;
        self
    }
    ///Sets the raw value of [`Self::image_memory_barrier_count`]
    pub fn set_image_memory_barrier_count(&mut self, value: u32) -> &mut Self {
        self.image_memory_barrier_count = value;
        self
    }
    ///Sets the raw value of [`Self::image_memory_barriers`]
    pub fn set_image_memory_barriers(&mut self, value: &'lt [crate::vulkan1_3::ImageMemoryBarrier2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.image_memory_barriers = value.as_ptr();
        self.image_memory_barrier_count = len_;
        self
    }
}
///[VkSemaphoreSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSubmitInfo.html) - Structure specifying a semaphore signal or wait operation
///# C Specifications
///The [`SemaphoreSubmitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkSemaphoreSubmitInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkSemaphore              semaphore;
///    uint64_t                 value;
///    VkPipelineStageFlags2    stageMask;
///    uint32_t                 deviceIndex;
///} VkSemaphoreSubmitInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkSemaphoreSubmitInfo VkSemaphoreSubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is a [`Semaphore`] affected by this operation.
/// - [`value`] is either the value used to signal [`semaphore`] or the value waited on by
///   [`semaphore`], if [`semaphore`] is a timeline semaphore. Otherwise it is ignored.
/// - [`stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages which limit the first synchronization scope of a semaphore signal operation, or second synchronization scope of a semaphore wait operation as described in the [semaphore wait operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting) and [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) sections of [the synchronization chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).
/// - [`device_index`] is the index of the device within a device group that executes the semaphore
///   wait or signal operation.
///# Description
///Whether this structure defines a semaphore wait or signal operation is
///defined by how it is used.Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - If the `device` that [`semaphore`] was created on is not a device group,
///   [`device_index`]**must** be `0`
/// - If the `device` that [`semaphore`] was created on is a device group, [`device_index`]**must**
///   be a valid device index
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits2`] values
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`PipelineStageFlags2`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SubmitInfo2`]
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
#[repr(C)]
pub struct SemaphoreSubmitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is a [`Semaphore`] affected by this operation.
    semaphore: Semaphore,
    ///[`value`] is
    ///either the value used to signal [`semaphore`] or the value waited on
    ///by [`semaphore`], if [`semaphore`] is a timeline semaphore.
    ///Otherwise it is
    ///ignored.
    value: u64,
    ///[`stage_mask`] is a [`PipelineStageFlags2`] mask of pipeline stages
    ///which limit the first synchronization scope of a semaphore signal
    ///operation, or second synchronization scope of a semaphore wait operation
    ///as described in the [semaphore wait
    ///operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting) and [semaphore signal
    ///operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) sections of [the synchronization
    ///chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization).
    stage_mask: PipelineStageFlags2,
    ///[`device_index`] is the index of the device within a device group that
    ///executes the semaphore wait or signal operation.
    device_index: u32,
}
impl<'lt> Default for SemaphoreSubmitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: 0,
            stage_mask: Default::default(),
            device_index: 0,
        }
    }
}
impl<'lt> SemaphoreSubmitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::value`]
    pub fn value_raw(&self) -> u64 {
        self.value
    }
    ///Gets the raw value of [`Self::device_index`]
    pub fn device_index_raw(&self) -> u32 {
        self.device_index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::value`]
    pub fn set_value_raw(&mut self, value: u64) -> &mut Self {
        self.value = value;
        self
    }
    ///Sets the raw value of [`Self::device_index`]
    pub fn set_device_index_raw(&mut self, value: u32) -> &mut Self {
        self.device_index = value;
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::value`]
    pub fn value(&self) -> u64 {
        self.value
    }
    ///Gets the value of [`Self::stage_mask`]
    pub fn stage_mask(&self) -> PipelineStageFlags2 {
        self.stage_mask
    }
    ///Gets the value of [`Self::device_index`]
    pub fn device_index(&self) -> u32 {
        self.device_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::value`]
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::stage_mask`]
    pub fn stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::device_index`]
    pub fn device_index_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::value`]
    pub fn set_value(&mut self, value: u64) -> &mut Self {
        self.value = value;
        self
    }
    ///Sets the raw value of [`Self::stage_mask`]
    pub fn set_stage_mask(&mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> &mut Self {
        self.stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::device_index`]
    pub fn set_device_index(&mut self, value: u32) -> &mut Self {
        self.device_index = value;
        self
    }
}
///[VkCommandBufferSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferSubmitInfo.html) - Structure specifying a command buffer submission
///# C Specifications
///The [`CommandBufferSubmitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCommandBufferSubmitInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkCommandBuffer    commandBuffer;
///    uint32_t           deviceMask;
///} VkCommandBufferSubmitInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkCommandBufferSubmitInfo VkCommandBufferSubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`command_buffer`] is a [`CommandBuffer`] to be submitted for execution.
/// - [`device_mask`] is a bitmask indicating which devices in a device group execute the command
///   buffer. A [`device_mask`] of `0` is equivalent to setting all bits corresponding to valid
///   devices in the group to `1`.
///# Description
///Valid Usage
/// - [`command_buffer`]**must** not have been allocated with `VK_COMMAND_BUFFER_LEVEL_SECONDARY`
/// - If [`device_mask`] is not `0`, it **must** be a valid device mask
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`command_buffer`]**must** be a valid [`CommandBuffer`] handle
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`CommandBuffer`]
/// - [`StructureType`]
/// - [`SubmitInfo2`]
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
#[repr(C)]
pub struct CommandBufferSubmitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`command_buffer`] is a [`CommandBuffer`] to be submitted for
    ///execution.
    command_buffer: CommandBuffer,
    ///[`device_mask`] is a bitmask indicating which devices in a device group
    ///execute the command buffer.
    ///A [`device_mask`] of `0` is equivalent to setting all bits
    ///corresponding to valid devices in the group to `1`.
    device_mask: u32,
}
impl<'lt> Default for CommandBufferSubmitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            command_buffer: Default::default(),
            device_mask: 0,
        }
    }
}
impl<'lt> CommandBufferSubmitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_mask`]
    pub fn device_mask_raw(&self) -> u32 {
        self.device_mask
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask_raw(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
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
    ///Gets the value of [`Self::command_buffer`]
    pub fn command_buffer(&self) -> CommandBuffer {
        self.command_buffer
    }
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::command_buffer`]
    pub fn command_buffer_mut(&mut self) -> &mut CommandBuffer {
        &mut self.command_buffer
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::command_buffer`]
    pub fn set_command_buffer(&mut self, value: crate::vulkan1_0::CommandBuffer) -> &mut Self {
        self.command_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
        self
    }
}
///[VkSubmitInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubmitInfo2.html) - Structure specifying a queue submit operation
///# C Specifications
///The [`SubmitInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkSubmitInfo2 {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkSubmitFlags                       flags;
///    uint32_t                            waitSemaphoreInfoCount;
///    const VkSemaphoreSubmitInfo*        pWaitSemaphoreInfos;
///    uint32_t                            commandBufferInfoCount;
///    const VkCommandBufferSubmitInfo*    pCommandBufferInfos;
///    uint32_t                            signalSemaphoreInfoCount;
///    const VkSemaphoreSubmitInfo*        pSignalSemaphoreInfos;
///} VkSubmitInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkSubmitInfo2 VkSubmitInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`SubmitFlagBits`].
/// - [`wait_semaphore_info_count`] is the number of elements in [`wait_semaphore_infos`].
/// - [`wait_semaphore_infos`] is a pointer to an array of [`SemaphoreSubmitInfo`] structures defining [semaphore wait operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).
/// - [`command_buffer_info_count`] is the number of elements in [`command_buffer_infos`] and the
///   number of command buffers to execute in the batch.
/// - [`command_buffer_infos`] is a pointer to an array of [`CommandBufferSubmitInfo`] structures
///   describing command buffers to execute in the batch.
/// - [`signal_semaphore_info_count`] is the number of elements in [`signal_semaphore_infos`].
/// - [`signal_semaphore_infos`] is a pointer to an array of [`SemaphoreSubmitInfo`] describing [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).
///# Description
///Valid Usage
/// - If the same semaphore is used as the `semaphore` member of both an element of
///   [`signal_semaphore_infos`] and [`wait_semaphore_infos`], and that semaphore is a timeline
///   semaphore, the `value` member of the [`signal_semaphore_infos`] element **must** be greater
///   than the `value` member of the [`wait_semaphore_infos`] element
/// -    If the `semaphore` member of any element of [`signal_semaphore_infos`] is a timeline semaphore, the `value` member of that element **must** have a value greater than the current value of the semaphore when the [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) is executed
/// -    If the `semaphore` member of any element of [`signal_semaphore_infos`] is a timeline semaphore, the `value` member of that element **must** have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
/// -    If the `semaphore` member of any element of [`wait_semaphore_infos`] is a timeline semaphore, the `value` member of that element **must** have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on that semaphore by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
/// - If [`flags`] includes `VK_SUBMIT_PROTECTED_BIT`, all elements of `pCommandBuffers`**must** be
///   protected command buffers
/// - If [`flags`] does not include `VK_SUBMIT_PROTECTED_BIT`, each element of
///   `pCommandBuffers`**must** not be a protected command buffer
/// -    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [resumed render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they **must** be suspended by a render pass instance earlier in submission order within [`command_buffer_infos`]
/// -    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), they **must** be resumed by a render pass instance later in submission order within [`command_buffer_infos`]
/// -    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there **must** be no action or synchronization commands between that render pass instance and the render pass instance that resumes it
/// -    If any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), there **must** be no render pass instances between that render pass instance and the render pass instance that resumes it
/// -    If the [`variableSampleLocations`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-variableSampleLocations) limit is not supported, and any `commandBuffer` member of an element of [`command_buffer_infos`] contains any [suspended render pass instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-suspension), where a graphics pipeline has been bound, any pipelines bound in the render pass instance that resumes it, or any subsequent render pass instances that resume from that one and so on, **must** use the same sample locations
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBMIT_INFO_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`PerformanceQuerySubmitInfoKHR`],
///   [`Win32KeyedMutexAcquireReleaseInfoKHR`], or [`Win32KeyedMutexAcquireReleaseInfoNV`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`SubmitFlagBits`] values
/// - If [`wait_semaphore_info_count`] is not `0`, [`wait_semaphore_infos`]**must** be a valid
///   pointer to an array of [`wait_semaphore_info_count`] valid [`SemaphoreSubmitInfo`] structures
/// - If [`command_buffer_info_count`] is not `0`, [`command_buffer_infos`]**must** be a valid
///   pointer to an array of [`command_buffer_info_count`] valid [`CommandBufferSubmitInfo`]
///   structures
/// - If [`signal_semaphore_info_count`] is not `0`, [`signal_semaphore_infos`]**must** be a valid
///   pointer to an array of [`signal_semaphore_info_count`] valid [`SemaphoreSubmitInfo`]
///   structures
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
/// - [`CommandBufferSubmitInfo`]
/// - [`SemaphoreSubmitInfo`]
/// - [`StructureType`]
/// - [`SubmitFlags`]
/// - [`QueueSubmit2`]
/// - [`QueueSubmit2KHR`]
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
#[repr(C)]
pub struct SubmitInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SubmitFlagBits`].
    flags: SubmitFlags,
    ///[`wait_semaphore_info_count`] is the number of elements in
    ///[`wait_semaphore_infos`].
    wait_semaphore_info_count: u32,
    ///[`wait_semaphore_infos`] is a pointer to an array of
    ///[`SemaphoreSubmitInfo`] structures defining
    ///[semaphore wait operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-waiting).
    wait_semaphore_infos: *const SemaphoreSubmitInfo<'lt>,
    ///[`command_buffer_info_count`] is the number of elements in
    ///[`command_buffer_infos`] and the number of command buffers to execute
    ///in the batch.
    command_buffer_info_count: u32,
    ///[`command_buffer_infos`] is a pointer to an array of
    ///[`CommandBufferSubmitInfo`] structures describing command buffers to
    ///execute in the batch.
    command_buffer_infos: *const CommandBufferSubmitInfo<'lt>,
    ///[`signal_semaphore_info_count`] is the number of elements in
    ///[`signal_semaphore_infos`].
    signal_semaphore_info_count: u32,
    ///[`signal_semaphore_infos`] is a pointer to an array of
    ///[`SemaphoreSubmitInfo`] describing
    ///[semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling).
    signal_semaphore_infos: *const SemaphoreSubmitInfo<'lt>,
}
impl<'lt> Default for SubmitInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            wait_semaphore_info_count: 0,
            wait_semaphore_infos: std::ptr::null(),
            command_buffer_info_count: 0,
            command_buffer_infos: std::ptr::null(),
            signal_semaphore_info_count: 0,
            signal_semaphore_infos: std::ptr::null(),
        }
    }
}
impl<'lt> SubmitInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_info_count`]
    pub fn wait_semaphore_info_count_raw(&self) -> u32 {
        self.wait_semaphore_info_count
    }
    ///Gets the raw value of [`Self::wait_semaphore_infos`]
    pub fn wait_semaphore_infos_raw(&self) -> *const SemaphoreSubmitInfo<'lt> {
        self.wait_semaphore_infos
    }
    ///Gets the raw value of [`Self::command_buffer_info_count`]
    pub fn command_buffer_info_count_raw(&self) -> u32 {
        self.command_buffer_info_count
    }
    ///Gets the raw value of [`Self::command_buffer_infos`]
    pub fn command_buffer_infos_raw(&self) -> *const CommandBufferSubmitInfo<'lt> {
        self.command_buffer_infos
    }
    ///Gets the raw value of [`Self::signal_semaphore_info_count`]
    pub fn signal_semaphore_info_count_raw(&self) -> u32 {
        self.signal_semaphore_info_count
    }
    ///Gets the raw value of [`Self::signal_semaphore_infos`]
    pub fn signal_semaphore_infos_raw(&self) -> *const SemaphoreSubmitInfo<'lt> {
        self.signal_semaphore_infos
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_info_count`]
    pub fn set_wait_semaphore_info_count_raw(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_infos`]
    pub fn set_wait_semaphore_infos_raw(&mut self, value: *const SemaphoreSubmitInfo<'lt>) -> &mut Self {
        self.wait_semaphore_infos = value;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_info_count`]
    pub fn set_command_buffer_info_count_raw(&mut self, value: u32) -> &mut Self {
        self.command_buffer_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_infos`]
    pub fn set_command_buffer_infos_raw(&mut self, value: *const CommandBufferSubmitInfo<'lt>) -> &mut Self {
        self.command_buffer_infos = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_info_count`]
    pub fn set_signal_semaphore_info_count_raw(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_infos`]
    pub fn set_signal_semaphore_infos_raw(&mut self, value: *const SemaphoreSubmitInfo<'lt>) -> &mut Self {
        self.signal_semaphore_infos = value;
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
    pub fn flags(&self) -> SubmitFlags {
        self.flags
    }
    ///Gets the value of [`Self::wait_semaphore_info_count`]
    pub fn wait_semaphore_info_count(&self) -> u32 {
        self.wait_semaphore_info_count
    }
    ///Gets the value of [`Self::wait_semaphore_infos`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphore_infos(&self) -> &[SemaphoreSubmitInfo<'lt>] {
        std::slice::from_raw_parts(self.wait_semaphore_infos, self.wait_semaphore_info_count as usize)
    }
    ///Gets the value of [`Self::command_buffer_info_count`]
    pub fn command_buffer_info_count(&self) -> u32 {
        self.command_buffer_info_count
    }
    ///Gets the value of [`Self::command_buffer_infos`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn command_buffer_infos(&self) -> &[CommandBufferSubmitInfo<'lt>] {
        std::slice::from_raw_parts(self.command_buffer_infos, self.command_buffer_info_count as usize)
    }
    ///Gets the value of [`Self::signal_semaphore_info_count`]
    pub fn signal_semaphore_info_count(&self) -> u32 {
        self.signal_semaphore_info_count
    }
    ///Gets the value of [`Self::signal_semaphore_infos`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn signal_semaphore_infos(&self) -> &[SemaphoreSubmitInfo<'lt>] {
        std::slice::from_raw_parts(self.signal_semaphore_infos, self.signal_semaphore_info_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SubmitFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_info_count`]
    pub fn wait_semaphore_info_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::command_buffer_info_count`]
    pub fn command_buffer_info_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::signal_semaphore_info_count`]
    pub fn signal_semaphore_info_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    pub fn set_flags(&mut self, value: crate::vulkan1_3::SubmitFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_info_count`]
    pub fn set_wait_semaphore_info_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_infos`]
    pub fn set_wait_semaphore_infos(&mut self, value: &'lt [crate::vulkan1_3::SemaphoreSubmitInfo<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_infos = value.as_ptr();
        self.wait_semaphore_info_count = len_;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_info_count`]
    pub fn set_command_buffer_info_count(&mut self, value: u32) -> &mut Self {
        self.command_buffer_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_infos`]
    pub fn set_command_buffer_infos(
        &mut self,
        value: &'lt [crate::vulkan1_3::CommandBufferSubmitInfo<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.command_buffer_infos = value.as_ptr();
        self.command_buffer_info_count = len_;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_info_count`]
    pub fn set_signal_semaphore_info_count(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_infos`]
    pub fn set_signal_semaphore_infos(
        &mut self,
        value: &'lt [crate::vulkan1_3::SemaphoreSubmitInfo<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.signal_semaphore_infos = value.as_ptr();
        self.signal_semaphore_info_count = len_;
        self
    }
}
///[VkPhysicalDeviceSynchronization2Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSynchronization2Features.html) - Structure describing whether the implementation supports v2 synchronization commands
///# C Specifications
///The [`PhysicalDeviceSynchronization2Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceSynchronization2Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           synchronization2;
///} VkPhysicalDeviceSynchronization2Features;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_synchronization2
///typedef VkPhysicalDeviceSynchronization2Features VkPhysicalDeviceSynchronization2FeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`synchronization_2`] indicates whether the implementation supports the new set of
///   synchronization commands introduced in `[`VK_KHR_synchronization2`]`.
///If the [`PhysicalDeviceSynchronization2Features`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSynchronization2Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES`
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceSynchronization2Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`synchronization_2`]
    ///indicates whether the implementation supports the new set of
    ///synchronization commands introduced in `[`VK_KHR_synchronization2`]`.
    synchronization_2: Bool32,
}
impl<'lt> Default for PhysicalDeviceSynchronization2Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            synchronization_2: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSynchronization2Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::synchronization_2`]
    pub fn synchronization_2_raw(&self) -> Bool32 {
        self.synchronization_2
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::synchronization_2`]
    pub fn set_synchronization_2_raw(&mut self, value: Bool32) -> &mut Self {
        self.synchronization_2 = value;
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
    ///Gets the value of [`Self::synchronization_2`]
    pub fn synchronization_2(&self) -> bool {
        unsafe { std::mem::transmute(self.synchronization_2 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::synchronization_2`]
    pub fn synchronization_2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.synchronization_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.synchronization_2 as *mut Bool32)
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
    ///Sets the raw value of [`Self::synchronization_2`]
    pub fn set_synchronization_2(&mut self, value: bool) -> &mut Self {
        self.synchronization_2 = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceShaderIntegerDotProductFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductFeatures.html) - Structure describing integer dot product features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderIntegerDotProductFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderIntegerDotProduct;
///} VkPhysicalDeviceShaderIntegerDotProductFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_integer_dot_product
///typedef VkPhysicalDeviceShaderIntegerDotProductFeatures
/// VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR;
///```
///# Members
///The members of the [`PhysicalDeviceShaderIntegerDotProductFeatures`]
///structure describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_integer_dot_product`] specifies whether shader modules **can** declare the
///   `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`, `DotProductInput4x8BitPackedKHR` and
///   `DotProductKHR` capabilities.
///If the [`PhysicalDeviceShaderIntegerDotProductFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderIntegerDotProductFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES`
///# Related
/// - [`VK_KHR_shader_integer_dot_product`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceShaderIntegerDotProductFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_integer_dot_product`] specifies whether shader modules **can**
    ///declare the `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`,
    ///`DotProductInput4x8BitPackedKHR` and `DotProductKHR` capabilities.
    shader_integer_dot_product: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderIntegerDotProductFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_integer_dot_product: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderIntegerDotProductFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product_raw(&self) -> Bool32 {
        self.shader_integer_dot_product
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_integer_dot_product`]
    pub fn set_shader_integer_dot_product_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_integer_dot_product = value;
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
    ///Gets the value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_integer_dot_product as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_integer_dot_product`]
    pub fn shader_integer_dot_product_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_integer_dot_product as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_integer_dot_product as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_integer_dot_product`]
    pub fn set_shader_integer_dot_product(&mut self, value: bool) -> &mut Self {
        self.shader_integer_dot_product = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceShaderIntegerDotProductProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerDotProductProperties.html) - Structure containing information about integer dot product support for a physical device
///# C Specifications
///The [`PhysicalDeviceShaderIntegerDotProductProperties`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceShaderIntegerDotProductProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           integerDotProduct8BitUnsignedAccelerated;
///    VkBool32           integerDotProduct8BitSignedAccelerated;
///    VkBool32           integerDotProduct8BitMixedSignednessAccelerated;
///    VkBool32           integerDotProduct4x8BitPackedUnsignedAccelerated;
///    VkBool32           integerDotProduct4x8BitPackedSignedAccelerated;
///    VkBool32           integerDotProduct4x8BitPackedMixedSignednessAccelerated;
///    VkBool32           integerDotProduct16BitUnsignedAccelerated;
///    VkBool32           integerDotProduct16BitSignedAccelerated;
///    VkBool32           integerDotProduct16BitMixedSignednessAccelerated;
///    VkBool32           integerDotProduct32BitUnsignedAccelerated;
///    VkBool32           integerDotProduct32BitSignedAccelerated;
///    VkBool32           integerDotProduct32BitMixedSignednessAccelerated;
///    VkBool32           integerDotProduct64BitUnsignedAccelerated;
///    VkBool32           integerDotProduct64BitSignedAccelerated;
///    VkBool32           integerDotProduct64BitMixedSignednessAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating8BitUnsignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating8BitSignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated;
///    VkBool32
/// integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating16BitUnsignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating16BitSignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating32BitUnsignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating32BitSignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating64BitUnsignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating64BitSignedAccelerated;
///    VkBool32           integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated;
///} VkPhysicalDeviceShaderIntegerDotProductProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_integer_dot_product
///typedef VkPhysicalDeviceShaderIntegerDotProductProperties
/// VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`integer_dot_product_8_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_8_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_8_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit unsigned dot product operations from operands packed into 32-bit integers using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_4_x_8_bit_packed_signed_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 8-bit signed dot product operations from operands packed into 32-bit integers
///   using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit mixed signedness dot product operations from operands packed into 32-bit integers using the `OpSUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_16_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 16-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_16_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 16-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_16_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 16-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V
///   instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_32_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 32-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_32_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 32-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_32_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 32-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V
///   instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_64_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 64-bit unsigned dot product operations using the `OpUDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_64_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 64-bit signed dot product operations using the `OpSDotKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_64_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`]
///   if the support for 64-bit mixed signedness dot product operations using the `OpSUDotKHR` SPIR-V
///   instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit unsigned accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit signed accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 8-bit mixed signedness accumulating saturating dot product operations from operands packed into 32-bit integers using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 16-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 16-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 16-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 32-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 32-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 32-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`] is a boolean that will be [`TRUE`] if the support for 64-bit unsigned accumulating saturating dot product operations using the `OpUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`] is a boolean that will be [`TRUE`] if the support for 64-bit signed accumulating saturating dot product operations using the `OpSDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
/// - [`integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`] is a boolean that will be [`TRUE`] if the support for 64-bit mixed signedness accumulating saturating dot product operations using the `OpSUDotAccSatKHR` SPIR-V instruction is accelerated [as defined below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#devsandqueues-integer-dot-product-accelerated).
///If the [`PhysicalDeviceShaderIntegerDotProductProperties`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These are properties of the integer dot product
/// acceleration information of
///a physical device.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES`
///# Related
/// - [`VK_KHR_shader_integer_dot_product`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceShaderIntegerDotProductProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    integer_dot_product_8_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_8_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_8_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_16_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_32_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_64_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: Bool32,
    ///No documentation found
    integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderIntegerDotProductProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            integer_dot_product_8_bit_unsigned_accelerated: 0,
            integer_dot_product_8_bit_signed_accelerated: 0,
            integer_dot_product_8_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_unsigned_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_signed_accelerated: 0,
            integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated: 0,
            integer_dot_product_16_bit_unsigned_accelerated: 0,
            integer_dot_product_16_bit_signed_accelerated: 0,
            integer_dot_product_16_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_32_bit_unsigned_accelerated: 0,
            integer_dot_product_32_bit_signed_accelerated: 0,
            integer_dot_product_32_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_64_bit_unsigned_accelerated: 0,
            integer_dot_product_64_bit_signed_accelerated: 0,
            integer_dot_product_64_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_signed_accelerated: 0,
            integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderIntegerDotProductProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_unsigned_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_signed_accelerated
    }
    ///Gets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &self,
    ) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated
    }
    ///Gets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_raw(&self) -> Bool32 {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_8_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_8_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_8_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_16_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_16_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_16_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_32_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_32_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_32_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_64_bit_unsigned_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_64_bit_signed_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_64_bit_mixed_signedness_accelerated_raw(&mut self, value: Bool32) -> &mut Self {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_raw(
        &mut self,
        value: Bool32,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated = value;
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
    ///Gets the value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_8_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_16_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_32_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_unsigned_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_signed_accelerated as u8) }
    }
    ///Gets the value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_64_bit_mixed_signedness_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated as u8,
            )
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as u8)
        }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated(&self) -> bool {
        unsafe { std::mem::transmute(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as u8) }
    }
    ///Gets the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated(&self) -> bool {
        unsafe {
            std::mem::transmute(
                self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated as u8,
            )
        }
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
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_8_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn integer_dot_product_8_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_8_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_8_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_8_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_16_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn integer_dot_product_16_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_16_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_16_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_16_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_32_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn integer_dot_product_32_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_32_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_32_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_32_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_64_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn integer_dot_product_64_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_64_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_64_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_64_bit_mixed_signedness_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated_mut(
        &mut self,
    ) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated_mut(
        &mut self,
    ) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_signed_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                    as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated
                    as *mut Bool32)
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
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_8_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_8_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_8_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_8_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_4_x_8_bit_packed_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_16_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_16_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_16_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_16_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_32_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_32_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_32_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_32_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_64_bit_unsigned_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_64_bit_signed_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::integer_dot_product_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_64_bit_mixed_signedness_accelerated(&mut self, value: bool) -> &mut Self {
        self.integer_dot_product_64_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_8_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_4_x_8_bit_packed_mixed_signedness_accelerated =
            value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_16_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_32_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_unsigned_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_signed_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_signed_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_signed_accelerated = value as u8 as u32;
        self
    }
    ///Sets the raw value of
    /// [`Self::integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated`]
    pub fn set_integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated(
        &mut self,
        value: bool,
    ) -> &mut Self {
        self.integer_dot_product_accumulating_saturating_64_bit_mixed_signedness_accelerated = value as u8 as u32;
        self
    }
}
///[VkFormatProperties3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties3.html) - Structure specifying image format properties
///# C Specifications
///To query supported format extended features which are properties of the
///physical device, add [`FormatProperties3`] structure to the [`p_next`]
///chain of [`FormatProperties2`].The [`FormatProperties3`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkFormatProperties3 {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkFormatFeatureFlags2    linearTilingFeatures;
///    VkFormatFeatureFlags2    optimalTilingFeatures;
///    VkFormatFeatureFlags2    bufferFeatures;
///} VkFormatProperties3;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_format_feature_flags2
///typedef VkFormatProperties3 VkFormatProperties3KHR;
///```
///# Members
/// - [`linear_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features
///   supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_LINEAR`.
/// - [`optimal_tiling_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features
///   supported by images created with a `tiling` parameter of `VK_IMAGE_TILING_OPTIMAL`.
/// - [`buffer_features`] is a bitmask of [`FormatFeatureFlagBits2`] specifying features supported
///   by buffers.
///# Description
///The bits reported in [`linear_tiling_features`], [`optimal_tiling_features`]
///and [`buffer_features`]**must** include the bits reported in the
///corresponding fields of [`FormatProperties2::format_properties`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3`
///# Related
/// - [`VK_KHR_format_feature_flags2`]
/// - [`crate::vulkan1_3`]
/// - [`FormatFeatureFlags2`]
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
pub struct FormatProperties3<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`linear_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits2`] specifying features supported by images
    ///created with a `tiling` parameter of `VK_IMAGE_TILING_LINEAR`.
    linear_tiling_features: FormatFeatureFlags2,
    ///[`optimal_tiling_features`] is a bitmask of
    ///[`FormatFeatureFlagBits2`] specifying features supported by images
    ///created with a `tiling` parameter of `VK_IMAGE_TILING_OPTIMAL`.
    optimal_tiling_features: FormatFeatureFlags2,
    ///[`buffer_features`] is a bitmask of [`FormatFeatureFlagBits2`]
    ///specifying features supported by buffers.
    buffer_features: FormatFeatureFlags2,
}
impl<'lt> Default for FormatProperties3<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            linear_tiling_features: Default::default(),
            optimal_tiling_features: Default::default(),
            buffer_features: Default::default(),
        }
    }
}
impl<'lt> FormatProperties3<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::linear_tiling_features`]
    pub fn linear_tiling_features(&self) -> FormatFeatureFlags2 {
        self.linear_tiling_features
    }
    ///Gets the value of [`Self::optimal_tiling_features`]
    pub fn optimal_tiling_features(&self) -> FormatFeatureFlags2 {
        self.optimal_tiling_features
    }
    ///Gets the value of [`Self::buffer_features`]
    pub fn buffer_features(&self) -> FormatFeatureFlags2 {
        self.buffer_features
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
    ///Gets a mutable reference to the value of [`Self::linear_tiling_features`]
    pub fn linear_tiling_features_mut(&mut self) -> &mut FormatFeatureFlags2 {
        &mut self.linear_tiling_features
    }
    ///Gets a mutable reference to the value of [`Self::optimal_tiling_features`]
    pub fn optimal_tiling_features_mut(&mut self) -> &mut FormatFeatureFlags2 {
        &mut self.optimal_tiling_features
    }
    ///Gets a mutable reference to the value of [`Self::buffer_features`]
    pub fn buffer_features_mut(&mut self) -> &mut FormatFeatureFlags2 {
        &mut self.buffer_features
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
    ///Sets the raw value of [`Self::linear_tiling_features`]
    pub fn set_linear_tiling_features(&mut self, value: crate::vulkan1_3::FormatFeatureFlags2) -> &mut Self {
        self.linear_tiling_features = value;
        self
    }
    ///Sets the raw value of [`Self::optimal_tiling_features`]
    pub fn set_optimal_tiling_features(&mut self, value: crate::vulkan1_3::FormatFeatureFlags2) -> &mut Self {
        self.optimal_tiling_features = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_features`]
    pub fn set_buffer_features(&mut self, value: crate::vulkan1_3::FormatFeatureFlags2) -> &mut Self {
        self.buffer_features = value;
        self
    }
}
///[VkPipelineRenderingCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRenderingCreateInfo.html) - Structure specifying attachment formats
///# C Specifications
///The [`PipelineRenderingCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPipelineRenderingCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           viewMask;
///    uint32_t           colorAttachmentCount;
///    const VkFormat*    pColorAttachmentFormats;
///    VkFormat           depthAttachmentFormat;
///    VkFormat           stencilAttachmentFormat;
///} VkPipelineRenderingCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering
///typedef VkPipelineRenderingCreateInfo VkPipelineRenderingCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`view_mask`] is the viewMask used for rendering.
/// - [`color_attachment_count`] is the number of entries in [`color_attachment_formats`]
/// - [`color_attachment_formats`] is a pointer to an array of [`Format`] values defining the format
///   of color attachments used in this pipeline.
/// - [`depth_attachment_format`] is a [`Format`] value defining the format of the depth attachment
///   used in this pipeline.
/// - [`stencil_attachment_format`] is a [`Format`] value defining the format of the stencil
///   attachment used in this pipeline.
///# Description
///When a pipeline is created without a [`RenderPass`], if this structure
///is present in the [`p_next`] chain of [`GraphicsPipelineCreateInfo`],
///it specifies the view mask and format of attachments used for rendering.
///If this structure is not specified, and the pipeline does not include a
///[`RenderPass`], [`view_mask`] and [`color_attachment_count`] are `0`,
///and [`depth_attachment_format`] and [`stencil_attachment_format`] are
///`VK_FORMAT_UNDEFINED`.
///If a graphics pipeline is created with a valid [`RenderPass`],
///parameters of this structure are ignored.If [`depth_attachment_format`],
/// [`stencil_attachment_format`], or any
///element of [`color_attachment_formats`] is `VK_FORMAT_UNDEFINED`, it
///indicates that the corresponding attachment is unused within the render
///pass.
///Valid formats indicate that an attachment **can** be used - but it is still
///valid to set the attachment to `NULL` when beginning rendering.Valid Usage
/// -    If any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that includes either `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// - If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format that
///   includes a depth aspect
/// - If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format that
///   includes a stencil aspect
/// -    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED` and [`stencil_attachment_format`]
///   is not `VK_FORMAT_UNDEFINED`, [`depth_attachment_format`]**must** equal
///   [`stencil_attachment_format`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, [`view_mask`]**must** be `0`
/// - The index of the most significant bit in [`view_mask`]**must** be less than [`maxMultiviewViewCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMultiviewViewCount)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO`
/// - If [`color_attachment_count`] is not `0`, [`color_attachment_formats`]**must** be a valid
///   pointer to an array of [`color_attachment_count`] valid [`Format`] values
/// - [`depth_attachment_format`]**must** be a valid [`Format`] value
/// - [`stencil_attachment_format`]**must** be a valid [`Format`] value
///# Related
/// - [`VK_KHR_dynamic_rendering`]
/// - [`crate::vulkan1_3`]
/// - [`Format`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct PipelineRenderingCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`view_mask`] is the viewMask used for rendering.
    view_mask: u32,
    ///[`color_attachment_count`] is the number of entries in
    ///[`color_attachment_formats`]
    color_attachment_count: u32,
    ///[`color_attachment_formats`] is a pointer to an array of [`Format`]
    ///values defining the format of color attachments used in this pipeline.
    color_attachment_formats: *const Format,
    ///[`depth_attachment_format`] is a [`Format`] value defining the
    ///format of the depth attachment used in this pipeline.
    depth_attachment_format: Format,
    ///[`stencil_attachment_format`] is a [`Format`] value defining the
    ///format of the stencil attachment used in this pipeline.
    stencil_attachment_format: Format,
}
impl<'lt> Default for PipelineRenderingCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            view_mask: 0,
            color_attachment_count: 0,
            color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
        }
    }
}
impl<'lt> PipelineRenderingCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view_mask`]
    pub fn view_mask_raw(&self) -> u32 {
        self.view_mask
    }
    ///Gets the raw value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_raw(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the raw value of [`Self::color_attachment_formats`]
    pub fn color_attachment_formats_raw(&self) -> *const Format {
        self.color_attachment_formats
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask_raw(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count_raw(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_formats`]
    pub fn set_color_attachment_formats_raw(&mut self, value: *const Format) -> &mut Self {
        self.color_attachment_formats = value;
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
    ///Gets the value of [`Self::view_mask`]
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Gets the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the value of [`Self::color_attachment_formats`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_attachment_formats(&self) -> &[Format] {
        std::slice::from_raw_parts(self.color_attachment_formats, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::depth_attachment_format`]
    pub fn depth_attachment_format(&self) -> Format {
        self.depth_attachment_format
    }
    ///Gets the value of [`Self::stencil_attachment_format`]
    pub fn stencil_attachment_format(&self) -> Format {
        self.stencil_attachment_format
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::view_mask`]
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::depth_attachment_format`]
    pub fn depth_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.depth_attachment_format
    }
    ///Gets a mutable reference to the value of [`Self::stencil_attachment_format`]
    pub fn stencil_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.stencil_attachment_format
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
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_formats`]
    pub fn set_color_attachment_formats(&mut self, value: &'lt [crate::vulkan1_0::Format]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachment_formats = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::depth_attachment_format`]
    pub fn set_depth_attachment_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.depth_attachment_format = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_attachment_format`]
    pub fn set_stencil_attachment_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.stencil_attachment_format = value;
        self
    }
}
///[VkRenderingInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingInfo.html) - Structure specifying render pass instance begin info
///# C Specifications
///The [`RenderingInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkRenderingInfo {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkRenderingFlags                    flags;
///    VkRect2D                            renderArea;
///    uint32_t                            layerCount;
///    uint32_t                            viewMask;
///    uint32_t                            colorAttachmentCount;
///    const VkRenderingAttachmentInfo*    pColorAttachments;
///    const VkRenderingAttachmentInfo*    pDepthAttachment;
///    const VkRenderingAttachmentInfo*    pStencilAttachment;
///} VkRenderingInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering
///typedef VkRenderingInfo VkRenderingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`RenderingFlagBits`].
/// - [`render_area`] is the render area that is affected by the render pass instance.
/// - [`layer_count`] is the number of layers rendered to in each attachment when [`view_mask`] is
///   `0`.
/// - [`view_mask`] is the view mask indicating the indices of attachment layers that will be
///   rendered when it is not `0`.
/// - [`color_attachment_count`] is the number of elements in [`color_attachments`].
/// - [`color_attachments`] is a pointer to an array of
///   [`color_attachment_count`][`RenderingAttachmentInfo`] structures describing any color
///   attachments used.
/// - [`depth_attachment`] is a pointer to a [`RenderingAttachmentInfo`] structure describing a
///   depth attachment.
/// - [`stencil_attachment`] is a pointer to a [`RenderingAttachmentInfo`] structure describing a
///   stencil attachment.
///# Description
///If [`view_mask`] is not `0`, multiview is enabled.If there is an instance of
/// [`DeviceGroupRenderPassBeginInfo`] included
///in the [`p_next`] chain and its `deviceCount` member is not `0`, then
///[`render_area`] is ignored, and the render area is defined per-device by
///that structure.Each element of the [`color_attachments`] array corresponds to an output
///location in the shader, i.e. if the shader declares an output variable
///decorated with a `Location` value of **X**, then it uses the attachment
///provided in [`color_attachments`][**X**].
///If the `imageView` member of any element of [`color_attachments`] is
///[`crate::utils::Handle::null`], writes to the corresponding location by a fragment are
///discarded.Valid Usage
/// - If [`view_mask`] is `0`, [`layer_count`]**must** not be `0`
/// - If neither the [`VK_AMD_mixed_attachment_samples`] nor the [`VK_NV_framebuffer_mixed_samples`]
///   extensions are enabled, `imageView` members of [`depth_attachment`], [`stencil_attachment`],
///   and elements of [`color_attachments`] that are not [`crate::utils::Handle::null`]**must** have
///   been created with the same `sampleCount`
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.x`**must** be greater than or
///   equal to 0
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0, `renderArea.offset.y`**must** be greater than or
///   equal to 0
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0, the width of the `imageView` member of any
///   element of [`color_attachments`], [`depth_attachment`], or [`stencil_attachment`] that is not
///   [`crate::utils::Handle::null`]**must** be greater than or equal to `renderArea.offset.x` +
///   `renderArea.extent.width`
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0, the height of the `imageView` member of any
///   element of [`color_attachments`], [`depth_attachment`], or [`stencil_attachment`] that is not
///   [`crate::utils::Handle::null`]**must** be greater than or equal to `renderArea.offset.y` +
///   `renderArea.extent.height`
/// - If the [`p_next`] chain contains [`DeviceGroupRenderPassBeginInfo`], the width of the
///   `imageView` member of any element of [`color_attachments`], [`depth_attachment`], or
///   [`stencil_attachment`] that is not [`crate::utils::Handle::null`]**must** be greater than or
///   equal to the sum of the `offset.x` and `extent.width` members of each element of
///   `pDeviceRenderAreas`
/// - If the [`p_next`] chain contains [`DeviceGroupRenderPassBeginInfo`], the height of the
///   `imageView` member of any element of [`color_attachments`], [`depth_attachment`], or
///   [`stencil_attachment`] that is not [`crate::utils::Handle::null`]**must** be greater than or
///   equal to the sum of the `offset.y` and `extent.height` members of each element of
///   `pDeviceRenderAreas`
/// - If neither [`depth_attachment`] or [`stencil_attachment`] are `NULL` and the `imageView`
///   member of either structure is not [`crate::utils::Handle::null`], the `imageView` member of
///   each structure **must** be the same
/// - If neither [`depth_attachment`] or [`stencil_attachment`] are `NULL`, and the `resolveMode`
///   member of each is not `VK_RESOLVE_MODE_NONE`, the `resolveImageView` member of each structure
///   **must** be the same
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], that `imageView`**must** have
///   been created with `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`
/// - If [`depth_attachment`] is not `NULL` and `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pDepthAttachment->imageView`**must** have been created with a
///   format that includes a depth aspect
/// - If [`depth_attachment`] is not `NULL` and `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pDepthAttachment->imageView`**must** have been created with
///   `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`stencil_attachment`] is not `NULL` and `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pStencilAttachment->imageView`**must** have been created with
///   a format that includes a stencil aspect
/// - If [`stencil_attachment`] is not `NULL` and `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pStencilAttachment->imageView`**must** have been created with
///   a stencil usage including `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], the `layout` member of that
///   element of [`color_attachments`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], if the `resolveMode` member of
///   that element of [`color_attachments`] is not `VK_RESOLVE_MODE_NONE`, its `resolveImageLayout`
///   member **must** not be `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`depth_attachment`] is not `NULL` and `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pDepthAttachment->layout`**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`depth_attachment`] is not `NULL`, `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], and `pDepthAttachment->resolveMode` is not
///   `VK_RESOLVE_MODE_NONE`, `pDepthAttachment->resolveImageLayout`**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`stencil_attachment`] is not `NULL` and `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pStencilAttachment->layout`**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`stencil_attachment`] is not `NULL`, `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], and `pStencilAttachment->resolveMode` is not
///   `VK_RESOLVE_MODE_NONE`, `pStencilAttachment->resolveImageLayout`**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], the `layout` member of that
///   element of [`color_attachments`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], if the `resolveMode` member of
///   that element of [`color_attachments`] is not `VK_RESOLVE_MODE_NONE`, its `resolveImageLayout`
///   member **must** not be `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`depth_attachment`] is not `NULL`, `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], and `pDepthAttachment->resolveMode` is not
///   `VK_RESOLVE_MODE_NONE`, `pDepthAttachment->resolveImageLayout`**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - If [`stencil_attachment`] is not `NULL`, `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], and `pStencilAttachment->resolveMode` is not
///   `VK_RESOLVE_MODE_NONE`, `pStencilAttachment->resolveImageLayout`**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], the `layout` member of that
///   element of [`color_attachments`]**must** not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`color_attachment_count`] is not `0` and the `imageView` member of an element of
///   [`color_attachments`] is not [`crate::utils::Handle::null`], if the `resolveMode` member of
///   that element of [`color_attachments`] is not `VK_RESOLVE_MODE_NONE`, its `resolveImageLayout`
///   member **must** not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`depth_attachment`] is not `NULL` and `pDepthAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pDepthAttachment->resolveMode`**must** be one of the bits set
///   in [`PhysicalDeviceDepthStencilResolveProperties::supported_depth_resolve_modes`]
/// - If [`stencil_attachment`] is not `NULL` and `pStencilAttachment->imageView` is not
///   [`crate::utils::Handle::null`], `pStencilAttachment->resolveMode`**must** be one of the bits
///   set in [`PhysicalDeviceDepthStencilResolveProperties::supported_stencil_resolve_modes`]
/// - If [`depth_attachment`] or [`stencil_attachment`] are both not `NULL`,
///   `pDepthAttachment->imageView` and `pStencilAttachment->imageView` are both not
///   [`crate::utils::Handle::null`], and
///   [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is [`FALSE`], the
///   `resolveMode` of both structures **must** be the same value
/// - If [`depth_attachment`] or [`stencil_attachment`] are both not `NULL`,
///   `pDepthAttachment->imageView` and `pStencilAttachment->imageView` are both not
///   [`crate::utils::Handle::null`],
///   [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`] is [`FALSE`], and the
///   `resolveMode` of neither structure is `VK_RESOLVE_MODE_NONE`, the `resolveMode` of both
///   structures **must** be the same value
/// - [`color_attachment_count`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_color_attachments`]
/// -    If the `imageView` member of a [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure included in the [`p_next`] chain is not [`crate::utils::Handle::null`], and [non-subsample image feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentDensityMapNonSubsampledImages) is not enabled, valid `imageView` and `resolveImageView` members of [`depth_attachment`], [`stencil_attachment`], and each element of [`color_attachments`]**must** be a [`ImageView`] created with `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
/// - If the `imageView` member of a [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], and [`view_mask`] is
///   not `0`, `imageView`**must** have a [`layer_count`] greater than or equal to the index of the
///   most significant bit in [`view_mask`]
/// - If the `imageView` member of a [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], and [`view_mask`] is
///   `0`, `imageView`**must** have a [`layer_count`] equal to `1`
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0 and the `imageView` member of a
///   [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure included in the [`p_next`] chain is
///   not [`crate::utils::Handle::null`], `imageView`**must** have a width greater than or equal to
///   <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span
///   style="height:1.80002em;vertical-align:-0.65002em;" class="strut"></span><span
///   class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.9019679999999999em;"><span
///   style="top:-2.6550000000000002em;"><span class="pstrut" style="height:3em;"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">a</span><span class="mord
///   mathdefault mtight">x</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">F</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">a</span><span
///   class="mord mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight">n</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">s</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">t</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">y</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">T</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight" style="margin-right:0.01968em;">l</span><span class="mord
///   mathdefault mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span class="mord mathdefault mtight"
///   style="margin-right:0.04398em;">z</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">e</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.3448em;"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span><span
///   style="top:-3.23em;"><span class="pstrut" style="height:3em;"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.41586em;"><span
///   class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">n</span><span class="mord mathdefault mtight">d</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">A</span><span
///   class="mord mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mtight"><span class="mord mathdefault
///   mtight">a</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.16454285714285719em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault
///   mtight">x</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span style="height:0.143em;"
///   class="vlist"><span></span></span></span></span></span></span><span class="mbin
///   mtight">+</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">n</span><span class="mord mathdefault mtight">d</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">A</span><span
///   class="mord mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mtight"><span class="mord mathdefault
///   mtight">a</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span></
///   span><span class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.481108em;" class="vlist"><span></span></span></span></span></span><span
///   class="mclose nulldelimiter"></span></span></span><span style="top:0em;" class="mclose
///   delimcenter"><span class="delimsizing size2">⌉</span></span></span></span></span></span>
/// - If the [`p_next`] chain contains a [`DeviceGroupRenderPassBeginInfo`] structure, its
///   `deviceRenderAreaCount` member is not 0, and the `imageView` member of a
///   [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure included in the [`p_next`] chain is
///   not [`crate::utils::Handle::null`], `imageView`**must** have a width greater than or equal to
///   <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span
///   class="strut" style="height:1.80002em;vertical-align:-0.65002em;"></span><span
///   class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.9322159999999999em;"><span
///   style="top:-2.6550000000000002em;"><span class="pstrut" style="height:3em;"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">a</span><span class="mord
///   mathdefault mtight">x</span><span style="margin-right:0.13889em;" class="mord mathdefault
///   mtight">F</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">a</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight">n</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">s</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">t</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">y</span><span style="margin-right:0.13889em;" class="mord mathdefault
///   mtight">T</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">x</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.01968em;">l</span><span class="mord mathdefault mtight"
///   style="margin-right:0.05764em;">S</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight" style="margin-right:0.04398em;">z</span><span class="mord
///   mtight"><span class="mord mathdefault mtight">e</span><span class="msupsub"><span
///   class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.3448em;"
///   class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span><span
///   style="top:-3.23em;"><span class="pstrut" style="height:3em;"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.446108em;"><span
///   class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span class="mord mathdefault mtight">p</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">D</span><span class="mord
///   mathdefault mtight">e</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">v</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">c</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.00773em;">R</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">d</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.16454285714285719em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span class="mord mathdefault mtight">x</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.143em;"
///   class="vlist"><span></span></span></span></span></span></span><span class="mbin
///   mtight">+</span><span class="mord mathdefault mtight">p</span><span class="mord mathdefault
///   mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">v</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">c</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.00773em;">R</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">d</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.3448em;"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span></
///   span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.481108em;"><span></span></span></span></span></span><span class="mclose
///   nulldelimiter"></span></span></span><span style="top:0em;" class="mclose delimcenter"><span
///   class="delimsizing size2">⌉</span></span></span></span></span></span> for each element of
///   `pDeviceRenderAreas`
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0 and the `imageView` member of a
///   [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure included in the [`p_next`] chain is
///   not [`crate::utils::Handle::null`], `imageView`**must** have a height greater than or equal to
///   <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span
///   style="height:1.80002em;vertical-align:-0.65002em;" class="strut"></span><span
///   class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.999188em;"><span
///   style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">a</span><span class="mord
///   mathdefault mtight">x</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">F</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">a</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight">n</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">s</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">t</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">y</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">T</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.01968em;" class="mord mathdefault mtight">l</span><span
///   style="margin-right:0.05764em;" class="mord mathdefault mtight">S</span><span class="mord
///   mathdefault mtight">i</span><span style="margin-right:0.04398em;" class="mord mathdefault
///   mtight">z</span><span class="mord mtight"><span class="mord mathdefault mtight">e</span><span
///   class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
///   style="height:0.3448em;"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.29011428571428566em;"><span></span></span></span></span></span></span></
///   span></span></span><span style="top:-3.23em;"><span style="height:3em;"
///   class="pstrut"></span><span style="border-bottom-width:0.04em;"
///   class="frac-line"></span></span><span style="top:-3.51308em;"><span class="pstrut"
///   style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord
///   mtight"><span style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span
///   class="mord mathdefault mtight">d</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span class="mord
///   mathdefault mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mtight"><span class="mord mathdefault mtight">a</span><span class="msupsub"><span
///   class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
///   style="height:0.16454285714285716em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">y</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.2818857142857143em;"><span></span></span></span></span></span></span><span
///   class="mbin mtight">+</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">n</span><span class="mord mathdefault mtight">d</span><span class="mord mathdefault
///   mtight">e</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">A</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mtight"><span class="mord mathdefault
///   mtight">a</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.29011428571428566em;"><span></span></span></span></span></span></span></
///   span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.5480799999999999em;"
///   class="vlist"><span></span></span></span></span></span><span class="mclose
///   nulldelimiter"></span></span></span><span class="mclose delimcenter" style="top:0em;"><span
///   class="delimsizing size2">⌉</span></span></span></span></span></span>
/// - If the [`p_next`] chain contains a [`DeviceGroupRenderPassBeginInfo`] structure, its
///   `deviceRenderAreaCount` member is not 0, and the `imageView` member of a
///   [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure included in the [`p_next`] chain is
///   not [`crate::utils::Handle::null`], `imageView`**must** have a height greater than or equal to
///   <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span
///   style="height:1.80002em;vertical-align:-0.65002em;" class="strut"></span><span
///   class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.999188em;" class="vlist"><span
///   style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">a</span><span class="mord
///   mathdefault mtight">x</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">F</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">a</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">m</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight">n</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">s</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">t</span><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">y</span><span style="margin-right:0.13889em;" class="mord
///   mathdefault mtight">T</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight">x</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight" style="margin-right:0.01968em;">l</span><span class="mord mathdefault
///   mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span class="mord mathdefault mtight"
///   style="margin-right:0.04398em;">z</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">e</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.3448em;"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.29011428571428566em;"><span></span></span></span></span></span></span></
///   span></span></span><span style="top:-3.23em;"><span style="height:3em;"
///   class="pstrut"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.51308em;"><span
///   style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span class="mord mathdefault mtight">p</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">D</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">v</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight">c</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight" style="margin-right:0.00773em;">R</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.16454285714285716em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">y</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.2818857142857143em;"><span></span></span></span></span></span></span><span
///   class="mbin mtight">+</span><span class="mord mathdefault mtight">p</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">v</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight">c</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.00773em;" class="mord mathdefault mtight">R</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span class="mord
///   mathdefault mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span style="height:0.29011428571428566em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span></
///   span><span class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.5480799999999999em;"
///   class="vlist"><span></span></span></span></span></span><span class="mclose
///   nulldelimiter"></span></span></span><span class="mclose delimcenter" style="top:0em;"><span
///   class="delimsizing size2">⌉</span></span></span></span></span></span> for each element of
///   `pDeviceRenderAreas`
/// - If the `imageView` member of a [`RenderingFragmentDensityMapAttachmentInfoEXT`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], it **must** not be
///   equal to the `imageView` or `resolveImageView` member of [`depth_attachment`],
///   [`stencil_attachment`], or any element of [`color_attachments`]
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0 and the `imageView` member of a
///   [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure included in the [`p_next`] chain
///   is not [`crate::utils::Handle::null`], `imageView`**must** have a width greater than or equal
///   to <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span
///   class="strut" style="height:1.80002em;vertical-align:-0.65002em;"></span><span
///   class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.9019679999999999em;" class="vlist"><span
///   style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="mord mathdefault mtight">h</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">d</span><span class="mord
///   mathdefault mtight">i</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord mathdefault
///   mtight" style="margin-right:0.00773em;">R</span><span class="mord mathdefault
///   mtight">a</span><span class="mord mathdefault mtight">t</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">A</span><span class="mord mathdefault
///   mtight">t</span><span class="mord mathdefault mtight">t</span><span class="mord mathdefault
///   mtight">a</span><span class="mord mathdefault mtight">c</span><span class="mord mathdefault
///   mtight">h</span><span class="mord mathdefault mtight">m</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">t</span><span style="margin-right:0.13889em;" class="mord mathdefault
///   mtight">T</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.01968em;" class="mord mathdefault mtight">l</span><span class="mord
///   mathdefault mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span class="mord mathdefault mtight"
///   style="margin-right:0.04398em;">z</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">e</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight"
///   style="margin-right:0.02691em;">w</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span
///   class="mord mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span><span
///   style="top:-3.23em;"><span class="pstrut" style="height:3em;"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.41586em;"><span
///   class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">n</span><span class="mord mathdefault mtight">d</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">A</span><span
///   class="mord mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mtight"><span class="mord mathdefault
///   mtight">a</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.16454285714285719em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span class="mord mathdefault mtight">x</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.143em;"><span></span></span></span></span></span></span><span class="mbin
///   mtight">+</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">n</span><span class="mord mathdefault mtight">d</span><span class="mord mathdefault
///   mtight">e</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">A</span><span class="mord mathdefault
///   mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mtight"><span class="mord mathdefault mtight">a</span><span
///   class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
///   style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.15122857142857138em;"><span></span></span></span></span></span></span></
///   span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span
///   class="vlist" style="height:0.481108em;"><span></span></span></span></span></span><span
///   class="mclose nulldelimiter"></span></span></span><span class="mclose delimcenter"
///   style="top:0em;"><span class="delimsizing size2">⌉</span></span></span></span></span></span>
/// - If the [`p_next`] chain contains a [`DeviceGroupRenderPassBeginInfo`] structure, its
///   `deviceRenderAreaCount` member is not 0, and the `imageView` member of a
///   [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure included in the [`p_next`] chain
///   is not [`crate::utils::Handle::null`], `imageView`**must** have a width greater than or equal
///   to <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span
///   style="height:1.80002em;vertical-align:-0.65002em;" class="strut"></span><span
///   class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span class="vlist" style="height:0.9322159999999999em;"><span
///   style="top:-2.6550000000000002em;"><span class="pstrut" style="height:3em;"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="mord mathdefault mtight">h</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">d</span><span class="mord
///   mathdefault mtight">i</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight" style="margin-right:0.03588em;">g</span><span
///   style="margin-right:0.00773em;" class="mord mathdefault mtight">R</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">A</span><span class="mord
///   mathdefault mtight">t</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">c</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault mtight">m</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">t</span><span style="margin-right:0.13889em;" class="mord mathdefault
///   mtight">T</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.01968em;" class="mord mathdefault mtight">l</span><span class="mord
///   mathdefault mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span style="margin-right:0.04398em;" class="mord mathdefault
///   mtight">z</span><span class="mord mtight"><span class="mord mathdefault mtight">e</span><span
///   class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
///   style="height:0.3448em;"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight"
///   style="margin-right:0.02691em;">w</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span
///   class="mord mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span
///   style="height:0.15122857142857138em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span><span
///   style="top:-3.23em;"><span class="pstrut" style="height:3em;"></span><span
///   style="border-bottom-width:0.04em;" class="frac-line"></span></span><span
///   style="top:-3.446108em;"><span class="pstrut" style="height:3em;"></span><span class="sizing
///   reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mathdefault
///   mtight">p</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">D</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">v</span><span class="mord
///   mathdefault mtight">i</span><span class="mord mathdefault mtight">c</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.00773em;">R</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">n</span><span class="mord mathdefault mtight">d</span><span
///   class="mord mathdefault mtight">e</span><span style="margin-right:0.02778em;" class="mord
///   mathdefault mtight">r</span><span class="mord mathdefault mtight">A</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">a</span><span class="mord
///   mtight"><span class="mord mathdefault mtight">s</span><span class="msupsub"><span
///   class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.16454285714285719em;"
///   class="vlist"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span class="mord mathdefault mtight">x</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.143em;"><span></span></span></span></span></span></span><span class="mbin
///   mtight">+</span><span class="mord mathdefault mtight">p</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">D</span><span class="mord
///   mathdefault mtight">e</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">v</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">c</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.00773em;" class="mord mathdefault mtight">R</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   style="height:2.5em;" class="pstrut"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span style="margin-right:0.02691em;" class="mord
///   mathdefault mtight">w</span><span class="mord mathdefault mtight">i</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">h</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.15122857142857138em;"><span></span></span></span></span></span></span></
///   span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span
///   class="vlist" style="height:0.481108em;"><span></span></span></span></span></span><span
///   class="mclose nulldelimiter"></span></span></span><span class="mclose delimcenter"
///   style="top:0em;"><span class="delimsizing size2">⌉</span></span></span></span></span></span>
///   for each element of `pDeviceRenderAreas`
/// - If the [`p_next`] chain does not contain [`DeviceGroupRenderPassBeginInfo`] or its
///   `deviceRenderAreaCount` member is equal to 0 and the `imageView` member of a
///   [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure included in the [`p_next`] chain
///   is not [`crate::utils::Handle::null`], `imageView`**must** have a height greater than or equal
///   to <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span
///   class="strut" style="height:1.80002em;vertical-align:-0.65002em;"></span><span
///   class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.999188em;" class="vlist"><span
///   style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="mord mathdefault mtight">h</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">d</span><span class="mord
///   mathdefault mtight">i</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight" style="margin-right:0.03588em;">g</span><span
///   style="margin-right:0.00773em;" class="mord mathdefault mtight">R</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">A</span><span class="mord
///   mathdefault mtight">t</span><span class="mord mathdefault mtight">t</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">c</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault mtight">m</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">t</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">T</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight" style="margin-right:0.01968em;">l</span><span class="mord
///   mathdefault mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span class="mord mathdefault mtight"
///   style="margin-right:0.04398em;">z</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">e</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.29011428571428566em;"><span></span></span></span></span></span></span></
///   span></span></span><span style="top:-3.23em;"><span class="pstrut"
///   style="height:3em;"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.51308em;"><span
///   style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">n</span><span class="mord mathdefault mtight">d</span><span class="mord mathdefault
///   mtight">e</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">A</span><span class="mord mathdefault
///   mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mtight"><span class="mord mathdefault mtight">a</span><span
///   class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist"
///   style="height:0.16454285714285716em;"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">y</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.2818857142857143em;"><span></span></span></span></span></span></span><span
///   class="mbin mtight">+</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">n</span><span class="mord mathdefault mtight">d</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">A</span><span
///   class="mord mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mtight"><span class="mord mathdefault
///   mtight">a</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span style="height:0.29011428571428566em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span></
///   span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.5480799999999999em;"><span></span></span></span></span></span><span
///   class="mclose nulldelimiter"></span></span></span><span class="mclose delimcenter"
///   style="top:0em;"><span class="delimsizing size2">⌉</span></span></span></span></span></span>
/// - If the [`p_next`] chain contains a [`DeviceGroupRenderPassBeginInfo`] structure, its
///   `deviceRenderAreaCount` member is not 0, and the `imageView` member of a
///   [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure included in the [`p_next`] chain
///   is not [`crate::utils::Handle::null`], `imageView`**must** have a height greater than or equal
///   to <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span
///   class="strut" style="height:1.80002em;vertical-align:-0.65002em;"></span><span
///   class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing
///   size2">⌈</span></span><span class="mord"><span class="mord"><span class="mopen
///   nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.999188em;" class="vlist"><span
///   style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span
///   class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="mord mathdefault mtight">h</span><span class="mord
///   mathdefault mtight">a</span><span class="mord mathdefault mtight">d</span><span class="mord
///   mathdefault mtight">i</span><span class="mord mathdefault mtight">n</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight" style="margin-right:0.00773em;">R</span><span class="mord mathdefault
///   mtight">a</span><span class="mord mathdefault mtight">t</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">A</span><span class="mord mathdefault
///   mtight">t</span><span class="mord mathdefault mtight">t</span><span class="mord mathdefault
///   mtight">a</span><span class="mord mathdefault mtight">c</span><span class="mord mathdefault
///   mtight">h</span><span class="mord mathdefault mtight">m</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">t</span><span class="mord mathdefault mtight"
///   style="margin-right:0.13889em;">T</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">x</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight" style="margin-right:0.01968em;">l</span><span class="mord
///   mathdefault mtight" style="margin-right:0.05764em;">S</span><span class="mord mathdefault
///   mtight">i</span><span class="mord mathdefault mtight"
///   style="margin-right:0.04398em;">z</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">e</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   style="margin-right:0.03588em;" class="mord mathdefault mtight">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span class="vlist"
///   style="height:0.29011428571428566em;"><span></span></span></span></span></span></span></
///   span></span></span><span style="top:-3.23em;"><span class="pstrut"
///   style="height:3em;"></span><span class="frac-line"
///   style="border-bottom-width:0.04em;"></span></span><span style="top:-3.51308em;"><span
///   class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span
///   class="mord mtight"><span class="mord mathdefault mtight">p</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">D</span><span class="mord mathdefault
///   mtight">e</span><span style="margin-right:0.03588em;" class="mord mathdefault
///   mtight">v</span><span class="mord mathdefault mtight">i</span><span class="mord mathdefault
///   mtight">c</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight" style="margin-right:0.00773em;">R</span><span class="mord mathdefault
///   mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord mathdefault
///   mtight">d</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">r</span><span class="mord
///   mathdefault mtight">A</span><span class="mord mathdefault mtight"
///   style="margin-right:0.02778em;">r</span><span class="mord mathdefault mtight">e</span><span
///   class="mord mathdefault mtight">a</span><span class="mord mtight"><span class="mord
///   mathdefault mtight">s</span><span class="msupsub"><span class="vlist-t vlist-t2"><span
///   class="vlist-r"><span style="height:0.16454285714285716em;" class="vlist"><span
///   style="top:-2.357em;margin-left:0em;margin-right:0.07142857142857144em;"><span class="pstrut"
///   style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord
///   mtight"><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">y</span></span></span></span></span><span
///   class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.2818857142857143em;"
///   class="vlist"><span></span></span></span></span></span></span><span class="mbin
///   mtight">+</span><span class="mord mathdefault mtight">p</span><span
///   style="margin-right:0.02778em;" class="mord mathdefault mtight">D</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight"
///   style="margin-right:0.03588em;">v</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight">c</span><span class="mord mathdefault mtight">e</span><span
///   style="margin-right:0.00773em;" class="mord mathdefault mtight">R</span><span class="mord
///   mathdefault mtight">e</span><span class="mord mathdefault mtight">n</span><span class="mord
///   mathdefault mtight">d</span><span class="mord mathdefault mtight">e</span><span class="mord
///   mathdefault mtight" style="margin-right:0.02778em;">r</span><span class="mord mathdefault
///   mtight">A</span><span style="margin-right:0.02778em;" class="mord mathdefault
///   mtight">r</span><span class="mord mathdefault mtight">e</span><span class="mord mathdefault
///   mtight">a</span><span class="mord mtight"><span class="mord mathdefault mtight">s</span><span
///   class="msupsub"><span class="vlist-t vlist-t2"><span class="vlist-r"><span
///   style="height:0.3448em;" class="vlist"><span
///   style="top:-2.3487714285714287em;margin-left:0em;margin-right:0.07142857142857144em;"><span
///   class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1
///   mtight"><span class="mord mtight"><span class="mord mathdefault mtight">h</span><span
///   class="mord mathdefault mtight">e</span><span class="mord mathdefault mtight">i</span><span
///   class="mord mathdefault mtight" style="margin-right:0.03588em;">g</span><span class="mord
///   mathdefault mtight">h</span><span class="mord mathdefault
///   mtight">t</span></span></span></span></span><span class="vlist-s">​</span></span><span
///   class="vlist-r"><span style="height:0.29011428571428566em;"
///   class="vlist"><span></span></span></span></span></span></span></span></span></span></
///   span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist"
///   style="height:0.5480799999999999em;"><span></span></span></span></span></span><span
///   class="mclose nulldelimiter"></span></span></span><span class="mclose delimcenter"
///   style="top:0em;"><span class="delimsizing size2">⌉</span></span></span></span></span></span>
///   for each element of `pDeviceRenderAreas`
/// - If the `imageView` member of a [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], and [`view_mask`] is
///   `0`, `imageView`**must** have a [`layer_count`] that is either equal to `1` or greater than or
///   equal to [`layer_count`]
/// - If the `imageView` member of a [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], and [`view_mask`] is
///   not `0`, `imageView`**must** have a [`layer_count`] that either equal to `1` or greater than
///   or equal to the index of the most significant bit in [`view_mask`]
/// - If the `imageView` member of a [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], it **must** not be
///   equal to the `imageView` or `resolveImageView` member of [`depth_attachment`],
///   [`stencil_attachment`], or any element of [`color_attachments`]
/// - If the `imageView` member of a [`RenderingFragmentShadingRateAttachmentInfoKHR`] structure
///   included in the [`p_next`] chain is not [`crate::utils::Handle::null`], it **must** not be
///   equal to the `imageView` member of a [`RenderingFragmentDensityMapAttachmentInfoEXT`]
///   structure included in the [`p_next`] chain
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, [`view_mask`]**must** be `0`
/// - The index of the most significant bit in [`view_mask`]**must** be less than [`maxMultiviewViewCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMultiviewViewCount)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDERING_INFO`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`DeviceGroupRenderPassBeginInfo`],
///   [`MultiviewPerViewAttributesInfoNVX`], [`RenderingFragmentDensityMapAttachmentInfoEXT`], or
///   [`RenderingFragmentShadingRateAttachmentInfoKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`RenderingFlagBits`] values
/// - If [`color_attachment_count`] is not `0`, [`color_attachments`]**must** be a valid pointer to
///   an array of [`color_attachment_count`] valid [`RenderingAttachmentInfo`] structures
/// - If [`depth_attachment`] is not `NULL`, [`depth_attachment`]**must** be a valid pointer to a
///   valid [`RenderingAttachmentInfo`] structure
/// - If [`stencil_attachment`] is not `NULL`, [`stencil_attachment`]**must** be a valid pointer to
///   a valid [`RenderingAttachmentInfo`] structure
///# Related
/// - [`VK_KHR_dynamic_rendering`]
/// - [`crate::vulkan1_3`]
/// - [`Rect2D`]
/// - [`RenderingAttachmentInfo`]
/// - [`RenderingFlags`]
/// - [`StructureType`]
/// - [`CmdBeginRendering`]
/// - [`CmdBeginRenderingKHR`]
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
#[repr(C)]
pub struct RenderingInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`RenderingFlagBits`].
    flags: RenderingFlags,
    ///[`render_area`] is the render area that is affected by the render pass
    ///instance.
    render_area: Rect2D,
    ///[`layer_count`] is the number of layers rendered to in each attachment
    ///when [`view_mask`] is `0`.
    layer_count: u32,
    ///[`view_mask`] is the view mask indicating the indices of attachment
    ///layers that will be rendered when it is not `0`.
    view_mask: u32,
    ///[`color_attachment_count`] is the number of elements in
    ///[`color_attachments`].
    color_attachment_count: u32,
    ///[`color_attachments`] is a pointer to an array of
    ///[`color_attachment_count`][`RenderingAttachmentInfo`] structures
    ///describing any color attachments used.
    color_attachments: *const RenderingAttachmentInfo<'lt>,
    ///[`depth_attachment`] is a pointer to a [`RenderingAttachmentInfo`]
    ///structure describing a depth attachment.
    depth_attachment: *const RenderingAttachmentInfo<'lt>,
    ///[`stencil_attachment`] is a pointer to a
    ///[`RenderingAttachmentInfo`] structure describing a stencil
    ///attachment.
    stencil_attachment: *const RenderingAttachmentInfo<'lt>,
}
impl<'lt> Default for RenderingInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            render_area: Default::default(),
            layer_count: 0,
            view_mask: 0,
            color_attachment_count: 0,
            color_attachments: std::ptr::null(),
            depth_attachment: std::ptr::null(),
            stencil_attachment: std::ptr::null(),
        }
    }
}
impl<'lt> RenderingInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::layer_count`]
    pub fn layer_count_raw(&self) -> u32 {
        self.layer_count
    }
    ///Gets the raw value of [`Self::view_mask`]
    pub fn view_mask_raw(&self) -> u32 {
        self.view_mask
    }
    ///Gets the raw value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_raw(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the raw value of [`Self::color_attachments`]
    pub fn color_attachments_raw(&self) -> *const RenderingAttachmentInfo<'lt> {
        self.color_attachments
    }
    ///Gets the raw value of [`Self::depth_attachment`]
    pub fn depth_attachment_raw(&self) -> *const RenderingAttachmentInfo<'lt> {
        self.depth_attachment
    }
    ///Gets the raw value of [`Self::stencil_attachment`]
    pub fn stencil_attachment_raw(&self) -> *const RenderingAttachmentInfo<'lt> {
        self.stencil_attachment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::layer_count`]
    pub fn set_layer_count_raw(&mut self, value: u32) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask_raw(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count_raw(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachments`]
    pub fn set_color_attachments_raw(&mut self, value: *const RenderingAttachmentInfo<'lt>) -> &mut Self {
        self.color_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::depth_attachment`]
    pub fn set_depth_attachment_raw(&mut self, value: *const RenderingAttachmentInfo<'lt>) -> &mut Self {
        self.depth_attachment = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_attachment`]
    pub fn set_stencil_attachment_raw(&mut self, value: *const RenderingAttachmentInfo<'lt>) -> &mut Self {
        self.stencil_attachment = value;
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
    pub fn flags(&self) -> RenderingFlags {
        self.flags
    }
    ///Gets the value of [`Self::render_area`]
    pub fn render_area(&self) -> Rect2D {
        self.render_area
    }
    ///Gets the value of [`Self::layer_count`]
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }
    ///Gets the value of [`Self::view_mask`]
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Gets the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the value of [`Self::color_attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_attachments(&self) -> &[RenderingAttachmentInfo<'lt>] {
        std::slice::from_raw_parts(self.color_attachments, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::depth_attachment`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn depth_attachment(&self) -> &RenderingAttachmentInfo<'lt> {
        &*self.depth_attachment
    }
    ///Gets the value of [`Self::stencil_attachment`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn stencil_attachment(&self) -> &RenderingAttachmentInfo<'lt> {
        &*self.stencil_attachment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut RenderingFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::render_area`]
    pub fn render_area_mut(&mut self) -> &mut Rect2D {
        &mut self.render_area
    }
    ///Gets a mutable reference to the value of [`Self::layer_count`]
    pub fn layer_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::view_mask`]
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    pub fn set_flags(&mut self, value: crate::vulkan1_3::RenderingFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::render_area`]
    pub fn set_render_area(&mut self, value: crate::vulkan1_0::Rect2D) -> &mut Self {
        self.render_area = value;
        self
    }
    ///Sets the raw value of [`Self::layer_count`]
    pub fn set_layer_count(&mut self, value: u32) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachments`]
    pub fn set_color_attachments(&mut self, value: &'lt [crate::vulkan1_3::RenderingAttachmentInfo<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachments = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::depth_attachment`]
    pub fn set_depth_attachment(&mut self, value: &'lt crate::vulkan1_3::RenderingAttachmentInfo<'lt>) -> &mut Self {
        self.depth_attachment = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::stencil_attachment`]
    pub fn set_stencil_attachment(&mut self, value: &'lt crate::vulkan1_3::RenderingAttachmentInfo<'lt>) -> &mut Self {
        self.stencil_attachment = value as *const _;
        self
    }
}
///[VkRenderingAttachmentInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingAttachmentInfo.html) - Structure specifying attachment information
///# C Specifications
///The [`RenderingAttachmentInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkRenderingAttachmentInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkImageView              imageView;
///    VkImageLayout            imageLayout;
///    VkResolveModeFlagBits    resolveMode;
///    VkImageView              resolveImageView;
///    VkImageLayout            resolveImageLayout;
///    VkAttachmentLoadOp       loadOp;
///    VkAttachmentStoreOp      storeOp;
///    VkClearValue             clearValue;
///} VkRenderingAttachmentInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering
///typedef VkRenderingAttachmentInfo VkRenderingAttachmentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_view`] is the image view that will be used for rendering.
/// - [`image_layout`] is the layout that [`image_view`] will be in during rendering.
/// - [`resolve_mode`] is a [`ResolveModeFlagBits`] value defining how multisampled data written to
///   [`image_view`] will be resolved.
/// - [`resolve_image_view`] is an image view used to write resolved multisample data at the end of
///   rendering.
/// - [`resolve_image_layout`] is the layout that [`resolve_image_view`] will be in during
///   rendering.
/// - [`load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of [`image_view`] are
///   treated at the start of the render pass instance.
/// - [`store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of [`image_view`]
///   are treated at the end of the render pass instance.
/// - [`clear_value`] is a [`ClearValue`] structure defining values used to clear [`image_view`]
///   when [`load_op`] is `VK_ATTACHMENT_LOAD_OP_CLEAR`.
///# Description
///Values in [`image_view`] are loaded and stored according to the values of
///[`load_op`] and [`store_op`], within the render area
///for each device
///specified in [`RenderingInfo`].
///If [`image_view`] is [`crate::utils::Handle::null`], other members of this structure
///are ignored; writes to this attachment will be discarded, and no load,
///store, or resolve operations will be performed.If [`resolve_mode`] is `VK_RESOLVE_MODE_NONE`,
/// then
///[`resolve_image_view`] is ignored.
///If [`resolve_mode`] is not `VK_RESOLVE_MODE_NONE`, values in
///[`resolve_image_view`] within the render area become undefined once
///rendering begins.
///At the end of rendering, the color values written to each pixel location in
///[`image_view`] will be resolved according to [`resolve_mode`] and stored
///into the the same location in [`resolve_image_view`].Store and resolve operations are only
/// performed at the end of a render pass
///instance that does not specify the `VK_RENDERING_SUSPENDING_BIT_KHR`
///flag.Load operations are only performed at the beginning of a render pass
///instance that does not specify the `VK_RENDERING_RESUMING_BIT_KHR` flag.Image contents at the
/// end of a suspended render pass instance remain defined
///for access by a resuming render pass instance.Valid Usage
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and has a non-integer color format,
///   [`resolve_mode`]**must** be `VK_RESOLVE_MODE_NONE` or `VK_RESOLVE_MODE_AVERAGE_BIT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and has an integer color format,
///   [`resolve_mode`]**must** be `VK_RESOLVE_MODE_NONE` or `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`image_view`]**must** not have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_view`]**must** have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`image_view`] and [`resolve_image_view`]**must** have the same
///   [`Format`]
/// - If [`image_view`] is not [`crate::utils::Handle::null`], `layout`**must** not be
///   `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be `VK_IMAGE_LAYOUT_UNDEFINED`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL`, `VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`image_view`] is not [`crate::utils::Handle::null`], `layout`**must** not be
///   `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
/// - If [`image_view`] is not [`crate::utils::Handle::null`], `layout`**must** not be
///   `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR`
/// - If [`image_view`] is not [`crate::utils::Handle::null`], `layout`**must** not be
///   `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
/// - If [`image_view`] is not [`crate::utils::Handle::null`], `layout`**must** not be
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
/// - If [`image_view`] is not [`crate::utils::Handle::null`] and [`resolve_mode`] is not
///   `VK_RESOLVE_MODE_NONE`, [`resolve_image_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO`
/// - [`p_next`]**must** be `NULL`
/// - If [`image_view`] is not [`crate::utils::Handle::null`], [`image_view`]**must** be a valid
///   [`ImageView`] handle
/// - [`image_layout`]**must** be a valid [`ImageLayout`] value
/// - If [`resolve_mode`] is not `0`, [`resolve_mode`]**must** be a valid [`ResolveModeFlagBits`]
///   value
/// - If [`resolve_image_view`] is not [`crate::utils::Handle::null`],
///   [`resolve_image_view`]**must** be a valid [`ImageView`] handle
/// - [`resolve_image_layout`]**must** be a valid [`ImageLayout`] value
/// - [`load_op`]**must** be a valid [`AttachmentLoadOp`] value
/// - [`store_op`]**must** be a valid [`AttachmentStoreOp`] value
/// - [`clear_value`]**must** be a valid [`ClearValue`] union
/// - Both of [`image_view`], and [`resolve_image_view`] that are valid handles of non-ignored
///   parameters **must** have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`VK_KHR_dynamic_rendering`]
/// - [`crate::vulkan1_3`]
/// - [`AttachmentLoadOp`]
/// - [`AttachmentStoreOp`]
/// - [`ClearValue`]
/// - [`ImageLayout`]
/// - [`ImageView`]
/// - [`RenderingInfo`]
/// - [`ResolveModeFlagBits`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderingAttachmentInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`image_view`] is the image view that will be used for rendering.
    image_view: ImageView,
    ///[`image_layout`] is the layout that [`image_view`] will be in during
    ///rendering.
    image_layout: ImageLayout,
    ///[`resolve_mode`] is a [`ResolveModeFlagBits`] value defining how
    ///multisampled data written to [`image_view`] will be resolved.
    resolve_mode: ResolveModeFlagBits,
    ///[`resolve_image_view`] is an image view used to write resolved
    ///multisample data at the end of rendering.
    resolve_image_view: ImageView,
    ///[`resolve_image_layout`] is the layout that [`resolve_image_view`] will
    ///be in during rendering.
    resolve_image_layout: ImageLayout,
    ///[`load_op`] is a [`AttachmentLoadOp`] value specifying how the
    ///contents of [`image_view`] are treated at the start of the render pass
    ///instance.
    load_op: AttachmentLoadOp,
    ///[`store_op`] is a [`AttachmentStoreOp`] value specifying how the
    ///contents of [`image_view`] are treated at the end of the render pass
    ///instance.
    store_op: AttachmentStoreOp,
    ///[`clear_value`] is a [`ClearValue`] structure defining values used
    ///to clear [`image_view`] when [`load_op`] is
    ///`VK_ATTACHMENT_LOAD_OP_CLEAR`.
    clear_value: ClearValue,
}
impl<'lt> Default for RenderingAttachmentInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            image_view: Default::default(),
            image_layout: Default::default(),
            resolve_mode: Default::default(),
            resolve_image_view: Default::default(),
            resolve_image_layout: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            clear_value: Default::default(),
        }
    }
}
impl<'lt> RenderingAttachmentInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::image_view`]
    pub fn image_view(&self) -> ImageView {
        self.image_view
    }
    ///Gets the value of [`Self::image_layout`]
    pub fn image_layout(&self) -> ImageLayout {
        self.image_layout
    }
    ///Gets the value of [`Self::resolve_mode`]
    pub fn resolve_mode(&self) -> ResolveModeFlagBits {
        self.resolve_mode
    }
    ///Gets the value of [`Self::resolve_image_view`]
    pub fn resolve_image_view(&self) -> ImageView {
        self.resolve_image_view
    }
    ///Gets the value of [`Self::resolve_image_layout`]
    pub fn resolve_image_layout(&self) -> ImageLayout {
        self.resolve_image_layout
    }
    ///Gets the value of [`Self::load_op`]
    pub fn load_op(&self) -> AttachmentLoadOp {
        self.load_op
    }
    ///Gets the value of [`Self::store_op`]
    pub fn store_op(&self) -> AttachmentStoreOp {
        self.store_op
    }
    ///Gets the value of [`Self::clear_value`]
    pub fn clear_value(&self) -> ClearValue {
        self.clear_value
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image_view`]
    pub fn image_view_mut(&mut self) -> &mut ImageView {
        &mut self.image_view
    }
    ///Gets a mutable reference to the value of [`Self::image_layout`]
    pub fn image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.image_layout
    }
    ///Gets a mutable reference to the value of [`Self::resolve_mode`]
    pub fn resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.resolve_mode
    }
    ///Gets a mutable reference to the value of [`Self::resolve_image_view`]
    pub fn resolve_image_view_mut(&mut self) -> &mut ImageView {
        &mut self.resolve_image_view
    }
    ///Gets a mutable reference to the value of [`Self::resolve_image_layout`]
    pub fn resolve_image_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.resolve_image_layout
    }
    ///Gets a mutable reference to the value of [`Self::load_op`]
    pub fn load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.load_op
    }
    ///Gets a mutable reference to the value of [`Self::store_op`]
    pub fn store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.store_op
    }
    ///Gets a mutable reference to the value of [`Self::clear_value`]
    pub fn clear_value_mut(&mut self) -> &mut ClearValue {
        &mut self.clear_value
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
    ///Sets the raw value of [`Self::image_view`]
    pub fn set_image_view(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view = value;
        self
    }
    ///Sets the raw value of [`Self::image_layout`]
    pub fn set_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::resolve_mode`]
    pub fn set_resolve_mode(&mut self, value: crate::vulkan1_2::ResolveModeFlagBits) -> &mut Self {
        self.resolve_mode = value;
        self
    }
    ///Sets the raw value of [`Self::resolve_image_view`]
    pub fn set_resolve_image_view(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.resolve_image_view = value;
        self
    }
    ///Sets the raw value of [`Self::resolve_image_layout`]
    pub fn set_resolve_image_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.resolve_image_layout = value;
        self
    }
    ///Sets the raw value of [`Self::load_op`]
    pub fn set_load_op(&mut self, value: crate::vulkan1_0::AttachmentLoadOp) -> &mut Self {
        self.load_op = value;
        self
    }
    ///Sets the raw value of [`Self::store_op`]
    pub fn set_store_op(&mut self, value: crate::vulkan1_0::AttachmentStoreOp) -> &mut Self {
        self.store_op = value;
        self
    }
    ///Sets the raw value of [`Self::clear_value`]
    pub fn set_clear_value(&mut self, value: crate::vulkan1_0::ClearValue) -> &mut Self {
        self.clear_value = value;
        self
    }
}
///[VkPhysicalDeviceDynamicRenderingFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDynamicRenderingFeatures.html) - Structure indicating support for dynamic render pass instances
///# C Specifications
///The [`PhysicalDeviceDynamicRenderingFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkPhysicalDeviceDynamicRenderingFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           dynamicRendering;
///} VkPhysicalDeviceDynamicRenderingFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering
///typedef VkPhysicalDeviceDynamicRenderingFeatures VkPhysicalDeviceDynamicRenderingFeaturesKHR;
///```
///# Members
///The members of the [`PhysicalDeviceDynamicRenderingFeatures`] structure
///describe the following features:
///# Description
/// - [`dynamic_rendering`] specifies that the implementation supports dynamic render pass instances
///   using the [`CmdBeginRendering`] command.
///If the [`PhysicalDeviceDynamicRenderingFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDynamicRenderingFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`
///# Related
/// - [`VK_KHR_dynamic_rendering`]
/// - [`crate::vulkan1_3`]
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
pub struct PhysicalDeviceDynamicRenderingFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES`
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`dynamic_rendering`]
    ///specifies that the implementation supports dynamic render pass instances
    ///using the [`CmdBeginRendering`] command.
    dynamic_rendering: Bool32,
}
impl<'lt> Default for PhysicalDeviceDynamicRenderingFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            dynamic_rendering: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDynamicRenderingFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering_raw(&self) -> Bool32 {
        self.dynamic_rendering
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dynamic_rendering`]
    pub fn set_dynamic_rendering_raw(&mut self, value: Bool32) -> &mut Self {
        self.dynamic_rendering = value;
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
    ///Gets the value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.dynamic_rendering as u8) }
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
    ///Gets a mutable reference to the value of [`Self::dynamic_rendering`]
    pub fn dynamic_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dynamic_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dynamic_rendering as *mut Bool32)
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
    ///Sets the raw value of [`Self::dynamic_rendering`]
    pub fn set_dynamic_rendering(&mut self, value: bool) -> &mut Self {
        self.dynamic_rendering = value as u8 as u32;
        self
    }
}
///[VkCommandBufferInheritanceRenderingInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderingInfo.html) - Structure specifying command buffer inheritance info for dynamic render pass instances
///# C Specifications
///The [`CommandBufferInheritanceRenderingInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_3
///typedef struct VkCommandBufferInheritanceRenderingInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkRenderingFlags         flags;
///    uint32_t                 viewMask;
///    uint32_t                 colorAttachmentCount;
///    const VkFormat*          pColorAttachmentFormats;
///    VkFormat                 depthAttachmentFormat;
///    VkFormat                 stencilAttachmentFormat;
///    VkSampleCountFlagBits    rasterizationSamples;
///} VkCommandBufferInheritanceRenderingInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dynamic_rendering
///typedef VkCommandBufferInheritanceRenderingInfo VkCommandBufferInheritanceRenderingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure
/// - [`flags`] is a bitmask of [`RenderingFlagBits`] used by the render pass instance.
/// - [`view_mask`] is the view mask used for rendering.
/// - [`color_attachment_count`] is the number of color attachments specified in the render pass
///   instance.
/// - [`color_attachment_formats`] is a pointer to an array of [`Format`] values defining the format
///   of color attachments.
/// - [`depth_attachment_format`] is a [`Format`] value defining the format of the depth attachment.
/// - [`stencil_attachment_format`] is a [`Format`] value defining the format of the stencil
///   attachment.
/// - [`rasterization_samples`] is a [`SampleCountFlagBits`] specifying the number of samples used
///   in rasterization.
///# Description
///If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
///[`CommandBufferInheritanceRenderingInfo`] structure, then that structure
///controls parameters of dynamic render pass instances that the
///[`CommandBuffer`]**can** be executed within.
///If [`CommandBufferInheritanceInfo::render_pass`] is not
///[`crate::utils::Handle::null`], or
///`VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT` is not specified in
///[`CommandBufferBeginInfo`]::[`flags`], parameters of this structure
///are ignored.If [`color_attachment_count`] is `0` and the
///[`variableMultisampleRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-variableMultisampleRate) feature
///is enabled, [`rasterization_samples`] is ignored.If [`depth_attachment_format`],
/// [`stencil_attachment_format`], or any
///element of [`color_attachment_formats`] is `VK_FORMAT_UNDEFINED`, it
///indicates that the corresponding attachment is unused within the render
///pass.Valid Usage
/// - If [`color_attachment_count`] is not `0`, [`rasterization_samples`]**must** be a valid
///   [`SampleCountFlagBits`] value
/// - If the [`variableMultisampleRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-variableMultisampleRate)
///   feature is not enabled, [`rasterization_samples`]**must** be a valid [`SampleCountFlagBits`]
///   value
/// -    If any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// - If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format that
///   includes a depth aspect
/// -    If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    When rendering to a [Linear Color attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary), if any element of [`color_attachment_formats`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// - If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format that
///   includes a stencil aspect
/// -    If [`stencil_attachment_format`] is not `VK_FORMAT_UNDEFINED`, it **must** be a format with [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) that include `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`depth_attachment_format`] is not `VK_FORMAT_UNDEFINED` and [`stencil_attachment_format`]
///   is not `VK_FORMAT_UNDEFINED`, [`depth_attachment_format`]**must** equal
///   [`stencil_attachment_format`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, [`view_mask`]**must** be `0`
/// - The index of the most significant bit in [`view_mask`]**must** be less than [`maxMultiviewViewCount`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxMultiviewViewCount)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO`
/// - [`flags`]**must** be a valid combination of [`RenderingFlagBits`] values
/// - If [`color_attachment_count`] is not `0`, [`color_attachment_formats`]**must** be a valid
///   pointer to an array of [`color_attachment_count`] valid [`Format`] values
/// - [`depth_attachment_format`]**must** be a valid [`Format`] value
/// - [`stencil_attachment_format`]**must** be a valid [`Format`] value
/// - If [`rasterization_samples`] is not `0`, [`rasterization_samples`]**must** be a valid
///   [`SampleCountFlagBits`] value
///# Related
/// - [`VK_KHR_dynamic_rendering`]
/// - [`crate::vulkan1_3`]
/// - [`Format`]
/// - [`RenderingFlags`]
/// - [`SampleCountFlagBits`]
/// - [`StructureType`]
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
#[repr(C)]
pub struct CommandBufferInheritanceRenderingInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`RenderingFlagBits`] used by the render
    ///pass instance.
    flags: RenderingFlags,
    ///[`view_mask`] is the view mask used for rendering.
    view_mask: u32,
    ///[`color_attachment_count`] is the number of color attachments specified
    ///in the render pass instance.
    color_attachment_count: u32,
    ///[`color_attachment_formats`] is a pointer to an array of [`Format`]
    ///values defining the format of color attachments.
    color_attachment_formats: *const Format,
    ///[`depth_attachment_format`] is a [`Format`] value defining the
    ///format of the depth attachment.
    depth_attachment_format: Format,
    ///[`stencil_attachment_format`] is a [`Format`] value defining the
    ///format of the stencil attachment.
    stencil_attachment_format: Format,
    ///[`rasterization_samples`] is a [`SampleCountFlagBits`] specifying
    ///the number of samples used in rasterization.
    rasterization_samples: SampleCountFlagBits,
}
impl<'lt> Default for CommandBufferInheritanceRenderingInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            view_mask: 0,
            color_attachment_count: 0,
            color_attachment_formats: std::ptr::null(),
            depth_attachment_format: Default::default(),
            stencil_attachment_format: Default::default(),
            rasterization_samples: Default::default(),
        }
    }
}
impl<'lt> CommandBufferInheritanceRenderingInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view_mask`]
    pub fn view_mask_raw(&self) -> u32 {
        self.view_mask
    }
    ///Gets the raw value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_raw(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the raw value of [`Self::color_attachment_formats`]
    pub fn color_attachment_formats_raw(&self) -> *const Format {
        self.color_attachment_formats
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask_raw(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count_raw(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_formats`]
    pub fn set_color_attachment_formats_raw(&mut self, value: *const Format) -> &mut Self {
        self.color_attachment_formats = value;
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
    pub fn flags(&self) -> RenderingFlags {
        self.flags
    }
    ///Gets the value of [`Self::view_mask`]
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Gets the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the value of [`Self::color_attachment_formats`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_attachment_formats(&self) -> &[Format] {
        std::slice::from_raw_parts(self.color_attachment_formats, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::depth_attachment_format`]
    pub fn depth_attachment_format(&self) -> Format {
        self.depth_attachment_format
    }
    ///Gets the value of [`Self::stencil_attachment_format`]
    pub fn stencil_attachment_format(&self) -> Format {
        self.stencil_attachment_format
    }
    ///Gets the value of [`Self::rasterization_samples`]
    pub fn rasterization_samples(&self) -> SampleCountFlagBits {
        self.rasterization_samples
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut RenderingFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::view_mask`]
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::depth_attachment_format`]
    pub fn depth_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.depth_attachment_format
    }
    ///Gets a mutable reference to the value of [`Self::stencil_attachment_format`]
    pub fn stencil_attachment_format_mut(&mut self) -> &mut Format {
        &mut self.stencil_attachment_format
    }
    ///Gets a mutable reference to the value of [`Self::rasterization_samples`]
    pub fn rasterization_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.rasterization_samples
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
    pub fn set_flags(&mut self, value: crate::vulkan1_3::RenderingFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_formats`]
    pub fn set_color_attachment_formats(&mut self, value: &'lt [crate::vulkan1_0::Format]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachment_formats = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::depth_attachment_format`]
    pub fn set_depth_attachment_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.depth_attachment_format = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_attachment_format`]
    pub fn set_stencil_attachment_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.stencil_attachment_format = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_samples`]
    pub fn set_rasterization_samples(&mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> &mut Self {
        self.rasterization_samples = value;
        self
    }
}
///[VkPrivateDataSlot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html) - Opaque handle to a private data slot object
///# C Specifications
///Private data slots are represented by [`PrivateDataSlot`] handles:
///```c
///// Provided by VK_VERSION_1_3
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPrivateDataSlot)
///```
///or the equivalent
///```c
///// Provided by VK_EXT_private_data
///typedef VkPrivateDataSlot VkPrivateDataSlotEXT;
///```
///# Related
/// - [`VK_EXT_private_data`]
/// - [`crate::vulkan1_3`]
/// - [`CreatePrivateDataSlot`]
/// - [`CreatePrivateDataSlotEXT`]
/// - [`DestroyPrivateDataSlot`]
/// - [`DestroyPrivateDataSlotEXT`]
/// - [`GetPrivateData`]
/// - [`GetPrivateDataEXT`]
/// - [`SetPrivateData`]
/// - [`SetPrivateDataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct PrivateDataSlot(pub u64);
impl PrivateDataSlot {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub const fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for PrivateDataSlot {}
impl Default for PrivateDataSlot {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for PrivateDataSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for PrivateDataSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
