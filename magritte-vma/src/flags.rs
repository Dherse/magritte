bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaAllocatorCreateFlags: u32 {
        const EXTERNALLY_SYNCHRONIZED = 0x00000001;
        const DEDICATED_ALLOCATION = 0x00000002;
        const BIND_MEMORY2 = 0x00000004;
        const EXT_MEMORY_BUDGET = 0x00000008;
        const AMD_DEVICE_COHERENT_MEMORY = 0x00000010;
        const BUFFER_DEVICE_ADDRESS = 0x00000020;
        const EXT_MEMORY_PRIORITY = 0x00000040;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaAllocationCreateFlags: u32 {
        const DEDICATED_MEMORY = 0x00000001;
        const NEVER_ALLOCATE = 0x00000002;
        const MAPPED = 0x00000004;
        const USER_DATA_COPY_STRING = 0x00000020;
        const UPPER_ADDRESS = 0x00000040;
        const DONT_BIND = 0x00000080;
        const WITHIN_BUDGET = 0x00000100;
        const CAN_ALIAS = 0x00000200;
        const HOST_ACCESS_SEQUENTIAL_WRITE = 0x00000400;
        const HOST_ACCESS_RANDOM = 0x00000800;
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD = 0x00001000;
        const STRATEGY_MIN_MEMORY = 0x00010000;
        const STRATEGY_MIN_TIME = 0x00010000;
        const STRATEGY_MIN_OFFSET = 0x00010000;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaPoolCreateFlags: u32 {
        const IGNORE_BUFFER_IMAGE_GRANULARITY = 0x00000001;
        const LINEAR_ALGORITHM = 0x00000002;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaDefragmentationFlags: u32 {
        const ALGORITHM_FAST = 0x00000001;
        const ALGORITHM_BALANCED = 0x00000002;
        const ALGORITHM_FULL = 0x00000004;
        const ALGORITHM_EXTENSIVE = 0x00000020;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaVirtualBlockCreateFlags: u32 {
        const LINEAR_ALGORITHM = 0x00000001;
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct VmaVirtualAllocationCreateFlags: u32 {
        const UPPER_ADDRESS = VmaAllocationCreateFlags::UPPER_ADDRESS.bits();
        const STRATEGY_MIN_MEMORY = VmaAllocationCreateFlags::STRATEGY_MIN_MEMORY.bits();
        const STRATEGY_MIN_TIME = VmaAllocationCreateFlags::STRATEGY_MIN_TIME.bits();
        const STRATEGY_MIN_OFFSET = VmaAllocationCreateFlags::STRATEGY_MIN_OFFSET.bits();
    }
}
