use crate::vulkan1_0::{BaseInStructure, ObjectType, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_utils");
///[VkDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) - Specify parameters of a name to give to an object
///# C Specifications
///The [`DebugUtilsObjectNameInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsObjectNameInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkObjectType       objectType;
///    uint64_t           objectHandle;
///    const char*        pObjectName;
///} VkDebugUtilsObjectNameInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be named.
/// - [`object_name`] is either `NULL` or a null-terminated UTF-8 string specifying the name to
///   apply to [`object_handle`].
///# Description
///Applications **may** change the name associated with an object simply by
///calling [`SetDebugUtilsObjectNameEXT`] again with a new string.
///If [`object_name`] is either `NULL` or an empty string, then any
///previously set name is removed.Valid Usage
/// - If [`object_type`] is `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`]**must** not be
///   [`crate::utils::Handle::null`]
/// -    If [`object_type`] is not `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`]**must** be [`crate::utils::Handle::null`] or a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`ObjectType`] value
/// - If [`object_name`] is not `NULL`, [`object_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`SetDebugUtilsObjectNameEXT`]
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
pub struct DebugUtilsObjectNameInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    object_type: ObjectType,
    ///[`object_handle`] is the object to be named.
    object_handle: u64,
    ///[`object_name`] is either `NULL` or a null-terminated UTF-8 string
    ///specifying the name to apply to [`object_handle`].
    object_name: &'lt CStr,
}
impl<'lt> Default for DebugUtilsObjectNameInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: 0,
            object_name: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsObjectNameInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::object_handle`]
    pub fn object_handle_raw(&self) -> u64 {
        self.object_handle
    }
    ///Gets the raw value of [`Self::object_name`]
    pub fn object_name_raw(&self) -> &'lt CStr {
        self.object_name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::object_handle`]
    pub fn set_object_handle_raw(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn set_object_name_raw(&mut self, value: &'lt CStr) -> &mut Self {
        self.object_name = value;
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
    ///Gets the value of [`Self::object_type`]
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Gets the value of [`Self::object_handle`]
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Gets the value of [`Self::object_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn object_name(&self) -> &'lt CStr {
        self.object_name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::object_type`]
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object_handle`]
    pub fn object_handle_mut(&mut self) -> &mut u64 {
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
    ///Sets the raw value of [`Self::object_type`]
    pub fn set_object_type(&mut self, value: crate::vulkan1_0::ObjectType) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the raw value of [`Self::object_handle`]
    pub fn set_object_handle(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the raw value of [`Self::object_name`]
    pub fn set_object_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.object_name = value;
        self
    }
}
///[VkDebugUtilsObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) - Specify parameters of a tag to attach to an object
///# C Specifications
///The [`DebugUtilsObjectTagInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsObjectTagInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkObjectType       objectType;
///    uint64_t           objectHandle;
///    uint64_t           tagName;
///    size_t             tagSize;
///    const void*        pTag;
///} VkDebugUtilsObjectTagInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
/// - [`object_handle`] is the object to be tagged.
/// - [`tag_name`] is a numerical identifier of the tag.
/// - [`tag_size`] is the number of bytes of data to attach to the object.
/// - [`tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated
///   with the object.
///# Description
///The [`tag_name`] parameter gives a name or identifier to the type of data
///being tagged.
///This can be used by debugging layers to easily filter for only data that can
///be used by that implementation.Valid Usage
/// - [`object_type`]**must** not be `VK_OBJECT_TYPE_UNKNOWN`
/// - [`object_handle`]**must** be a valid Vulkan handle of the type associated with [`object_type`]
///   as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types)
///   table
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`object_type`]**must** be a valid [`ObjectType`] value
/// - [`tag`]**must** be a valid pointer to an array of [`tag_size`] bytes
/// - [`tag_size`]**must** be greater than `0`
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`ObjectType`]
/// - [`StructureType`]
/// - [`SetDebugUtilsObjectTagEXT`]
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
pub struct DebugUtilsObjectTagInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`object_type`] is a [`ObjectType`] specifying the type of the
    ///object to be named.
    object_type: ObjectType,
    ///[`object_handle`] is the object to be tagged.
    object_handle: u64,
    ///[`tag_name`] is a numerical identifier of the tag.
    tag_name: u64,
    ///[`tag_size`] is the number of bytes of data to attach to the object.
    tag_size: usize,
    ///[`tag`] is a pointer to an array of [`tag_size`] bytes containing
    ///the data to be associated with the object.
    tag: *const c_void,
}
impl<'lt> Default for DebugUtilsObjectTagInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: 0,
            tag_name: 0,
            tag_size: 0,
            tag: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsObjectTagInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::object_handle`]
    pub fn object_handle_raw(&self) -> u64 {
        self.object_handle
    }
    ///Gets the raw value of [`Self::tag_name`]
    pub fn tag_name_raw(&self) -> u64 {
        self.tag_name
    }
    ///Gets the raw value of [`Self::tag_size`]
    pub fn tag_size_raw(&self) -> usize {
        self.tag_size
    }
    ///Gets the raw value of [`Self::tag`]
    pub fn tag_raw(&self) -> *const c_void {
        self.tag
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::object_handle`]
    pub fn set_object_handle_raw(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the raw value of [`Self::tag_name`]
    pub fn set_tag_name_raw(&mut self, value: u64) -> &mut Self {
        self.tag_name = value;
        self
    }
    ///Sets the raw value of [`Self::tag_size`]
    pub fn set_tag_size_raw(&mut self, value: usize) -> &mut Self {
        self.tag_size = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn set_tag_raw(&mut self, value: *const c_void) -> &mut Self {
        self.tag = value;
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
    ///Gets the value of [`Self::object_type`]
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Gets the value of [`Self::object_handle`]
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Gets the value of [`Self::tag_name`]
    pub fn tag_name(&self) -> u64 {
        self.tag_name
    }
    ///Gets the value of [`Self::tag_size`]
    pub fn tag_size(&self) -> usize {
        self.tag_size
    }
    ///Gets the value of [`Self::tag`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn tag(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.tag, self.tag_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::object_type`]
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object_handle`]
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::tag_name`]
    pub fn tag_name_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::tag_size`]
    pub fn tag_size_mut(&mut self) -> &mut usize {
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
    ///Sets the raw value of [`Self::object_type`]
    pub fn set_object_type(&mut self, value: crate::vulkan1_0::ObjectType) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the raw value of [`Self::object_handle`]
    pub fn set_object_handle(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the raw value of [`Self::tag_name`]
    pub fn set_tag_name(&mut self, value: u64) -> &mut Self {
        self.tag_name = value;
        self
    }
    ///Sets the raw value of [`Self::tag_size`]
    pub fn set_tag_size(&mut self, value: usize) -> &mut Self {
        self.tag_size = value;
        self
    }
    ///Sets the raw value of [`Self::tag`]
    pub fn set_tag(&mut self, value: &'lt [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.tag = value.as_ptr();
        self.tag_size = len_;
        self
    }
}
///[VkDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html) - Specify parameters of a label region
///# C Specifications
///The [`DebugUtilsLabelEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsLabelEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    const char*        pLabelName;
///    float              color[4];
///} VkDebugUtilsLabelEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`label_name`] is a pointer to a null-terminated UTF-8 string containing the name of the
///   label.
/// - [`color`] is an optional RGBA color value that can be associated with the label. A particular
///   implementation **may** choose to ignore this color value. The values contain RGBA values in
///   order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it is
///   ignored.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`label_name`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessengerCallbackDataEXT`]
/// - [`StructureType`]
/// - [`CmdBeginDebugUtilsLabelEXT`]
/// - [`CmdInsertDebugUtilsLabelEXT`]
/// - [`QueueBeginDebugUtilsLabelEXT`]
/// - [`QueueInsertDebugUtilsLabelEXT`]
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
pub struct DebugUtilsLabelEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`label_name`] is a pointer to a null-terminated UTF-8 string
    ///containing the name of the label.
    label_name: &'lt CStr,
    ///[`color`] is an optional RGBA color value that can be associated with
    ///the label.
    ///A particular implementation **may** choose to ignore this color value.
    ///The values contain RGBA values in order, in the range 0.0 to 1.0.
    ///If all elements in [`color`] are set to 0.0 then it is ignored.
    color: [f32; 4],
}
impl<'lt> Default for DebugUtilsLabelEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            label_name: std::ptr::null(),
            color: [0.0; 4],
        }
    }
}
impl<'lt> DebugUtilsLabelEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::label_name`]
    pub fn label_name_raw(&self) -> &'lt CStr {
        self.label_name
    }
    ///Gets the raw value of [`Self::color`]
    pub fn color_raw(&self) -> [f32; 4] {
        self.color
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::label_name`]
    pub fn set_label_name_raw(&mut self, value: &'lt CStr) -> &mut Self {
        self.label_name = value;
        self
    }
    ///Sets the raw value of [`Self::color`]
    pub fn set_color_raw(&mut self, value: [f32; 4]) -> &mut Self {
        self.color = value;
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
    ///Gets the value of [`Self::label_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn label_name(&self) -> &'lt CStr {
        self.label_name
    }
    ///Gets the value of [`Self::color`]
    pub fn color(&self) -> &[f32; 4] {
        &getter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::color`]
    pub fn color_mut(&mut self) -> &mut [f32; 4] {
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
    ///Sets the raw value of [`Self::label_name`]
    pub fn set_label_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.label_name = value;
        self
    }
    ///Sets the raw value of [`Self::color`]
    pub fn set_color(&mut self, value: [f32; 4]) -> &mut Self {
        self.color = value;
        self
    }
}
///[VkDebugUtilsMessengerCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) - Structure specifying parameters of a newly created debug messenger
///# C Specifications
///The definition of [`DebugUtilsMessengerCreateInfoEXT`] is:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsMessengerCreateInfoEXT {
///    VkStructureType                         sType;
///    const void*                             pNext;
///    VkDebugUtilsMessengerCreateFlagsEXT     flags;
///    VkDebugUtilsMessageSeverityFlagsEXT     messageSeverity;
///    VkDebugUtilsMessageTypeFlagsEXT         messageType;
///    PFN_vkDebugUtilsMessengerCallbackEXT    pfnUserCallback;
///    void*                                   pUserData;
///} VkDebugUtilsMessengerCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`message_severity`] is a bitmask of [`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which
///   severity of event(s) will cause this callback to be called.
/// - [`message_type`] is a bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
///   event(s) will cause this callback to be called.
/// - [`pfn_user_callback`] is the application callback function to call.
/// - [`user_data`] is user data to be passed to the callback.
///# Description
///For each [`DebugUtilsMessengerEXT`] that is created the
///[`DebugUtilsMessengerCreateInfoEXT`]::[`message_severity`] and
///[`DebugUtilsMessengerCreateInfoEXT`]::[`message_type`] determine when
///that [`DebugUtilsMessengerCreateInfoEXT`]::[`pfn_user_callback`] is
///called.
///The process to determine if the user’s [`pfn_user_callback`] is triggered
///when an event occurs is as follows:
///0. The implementation will perform a bitwise AND of the event’s
/// [`DebugUtilsMessageSeverityFlagBitsEXT`] with the [`message_severity`] provided during creation
/// of the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
///2. The implementation will perform bitwise AND of the event’s
/// [`DebugUtilsMessageTypeFlagBitsEXT`] with the [`message_type`] provided during the creation of
/// the [`DebugUtilsMessengerEXT`] object.  0. If the value is 0, the message is skipped.
///4. The callback will trigger a debug message for the current event
///The callback will come directly from the component that detected the event,
///unless some other layer intercepts the calls for its own purposes (filter
///them in a different way, log to a system error log, etc.).An application **can** receive
/// multiple callbacks if multiple
///[`DebugUtilsMessengerEXT`] objects are created.
///A callback will always be executed in the same thread as the originating
///Vulkan call.A callback **can** be called from multiple threads simultaneously (if the
///application is making Vulkan calls from multiple threads).Valid Usage
/// - [`pfn_user_callback`]**must** be a valid [`PFNDebugUtilsMessengerCallbackEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`message_severity`]**must** be a valid combination of
///   [`DebugUtilsMessageSeverityFlagBitsEXT`] values
/// - [`message_severity`]**must** not be `0`
/// - [`message_type`]**must** be a valid combination of [`DebugUtilsMessageTypeFlagBitsEXT`] values
/// - [`message_type`]**must** not be `0`
/// - [`pfn_user_callback`]**must** be a valid [`PFNDebugUtilsMessengerCallbackEXT`] value
///# Related
/// - [`PFNDebugUtilsMessengerCallbackEXT`]
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsMessageSeverityFlagsEXT`]
/// - [`DebugUtilsMessageTypeFlagsEXT`]
/// - [`DebugUtilsMessengerCreateFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDebugUtilsMessengerEXT`]
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
pub struct DebugUtilsMessengerCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    flags: DebugUtilsMessengerCreateFlagsEXT,
    ///[`message_severity`] is a bitmask of
    ///[`DebugUtilsMessageSeverityFlagBitsEXT`] specifying which severity
    ///of event(s) will cause this callback to be called.
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    ///[`message_type`] is a bitmask of
    ///[`DebugUtilsMessageTypeFlagBitsEXT`] specifying which type of
    ///event(s) will cause this callback to be called.
    message_type: DebugUtilsMessageTypeFlagsEXT,
    ///[`pfn_user_callback`] is the application callback function to call.
    pfn_user_callback: PFNDebugUtilsMessengerCallbackEXT<'lt>,
    ///[`user_data`] is user data to be passed to the callback.
    user_data: *mut c_void,
}
impl<'lt> Default for DebugUtilsMessengerCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_severity: Default::default(),
            message_type: Default::default(),
            pfn_user_callback: Default::default(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DebugUtilsMessengerCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::user_data`]
    pub fn user_data_raw(&self) -> &*mut c_void {
        &self.user_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.user_data = value;
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
    pub fn flags(&self) -> DebugUtilsMessengerCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::message_severity`]
    pub fn message_severity(&self) -> DebugUtilsMessageSeverityFlagsEXT {
        self.message_severity
    }
    ///Gets the value of [`Self::message_type`]
    pub fn message_type(&self) -> DebugUtilsMessageTypeFlagsEXT {
        self.message_type
    }
    ///Gets the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback(&self) -> &PFNDebugUtilsMessengerCallbackEXT<'lt> {
        &self.pfn_user_callback
    }
    ///Gets the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data(&self) -> &c_void {
        &*self.user_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DebugUtilsMessengerCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::message_severity`]
    pub fn message_severity_mut(&mut self) -> &mut DebugUtilsMessageSeverityFlagsEXT {
        &mut self.message_severity
    }
    ///Gets a mutable reference to the value of [`Self::message_type`]
    pub fn message_type_mut(&mut self) -> &mut DebugUtilsMessageTypeFlagsEXT {
        &mut self.message_type
    }
    ///Gets a mutable reference to the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback_mut(&mut self) -> &mut PFNDebugUtilsMessengerCallbackEXT<'lt> {
        &mut self.pfn_user_callback
    }
    ///Gets a mutable reference to the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data_mut(&mut self) -> &mut c_void {
        &mut *self.user_data
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::message_severity`]
    pub fn set_message_severity(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    ) -> &mut Self {
        self.message_severity = value;
        self
    }
    ///Sets the raw value of [`Self::message_type`]
    pub fn set_message_type(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    ) -> &mut Self {
        self.message_type = value;
        self
    }
    ///Sets the raw value of [`Self::pfn_user_callback`]
    pub fn set_pfn_user_callback(
        &mut self,
        value: crate::extensions::ext_debug_utils::PFNDebugUtilsMessengerCallbackEXT<'lt>,
    ) -> &mut Self {
        self.pfn_user_callback = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
}
///[VkDebugUtilsMessengerCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) - Structure specifying parameters returned to the callback
///# C Specifications
///The definition of [`DebugUtilsMessengerCallbackDataEXT`] is:
///```c
///// Provided by VK_EXT_debug_utils
///typedef struct VkDebugUtilsMessengerCallbackDataEXT {
///    VkStructureType                              sType;
///    const void*                                  pNext;
///    VkDebugUtilsMessengerCallbackDataFlagsEXT    flags;
///    const char*                                  pMessageIdName;
///    int32_t                                      messageIdNumber;
///    const char*                                  pMessage;
///    uint32_t                                     queueLabelCount;
///    const VkDebugUtilsLabelEXT*                  pQueueLabels;
///    uint32_t                                     cmdBufLabelCount;
///    const VkDebugUtilsLabelEXT*                  pCmdBufLabels;
///    uint32_t                                     objectCount;
///    const VkDebugUtilsObjectNameInfoEXT*         pObjects;
///} VkDebugUtilsMessengerCallbackDataEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is `0` and is reserved for future use.
/// - [`message_id_name`] is a null-terminated string that identifies the particular message ID that
///   is associated with the provided message. If the message corresponds to a validation layer
///   message, then this string may contain the portion of the Vulkan specification that is believed
///   to have been violated.
/// - [`message_id_number`] is the ID number of the triggering message. If the message corresponds
///   to a validation layer message, then this number is related to the internal number associated
///   with the message being triggered.
/// - [`message`] is a null-terminated string detailing the trigger conditions.
/// - [`queue_label_count`] is a count of items contained in the [`queue_labels`] array.
/// - [`queue_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`Queue`] at the time the callback was triggered. Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
/// - [`cmd_buf_label_count`] is a count of items contained in the [`cmd_buf_labels`] array.
/// - [`cmd_buf_labels`] is `NULL` or a pointer to an array of [`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`] at the time the callback was triggered. Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for more information.
/// - [`object_count`] is a count of items contained in the [`objects`] array.
/// - [`objects`] is a pointer to an array of [`DebugUtilsObjectNameInfoEXT`] objects related to the
///   detected issue. The array is roughly in order or importance, but the 0th element is always
///   guaranteed to be the most important object for this message.
///# Description
///Since adding queue and command buffer labels behaves like pushing and
///popping onto a stack, the order of both [`queue_labels`] and
///[`cmd_buf_labels`] is based on the order the labels were defined.
///The result is that the first label in either [`queue_labels`] or
///[`cmd_buf_labels`] will be the first defined (and therefore the oldest)
///while the last label in each list will be the most recent.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - If [`message_id_name`] is not `NULL`, [`message_id_name`]**must** be a null-terminated UTF-8
///   string
/// - [`message`]**must** be a null-terminated UTF-8 string
/// - If [`queue_label_count`] is not `0`, [`queue_labels`]**must** be a valid pointer to an array
///   of [`queue_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`cmd_buf_label_count`] is not `0`, [`cmd_buf_labels`]**must** be a valid pointer to an
///   array of [`cmd_buf_label_count`] valid [`DebugUtilsLabelEXT`] structures
/// - If [`object_count`] is not `0`, [`objects`]**must** be a valid pointer to an array of
///   [`object_count`] valid [`DebugUtilsObjectNameInfoEXT`] structures
///# Related
/// - [`VK_EXT_debug_utils`]
/// - [`DebugUtilsLabelEXT`]
/// - [`DebugUtilsMessengerCallbackDataFlagsEXT`]
/// - [`DebugUtilsObjectNameInfoEXT`]
/// - [`StructureType`]
/// - [`SubmitDebugUtilsMessageEXT`]
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
pub struct DebugUtilsMessengerCallbackDataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is `0` and is reserved for future use.
    flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    ///[`message_id_name`] is a null-terminated string that identifies the
    ///particular message ID that is associated with the provided message.
    ///If the message corresponds to a validation layer message, then this
    ///string may contain the portion of the Vulkan specification that is
    ///believed to have been violated.
    message_id_name: &'lt CStr,
    ///[`message_id_number`] is the ID number of the triggering message.
    ///If the message corresponds to a validation layer message, then this
    ///number is related to the internal number associated with the message
    ///being triggered.
    message_id_number: i32,
    ///[`message`] is a null-terminated string detailing the trigger
    ///conditions.
    message: &'lt CStr,
    ///[`queue_label_count`] is a count of items contained in the
    ///[`queue_labels`] array.
    queue_label_count: u32,
    ///[`queue_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`Queue`] at the
    ///time the callback was triggered.
    ///Refer to [Queue Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-queue-labels) for more information.
    queue_labels: *const DebugUtilsLabelEXT<'lt>,
    ///[`cmd_buf_label_count`] is a count of items contained in the
    ///[`cmd_buf_labels`] array.
    cmd_buf_label_count: u32,
    ///[`cmd_buf_labels`] is `NULL` or a pointer to an array of
    ///[`DebugUtilsLabelEXT`] active in the current [`CommandBuffer`]
    ///at the time the callback was triggered.
    ///Refer to [Command Buffer Labels](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-command-buffer-labels) for
    ///more information.
    cmd_buf_labels: *const DebugUtilsLabelEXT<'lt>,
    ///[`object_count`] is a count of items contained in the [`objects`]
    ///array.
    object_count: u32,
    ///[`objects`] is a pointer to an array of
    ///[`DebugUtilsObjectNameInfoEXT`] objects related to the detected
    ///issue.
    ///The array is roughly in order or importance, but the 0th element is
    ///always guaranteed to be the most important object for this message.
    objects: *const DebugUtilsObjectNameInfoEXT<'lt>,
}
impl<'lt> Default for DebugUtilsMessengerCallbackDataEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_id_name: std::ptr::null(),
            message_id_number: 0,
            message: std::ptr::null(),
            queue_label_count: 0,
            queue_labels: std::ptr::null(),
            cmd_buf_label_count: 0,
            cmd_buf_labels: std::ptr::null(),
            object_count: 0,
            objects: std::ptr::null(),
        }
    }
}
impl<'lt> DebugUtilsMessengerCallbackDataEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::message_id_name`]
    pub fn message_id_name_raw(&self) -> &'lt CStr {
        self.message_id_name
    }
    ///Gets the raw value of [`Self::message_id_number`]
    pub fn message_id_number_raw(&self) -> i32 {
        self.message_id_number
    }
    ///Gets the raw value of [`Self::message`]
    pub fn message_raw(&self) -> &'lt CStr {
        self.message
    }
    ///Gets the raw value of [`Self::queue_label_count`]
    pub fn queue_label_count_raw(&self) -> u32 {
        self.queue_label_count
    }
    ///Gets the raw value of [`Self::queue_labels`]
    pub fn queue_labels_raw(&self) -> *const DebugUtilsLabelEXT<'lt> {
        self.queue_labels
    }
    ///Gets the raw value of [`Self::cmd_buf_label_count`]
    pub fn cmd_buf_label_count_raw(&self) -> u32 {
        self.cmd_buf_label_count
    }
    ///Gets the raw value of [`Self::cmd_buf_labels`]
    pub fn cmd_buf_labels_raw(&self) -> *const DebugUtilsLabelEXT<'lt> {
        self.cmd_buf_labels
    }
    ///Gets the raw value of [`Self::object_count`]
    pub fn object_count_raw(&self) -> u32 {
        self.object_count
    }
    ///Gets the raw value of [`Self::objects`]
    pub fn objects_raw(&self) -> *const DebugUtilsObjectNameInfoEXT<'lt> {
        self.objects
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_name`]
    pub fn set_message_id_name_raw(&mut self, value: &'lt CStr) -> &mut Self {
        self.message_id_name = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_number`]
    pub fn set_message_id_number_raw(&mut self, value: i32) -> &mut Self {
        self.message_id_number = value;
        self
    }
    ///Sets the raw value of [`Self::message`]
    pub fn set_message_raw(&mut self, value: &'lt CStr) -> &mut Self {
        self.message = value;
        self
    }
    ///Sets the raw value of [`Self::queue_label_count`]
    pub fn set_queue_label_count_raw(&mut self, value: u32) -> &mut Self {
        self.queue_label_count = value;
        self
    }
    ///Sets the raw value of [`Self::queue_labels`]
    pub fn set_queue_labels_raw(&mut self, value: *const DebugUtilsLabelEXT<'lt>) -> &mut Self {
        self.queue_labels = value;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_label_count`]
    pub fn set_cmd_buf_label_count_raw(&mut self, value: u32) -> &mut Self {
        self.cmd_buf_label_count = value;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_labels`]
    pub fn set_cmd_buf_labels_raw(&mut self, value: *const DebugUtilsLabelEXT<'lt>) -> &mut Self {
        self.cmd_buf_labels = value;
        self
    }
    ///Sets the raw value of [`Self::object_count`]
    pub fn set_object_count_raw(&mut self, value: u32) -> &mut Self {
        self.object_count = value;
        self
    }
    ///Sets the raw value of [`Self::objects`]
    pub fn set_objects_raw(&mut self, value: *const DebugUtilsObjectNameInfoEXT<'lt>) -> &mut Self {
        self.objects = value;
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
    pub fn flags(&self) -> DebugUtilsMessengerCallbackDataFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::message_id_name`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn message_id_name(&self) -> &'lt CStr {
        self.message_id_name
    }
    ///Gets the value of [`Self::message_id_number`]
    pub fn message_id_number(&self) -> i32 {
        self.message_id_number
    }
    ///Gets the value of [`Self::message`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn message(&self) -> &'lt CStr {
        self.message
    }
    ///Gets the value of [`Self::queue_label_count`]
    pub fn queue_label_count(&self) -> u32 {
        self.queue_label_count
    }
    ///Gets the value of [`Self::queue_labels`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn queue_labels(&self) -> &[DebugUtilsLabelEXT<'lt>] {
        std::slice::from_raw_parts(self.queue_labels, self.queue_label_count as usize)
    }
    ///Gets the value of [`Self::cmd_buf_label_count`]
    pub fn cmd_buf_label_count(&self) -> u32 {
        self.cmd_buf_label_count
    }
    ///Gets the value of [`Self::cmd_buf_labels`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn cmd_buf_labels(&self) -> &[DebugUtilsLabelEXT<'lt>] {
        std::slice::from_raw_parts(self.cmd_buf_labels, self.cmd_buf_label_count as usize)
    }
    ///Gets the value of [`Self::object_count`]
    pub fn object_count(&self) -> u32 {
        self.object_count
    }
    ///Gets the value of [`Self::objects`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn objects(&self) -> &[DebugUtilsObjectNameInfoEXT<'lt>] {
        std::slice::from_raw_parts(self.objects, self.object_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DebugUtilsMessengerCallbackDataFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::message_id_number`]
    pub fn message_id_number_mut(&mut self) -> &mut i32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::queue_label_count`]
    pub fn queue_label_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::cmd_buf_label_count`]
    pub fn cmd_buf_label_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::object_count`]
    pub fn object_count_mut(&mut self) -> &mut u32 {
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_name`]
    pub fn set_message_id_name(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.message_id_name = value;
        self
    }
    ///Sets the raw value of [`Self::message_id_number`]
    pub fn set_message_id_number(&mut self, value: i32) -> &mut Self {
        self.message_id_number = value;
        self
    }
    ///Sets the raw value of [`Self::message`]
    pub fn set_message(&mut self, value: &'lt std::ffi::CStr) -> &mut Self {
        self.message = value;
        self
    }
    ///Sets the raw value of [`Self::queue_label_count`]
    pub fn set_queue_label_count(&mut self, value: u32) -> &mut Self {
        self.queue_label_count = value;
        self
    }
    ///Sets the raw value of [`Self::queue_labels`]
    pub fn set_queue_labels(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_labels = value.as_ptr();
        self.queue_label_count = len_;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_label_count`]
    pub fn set_cmd_buf_label_count(&mut self, value: u32) -> &mut Self {
        self.cmd_buf_label_count = value;
        self
    }
    ///Sets the raw value of [`Self::cmd_buf_labels`]
    pub fn set_cmd_buf_labels(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsLabelEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.cmd_buf_labels = value.as_ptr();
        self.cmd_buf_label_count = len_;
        self
    }
    ///Sets the raw value of [`Self::object_count`]
    pub fn set_object_count(&mut self, value: u32) -> &mut Self {
        self.object_count = value;
        self
    }
    ///Sets the raw value of [`Self::objects`]
    pub fn set_objects(
        &mut self,
        value: &'lt [crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.objects = value.as_ptr();
        self.object_count = len_;
        self
    }
}
