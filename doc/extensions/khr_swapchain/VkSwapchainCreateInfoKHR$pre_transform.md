[`pre_transform`] is a [`SurfaceTransformFlagBitsKHR`] value
describing the transform, relative to the presentation engine’s natural
orientation, applied to the image content prior to presentation.
If it does not match the `currentTransform` value returned by
[`get_physical_device_surface_capabilities_khr`], the presentation engine
will transform the image content as part of the presentation operation.