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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`representative_fragment_test`]
    ///indicates whether the implementation supports the representative
    ///fragment test.
    ///See [Representative Fragment Test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-rep-frag-test).
    representative_fragment_test: Bool32,
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`representative_fragment_test_enable`] controls whether the
    ///representative fragment test is enabled.
    representative_fragment_test_enable: Bool32,
}
