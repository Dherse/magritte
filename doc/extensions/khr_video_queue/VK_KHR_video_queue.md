[VK_KHR_video_queue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_queue.html) - device extension

# Registered extension number
24

# Revision
3

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`
- Requires `[`khr_sampler_ycbcr_conversion`]`
-  **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**

# Contacts
- Tony Zlatinski [tzlatinski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_video_queue] @tzlatinski%0A<<Here describe the issue or question you have about the VK_KHR_video_queue extension>>)

# New object types
- [`VideoSessionKHR`]
- [`VideoSessionParametersKHR`]

# New commands
- [`bind_video_session_memory_khr`]
- [`cmd_begin_video_coding_khr`]
- [`cmd_control_video_coding_khr`]
- [`cmd_end_video_coding_khr`]
- [`create_video_session_khr`]
- [`create_video_session_parameters_khr`]
- [`destroy_video_session_khr`]
- [`destroy_video_session_parameters_khr`]
- [`get_physical_device_video_capabilities_khr`]
- [`get_physical_device_video_format_properties_khr`]
- [`get_video_session_memory_requirements_khr`]
- [`update_video_session_parameters_khr`]

# New structures
- [`PhysicalDeviceVideoFormatInfoKHR`]
- [`VideoBeginCodingInfoKHR`]
- [`VideoBindMemoryKHR`]
- [`VideoCapabilitiesKHR`]
- [`VideoCodingControlInfoKHR`]
- [`VideoEndCodingInfoKHR`]
- [`VideoFormatPropertiesKHR`]
- [`VideoGetMemoryPropertiesKHR`]
- [`VideoPictureResourceKHR`]
- [`VideoReferenceSlotKHR`]
- [`VideoSessionCreateInfoKHR`]
- [`VideoSessionParametersCreateInfoKHR`]
- [`VideoSessionParametersUpdateInfoKHR`]
- Extending [`FormatProperties2`], [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  - [`VideoProfilesKHR`] 
- Extending [`QueryPoolCreateInfo`], [`FormatProperties2`], [`ImageCreateInfo`], [`ImageViewCreateInfo`], [`BufferCreateInfo`]:  - [`VideoProfileKHR`] 
- Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyQueryResultStatusProperties2KHR`]  - [`VideoQueueFamilyProperties2KHR`]

# New enums
- [`QueryResultStatusKHR`]
- [`VideoCapabilityFlagBitsKHR`]
- [`VideoChromaSubsamplingFlagBitsKHR`]
- [`VideoCodecOperationFlagBitsKHR`]
- [`VideoCodingControlFlagBitsKHR`]
- [`VideoCodingQualityPresetFlagBitsKHR`]
- [`VideoComponentBitDepthFlagBitsKHR`]
- [`VideoSessionCreateFlagBitsKHR`]

# New bitmasks
- [`VideoBeginCodingFlagsKHR`]
- [VkVideoCapabilityFlagsKHR]()
- [VkVideoChromaSubsamplingFlagsKHR]()
- [VkVideoCodecOperationFlagsKHR]()
- [VkVideoCodingControlFlagsKHR]()
- [VkVideoCodingQualityPresetFlagsKHR]()
- [VkVideoComponentBitDepthFlagsKHR]()
- [`VideoEndCodingFlagsKHR`]
- [VkVideoSessionCreateFlagsKHR]()

