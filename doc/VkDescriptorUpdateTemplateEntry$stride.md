[`stride`] is the stride in bytes between two consecutive array
elements of the descriptor update informations in the raw data
structure.
The actual pointer ptr for each array element j of update entry i is
computed using the following formula:
```c
    const char *ptr = (const char *)pData + pDescriptorUpdateEntries[i].offset + j * pDescriptorUpdateEntries[i].stride
```
The stride is useful in case the bindings are stored in structs along with
other data.
If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
then the value of [`stride`] is ignored and the stride is assumed to be
`1`, i.e. the descriptor update information for them is always specified as
a contiguous range.