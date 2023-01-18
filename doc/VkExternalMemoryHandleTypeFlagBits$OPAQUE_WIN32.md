[`OPAQUE_WIN32`] specifies an NT
handle that has only limited valid usage outside of Vulkan and other
compatible APIs.
It  **must**  be compatible with the functions `DuplicateHandle`,
`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
and `SetHandleInformation`.
It owns a reference to the underlying memory resource represented by its
Vulkan memory object.