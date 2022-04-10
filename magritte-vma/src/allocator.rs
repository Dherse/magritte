use magritte::{vulkan1_0::Device, Handle, Unique};

use crate::ffi::{VmaVulkanFunctions, VmaAllocatorCreateInfo};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct VmaAllocator(pub *mut ());

impl VmaAllocator {
    pub fn new(device: &Unique<Device>) -> Unique<Self> {
        let mut allocator = Self::null();

        let functions = VmaVulkanFunctions::new(
            parent.instance().vtable(),
            parent.vtable(),
        );

        let create_info = VmaAllocatorCreateInfo {
            flags: todo!(),
            physicalDevice: device.physical_device().as_raw(),
            device: device.as_raw(),
            preferredLargeHeapBlockSize: todo!(),
            pAllocationCallbacks: std::ptr::null(),
            pDeviceMemoryCallbacks: std::ptr::null(),
            pHeapSizeLimit: todo!(),
            pVulkanFunctions: &functions,
            instance: device.instance().as_raw(),
            vulkanApiVersion: device.instance().metadata().version().into(),
            pTypeExternalMemoryHandleTypes: std::ptr::null(),
        };

        unsafe {
            crate::ffi::vmaCreateAllocator(&create_info, &mut allocator);
        }

        todo!()
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
    type Raw = *mut ();

    #[inline]
    fn as_raw(self) -> Self::Raw {
        self.0
    }

    #[inline]
    unsafe fn from_raw(this: Self::Raw) -> Self {
        Self(this)
    }

    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {}

    #[inline]
    unsafe fn load_vtable(&self, parent: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}