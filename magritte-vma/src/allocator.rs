use magritte::{
    cstr,
    vulkan1_0::{Device, PhysicalDevice, VulkanResultCodes},
    AsRaw, Extensions, Handle, Unique,
};

use crate::{
    ffi::{vmaDestroyAllocator, VmaAllocatorCreateInfo, VmaVulkanFunctions},
    flags::VmaAllocatorCreateFlags,
    AsCStr,
};
use std::{ffi::CStr, sync::atomic::Ordering};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VmaAllocator(pub *mut ());

impl VmaAllocator {
    pub fn new(
        device: &Unique<Device>,
        heap_block_size: Option<u64>,
        heap_size_limits: Option<&[u64]>,
    ) -> Result<Unique<Self>, VulkanResultCodes> {
        let memory_properties = unsafe { device.physical_device().get_physical_device_memory_properties() };

        if let Some(heap_size_limits) = heap_size_limits {
            assert_eq!(
                memory_properties.memory_heap_count,
                heap_size_limits.len() as u32,
                "The number of heap size limits and heap size must match"
            );
        }

        let functions = VmaVulkanFunctions::new(device.instance().vtable(), device.vtable());

        let extensions = device.instance().metadata().load(Ordering::Acquire);
        let mut flags = VmaAllocatorCreateFlags::empty();
        if extensions.khr_dedicated_allocation() {
            flags |= VmaAllocatorCreateFlags::DEDICATED_ALLOCATION;
        }

        if extensions.khr_bind_memory2() {
            flags |= VmaAllocatorCreateFlags::BIND_MEMORY2;
        }

        if extensions.ext_memory_budget() {
            flags |= VmaAllocatorCreateFlags::EXT_MEMORY_BUDGET;
        }

        if extensions.amd_device_coherent_memory() {
            flags |= VmaAllocatorCreateFlags::AMD_DEVICE_COHERENT_MEMORY;
        }

        if extensions.khr_buffer_device_address() {
            flags |= VmaAllocatorCreateFlags::BUFFER_DEVICE_ADDRESS;
        }

        if extensions.ext_memory_priority() {
            flags |= VmaAllocatorCreateFlags::EXT_MEMORY_PRIORITY;
        }

        let create_info = VmaAllocatorCreateInfo {
            flags,
            physicalDevice: device.physical_device().as_raw(),
            device: device.as_raw(),
            preferredLargeHeapBlockSize: heap_block_size.unwrap_or_default(),
            pAllocationCallbacks: std::ptr::null(),
            pDeviceMemoryCallbacks: std::ptr::null(),
            pHeapSizeLimit: heap_size_limits.map(|s| s.as_ptr()).unwrap_or_else(std::ptr::null),
            pVulkanFunctions: &functions,
            instance: device.instance().as_raw(),
            vulkanApiVersion: extensions.version().into(),
            pTypeExternalMemoryHandleTypes: std::ptr::null(),
        };

        let mut allocator = Self::null();

        let code = unsafe { crate::ffi::vmaCreateAllocator(&create_info, &mut allocator) };

        if code != VulkanResultCodes::SUCCESS {
            return Err(code);
        }

        Ok(unsafe { Unique::new(device, allocator, ()) })
    }

    pub fn create_buffer(self: &Unique<Self>) -> Result<(), VulkanResultCodes> {
        Ok(())
    }

    pub fn enable_extensions(
        physical_device: &Unique<PhysicalDevice>,
        extensions: &mut Extensions,
    ) -> Result<(), VulkanResultCodes> {
        const DEDICATED_ALLOCATION: &str = "VK_KHR_dedicated_allocation";
        const BIND_MEMORY2: &str = "VK_KHR_bind_memory2";
        const MEMORY_BUDGET: &str = "VK_EXT_memory_budget";
        const DEVICE_COHERENT_MEMORY: &str = "VK_AMD_device_coherent_memory";
        const BUFFER_DEVICE_ADDRESS: &str = "VK_KHR_buffer_device_address";
        const MEMORY_PRIORITY: &str = "VK_EXT_memory_priority";

        let extension_list = unsafe {
            physical_device
                .enumerate_device_extension_properties(None, None)
                .result()?
        };

        let mut sum = 0;
        for extension in extension_list {
            *extensions = match extension.extension_name().as_cstr().to_str().unwrap() {
                DEDICATED_ALLOCATION => {
                    sum += 1;
                    extensions.enable_khr_dedicated_allocation()
                },
                BIND_MEMORY2 => {
                    sum += 1;
                    extensions.enable_khr_bind_memory2()
                },
                MEMORY_BUDGET => {
                    sum += 1;
                    extensions.enable_ext_memory_budget()
                },
                DEVICE_COHERENT_MEMORY => {
                    sum += 1;
                    extensions.enable_amd_device_coherent_memory()
                },
                BUFFER_DEVICE_ADDRESS => {
                    sum += 1;
                    extensions.enable_khr_buffer_device_address()
                },
                MEMORY_PRIORITY => {
                    sum += 1;
                    extensions.enable_ext_memory_priority()
                },
                _ => continue,
            };

            if sum == 6 {
                break;
            }
        }

        Ok(())
    }

    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(std::ptr::null_mut())
    }

    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }

    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> *mut () {
        self.0
    }
}

unsafe impl Send for VmaAllocator {}
impl Default for VmaAllocator {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for VmaAllocator {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = ();
    type Storage = *mut ();

    #[inline]
    fn as_stored(self) -> Self::Storage {
        self.0
    }

    #[inline]
    unsafe fn from_stored(this: Self::Storage) -> Self {
        Self(this)
    }

    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        vmaDestroyAllocator(self.as_raw())
    }

    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
