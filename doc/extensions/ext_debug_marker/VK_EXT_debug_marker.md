[VK_EXT_debug_marker](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_marker.html) - device extension

# Description
The [`ext_debug_marker`] extension is a device extension.
It introduces concepts of object naming and tagging, for better tracking of
Vulkan objects, as well as additional commands for recording annotations of
named sections of a workload to aid organization and offline analysis in
external tools.

# Registered extension number
23

# Revision
4

# Dependencies
- Requires Vulkan 1.0
- Requires `[`ext_debug_report`]`

# Deprecation state
- *Promoted* to `[`ext_debug_utils`]` extension

# Contacts
- Baldur Karlsson [baldurk](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_debug_marker] @baldurk%0A<<Here describe the issue or question you have about the VK_EXT_debug_marker extension>>)

# New commands
- [`cmd_debug_marker_begin_ext`]
- [`cmd_debug_marker_end_ext`]
- [`cmd_debug_marker_insert_ext`]
- [`debug_marker_set_object_name_ext`]
- [`debug_marker_set_object_tag_ext`]

# New structures
- [`DebugMarkerMarkerInfoEXT`]
- [`DebugMarkerObjectNameInfoEXT`]
- [`DebugMarkerObjectTagInfoEXT`]

# New enums
- [`DebugReportObjectTypeEXT`]

# New constants
- `VK_EXT_DEBUG_MARKER_EXTENSION_NAME`
- `VK_EXT_DEBUG_MARKER_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`  - `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`  - `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT`

# Known issues & F.A.Q.
1) Should the tag or name for an object be specified using the `pNext`
parameter in the object’s `Vk*CreateInfo` structure? **RESOLVED** : No.
While this fits with other Vulkan patterns and would allow more type safety
and future proofing against future objects, it has notable downsides.
In particular passing the name at `Vk*CreateInfo` time does not allow
renaming, prevents late binding of naming information, and does not allow
naming of implicitly created objects such as queues and swapchain images.2) Should the command annotation functions [`cmd_debug_marker_begin_ext`]
and [`cmd_debug_marker_end_ext`] support the ability to specify a color? **RESOLVED** : Yes.
The functions have been expanded to take an optional color which can be used
at will by implementations consuming the command buffer annotations in their
visualisation.3) Should the functions added in this extension accept an extensible
structure as their parameter for a more flexible API, as opposed to direct
function parameters? If so, which functions? **RESOLVED** : Yes.
All functions have been modified to take a structure type with extensible
`pNext` pointer, to allow future extensions to add additional annotation
information in the same commands.

# Version history
- Revision 1, 2016-02-24 (Baldur Karlsson)  - Initial draft, based on LunarG marker spec 
- Revision 2, 2016-02-26 (Baldur Karlsson)  - Renamed Dbg to DebugMarker in function names  - Allow markers in secondary command buffers under certain circumstances  - Minor language tweaks and edits 
- Revision 3, 2016-04-23 (Baldur Karlsson)  - Reorganise spec layout to closer match desired organisation  - Added optional color to markers (both regions and inserted labels)  - Changed functions to take extensible structs instead of direct function parameters 
- Revision 4, 2017-01-31 (Baldur Karlsson)  - Added explicit dependency on VK_EXT_debug_report  - Moved definition of [`DebugReportObjectTypeEXT`] to debug report chapter.  - Fixed typo in dates in revision history

# Other information
* 2017-01-31
* No known IP claims.
*   - Baldur Karlsson  - Dan Ginsburg, Valve  - Jon Ashburn, LunarG  - Kyle Spagnoli, NVIDIA

# Related
- [`DebugMarkerMarkerInfoEXT`]
- [`DebugMarkerObjectNameInfoEXT`]
- [`DebugMarkerObjectTagInfoEXT`]
- [`DebugReportObjectTypeEXT`]
- [`cmd_debug_marker_begin_ext`]
- [`cmd_debug_marker_end_ext`]
- [`cmd_debug_marker_insert_ext`]
- [`debug_marker_set_object_name_ext`]
- [`debug_marker_set_object_tag_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        