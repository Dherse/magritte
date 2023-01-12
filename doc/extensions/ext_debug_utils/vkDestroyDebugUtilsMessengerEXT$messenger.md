[`messenger`] is the [`DebugUtilsMessengerEXT`] object to destroy.
[`messenger`] is an externally synchronized object and  **must**  not be
used on more than one thread at a time.
This means that [`destroy_debug_utils_messenger_ext`] **must**  not be
called when a callback is active.