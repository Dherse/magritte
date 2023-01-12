[`callback`] is the [`DebugReportCallbackEXT`] object to destroy.
[`callback`] is an externally synchronized object and  **must**  not be
used on more than one thread at a time.
This means that [`destroy_debug_report_callback_ext`] **must**  not be
called when a callback is active.