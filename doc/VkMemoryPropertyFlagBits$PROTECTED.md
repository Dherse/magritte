[`PROTECTED`] bit specifies that the memory
type only allows device access to the memory, and allows protected queue
operations to access the memory.
Memory types  **must**  not have [`PROTECTED`] set
and any of [`HOST_VISIBLE`] set, or
[`HOST_COHERENT`] set, or
[`HOST_CACHED`] set.