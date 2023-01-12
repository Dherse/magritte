The returned function pointer  **must**  only be called with a dispatchable
object (the first parameter) that is [`device`] or a child of
[`device`] e.g. [`Device`], [`Queue`], or
[`CommandBuffer`].