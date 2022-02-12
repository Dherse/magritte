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
//! - Extending [`GraphicsPipelineCreateInfo`]:
//! - [`PipelineRepresentativeFragmentTestStateCreateInfoNV`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceRepresentativeFragmentTestFeaturesNV`]
//!# New constants
//! - [`NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME`]
//! - [`NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!(1) Is the representative fragment test guaranteed to have any effect?**RESOLVED**: No.
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
//!repeatable if you draw the same scene over and over again?**RESOLVED**: No.
//!The set of fragments that pass the representative fragment test is
//!implementation-dependent and may vary due to the timing of operations
//!performed by the GPU.(3) What happens if you enable the representative fragment test with writes
//!to color and/or depth render targets enabled?**RESOLVED**: If writes to the color or depth
//! buffer are enabled, they will be
//!performed for any fragments that survive the relevant tests.
//!Any fragments that fail the representative fragment test will not update
//!color buffers.
//!For the use cases intended for this feature, we do not expect color or depth
//!writes to be enabled.(4) How do derivatives and automatic texture level of detail computations
//!work with the representative fragment test enabled?**RESOLVED**: If a fragment shader uses
//! derivative functions or texture
//!lookups using automatic level of detail computation, derivatives will be
//!computed identically whether or not the representative fragment test is
//!enabled.
//!For the use cases intended for this feature, we do not expect the use of
//!derivatives in the fragment shader.
//!# Version History
//! - Revision 2, 2018-09-13 (pbrown)
//! - Add issues.
//!
//! - Revision 1, 2018-08-22 (Kedarnath Thangudu)
//! - Internal Revisions
//!# Other info
//! * 2018-09-13
//!*
//! - Kedarnath Thangudu, NVIDIA
//! - Christoph Kubisch, NVIDIA
//! - Pierre Boudier, NVIDIA
//! - Pat Brown, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Eric Werness, NVIDIA
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
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_representative_fragment_test");
