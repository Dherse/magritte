#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::c_void, os::raw::c_char};

use magritte::{
    core::{MAX_MEMORY_HEAPS, MAX_MEMORY_TYPES},
    vulkan1_0::{
        Bool32, Buffer, BufferCreateInfo, Device, DeviceMemory, DeviceSize, DeviceVTable, FNAllocateMemory,
        FNBindBufferMemory, FNBindImageMemory, FNCmdCopyBuffer, FNCreateBuffer, FNCreateImage, FNDestroyBuffer,
        FNDestroyImage, FNFlushMappedMemoryRanges, FNFreeMemory, FNGetBufferMemoryRequirements, FNGetDeviceProcAddr,
        FNGetImageMemoryRequirements, FNGetInstanceProcAddr, FNGetPhysicalDeviceMemoryProperties,
        FNGetPhysicalDeviceProperties, FNInvalidateMappedMemoryRanges, FNMapMemory, FNUnmapMemory, Image,
        ImageCreateInfo, Instance, InstanceVTable, MemoryPropertyFlags, MemoryRequirements, PhysicalDevice,
        PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, VulkanResultCodes,
    },
    vulkan1_1::{
        ExternalMemoryHandleTypeFlags, ExternalMemoryHandleTypeFlags as ExternalMemoryHandleTypeFlagsKHR,
        FNBindBufferMemory2, FNBindImageMemory2, FNGetBufferMemoryRequirements2, FNGetImageMemoryRequirements2,
        FNGetPhysicalDeviceMemoryProperties2,
    },
    vulkan1_3::{FNGetDeviceBufferMemoryRequirements, FNGetDeviceImageMemoryRequirements},
    Unique,
};

use crate::{
    allocation::VmaAllocation,
    allocator::VmaAllocator,
    defragmentation_context::VmaDefragmentationContext,
    flags::{
        VmaAllocationCreateFlags, VmaAllocatorCreateFlags, VmaDefragmentationFlags, VmaPoolCreateFlags,
        VmaVirtualAllocationCreateFlags, VmaVirtualBlockCreateFlags,
    },
    pool::VmaPool,
    virtual_allocation::VmaVirtualAllocation,
    virtual_block::VmaVirtualBlock,
};

type Result = VulkanResultCodes;
type AllocationCallbacks = magritte::vulkan1_0::AllocationCallbacks<'static>;

include!("../bindings.rs");

/// All the Vulkan function used by VMA
#[repr(C)]
pub struct VmaVulkanFunctions {
    pub get_instance_proc_addr: FNGetInstanceProcAddr,
    pub get_device_proc_addr: FNGetDeviceProcAddr,
    pub get_physical_device_properties: FNGetPhysicalDeviceProperties,
    pub get_physical_device_memory_properties: FNGetPhysicalDeviceMemoryProperties,
    pub allocate_memory: FNAllocateMemory,
    pub free_memory: FNFreeMemory,
    pub map_memory: FNMapMemory,
    pub unmap_memory: FNUnmapMemory,
    pub flush_mapped_memory_ranges: FNFlushMappedMemoryRanges,
    pub invalidate_mapped_memory_ranges: FNInvalidateMappedMemoryRanges,
    pub bind_buffer_memory: FNBindBufferMemory,
    pub bind_image_memory: FNBindImageMemory,
    pub get_buffer_memory_requirements: FNGetBufferMemoryRequirements,
    pub get_image_memory_requirements: FNGetImageMemoryRequirements,
    pub create_buffer: FNCreateBuffer,
    pub destroy_buffer: FNDestroyBuffer,
    pub create_image: FNCreateImage,
    pub destroy_image: FNDestroyImage,
    pub cmd_copy_buffer: FNCmdCopyBuffer,
    pub get_buffer_memory_requirements2: FNGetBufferMemoryRequirements2,
    pub get_image_memory_requirements2: FNGetImageMemoryRequirements2,
    pub bind_buffer_memory2: FNBindBufferMemory2,
    pub bind_image_memory2: FNBindImageMemory2,
    pub get_physical_device_memory_properties2: FNGetPhysicalDeviceMemoryProperties2,
    pub get_device_buffer_memory_requirements: FNGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements: FNGetDeviceImageMemoryRequirements,
}

