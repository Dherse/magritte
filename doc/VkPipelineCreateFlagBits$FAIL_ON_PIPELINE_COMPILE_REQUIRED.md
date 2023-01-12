[`FAIL_ON_PIPELINE_COMPILE_REQUIRED`] specifies
that pipeline creation will fail if a compile is required for creation
of a valid [`Pipeline`] object; `VK_PIPELINE_COMPILE_REQUIRED`
will be returned by pipeline creation, and the [`Pipeline`] will be
set to [`crate::Handle::null`].