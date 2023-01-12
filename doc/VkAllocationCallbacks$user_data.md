[`user_data`] is a value to be interpreted by the implementation of
the callbacks.
When any of the callbacks in [`AllocationCallbacks`] are called, the
Vulkan implementation will pass this value as the first parameter to the
callback.
This value  **can**  vary each time an allocator is passed into a command,
even when the same object takes an allocator in multiple commands.