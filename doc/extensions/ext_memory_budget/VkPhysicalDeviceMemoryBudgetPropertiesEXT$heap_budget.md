[`heap_budget`] is an array of `VK_MAX_MEMORY_HEAPS`[`DeviceSize`] values in which memory budgets are returned, with
one element for each memory heap.
A heap’s budget is a rough estimate of how much memory the process  **can** 
allocate from that heap before allocations  **may**  fail or cause
performance degradation.
The budget includes any currently allocated device memory.