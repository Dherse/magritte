use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

use crate::{
    cstr_ptr,
    entry::Entry,
    extensions::ext_debug_utils::{
        DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT,
    },
    vulkan1_0::{Bool32, Instance, VulkanResultCodes, FALSE},
    InstanceExtensions, Unique,
};
pub use log::Level;
use log::{debug, error, info, warn};

pub const VALIDATION_LAYER_NAME: *const c_char = cstr_ptr!("VK_LAYER_KHRONOS_validation");

/// Enables the Vulkan validation layer in the extension set
#[inline]
pub fn enable_validation(extension: InstanceExtensions) -> InstanceExtensions {
    extension.enable_ext_debug_utils()
}

/// Checks if the validation layers are present
#[inline]
pub unsafe fn is_present(entry: &Entry) -> Result<bool, VulkanResultCodes> {
    let extensions = entry.enumerate_instance_extension_properties(None, None)?;
    if !extensions
        .into_iter()
        .any(|ext| CStr::from_ptr(ext.extension_name().as_ptr()).to_string_lossy() == "VK_EXT_debug_utils")
    {
        return Ok(false);
    }

    let layers = entry.enumerate_instance_layer_properties(None)?;

    Ok(layers
        .into_iter()
        .any(|layer| CStr::from_ptr(layer.layer_name().as_ptr()).to_string_lossy() == "VK_LAYER_KHRONOS_validation"))
}

/// Creates a debug utils messenger using [`log`] as the logger.
#[inline]
pub unsafe fn create_debug_utils_messenger(
    instance: &Unique<Instance>,
    level: Level,
    types: Option<DebugUtilsMessageTypeFlagsEXT>,
) -> Result<Unique<DebugUtilsMessengerEXT>, VulkanResultCodes> {
    let debug_info = DebugUtilsMessengerCreateInfoEXT::default()
        .set_message_severity(match level {
            Level::Error => DebugUtilsMessageSeverityFlagsEXT::ERROR,
            Level::Warn => DebugUtilsMessageSeverityFlagsEXT::WARNING | DebugUtilsMessageSeverityFlagsEXT::ERROR,
            Level::Info => {
                DebugUtilsMessageSeverityFlagsEXT::INFO
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | DebugUtilsMessageSeverityFlagsEXT::ERROR
            },
            Level::Debug | Level::Trace => DebugUtilsMessageSeverityFlagsEXT::all(),
        })
        .set_message_type(types.unwrap_or_else(DebugUtilsMessageTypeFlagsEXT::all))
        .set_pfn_user_callback(Some(vulkan_debug_callback));

    let (debug_utils, _) = instance.create_debug_utils_messenger_ext(&debug_info, None)?;

    Ok(debug_utils)
}

#[doc(hidden)]
unsafe extern "system" fn vulkan_debug_callback<'lt>(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'lt>,
    _user_data: *mut std::os::raw::c_void,
) -> Bool32 {
    let callback_data = &*p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.message_id_name).to_string_lossy()
    };

    let message = if callback_data.message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.message).to_string_lossy()
    };

    match message_severity {
        DebugUtilsMessageSeverityFlagBitsEXT::VERBOSE => debug!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::INFO => info!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::WARNING => warn!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        DebugUtilsMessageSeverityFlagBitsEXT::ERROR => error!(
            "{:?} [{} ({})] : {}",
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
        other => error!(
            "[UNKNOWN]: {:0X}] {:?} [{} ({})] : {}",
            other.bits(),
            message_type,
            message_id_name,
            &message_id_number.to_string(),
            message,
        ),
    }

    FALSE
}
