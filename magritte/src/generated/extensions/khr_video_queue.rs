use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, DeviceMemory, DeviceSize, Extent2D, Format, ImageUsageFlags,
        ImageView, Offset2D, StructureType,
    },
    vulkan1_1::MemoryRequirements2,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_queue");
///[VkQueryResultStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) - Specific status codes for operations
///# C Specifications
///Specific status codes that **can** be returned from a query are:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkQueryResultStatusKHR {
///    VK_QUERY_RESULT_STATUS_ERROR_KHR = -1,
///    VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0,
///    VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1,
///} VkQueryResultStatusKHR;
///```
///# Description
/// - [`QueryResultStatusNotReadyKhr`] specifies that the query result is not yet available.
/// - [`QueryResultStatusErrorKhr`] specifies that operations did not complete successfully.
/// - [`QueryResultStatusCompleteKhr`] specifies that operations completed successfully and the
///   query result is available.
///# Related
/// - [`VK_KHR_video_queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryResultStatusKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum QueryResultStatusKHR {
    ///[`QueryResultStatusErrorKhr`] specifies that operations did not
    ///complete successfully.
    QueryResultStatusErrorKhr = -1,
    ///[`QueryResultStatusNotReadyKhr`] specifies that the query
    ///result is not yet available.
    QueryResultStatusNotReadyKhr = 0,
    ///[`QueryResultStatusCompleteKhr`] specifies that operations
    ///completed successfully and the query result is available.
    QueryResultStatusCompleteKhr = 1,
}
impl const Default for QueryResultStatusKHR {
    fn default() -> Self {
        QueryResultStatusNotReadyKhr
    }
}
impl QueryResultStatusKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkVideoQueueFamilyProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoQueueFamilyProperties2KHR.html) - Structure specifying the codec video operations
///# C Specifications
///The [`VideoQueueFamilyProperties2KHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoQueueFamilyProperties2KHR {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkVideoCodecOperationFlagsKHR    videoCodecOperations;
///} VkVideoQueueFamilyProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_codec_operations`] is a bitmask of [`VideoCodecOperationFlagBitsKHR`] specifying
///   supported video codec operation(s).
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`
/// - [`video_codec_operations`]**must** be a valid combination of
///   [`VideoCodecOperationFlagBitsKHR`] values
/// - [`video_codec_operations`]**must** not be `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoCodecOperationFlagsKHR`]
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
pub struct VideoQueueFamilyProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`video_codec_operations`] is a bitmask of
    ///[`VideoCodecOperationFlagBitsKHR`] specifying supported video codec
    ///operation(s).
    video_codec_operations: VideoCodecOperationFlagsKHR,
}
impl<'lt> Default for VideoQueueFamilyProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            video_codec_operations: Default::default(),
        }
    }
}
impl<'lt> VideoQueueFamilyProperties2KHR<'lt> {
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
    ///Gets the value of [`Self::video_codec_operations`]
    pub fn video_codec_operations(&self) -> VideoCodecOperationFlagsKHR {
        self.video_codec_operations
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
    ///Gets a mutable reference to the value of [`Self::video_codec_operations`]
    pub fn video_codec_operations_mut(&mut self) -> &mut VideoCodecOperationFlagsKHR {
        &mut self.video_codec_operations
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
    ///Sets the raw value of [`Self::video_codec_operations`]
    pub fn set_video_codec_operations(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagsKHR,
    ) -> &mut Self {
        self.video_codec_operations = value;
        self
    }
}
///[VkQueueFamilyQueryResultStatusProperties2KHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyQueryResultStatusProperties2KHR.html) - Structure specifying support for result status query
///# C Specifications
///The [`QueueFamilyQueryResultStatusProperties2KHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkQueueFamilyQueryResultStatusProperties2KHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           supported;
///} VkQueueFamilyQueryResultStatusProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supported`] reports [`TRUE`] if query type `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of
///   `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`
///# Related
/// - [`VK_KHR_video_queue`]
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
pub struct QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`supported`] reports [`TRUE`] if query type
    ///`VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` and use of
    ///`VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` are supported.
    supported: Bool32,
}
impl<'lt> Default for QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            supported: 0,
        }
    }
}
impl<'lt> QueueFamilyQueryResultStatusProperties2KHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::supported`]
    pub fn supported_raw(&self) -> Bool32 {
        self.supported
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supported`]
    pub fn set_supported_raw(&mut self, value: Bool32) -> &mut Self {
        self.supported = value;
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
    ///Gets the value of [`Self::supported`]
    pub fn supported(&self) -> bool {
        unsafe { std::mem::transmute(self.supported as u8) }
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
    ///Gets a mutable reference to the value of [`Self::supported`]
    pub fn supported_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supported as *mut Bool32)
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
    ///Sets the raw value of [`Self::supported`]
    pub fn set_supported(&mut self, value: bool) -> &mut Self {
        self.supported = value as u8 as u32;
        self
    }
}
///[VkVideoProfilesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfilesKHR.html) - Structure enumerating the video profiles
///# C Specifications
///The [`VideoProfilesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoProfilesKHR {
///    VkStructureType             sType;
///    void*                       pNext;
///    uint32_t                    profileCount;
///    const VkVideoProfileKHR*    pProfiles;
///} VkVideoProfilesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`profile_count`] is an integer which holds the number of video profiles included in
///   [`profiles`].
/// - [`profiles`] is a pointer to an array of [`VideoProfileKHR`] structures. Each
///   [`VideoProfileKHR`] structure **must** chain the corresponding codec-operation specific
///   extension video profile structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`
/// - [`profiles`]**must** be a valid pointer to an array of [`profile_count`] valid
///   [`VideoProfileKHR`] structures
/// - [`profile_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`PhysicalDeviceVideoFormatInfoKHR`]
/// - [`StructureType`]
/// - [`VideoProfileKHR`]
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
pub struct VideoProfilesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`profile_count`] is an integer which holds the number of video
    ///profiles included in [`profiles`].
    profile_count: u32,
    ///[`profiles`] is a pointer to an array of [`VideoProfileKHR`]
    ///structures.
    ///Each [`VideoProfileKHR`] structure **must** chain the corresponding
    ///codec-operation specific extension video profile structure.
    profiles: *const VideoProfileKHR<'lt>,
}
impl<'lt> Default for VideoProfilesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            profile_count: 0,
            profiles: std::ptr::null(),
        }
    }
}
impl<'lt> VideoProfilesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::profile_count`]
    pub fn profile_count_raw(&self) -> u32 {
        self.profile_count
    }
    ///Gets the raw value of [`Self::profiles`]
    pub fn profiles_raw(&self) -> *const VideoProfileKHR<'lt> {
        self.profiles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::profile_count`]
    pub fn set_profile_count_raw(&mut self, value: u32) -> &mut Self {
        self.profile_count = value;
        self
    }
    ///Sets the raw value of [`Self::profiles`]
    pub fn set_profiles_raw(&mut self, value: *const VideoProfileKHR<'lt>) -> &mut Self {
        self.profiles = value;
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
    ///Gets the value of [`Self::profile_count`]
    pub fn profile_count(&self) -> u32 {
        self.profile_count
    }
    ///Gets the value of [`Self::profiles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn profiles(&self) -> &[VideoProfileKHR<'lt>] {
        std::slice::from_raw_parts(self.profiles, self.profile_count as usize)
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
    ///Gets a mutable reference to the value of [`Self::profile_count`]
    pub fn profile_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::profile_count`]
    pub fn set_profile_count(&mut self, value: u32) -> &mut Self {
        self.profile_count = value;
        self
    }
    ///Sets the raw value of [`Self::profiles`]
    pub fn set_profiles(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoProfileKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.profiles = value.as_ptr();
        self.profile_count = len_;
        self
    }
}
///[VkPhysicalDeviceVideoFormatInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVideoFormatInfoKHR.html) - Structure specifying the codec video format
///# C Specifications
///The [`PhysicalDeviceVideoFormatInfoKHR`] input structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkPhysicalDeviceVideoFormatInfoKHR {
///    VkStructureType              sType;
///    void*                        pNext;
///    VkImageUsageFlags            imageUsage;
///    const VkVideoProfilesKHR*    pVideoProfiles;
///} VkPhysicalDeviceVideoFormatInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying intended video image usages.
/// - [`video_profiles`] is a pointer to a [`VideoProfilesKHR`] structure providing the video
///   profile(s) of video session(s) that will use the image. For most use cases, the image is used
///   by a single video session and a single video profile is provided. For a use case such as
///   transcode, where a decode session output image **may** be used as encode input for one or more
///   encode sessions, multiple video profiles representing the video sessions that will share the
///   image **may** be provided.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`VideoProfilesKHR`]
/// - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
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
pub struct PhysicalDeviceVideoFormatInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`image_usage`] is a bitmask of [`ImageUsageFlagBits`] specifying
    ///intended video image usages.
    image_usage: ImageUsageFlags,
    ///[`video_profiles`] is a pointer to a [`VideoProfilesKHR`]
    ///structure providing the video profile(s) of video session(s) that will
    ///use the image.
    ///For most use cases, the image is used by a single video session and a
    ///single video profile is provided.
    ///For a use case such as transcode, where a decode session output image
    ///**may** be used as encode input for one or more encode sessions, multiple
    ///video profiles representing the video sessions that will share the image
    ///**may** be provided.
    video_profiles: *const VideoProfilesKHR<'lt>,
}
impl<'lt> Default for PhysicalDeviceVideoFormatInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            image_usage: Default::default(),
            video_profiles: std::ptr::null(),
        }
    }
}
impl<'lt> PhysicalDeviceVideoFormatInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::video_profiles`]
    pub fn video_profiles_raw(&self) -> *const VideoProfilesKHR<'lt> {
        self.video_profiles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::video_profiles`]
    pub fn set_video_profiles_raw(&mut self, value: *const VideoProfilesKHR<'lt>) -> &mut Self {
        self.video_profiles = value;
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
    ///Gets the value of [`Self::image_usage`]
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Gets the value of [`Self::video_profiles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn video_profiles(&self) -> &VideoProfilesKHR<'lt> {
        &*self.video_profiles
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
    ///Gets a mutable reference to the value of [`Self::image_usage`]
    pub fn image_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.image_usage
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
    ///Sets the raw value of [`Self::image_usage`]
    pub fn set_image_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.image_usage = value;
        self
    }
    ///Sets the raw value of [`Self::video_profiles`]
    pub fn set_video_profiles(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoProfilesKHR<'lt>,
    ) -> &mut Self {
        self.video_profiles = value as *const _;
        self
    }
}
///[VkVideoFormatPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoFormatPropertiesKHR.html) - Structure enumerating the video image formats
///# C Specifications
///The [`VideoFormatPropertiesKHR`] output structure for
///[`GetPhysicalDeviceVideoFormatPropertiesKHR`] is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoFormatPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkFormat           format;
///} VkVideoFormatPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is one of the supported formats reported by the implementation.
///# Description
///If the `pVideoProfiles` or `imageUsage` provided in input structure
///`pVideoFormatInfo` are not supported,
///`VK_ERROR_FORMAT_NOT_SUPPORTED` is returned.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceVideoFormatPropertiesKHR`]
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
pub struct VideoFormatPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`format`] is one of the supported formats reported by the
    ///implementation.
    format: Format,
}
impl<'lt> Default for VideoFormatPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            format: Default::default(),
        }
    }
}
impl<'lt> VideoFormatPropertiesKHR<'lt> {
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
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
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
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
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
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
}
///[VkVideoProfileKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoProfileKHR.html) - Structure specifying the codec video profile
///# C Specifications
///A video profile is defined by [`VideoProfileKHR`] structure as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoProfileKHR {
///    VkStructureType                     sType;
///    void*                               pNext;
///    VkVideoCodecOperationFlagBitsKHR    videoCodecOperation;
///    VkVideoChromaSubsamplingFlagsKHR    chromaSubsampling;
///    VkVideoComponentBitDepthFlagsKHR    lumaBitDepth;
///    VkVideoComponentBitDepthFlagsKHR    chromaBitDepth;
///} VkVideoProfileKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`] value specifying a video
///   codec operation.
/// - [`chroma_subsampling`] is a bitmask of [`VideoChromaSubsamplingFlagBitsKHR`] specifying video
///   chroma subsampling information.
/// - [`luma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video luma
///   bit depth information.
/// - [`chroma_bit_depth`] is a bitmask of [`VideoComponentBitDepthFlagBitsKHR`] specifying video
///   chroma bit depth information.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`
/// - [`video_codec_operation`]**must** be a valid [`VideoCodecOperationFlagBitsKHR`] value
/// - [`chroma_subsampling`]**must** be a valid combination of [`VideoChromaSubsamplingFlagBitsKHR`]
///   values
/// - [`chroma_subsampling`]**must** not be `0`
/// - [`luma_bit_depth`]**must** be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`luma_bit_depth`]**must** not be `0`
/// - [`chroma_bit_depth`]**must** be a valid combination of [`VideoComponentBitDepthFlagBitsKHR`]
///   values
/// - [`chroma_bit_depth`]**must** not be `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoChromaSubsamplingFlagsKHR`]
/// - [`VideoCodecOperationFlagBitsKHR`]
/// - [`VideoComponentBitDepthFlagsKHR`]
/// - [`VideoProfilesKHR`]
/// - [`VideoSessionCreateInfoKHR`]
/// - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
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
pub struct VideoProfileKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`video_codec_operation`] is a [`VideoCodecOperationFlagBitsKHR`]
    ///value specifying a video codec operation.
    video_codec_operation: VideoCodecOperationFlagBitsKHR,
    ///[`chroma_subsampling`] is a bitmask of
    ///[`VideoChromaSubsamplingFlagBitsKHR`] specifying video chroma
    ///subsampling information.
    chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    ///[`luma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video luma bit
    ///depth information.
    luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    ///[`chroma_bit_depth`] is a bitmask of
    ///[`VideoComponentBitDepthFlagBitsKHR`] specifying video chroma bit
    ///depth information.
    chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
