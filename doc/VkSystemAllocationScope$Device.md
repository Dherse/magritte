If an allocation is scoped to the lifetime of an object, that object is
being created or manipulated by the command, and that object’s type is
not [`Device`] or [`Instance`], the allocator will use an
allocation scope of [`VK_SYSTEM_ALLOCATION_SCOPE`].
The most specific allocator available is used (object, else device, else
instance).
Else,