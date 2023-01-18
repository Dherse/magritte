[`local_dimming_support`] specifies whether the surface supports local
dimming.
If this is [`TRUE`], [`SwapchainDisplayNativeHdrCreateInfoAMD`] **can**  be used to explicitly enable or disable local dimming for the
surface.
Local dimming may also be overriden by [`set_local_dimming_amd`] during
the lifetime of the swapchain.