impl<'lt> Default for VideoProfileKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            video_codec_operation: Default::default(),
            chroma_subsampling: Default::default(),
            luma_bit_depth: Default::default(),
            chroma_bit_depth: Default::default(),
        }
    }
}
impl<'lt> VideoProfileKHR<'lt> {
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
    ///Gets the value of [`Self::video_codec_operation`]
    pub fn video_codec_operation(&self) -> VideoCodecOperationFlagBitsKHR {
        self.video_codec_operation
    }
    ///Gets the value of [`Self::chroma_subsampling`]
    pub fn chroma_subsampling(&self) -> VideoChromaSubsamplingFlagsKHR {
        self.chroma_subsampling
    }
    ///Gets the value of [`Self::luma_bit_depth`]
    pub fn luma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.luma_bit_depth
    }
    ///Gets the value of [`Self::chroma_bit_depth`]
    pub fn chroma_bit_depth(&self) -> VideoComponentBitDepthFlagsKHR {
        self.chroma_bit_depth
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
    ///Gets a mutable reference to the value of [`Self::video_codec_operation`]
    pub fn video_codec_operation_mut(&mut self) -> &mut VideoCodecOperationFlagBitsKHR {
        &mut self.video_codec_operation
    }
    ///Gets a mutable reference to the value of [`Self::chroma_subsampling`]
    pub fn chroma_subsampling_mut(&mut self) -> &mut VideoChromaSubsamplingFlagsKHR {
        &mut self.chroma_subsampling
    }
    ///Gets a mutable reference to the value of [`Self::luma_bit_depth`]
    pub fn luma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.luma_bit_depth
    }
    ///Gets a mutable reference to the value of [`Self::chroma_bit_depth`]
    pub fn chroma_bit_depth_mut(&mut self) -> &mut VideoComponentBitDepthFlagsKHR {
        &mut self.chroma_bit_depth
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
    ///Sets the raw value of [`Self::video_codec_operation`]
    pub fn set_video_codec_operation(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR,
    ) -> &mut Self {
        self.video_codec_operation = value;
        self
    }
    ///Sets the raw value of [`Self::chroma_subsampling`]
    pub fn set_chroma_subsampling(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoChromaSubsamplingFlagsKHR,
    ) -> &mut Self {
        self.chroma_subsampling = value;
        self
    }
    ///Sets the raw value of [`Self::luma_bit_depth`]
    pub fn set_luma_bit_depth(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> &mut Self {
        self.luma_bit_depth = value;
        self
    }
    ///Sets the raw value of [`Self::chroma_bit_depth`]
    pub fn set_chroma_bit_depth(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoComponentBitDepthFlagsKHR,
    ) -> &mut Self {
        self.chroma_bit_depth = value;
        self
    }
}
///[VkVideoCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCapabilitiesKHR.html) - Structure specifying parameters of video capabilities
///# C Specifications
///The [`VideoCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoCapabilitiesKHR {
///    VkStructureType              sType;
///    void*                        pNext;
///    VkVideoCapabilityFlagsKHR    capabilityFlags;
///    VkDeviceSize                 minBitstreamBufferOffsetAlignment;
///    VkDeviceSize                 minBitstreamBufferSizeAlignment;
///    VkExtent2D                   videoPictureExtentGranularity;
///    VkExtent2D                   minExtent;
///    VkExtent2D                   maxExtent;
///    uint32_t                     maxReferencePicturesSlotsCount;
///    uint32_t                     maxReferencePicturesActiveCount;
///} VkVideoCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`] specifying capability
///   flags.
/// - [`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the input or output
///   bitstream buffer offset.
/// - [`min_bitstream_buffer_size_alignment`] is the minimum alignment for the input or output
///   bitstream buffer size
/// - [`video_picture_extent_granularity`] is the minimum size alignment of the extent with the
///   required padding for the decoded or encoded video images.
/// - [`min_extent`] is the minimum width and height of the decoded or encoded video.
/// - [`max_extent`] is the maximum width and height of the decoded or encoded video.
/// - [`max_reference_pictures_slots_count`] is the maximum number of DPB Slots supported by the
///   implementation for a single video session instance.
/// - [`max_reference_pictures_active_count`] is the maximum slots that can be used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   with a single decode or encode operation.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeCapabilitiesKHR`] or
///   [`VideoEncodeCapabilitiesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`DeviceSize`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`VideoCapabilityFlagsKHR`]
/// - [`GetPhysicalDeviceVideoCapabilitiesKHR`]
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
pub struct VideoCapabilitiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`capability_flags`] is a bitmask of [`VideoCapabilityFlagBitsKHR`]
    ///specifying capability flags.
    capability_flags: VideoCapabilityFlagsKHR,
    ///[`min_bitstream_buffer_offset_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer offset.
    min_bitstream_buffer_offset_alignment: DeviceSize,
    ///[`min_bitstream_buffer_size_alignment`] is the minimum alignment for the
    ///input or output bitstream buffer size
    min_bitstream_buffer_size_alignment: DeviceSize,
    ///[`video_picture_extent_granularity`] is the minimum size alignment of the
    ///extent with the required padding for the decoded or encoded video
    ///images.
    video_picture_extent_granularity: Extent2D,
    ///[`min_extent`] is the minimum width and height of the decoded or
    ///encoded video.
    min_extent: Extent2D,
    ///[`max_extent`] is the maximum width and height of the decoded or
    ///encoded video.
    max_extent: Extent2D,
    ///[`max_reference_pictures_slots_count`] is the maximum number of DPB Slots
    ///supported by the implementation for a single video session instance.
    max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum slots that can be
    ///used as [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) with a single decode or
    ///encode operation.
    max_reference_pictures_active_count: u32,
}
impl<'lt> Default for VideoCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            capability_flags: Default::default(),
            min_bitstream_buffer_offset_alignment: Default::default(),
            min_bitstream_buffer_size_alignment: Default::default(),
            video_picture_extent_granularity: Default::default(),
            min_extent: Default::default(),
            max_extent: Default::default(),
            max_reference_pictures_slots_count: 0,
            max_reference_pictures_active_count: 0,
        }
    }
}
impl<'lt> VideoCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_raw(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_raw(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
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
    ///Gets the value of [`Self::capability_flags`]
    pub fn capability_flags(&self) -> VideoCapabilityFlagsKHR {
        self.capability_flags
    }
    ///Gets the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn min_bitstream_buffer_offset_alignment(&self) -> DeviceSize {
        self.min_bitstream_buffer_offset_alignment
    }
    ///Gets the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn min_bitstream_buffer_size_alignment(&self) -> DeviceSize {
        self.min_bitstream_buffer_size_alignment
    }
    ///Gets the value of [`Self::video_picture_extent_granularity`]
    pub fn video_picture_extent_granularity(&self) -> Extent2D {
        self.video_picture_extent_granularity
    }
    ///Gets the value of [`Self::min_extent`]
    pub fn min_extent(&self) -> Extent2D {
        self.min_extent
    }
    ///Gets the value of [`Self::max_extent`]
    pub fn max_extent(&self) -> Extent2D {
        self.max_extent
    }
    ///Gets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
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
    ///Gets a mutable reference to the value of [`Self::capability_flags`]
    pub fn capability_flags_mut(&mut self) -> &mut VideoCapabilityFlagsKHR {
        &mut self.capability_flags
    }
    ///Gets a mutable reference to the value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn min_bitstream_buffer_offset_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.min_bitstream_buffer_offset_alignment
    }
    ///Gets a mutable reference to the value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn min_bitstream_buffer_size_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.min_bitstream_buffer_size_alignment
    }
    ///Gets a mutable reference to the value of [`Self::video_picture_extent_granularity`]
    pub fn video_picture_extent_granularity_mut(&mut self) -> &mut Extent2D {
        &mut self.video_picture_extent_granularity
    }
    ///Gets a mutable reference to the value of [`Self::min_extent`]
    pub fn min_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_extent`]
    pub fn max_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::capability_flags`]
    pub fn set_capability_flags(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCapabilityFlagsKHR,
    ) -> &mut Self {
        self.capability_flags = value;
        self
    }
    ///Sets the raw value of [`Self::min_bitstream_buffer_offset_alignment`]
    pub fn set_min_bitstream_buffer_offset_alignment(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.min_bitstream_buffer_offset_alignment = value;
        self
    }
    ///Sets the raw value of [`Self::min_bitstream_buffer_size_alignment`]
    pub fn set_min_bitstream_buffer_size_alignment(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.min_bitstream_buffer_size_alignment = value;
        self
    }
    ///Sets the raw value of [`Self::video_picture_extent_granularity`]
    pub fn set_video_picture_extent_granularity(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.video_picture_extent_granularity = value;
        self
    }
    ///Sets the raw value of [`Self::min_extent`]
    pub fn set_min_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_extent`]
    pub fn set_max_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
        self
    }
}
///[VkVideoGetMemoryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoGetMemoryPropertiesKHR.html) - Structure specifying video session required memory heap type
///# C Specifications
///The [`VideoGetMemoryPropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoGetMemoryPropertiesKHR {
///    VkStructureType           sType;
///    const void*               pNext;
///    uint32_t                  memoryBindIndex;
///    VkMemoryRequirements2*    pMemoryRequirements;
///} VkVideoGetMemoryPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_bind_index`] is the memory bind index of the memory heap type described by the
///   information returned in [`memory_requirements`].
/// - [`memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the
///   requested memory heap requirements for the heap with index [`memory_bind_index`] are returned.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`memory_requirements`]**must** be a valid pointer to a [`MemoryRequirements2`] structure
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`MemoryRequirements2`]
/// - [`StructureType`]
/// - [`GetVideoSessionMemoryRequirementsKHR`]
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
pub struct VideoGetMemoryPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the memory bind index of the memory heap type
    ///described by the information returned in [`memory_requirements`].
    memory_bind_index: u32,
    ///[`memory_requirements`] is a pointer to a [`MemoryRequirements2`]
    ///structure in which the requested memory heap requirements for the heap
    ///with index [`memory_bind_index`] are returned.
    memory_requirements: *mut MemoryRequirements2<'lt>,
}
impl<'lt> Default for VideoGetMemoryPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            memory_bind_index: 0,
            memory_requirements: std::ptr::null_mut(),
        }
    }
}
impl<'lt> VideoGetMemoryPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_raw(&self) -> u32 {
        self.memory_bind_index
    }
    ///Gets the raw value of [`Self::memory_requirements`]
    pub fn memory_requirements_raw(&self) -> &*mut MemoryRequirements2<'lt> {
        &self.memory_requirements
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index_raw(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn set_memory_requirements_raw(&mut self, value: *mut MemoryRequirements2<'lt>) -> &mut Self {
        self.memory_requirements = value;
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
    ///Gets the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Gets the value of [`Self::memory_requirements`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn memory_requirements(&self) -> &MemoryRequirements2<'lt> {
        &*self.memory_requirements
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::memory_requirements`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn memory_requirements_mut(&mut self) -> &mut MemoryRequirements2<'lt> {
        &mut *self.memory_requirements
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
    ///Sets the raw value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn set_memory_requirements(&mut self, value: &'lt mut crate::vulkan1_1::MemoryRequirements2<'lt>) -> &mut Self {
        self.memory_requirements = value as *mut _;
        self
    }
}
///[VkVideoBindMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBindMemoryKHR.html) - Structure specifying device memory heap entry for video session object
///# C Specifications
///The [`VideoBindMemoryKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoBindMemoryKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           memoryBindIndex;
///    VkDeviceMemory     memory;
///    VkDeviceSize       memoryOffset;
///    VkDeviceSize       memorySize;
///} VkVideoBindMemoryKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_bind_index`] is the index of the device memory heap returned in
///   [`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from
///   [`GetVideoSessionMemoryRequirementsKHR`].
/// - [`memory`] is the allocated device memory to be bound to the video session’s heap with index
///   [`memory_bind_index`].
/// - [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound.
/// - [`memory_size`] is the size in bytes of the region of [`memory`], starting from
///   [`memory_offset`] bytes, to be bound.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`BindVideoSessionMemoryKHR`]
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
pub struct VideoBindMemoryKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`memory_bind_index`] is the index of the device memory heap returned in
    ///[`VideoGetMemoryPropertiesKHR`]::[`memory_bind_index`] from
    ///[`GetVideoSessionMemoryRequirementsKHR`].
    memory_bind_index: u32,
    ///[`memory`] is the allocated device memory to be bound to the video
    ///session’s heap with index [`memory_bind_index`].
    memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound.
    memory_offset: DeviceSize,
    ///[`memory_size`] is the size in bytes of the region of [`memory`],
    ///starting from [`memory_offset`] bytes, to be bound.
    memory_size: DeviceSize,
}
impl<'lt> Default for VideoBindMemoryKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            memory_bind_index: 0,
            memory: Default::default(),
            memory_offset: Default::default(),
            memory_size: Default::default(),
        }
    }
}
impl<'lt> VideoBindMemoryKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_raw(&self) -> u32 {
        self.memory_bind_index
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index_raw(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
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
    ///Gets the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index(&self) -> u32 {
        self.memory_bind_index
    }
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::memory_offset`]
    pub fn memory_offset(&self) -> DeviceSize {
        self.memory_offset
    }
    ///Gets the value of [`Self::memory_size`]
    pub fn memory_size(&self) -> DeviceSize {
        self.memory_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory_bind_index`]
    pub fn memory_bind_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::memory_offset`]
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
    }
    ///Gets a mutable reference to the value of [`Self::memory_size`]
    pub fn memory_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_size
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
    ///Sets the raw value of [`Self::memory_bind_index`]
    pub fn set_memory_bind_index(&mut self, value: u32) -> &mut Self {
        self.memory_bind_index = value;
        self
    }
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the raw value of [`Self::memory_offset`]
    pub fn set_memory_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_offset = value;
        self
    }
    ///Sets the raw value of [`Self::memory_size`]
    pub fn set_memory_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_size = value;
        self
    }
}
///[VkVideoPictureResourceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoPictureResourceKHR.html) - Structure specifying the picture resources
///# C Specifications
///The [`VideoPictureResourceKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoPictureResourceKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkOffset2D         codedOffset;
///    VkExtent2D         codedExtent;
///    uint32_t           baseArrayLayer;
///    VkImageView        imageViewBinding;
///} VkVideoPictureResourceKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`coded_offset`] is the offset to be used for the picture resource.
/// - [`coded_extent`] is the extent to be used for the picture resource.
/// - [`base_array_layer`] is the first array layer to be accessed for the Decode or Encode
///   Operations.
/// - [`image_view_binding`] is a [`ImageView`] image view representing this picture resource.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`image_view_binding`]**must** be a valid [`ImageView`] handle
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`Extent2D`]
/// - [`ImageView`]
/// - [`Offset2D`]
/// - [`StructureType`]
/// - [`VideoDecodeInfoKHR`]
/// - [`VideoEncodeInfoKHR`]
/// - [`VideoReferenceSlotKHR`]
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
pub struct VideoPictureResourceKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`coded_offset`] is the offset to be used for the picture resource.
    coded_offset: Offset2D,
    ///[`coded_extent`] is the extent to be used for the picture resource.
    coded_extent: Extent2D,
    ///[`base_array_layer`] is the first array layer to be accessed for the
    ///Decode or Encode Operations.
    base_array_layer: u32,
    ///[`image_view_binding`] is a [`ImageView`] image view representing
    ///this picture resource.
    image_view_binding: ImageView,
}
impl<'lt> Default for VideoPictureResourceKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            base_array_layer: 0,
            image_view_binding: Default::default(),
        }
    }
}
impl<'lt> VideoPictureResourceKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::base_array_layer`]
    pub fn base_array_layer_raw(&self) -> u32 {
        self.base_array_layer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::base_array_layer`]
    pub fn set_base_array_layer_raw(&mut self, value: u32) -> &mut Self {
        self.base_array_layer = value;
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
    ///Gets the value of [`Self::coded_offset`]
    pub fn coded_offset(&self) -> Offset2D {
        self.coded_offset
    }
    ///Gets the value of [`Self::coded_extent`]
    pub fn coded_extent(&self) -> Extent2D {
        self.coded_extent
    }
    ///Gets the value of [`Self::base_array_layer`]
    pub fn base_array_layer(&self) -> u32 {
        self.base_array_layer
    }
    ///Gets the value of [`Self::image_view_binding`]
    pub fn image_view_binding(&self) -> ImageView {
        self.image_view_binding
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::coded_offset`]
    pub fn coded_offset_mut(&mut self) -> &mut Offset2D {
        &mut self.coded_offset
    }
    ///Gets a mutable reference to the value of [`Self::coded_extent`]
    pub fn coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::base_array_layer`]
    pub fn base_array_layer_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image_view_binding`]
    pub fn image_view_binding_mut(&mut self) -> &mut ImageView {
        &mut self.image_view_binding
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
    ///Sets the raw value of [`Self::coded_offset`]
    pub fn set_coded_offset(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.coded_offset = value;
        self
    }
    ///Sets the raw value of [`Self::coded_extent`]
    pub fn set_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.coded_extent = value;
        self
    }
    ///Sets the raw value of [`Self::base_array_layer`]
    pub fn set_base_array_layer(&mut self, value: u32) -> &mut Self {
        self.base_array_layer = value;
        self
    }
    ///Sets the raw value of [`Self::image_view_binding`]
    pub fn set_image_view_binding(&mut self, value: crate::vulkan1_0::ImageView) -> &mut Self {
        self.image_view_binding = value;
        self
    }
}
///[VkVideoReferenceSlotKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoReferenceSlotKHR.html) - Structure specifying the reference picture slot
///# C Specifications
///The [`VideoReferenceSlotKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoReferenceSlotKHR {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    int8_t                              slotIndex;
///    const VkVideoPictureResourceKHR*    pPictureResource;
///} VkVideoReferenceSlotKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`slot_index`] is the unique reference slot index used for the encode or decode operation.
/// - [`picture_resource`] is a pointer to a [`VideoPictureResourceKHR`] structure describing the
///   picture resource bound to this slot index.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264DpbSlotInfoEXT`] or
///   [`VideoDecodeH265DpbSlotInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`picture_resource`]**must** be a valid pointer to a valid [`VideoPictureResourceKHR`]
///   structure
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoDecodeInfoKHR`]
/// - [`VideoEncodeInfoKHR`]
/// - [`VideoPictureResourceKHR`]
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
pub struct VideoReferenceSlotKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`slot_index`] is the unique reference slot index used for the encode
    ///or decode operation.
    slot_index: i8,
    ///[`picture_resource`] is a pointer to a [`VideoPictureResourceKHR`]
    ///structure describing the picture resource bound to this slot index.
    picture_resource: *const VideoPictureResourceKHR<'lt>,
}
impl<'lt> Default for VideoReferenceSlotKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            slot_index: 0,
            picture_resource: std::ptr::null(),
        }
    }
}
impl<'lt> VideoReferenceSlotKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::slot_index`]
    pub fn slot_index_raw(&self) -> i8 {
        self.slot_index
    }
    ///Gets the raw value of [`Self::picture_resource`]
    pub fn picture_resource_raw(&self) -> *const VideoPictureResourceKHR<'lt> {
        self.picture_resource
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::slot_index`]
    pub fn set_slot_index_raw(&mut self, value: i8) -> &mut Self {
        self.slot_index = value;
        self
    }
    ///Sets the raw value of [`Self::picture_resource`]
    pub fn set_picture_resource_raw(&mut self, value: *const VideoPictureResourceKHR<'lt>) -> &mut Self {
        self.picture_resource = value;
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
    ///Gets the value of [`Self::slot_index`]
    pub fn slot_index(&self) -> i8 {
        self.slot_index
    }
    ///Gets the value of [`Self::picture_resource`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn picture_resource(&self) -> &VideoPictureResourceKHR<'lt> {
        &*self.picture_resource
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::slot_index`]
    pub fn slot_index_mut(&mut self) -> &mut i8 {
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
    ///Sets the raw value of [`Self::slot_index`]
    pub fn set_slot_index(&mut self, value: i8) -> &mut Self {
        self.slot_index = value;
        self
    }
    ///Sets the raw value of [`Self::picture_resource`]
    pub fn set_picture_resource(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoPictureResourceKHR<'lt>,
    ) -> &mut Self {
        self.picture_resource = value as *const _;
        self
    }
}
///[VkVideoSessionCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionCreateInfoKHR.html) - Structure specifying parameters of a newly created video decode session
///# C Specifications
///The [`VideoSessionCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionCreateInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    uint32_t                        queueFamilyIndex;
///    VkVideoSessionCreateFlagsKHR    flags;
///    const VkVideoProfileKHR*        pVideoProfile;
///    VkFormat                        pictureFormat;
///    VkExtent2D                      maxCodedExtent;
///    VkFormat                        referencePicturesFormat;
///    uint32_t                        maxReferencePicturesSlotsCount;
///    uint32_t                        maxReferencePicturesActiveCount;
///} VkVideoSessionCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_index`] is the queue family of the created video session.
/// - [`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`] specifying creation flags.
/// - [`video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
/// - [`picture_format`] is the format of the [image views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views)
///   representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture)
///   or encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture)
///   pictures.
/// - [`max_coded_extent`] is the maximum width and height of the coded pictures that this instance
///   will be able to support.
/// - [`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb)
///   image views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
/// - [`max_reference_pictures_slots_count`] is the maximum number of [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   that can be activated with associated [Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources)
///   for the created video session.
/// - [`max_reference_pictures_active_count`] is the maximum number of active [DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot)
///   that can be used as Dpb or Reconstructed [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   within a single decode or encode operation for the created video session.
///# Description
///Valid Usage
/// - [`video_profile`]**must** be a pointer to a valid [`VideoProfileKHR`] structure whose
///   [`p_next`] chain **must** include a valid codec-specific profile structure
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_slots_count`]**must** be set to a value bigger than `0`
/// - [`max_reference_pictures_slots_count`]**cannot** exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_slots_count`]
/// - If [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture)
///   are required for use with the created video session, the
///   [`max_reference_pictures_active_count`]**must** be set to a value bigger than `0`
/// - [`max_reference_pictures_active_count`]**cannot** exceed the implementation reported
///   [`VideoCapabilitiesKHR`]::[`max_reference_pictures_active_count`]
/// - [`max_reference_pictures_active_count`]**cannot** exceed the
///   [`max_reference_pictures_slots_count`]
/// - [`max_coded_extent`]**cannot** be smaller than [`VideoCapabilitiesKHR::min_extent`] and bigger
///   than [`VideoCapabilitiesKHR::max_extent`]
/// - [`reference_pictures_format`]**must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DPB_BIT_KHR` or `VK_IMAGE_USAGE_VIDEO_ENCODE_DPB_BIT_KHR`
///   depending on the session codec operation
/// - [`picture_format`] for decode output **must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_DECODE_DST_BIT_KHR`
/// - [`picture_format`] targeting encode operations **must** be one of the supported formats in
///   [`VideoFormatPropertiesKHR`]`format` returned by the
///   [`GetPhysicalDeviceVideoFormatPropertiesKHR`] when the
///   [`PhysicalDeviceVideoFormatInfoKHR`]`imageUsage` contains
///   `VK_IMAGE_USAGE_VIDEO_ENCODE_SRC_BIT_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoDecodeH264SessionCreateInfoEXT`],
///   [`VideoDecodeH265SessionCreateInfoEXT`], [`VideoEncodeH264SessionCreateInfoEXT`], or
///   [`VideoEncodeH265SessionCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoSessionCreateFlagBitsKHR`] values
/// - [`video_profile`]**must** be a valid pointer to a valid [`VideoProfileKHR`] structure
/// - [`picture_format`]**must** be a valid [`Format`] value
/// - [`reference_pictures_format`]**must** be a valid [`Format`] value
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`Extent2D`]
/// - [`Format`]
/// - [`StructureType`]
/// - [`VideoProfileKHR`]
/// - [`VideoSessionCreateFlagsKHR`]
/// - [`CreateVideoSessionKHR`]
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
pub struct VideoSessionCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family of the created video session.
    queue_family_index: u32,
    ///[`flags`] is a bitmask of [`VideoSessionCreateFlagBitsKHR`]
    ///specifying creation flags.
    flags: VideoSessionCreateFlagsKHR,
    ///[`video_profile`] is a pointer to a [`VideoProfileKHR`] structure.
    video_profile: *const VideoProfileKHR<'lt>,
    ///[`picture_format`] is the format of the [image
    ///views](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views) representing decoded [Output](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#decoded-output-picture) or
    ///encoded [Input](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#input-encode-picture) pictures.
    picture_format: Format,
    ///[`max_coded_extent`] is the maximum width and height of the coded
    ///pictures that this instance will be able to support.
    max_coded_extent: Extent2D,
    ///[`reference_pictures_format`] is the format of the [DPB](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb) image
    ///views representing the [Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture).
    reference_pictures_format: Format,
    ///[`max_reference_pictures_slots_count`] is the maximum number of
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be activated with associated
    ///[Video Picture Resources](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#video-picture-resources) for the created
    ///video session.
    max_reference_pictures_slots_count: u32,
    ///[`max_reference_pictures_active_count`] is the maximum number of active
    ///[DPB Slots](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#dpb-slot) that can be used as Dpb or Reconstructed
    ///[Reference Pictures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#reference-picture) within a single decode or
    ///encode operation for the created video session.
    max_reference_pictures_active_count: u32,
}
impl<'lt> Default for VideoSessionCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            queue_family_index: 0,
            flags: Default::default(),
            video_profile: std::ptr::null(),
            picture_format: Default::default(),
            max_coded_extent: Default::default(),
            reference_pictures_format: Default::default(),
            max_reference_pictures_slots_count: 0,
            max_reference_pictures_active_count: 0,
        }
    }
}
impl<'lt> VideoSessionCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::queue_family_index`]
    pub fn queue_family_index_raw(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the raw value of [`Self::video_profile`]
    pub fn video_profile_raw(&self) -> *const VideoProfileKHR<'lt> {
        self.video_profile
    }
    ///Gets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_raw(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_raw(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index`]
    pub fn set_queue_family_index_raw(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::video_profile`]
    pub fn set_video_profile_raw(&mut self, value: *const VideoProfileKHR<'lt>) -> &mut Self {
        self.video_profile = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
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
    ///Gets the value of [`Self::queue_family_index`]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> VideoSessionCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::video_profile`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn video_profile(&self) -> &VideoProfileKHR<'lt> {
        &*self.video_profile
    }
    ///Gets the value of [`Self::picture_format`]
    pub fn picture_format(&self) -> Format {
        self.picture_format
    }
    ///Gets the value of [`Self::max_coded_extent`]
    pub fn max_coded_extent(&self) -> Extent2D {
        self.max_coded_extent
    }
    ///Gets the value of [`Self::reference_pictures_format`]
    pub fn reference_pictures_format(&self) -> Format {
        self.reference_pictures_format
    }
    ///Gets the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count(&self) -> u32 {
        self.max_reference_pictures_slots_count
    }
    ///Gets the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count(&self) -> u32 {
        self.max_reference_pictures_active_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index`]
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoSessionCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::picture_format`]
    pub fn picture_format_mut(&mut self) -> &mut Format {
        &mut self.picture_format
    }
    ///Gets a mutable reference to the value of [`Self::max_coded_extent`]
    pub fn max_coded_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_coded_extent
    }
    ///Gets a mutable reference to the value of [`Self::reference_pictures_format`]
    pub fn reference_pictures_format_mut(&mut self) -> &mut Format {
        &mut self.reference_pictures_format
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_slots_count`]
    pub fn max_reference_pictures_slots_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_reference_pictures_active_count`]
    pub fn max_reference_pictures_active_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::queue_family_index`]
    pub fn set_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoSessionCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::video_profile`]
    pub fn set_video_profile(
        &mut self,
        value: &'lt crate::extensions::khr_video_queue::VideoProfileKHR<'lt>,
    ) -> &mut Self {
        self.video_profile = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::picture_format`]
    pub fn set_picture_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.picture_format = value;
        self
    }
    ///Sets the raw value of [`Self::max_coded_extent`]
    pub fn set_max_coded_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_coded_extent = value;
        self
    }
    ///Sets the raw value of [`Self::reference_pictures_format`]
    pub fn set_reference_pictures_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.reference_pictures_format = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_slots_count`]
    pub fn set_max_reference_pictures_slots_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_slots_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_reference_pictures_active_count`]
    pub fn set_max_reference_pictures_active_count(&mut self, value: u32) -> &mut Self {
        self.max_reference_pictures_active_count = value;
        self
    }
}
///[VkVideoSessionParametersCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersCreateInfoKHR.html) - Structure to set video session parameters
///# C Specifications
///The [`VideoSessionParametersCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionParametersCreateInfoKHR {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkVideoSessionParametersKHR    videoSessionParametersTemplate;
///    VkVideoSessionKHR              videoSession;
///} VkVideoSessionParametersCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`video_session_parameters_template`] is [`crate::utils::Handle::null`] or a valid handle to a
///   [`VideoSessionParametersKHR`] object. If this parameter represents a valid handle, then the
///   underlying Video Session Parameters object will be used as a template for constructing the new
///   video session parameters object. All of the template object’s current parameters will be
///   inherited by the new object in such a case. Optionally, some of the template’s parameters can
///   be updated or new parameters added to the newly constructed object via the extension-specific
///   parameters.
/// - [`video_session`] is the video session object against which the video session parameters
///   object is going to be created.
///# Description
///Valid Usage
/// - If [`video_session_parameters_template`] represents a valid handle, it **must** have been
///   created against [`video_session`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersCreateInfoEXT`],
///   [`VideoDecodeH265SessionParametersCreateInfoEXT`],
///   [`VideoEncodeH264SessionParametersCreateInfoEXT`], or
///   [`VideoEncodeH265SessionParametersCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - If [`video_session_parameters_template`] is not [`crate::utils::Handle::null`],
///   [`video_session_parameters_template`]**must** be a valid [`VideoSessionParametersKHR`] handle
/// - [`video_session`]**must** be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters_template`] is a valid handle, it **must** have been created,
///   allocated, or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters_template`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`CreateVideoSessionParametersKHR`]
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
pub struct VideoSessionParametersCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`video_session_parameters_template`] is [`crate::utils::Handle::null`] or a valid
    ///handle to a [`VideoSessionParametersKHR`] object.
    ///If this parameter represents a valid handle, then the underlying Video
    ///Session Parameters object will be used as a template for constructing
    ///the new video session parameters object.
    ///All of the template object’s current parameters will be inherited by the
    ///new object in such a case.
    ///Optionally, some of the template’s parameters can be updated or new
    ///parameters added to the newly constructed object via the
    ///extension-specific parameters.
    video_session_parameters_template: VideoSessionParametersKHR,
    ///[`video_session`] is the video session object against which the video
    ///session parameters object is going to be created.
    video_session: VideoSessionKHR,
}
impl<'lt> Default for VideoSessionParametersCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            video_session_parameters_template: Default::default(),
            video_session: Default::default(),
        }
    }
}
impl<'lt> VideoSessionParametersCreateInfoKHR<'lt> {
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
    ///Gets the value of [`Self::video_session_parameters_template`]
    pub fn video_session_parameters_template(&self) -> VideoSessionParametersKHR {
        self.video_session_parameters_template
    }
    ///Gets the value of [`Self::video_session`]
    pub fn video_session(&self) -> VideoSessionKHR {
        self.video_session
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::video_session_parameters_template`]
    pub fn video_session_parameters_template_mut(&mut self) -> &mut VideoSessionParametersKHR {
        &mut self.video_session_parameters_template
    }
    ///Gets a mutable reference to the value of [`Self::video_session`]
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
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
    ///Sets the raw value of [`Self::video_session_parameters_template`]
    pub fn set_video_session_parameters_template(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> &mut Self {
        self.video_session_parameters_template = value;
        self
    }
    ///Sets the raw value of [`Self::video_session`]
    pub fn set_video_session(&mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> &mut Self {
        self.video_session = value;
        self
    }
}
///[VkVideoSessionParametersUpdateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersUpdateInfoKHR.html) - Structure to update video session parameters
///# C Specifications
///The [`VideoSessionParametersUpdateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoSessionParametersUpdateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           updateSequenceCount;
///} VkVideoSessionParametersUpdateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`update_sequence_count`] is the sequence number of the object update with parameters,
///   starting from `1` and incrementing the value by one with each subsequent update.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`VideoDecodeH264SessionParametersAddInfoEXT`],
///   [`VideoDecodeH265SessionParametersAddInfoEXT`],
///   [`VideoEncodeH264SessionParametersAddInfoEXT`], or
///   [`VideoEncodeH265SessionParametersAddInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`UpdateVideoSessionParametersKHR`]
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
pub struct VideoSessionParametersUpdateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`update_sequence_count`] is the sequence number of the object update
    ///with parameters, starting from `1` and incrementing the value by one
    ///with each subsequent update.
    update_sequence_count: u32,
}
impl<'lt> Default for VideoSessionParametersUpdateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            update_sequence_count: 0,
        }
    }
}
impl<'lt> VideoSessionParametersUpdateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::update_sequence_count`]
    pub fn update_sequence_count_raw(&self) -> u32 {
        self.update_sequence_count
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::update_sequence_count`]
    pub fn set_update_sequence_count_raw(&mut self, value: u32) -> &mut Self {
        self.update_sequence_count = value;
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
    ///Gets the value of [`Self::update_sequence_count`]
    pub fn update_sequence_count(&self) -> u32 {
        self.update_sequence_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::update_sequence_count`]
    pub fn update_sequence_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::update_sequence_count`]
    pub fn set_update_sequence_count(&mut self, value: u32) -> &mut Self {
        self.update_sequence_count = value;
        self
    }
}
///[VkVideoBeginCodingInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoBeginCodingInfoKHR.html) - Structure specifying parameters of decode starts
///# C Specifications
///The [`VideoBeginCodingInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoBeginCodingInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkVideoBeginCodingFlagsKHR            flags;
///    VkVideoCodingQualityPresetFlagsKHR    codecQualityPreset;
///    VkVideoSessionKHR                     videoSession;
///    VkVideoSessionParametersKHR           videoSessionParameters;
///    uint32_t                              referenceSlotCount;
///    const VkVideoReferenceSlotKHR*        pReferenceSlots;
///} VkVideoBeginCodingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`codec_quality_preset`] is a bitmask of [`VideoCodingQualityPresetFlagBitsKHR`] specifying
///   the Video Decode or Encode quality preset.
/// - [`video_session`] is the video session object to be bound for the processing of the video
///   commands.
/// - [`video_session_parameters`] is [`crate::utils::Handle::null`] or a handle of a
///   [`VideoSessionParametersKHR`] object to be used for the processing of the video commands. If
///   [`crate::utils::Handle::null`], then no video session parameters apply to this command buffer
///   context.
/// - [`reference_slot_count`] is the number of reference slot entries provided in
///   [`reference_slots`].
/// - [`reference_slots`] is a pointer to an array of [`VideoReferenceSlotKHR`] structures
///   specifying reference slots, used within the video command context between this
///   [`CmdBeginVideoCodingKHR`] command and the [`CmdEndVideoCodingKHR`] commmand that follows.
///   Each reference slot provides a slot index and the [`VideoPictureResourceKHR`] specifying the
///   reference picture resource bound to this slot index. A slot index **must** not appear more
///   than once in [`reference_slots`] in a given command.
///# Description
///Valid Usage
/// - [`VideoBeginCodingInfoKHR`]::[`reference_slot_count`]**must** not exceed the value specified
///   in [`VideoSessionCreateInfoKHR::max_reference_pictures_slots_count`] when creating the video
///   session object that is being provided in [`video_session`]
/// - If [`video_session_parameters`] is not [`crate::utils::Handle::null`], it **must** have been
///   created using [`video_session`] as a parent object
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`codec_quality_preset`]**must** be a valid combination of
///   [`VideoCodingQualityPresetFlagBitsKHR`] values
/// - [`codec_quality_preset`]**must** not be `0`
/// - [`video_session`]**must** be a valid [`VideoSessionKHR`] handle
/// - If [`video_session_parameters`] is not [`crate::utils::Handle::null`],
///   [`video_session_parameters`]**must** be a valid [`VideoSessionParametersKHR`] handle
/// - If [`reference_slot_count`] is not `0`, [`reference_slots`]**must** be a valid pointer to an
///   array of [`reference_slot_count`] valid [`VideoReferenceSlotKHR`] structures
/// - If [`video_session_parameters`] is a valid handle, it **must** have been created, allocated,
///   or retrieved from [`video_session`]
/// - Both of [`video_session`], and [`video_session_parameters`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoBeginCodingFlagsKHR`]
/// - [`VideoCodingQualityPresetFlagsKHR`]
/// - [`VideoReferenceSlotKHR`]
/// - [`VideoSessionKHR`]
/// - [`VideoSessionParametersKHR`]
/// - [`CmdBeginVideoCodingKHR`]
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
pub struct VideoBeginCodingInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoBeginCodingFlagsKHR,
    ///[`codec_quality_preset`] is a bitmask of
    ///[`VideoCodingQualityPresetFlagBitsKHR`] specifying the Video Decode
    ///or Encode quality preset.
    codec_quality_preset: VideoCodingQualityPresetFlagsKHR,
    ///[`video_session`] is the video session object to be bound for the
    ///processing of the video commands.
    video_session: VideoSessionKHR,
    ///[`video_session_parameters`] is [`crate::utils::Handle::null`] or a handle of a
    ///[`VideoSessionParametersKHR`] object to be used for the processing
    ///of the video commands.
    ///If [`crate::utils::Handle::null`], then no video session parameters apply to this
    ///command buffer context.
    video_session_parameters: VideoSessionParametersKHR,
    ///[`reference_slot_count`] is the number of reference slot entries
    ///provided in [`reference_slots`].
    reference_slot_count: u32,
    ///[`reference_slots`] is a pointer to an array of
    ///[`VideoReferenceSlotKHR`] structures specifying reference slots,
    ///used within the video command context between this
    ///[`CmdBeginVideoCodingKHR`] command and the
    ///[`CmdEndVideoCodingKHR`] commmand that follows.
    ///Each reference slot provides a slot index and the
    ///[`VideoPictureResourceKHR`] specifying the reference picture
    ///resource bound to this slot index.
    ///A slot index **must** not appear more than once in [`reference_slots`] in
    ///a given command.
    reference_slots: *const VideoReferenceSlotKHR<'lt>,
}
impl<'lt> Default for VideoBeginCodingInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            codec_quality_preset: Default::default(),
            video_session: Default::default(),
            video_session_parameters: Default::default(),
            reference_slot_count: 0,
            reference_slots: std::ptr::null(),
        }
    }
}
impl<'lt> VideoBeginCodingInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_raw(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the raw value of [`Self::reference_slots`]
    pub fn reference_slots_raw(&self) -> *const VideoReferenceSlotKHR<'lt> {
        self.reference_slots
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count_raw(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots_raw(&mut self, value: *const VideoReferenceSlotKHR<'lt>) -> &mut Self {
        self.reference_slots = value;
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
    pub fn flags(&self) -> VideoBeginCodingFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::codec_quality_preset`]
    pub fn codec_quality_preset(&self) -> VideoCodingQualityPresetFlagsKHR {
        self.codec_quality_preset
    }
    ///Gets the value of [`Self::video_session`]
    pub fn video_session(&self) -> VideoSessionKHR {
        self.video_session
    }
    ///Gets the value of [`Self::video_session_parameters`]
    pub fn video_session_parameters(&self) -> VideoSessionParametersKHR {
        self.video_session_parameters
    }
    ///Gets the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count(&self) -> u32 {
        self.reference_slot_count
    }
    ///Gets the value of [`Self::reference_slots`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn reference_slots(&self) -> &[VideoReferenceSlotKHR<'lt>] {
        std::slice::from_raw_parts(self.reference_slots, self.reference_slot_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoBeginCodingFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::codec_quality_preset`]
    pub fn codec_quality_preset_mut(&mut self) -> &mut VideoCodingQualityPresetFlagsKHR {
        &mut self.codec_quality_preset
    }
    ///Gets a mutable reference to the value of [`Self::video_session`]
    pub fn video_session_mut(&mut self) -> &mut VideoSessionKHR {
        &mut self.video_session
    }
    ///Gets a mutable reference to the value of [`Self::video_session_parameters`]
    pub fn video_session_parameters_mut(&mut self) -> &mut VideoSessionParametersKHR {
        &mut self.video_session_parameters
    }
    ///Gets a mutable reference to the value of [`Self::reference_slot_count`]
    pub fn reference_slot_count_mut(&mut self) -> &mut u32 {
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoBeginCodingFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::codec_quality_preset`]
    pub fn set_codec_quality_preset(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoCodingQualityPresetFlagsKHR,
    ) -> &mut Self {
        self.codec_quality_preset = value;
        self
    }
    ///Sets the raw value of [`Self::video_session`]
    pub fn set_video_session(&mut self, value: crate::extensions::khr_video_queue::VideoSessionKHR) -> &mut Self {
        self.video_session = value;
        self
    }
    ///Sets the raw value of [`Self::video_session_parameters`]
    pub fn set_video_session_parameters(
        &mut self,
        value: crate::extensions::khr_video_queue::VideoSessionParametersKHR,
    ) -> &mut Self {
        self.video_session_parameters = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slot_count`]
    pub fn set_reference_slot_count(&mut self, value: u32) -> &mut Self {
        self.reference_slot_count = value;
        self
    }
    ///Sets the raw value of [`Self::reference_slots`]
    pub fn set_reference_slots(
        &mut self,
        value: &'lt [crate::extensions::khr_video_queue::VideoReferenceSlotKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.reference_slots = value.as_ptr();
        self.reference_slot_count = len_;
        self
    }
}
///[VkVideoEndCodingInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoEndCodingInfoKHR.html) - Structure specifying the end of decode encode commands sequence
///# C Specifications
///The [`VideoEndCodingInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoEndCodingInfoKHR {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkVideoEndCodingFlagsKHR    flags;
///} VkVideoEndCodingInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoEndCodingFlagsKHR`]
/// - [`CmdEndVideoCodingKHR`]
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
pub struct VideoEndCodingInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: VideoEndCodingFlagsKHR,
}
impl<'lt> Default for VideoEndCodingInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoEndCodingInfoKHR<'lt> {
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
    pub fn flags(&self) -> VideoEndCodingFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoEndCodingFlagsKHR {
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoEndCodingFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkVideoCodingControlInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoCodingControlInfoKHR.html) - Structure specifying parameters of coding control
///# C Specifications
///The [`VideoCodingControlInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_video_queue
///typedef struct VkVideoCodingControlInfoKHR {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkVideoCodingControlFlagsKHR    flags;
///} VkVideoCodingControlInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`] specifying control flags.
///# Description
///Valid Usage
/// - The first command buffer submitted for a newly created video session **must** set the
///   `VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR` bit in [`VideoCodingControlInfoKHR`]::[`flags`] to
///   reset the session device context before any video decode or encode operations are performed on
///   the session.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`VideoEncodeRateControlInfoKHR`] or
///   [`VideoEncodeRateControlLayerInfoKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`VideoCodingControlFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`StructureType`]
/// - [`VideoCodingControlFlagsKHR`]
/// - [`CmdControlVideoCodingKHR`]
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
pub struct VideoCodingControlInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`VideoCodingControlFlagsKHR`]
    ///specifying control flags.
    flags: VideoCodingControlFlagsKHR,
}
impl<'lt> Default for VideoCodingControlInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> VideoCodingControlInfoKHR<'lt> {
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
    pub fn flags(&self) -> VideoCodingControlFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut VideoCodingControlFlagsKHR {
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
    pub fn set_flags(&mut self, value: crate::extensions::khr_video_queue::VideoCodingControlFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionKHR.html) - Opaque handle to a video session object
///# C Specifications
///Video session objects are abstracted and represented by
///[`VideoSessionKHR`] handles:
///```c
///// Provided by VK_KHR_video_queue
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkVideoSessionKHR)
///```
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoSessionParametersCreateInfoKHR`]
/// - [`BindVideoSessionMemoryKHR`]
/// - [`CreateVideoSessionKHR`]
/// - [`DestroyVideoSessionKHR`]
/// - [`GetVideoSessionMemoryRequirementsKHR`]
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
pub struct VideoSessionKHR(pub u64);
impl VideoSessionKHR {
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
unsafe impl Send for VideoSessionKHR {}
impl Default for VideoSessionKHR {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for VideoSessionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for VideoSessionKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
///[VkVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkVideoSessionParametersKHR.html) - Opaque handle to a video video session parameters object
///# C Specifications
///Video session parameter objects are represented by
///[`VideoSessionParametersKHR`] handles:
///```c
///// Provided by VK_KHR_video_queue
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkVideoSessionParametersKHR)
///```
///# Related
/// - [`VK_KHR_video_queue`]
/// - [`VideoBeginCodingInfoKHR`]
/// - [`VideoSessionParametersCreateInfoKHR`]
/// - [`CreateVideoSessionParametersKHR`]
/// - [`DestroyVideoSessionParametersKHR`]
/// - [`UpdateVideoSessionParametersKHR`]
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
pub struct VideoSessionParametersKHR(pub u64);
impl VideoSessionParametersKHR {
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
unsafe impl Send for VideoSessionParametersKHR {}
impl Default for VideoSessionParametersKHR {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for VideoSessionParametersKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for VideoSessionParametersKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
