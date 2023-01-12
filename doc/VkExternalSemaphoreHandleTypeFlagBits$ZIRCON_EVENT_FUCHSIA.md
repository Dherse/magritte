[`ZIRCON_EVENT_FUCHSIA`]
specifies a handle to a Zircon event object.
It can be used with any native API that accepts a Zircon event handle.
Zircon event handles are created with `ZX_RIGHTS_BASIC` and
`ZX_RIGHTS_SIGNAL` rights.
Vulkan on Fuchsia uses only the ZX_EVENT_SIGNALED bit when signaling or
waiting.