impl VmaVulkanFunctions {
    pub fn new(instance: &InstanceVTable, device: &DeviceVTable) -> Self {
        Self {
            get_instance_proc_addr: None,
            get_device_proc_addr: instance.vulkan1_0().get_device_proc_addr(),
            get_physical_device_properties: instance.vulkan1_0().get_physical_device_properties(),
            get_physical_device_memory_properties: instance.vulkan1_0().get_physical_device_memory_properties(),
            allocate_memory: device.vulkan1_0().allocate_memory(),
            free_memory: device.vulkan1_0().free_memory(),
            map_memory: device.vulkan1_0().map_memory(),
            unmap_memory: device.vulkan1_0().unmap_memory(),
            flush_mapped_memory_ranges: device.vulkan1_0().flush_mapped_memory_ranges(),
            invalidate_mapped_memory_ranges: device.vulkan1_0().invalidate_mapped_memory_ranges(),
            bind_buffer_memory: device.vulkan1_0().bind_buffer_memory(),
            bind_image_memory: device.vulkan1_0().bind_image_memory(),
            get_buffer_memory_requirements: device.vulkan1_0().get_buffer_memory_requirements(),
            get_image_memory_requirements: device.vulkan1_0().get_image_memory_requirements(),
            create_buffer: device.vulkan1_0().create_buffer(),
            destroy_buffer: device.vulkan1_0().destroy_buffer(),
            create_image: device.vulkan1_0().create_image(),
            destroy_image: device.vulkan1_0().destroy_image(),
            cmd_copy_buffer: device.vulkan1_0().cmd_copy_buffer(),
            get_buffer_memory_requirements2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.get_buffer_memory_requirements2())
                .or_else(|| {
                    device
                        .khr_get_memory_requirements2()
                        .and_then(|vtable| vtable.get_buffer_memory_requirements2_khr())
                }),
            get_image_memory_requirements2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.get_image_memory_requirements2())
                .or_else(|| {
                    device
                        .khr_get_memory_requirements2()
                        .and_then(|vtable| vtable.get_image_memory_requirements2_khr())
                }),
            bind_buffer_memory2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.bind_buffer_memory2())
                .or_else(|| {
                    device
                        .khr_bind_memory2()
                        .and_then(|vtable| vtable.bind_buffer_memory2_khr())
                }),
            bind_image_memory2: device
                .vulkan1_1()
                .and_then(|vtable| vtable.bind_image_memory2())
                .or_else(|| {
                    device
                        .khr_bind_memory2()
                        .and_then(|vtable| vtable.bind_image_memory2_khr())
                }),
            get_physical_device_memory_properties2: instance
                .vulkan1_1()
                .and_then(|vtable| vtable.get_physical_device_memory_properties2())
                .or_else(|| {
                    instance
                        .khr_get_physical_device_properties2()
                        .and_then(|vtable| vtable.get_physical_device_memory_properties2_khr())
                }),
            get_device_buffer_memory_requirements: device
                .vulkan1_3()
                .and_then(|vtable| vtable.get_device_buffer_memory_requirements())
                .or_else(|| {
                    device
                        .khr_maintenance4()
                        .and_then(|vtable| vtable.get_device_buffer_memory_requirements_khr())
                }),
            get_device_image_memory_requirements: device
                .vulkan1_3()
                .and_then(|vtable| vtable.get_device_image_memory_requirements())
                .or_else(|| {
                    device
                        .khr_maintenance4()
                        .and_then(|vtable| vtable.get_device_image_memory_requirements_khr())
                }),
        }
    }
}
