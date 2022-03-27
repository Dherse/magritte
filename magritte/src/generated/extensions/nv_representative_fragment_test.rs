use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_representative_fragment_test");
///[VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html) - Structure describing the representative fragment test features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_representative_fragment_test
///typedef struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           representativeFragmentTest;
///} VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`representative_fragment_test`] indicates whether the implementation supports the representative fragment test. See [Representative Fragment Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-rep-frag-test).
///If the [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`
///# Related
/// - [`VK_NV_representative_fragment_test`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`representative_fragment_test`]
    ///indicates whether the implementation supports the representative
    ///fragment test.
    ///See [Representative Fragment Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-rep-frag-test).
    representative_fragment_test: Bool32,
}
impl<'lt> Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            representative_fragment_test: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::representative_fragment_test`]
    pub fn representative_fragment_test_raw(&self) -> Bool32 {
        self.representative_fragment_test
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test`]
    pub fn set_representative_fragment_test_raw(&mut self, value: Bool32) -> &mut Self {
        self.representative_fragment_test = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::representative_fragment_test`]
    pub fn representative_fragment_test(&self) -> bool {
        unsafe { std::mem::transmute(self.representative_fragment_test as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::representative_fragment_test`]
    pub fn representative_fragment_test_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.representative_fragment_test as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.representative_fragment_test as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test`]
    pub fn set_representative_fragment_test(&mut self, value: bool) -> &mut Self {
        self.representative_fragment_test = value as u8 as u32;
        self
    }
}
///[VkPipelineRepresentativeFragmentTestStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html) - Structure specifying representative fragment test
///# C Specifications
///If the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] includes a
///[`PipelineRepresentativeFragmentTestStateCreateInfoNV`] structure, then
///that structure includes parameters controlling the representative fragment
///test.The [`PipelineRepresentativeFragmentTestStateCreateInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_representative_fragment_test
///typedef struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           representativeFragmentTestEnable;
///} VkPipelineRepresentativeFragmentTestStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`representative_fragment_test_enable`] controls whether the representative fragment test is
///   enabled.
///# Description
///If this structure is not included in the [`p_next`] chain,
///[`representative_fragment_test_enable`] is considered to be [`FALSE`],
///and the representative fragment test is disabled.If the active fragment shader does not specify
/// the `EarlyFragmentTests`
///execution mode, the representative fragment shader test has no effect, even
///if enabled.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_representative_fragment_test`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`representative_fragment_test_enable`] controls whether the
    ///representative fragment test is enabled.
    representative_fragment_test_enable: Bool32,
}
impl<'lt> Default for PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            representative_fragment_test_enable: 0,
        }
    }
}
impl<'lt> PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::representative_fragment_test_enable`]
    pub fn representative_fragment_test_enable_raw(&self) -> Bool32 {
        self.representative_fragment_test_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test_enable`]
    pub fn set_representative_fragment_test_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.representative_fragment_test_enable = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::representative_fragment_test_enable`]
    pub fn representative_fragment_test_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.representative_fragment_test_enable as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::representative_fragment_test_enable`]
    pub fn representative_fragment_test_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.representative_fragment_test_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.representative_fragment_test_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test_enable`]
    pub fn set_representative_fragment_test_enable(&mut self, value: bool) -> &mut Self {
        self.representative_fragment_test_enable = value as u8 as u32;
        self
    }
}