# New constants
- `VK_KHR_VIDEO_QUEUE_EXTENSION_NAME`
- `VK_KHR_VIDEO_QUEUE_SPEC_VERSION`
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_VIDEO_SESSION_KHR`  - `VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR` 
- Extending [`QueryResultFlagBits`]:  - `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` 
- Extending [`QueryType`]:  - `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR`  - `VK_STRUCTURE_TYPE_QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_BEGIN_CODING_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_BIND_MEMORY_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_CAPABILITIES_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_CODING_CONTROL_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_END_CODING_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_FORMAT_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_GET_MEMORY_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_PICTURE_RESOURCE_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_PROFILES_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_PROFILE_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_REFERENCE_SLOT_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_SESSION_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR`

# Version history
- Revision 0.1, 2019-11-21 (Tony Zlatinski)  - Initial draft 
- Revision 0.2, 2019-11-27 (Tony Zlatinski)  - Make vulkan video core common between decode and encode 
- Revision 1, March 29 2021 (Tony Zlatinski)  - Spec and API updates. 
- Revision 2, August 1 2021 (Srinath Kumarapuram)  - Rename `VkVideoCapabilitiesFlagBitsKHR` to [`VideoCapabilityFlagBitsKHR`] (along with the names of enumerants it defines) and `VkVideoCapabilitiesFlagsKHR` to `VkVideoCapabilityFlagsKHR`, following Vulkan naming conventions. 
- Revision 3, 2022-03-16 (Ahmed Abdelkhalek)  - Relocate Std header version reporting/requesting from codec-operation specific extensions to this extension.  - Make Std header versions codec-operation specific instead of only codec-specific.

# Other information
* 2022-03-16
* No known IP claims.
*   - Ahmed Abdelkhalek, AMD  - George Hao, AMD  - Jake Beju, AMD  - Piers Daniell, NVIDIA  - Srinath Kumarapuram, NVIDIA  - Tobias Hector, AMD  - Tony Zlatinski, NVIDIA

# Related
- [`PhysicalDeviceVideoFormatInfoKHR`]
- [`QueryResultStatusKHR`]
- [`QueueFamilyQueryResultStatusProperties2KHR`]
- [`VideoBeginCodingFlagsKHR`]
- [`VideoBeginCodingInfoKHR`]
- [`VideoBindMemoryKHR`]
- [`VideoCapabilitiesKHR`]
- [`VideoCapabilityFlagBitsKHR`]
- [VkVideoCapabilityFlagsKHR]()
- [`VideoChromaSubsamplingFlagBitsKHR`]
- [VkVideoChromaSubsamplingFlagsKHR]()
- [`VideoCodecOperationFlagBitsKHR`]
- [VkVideoCodecOperationFlagsKHR]()
- [`VideoCodingControlFlagBitsKHR`]
- [VkVideoCodingControlFlagsKHR]()
- [`VideoCodingControlInfoKHR`]
- [`VideoCodingQualityPresetFlagBitsKHR`]
- [VkVideoCodingQualityPresetFlagsKHR]()
- [`VideoComponentBitDepthFlagBitsKHR`]
- [VkVideoComponentBitDepthFlagsKHR]()
- [`VideoEndCodingFlagsKHR`]
- [`VideoEndCodingInfoKHR`]
- [`VideoFormatPropertiesKHR`]
- [`VideoGetMemoryPropertiesKHR`]
- [`VideoPictureResourceKHR`]
- [`VideoProfileKHR`]
- [`VideoProfilesKHR`]
- [`VideoQueueFamilyProperties2KHR`]
- [`VideoReferenceSlotKHR`]
- [`VideoSessionCreateFlagBitsKHR`]
- [VkVideoSessionCreateFlagsKHR]()
- [`VideoSessionCreateInfoKHR`]
- [`VideoSessionKHR`]
- [`VideoSessionParametersCreateInfoKHR`]
- [`VideoSessionParametersKHR`]
- [`VideoSessionParametersUpdateInfoKHR`]
- [`bind_video_session_memory_khr`]
- [`cmd_begin_video_coding_khr`]
- [`cmd_control_video_coding_khr`]
- [`cmd_end_video_coding_khr`]
- [`create_video_session_khr`]
- [`create_video_session_parameters_khr`]
- [`destroy_video_session_khr`]
- [`destroy_video_session_parameters_khr`]
- [`get_physical_device_video_capabilities_khr`]
- [`get_physical_device_video_format_properties_khr`]
- [`get_video_session_memory_requirements_khr`]
- [`update_video_session_parameters_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        