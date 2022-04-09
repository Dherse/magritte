//![VK_NV_representative_fragment_test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_representative_fragment_test.html) - device extension
//!# Description
//!This extension provides a new representative fragment test that allows
//!implementations to reduce the amount of rasterization and fragment
//!processing work performed for each point, line, or triangle primitive.
//!For any primitive that produces one or more fragments that pass all other
//!early fragment tests, the implementation is permitted to choose one or more
//!“representative” fragments for processing and discard all other fragments.
//!For draw calls rendering multiple points, lines, or triangles arranged in
//!lists, strips, or fans, the representative fragment test is performed
//!independently for each of those primitives.This extension is useful for applications that use an
//! early render pass to
//!determine the full set of primitives that would be visible in the final
//!scene.
//!In this render pass, such applications would set up a fragment shader that
//!enables early fragment tests and writes to an image or shader storage buffer
//!to record the ID of the primitive that generated the fragment.
//!Without this extension, the shader would record the ID separately for each
//!visible fragment of each primitive.
//!With this extension, fewer stores will be performed, particularly for large
//!primitives.The representative fragment test has no effect if early fragment tests are
//!not enabled via the fragment shader.
//!The set of fragments discarded by the representative fragment test is
//!implementation-dependent and may vary from frame to frame.
//!In some cases, the representative fragment test may not discard any
//!fragments for a given primitive.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_representative_fragment_test]
//!   @kthangudu%0A<<Here describe the issue or question you have about the
//!   VK_NV_representative_fragment_test extension>>)
//!# New structures
//! - Extending [`GraphicsPipelineCreateInfo`]:  -
//!   [`PipelineRepresentativeFragmentTestStateCreateInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`]
//!# New constants
//! - [`NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME`]
//! - [`NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!(1) Is the representative fragment test guaranteed to have any effect? **RESOLVED** : No.
//!As specified, we only guarantee that each primitive with at least one
//!fragment that passes prior tests will have one fragment passing the
//!representative fragment tests.
//!We do not guarantee that any particular fragment will fail the test.In the initial
//! implementation of this extension, the representative fragment
//!test is treated as an optimization that may be completely disabled for some
//!pipeline states.
//!This feature was designed for a use case where the fragment shader records
//!information on individual primitives using shader storage buffers or storage
//!images, with no writes to color or depth buffers.(2) Will the set of fragments that pass the
//! representative fragment test be
//!repeatable if you draw the same scene over and over again? **RESOLVED** : No.
//!The set of fragments that pass the representative fragment test is
//!implementation-dependent and may vary due to the timing of operations
//!performed by the GPU.(3) What happens if you enable the representative fragment test with writes
//!to color and/or depth render targets enabled? **RESOLVED** : If writes to the color or depth
//! buffer are enabled, they will be
//!performed for any fragments that survive the relevant tests.
//!Any fragments that fail the representative fragment test will not update
//!color buffers.
//!For the use cases intended for this feature, we do not expect color or depth
//!writes to be enabled.(4) How do derivatives and automatic texture level of detail computations
//!work with the representative fragment test enabled? **RESOLVED** : If a fragment shader uses
//! derivative functions or texture
//!lookups using automatic level of detail computation, derivatives will be
//!computed identically whether or not the representative fragment test is
//!enabled.
//!For the use cases intended for this feature, we do not expect the use of
//!derivatives in the fragment shader.
//!# Version History
//! - Revision 2, 2018-09-13 (pbrown)  - Add issues.
//! - Revision 1, 2018-08-22 (Kedarnath Thangudu)  - Internal Revisions
//!# Other info
//! * 2018-09-13
//! * - Kedarnath Thangudu, NVIDIA  - Christoph Kubisch, NVIDIA  - Pierre Boudier, NVIDIA  - Pat
//!   Brown, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA
//!# Related
//! - [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`]
//! - [`PipelineRepresentativeFragmentTestStateCreateInfoNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`representative_fragment_test`]
    ///indicates whether the implementation supports the representative
    ///fragment test.
    ///See [Representative Fragment Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-rep-frag-test).
    pub representative_fragment_test: Bool32,
}
impl<'lt> Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            representative_fragment_test: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::representative_fragment_test`]
    pub fn representative_fragment_test_raw(&self) -> Bool32 {
        self.representative_fragment_test
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test`]
    pub fn set_representative_fragment_test_raw(mut self, value: Bool32) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::representative_fragment_test`]
    pub fn set_representative_fragment_test(mut self, value: bool) -> Self {
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
///if enabled.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPipelineRepresentativeFragmentTestStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`representative_fragment_test_enable`] controls whether the
    ///representative fragment test is enabled.
    pub representative_fragment_test_enable: Bool32,
}
impl<'lt> Default for PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::representative_fragment_test_enable`]
    pub fn set_representative_fragment_test_enable_raw(mut self, value: Bool32) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::representative_fragment_test_enable`]
    pub fn set_representative_fragment_test_enable(mut self, value: bool) -> Self {
        self.representative_fragment_test_enable = value as u8 as u32;
        self
    }
}
