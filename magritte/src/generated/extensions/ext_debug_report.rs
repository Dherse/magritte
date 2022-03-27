use crate::vulkan1_0::{BaseInStructure, StructureType};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_debug_report");
///[VkDebugReportCallbackCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) - Structure specifying parameters of a newly created debug report callback
///# C Specifications
///The definition of [`DebugReportCallbackCreateInfoEXT`] is:
///```c
///// Provided by VK_EXT_debug_report
///typedef struct VkDebugReportCallbackCreateInfoEXT {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkDebugReportFlagsEXT           flags;
///    PFN_vkDebugReportCallbackEXT    pfnCallback;
///    void*                           pUserData;
///} VkDebugReportCallbackCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying which event(s) will cause this
///   callback to be called.
/// - [`pfn_callback`] is the application callback function to call.
/// - [`user_data`] is user data to be passed to the callback.
///# Description
///For each [`DebugReportCallbackEXT`] that is created the
///[`DebugReportCallbackCreateInfoEXT`]::[`flags`] determine when that
///[`DebugReportCallbackCreateInfoEXT`]::[`pfn_callback`] is called.
///When an event happens, the implementation will do a bitwise AND of the
///event’s [`DebugReportFlagBitsEXT`] flags to each
///[`DebugReportCallbackEXT`] object’s flags.
///For each non-zero result the corresponding callback will be called.
///The callback will come directly from the component that detected the event,
///unless some other layer intercepts the calls for its own purposes (filter
///them in a different way, log to a system error log, etc.).An application **may** receive
/// multiple callbacks if multiple
///[`DebugReportCallbackEXT`] objects were created.
///A callback will always be executed in the same thread as the originating
///Vulkan call.A callback may be called from multiple threads simultaneously (if the
///application is making Vulkan calls from multiple threads).Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT`
/// - [`flags`]**must** be a valid combination of [`DebugReportFlagBitsEXT`] values
/// - [`pfn_callback`]**must** be a valid [`PFNDebugReportCallbackEXT`] value
///# Related
/// - [`PFNDebugReportCallbackEXT`]
/// - [`VK_EXT_debug_report`]
/// - [`DebugReportFlagsEXT`]
/// - [`StructureType`]
/// - [`CreateDebugReportCallbackEXT`]
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
pub struct DebugReportCallbackCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`DebugReportFlagBitsEXT`] specifying
    ///which event(s) will cause this callback to be called.
    flags: DebugReportFlagsEXT,
    ///[`pfn_callback`] is the application callback function to call.
    pfn_callback: PFNDebugReportCallbackEXT<'lt>,
    ///[`user_data`] is user data to be passed to the callback.
    user_data: *mut c_void,
}
impl<'lt> Default for DebugReportCallbackCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            pfn_callback: Default::default(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DebugReportCallbackCreateInfoEXT<'lt> {
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
    pub fn flags(&self) -> DebugReportFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::pfn_callback`]
    pub fn pfn_callback(&self) -> &PFNDebugReportCallbackEXT<'lt> {
        &self.pfn_callback
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
    pub fn flags_mut(&mut self) -> &mut DebugReportFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::pfn_callback`]
    pub fn pfn_callback_mut(&mut self) -> &mut PFNDebugReportCallbackEXT<'lt> {
        &mut self.pfn_callback
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
    pub fn set_flags(&mut self, value: crate::extensions::ext_debug_report::DebugReportFlagsEXT) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::pfn_callback`]
    pub fn set_pfn_callback(
        &mut self,
        value: crate::extensions::ext_debug_report::PFNDebugReportCallbackEXT<'lt>,
    ) -> &mut Self {
        self.pfn_callback = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
}
