//![VK_QCOM_render_pass_transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QCOM_render_pass_transform.html) - device extension
//!# Description
//!This extension provides a mechanism for applications to enable driver
//!support for [render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform).Mobile devices can be rotated and mobile applications need to render
//!properly when a device is held in a landscape or portrait orientation.
//!When the current orientation differs from the device’s native orientation, a
//!rotation is required so that the “up” direction of the rendered scene
//!matches the current orientation.If the Display Processing Unit (DPU) doesnt natively support
//! rotation, the
//!Vulkan presentation engine can handle this rotation in a separate
//!composition pass.
//!Alternatively, the application can render frames “pre-rotated” to avoid
//!this extra pass.
//!The latter is preferred to reduce power consumption and achieve the best
//!performance because it avoids tasking the GPU with extra work to perform the
//!copy/rotate operation.Unlike OpenGL ES, the burden of pre-rotation in Vulkan falls on the
//!application.
//!To implement pre-rotation, applications render into swapchain images
//!matching the device native aspect ratio of the display and “pre-rotate”
//!the rendering content to match the device’s current orientation.
//!The burden is more than adjusting the Model View Projection (MVP) matrix in
//!the vertex shader to account for rotation and aspect ratio.
//!The coordinate systems of scissors, viewports, derivatives and several
//!shader built-ins may need to be adapted to produce the correct result.It is difficult for some
//! game engines to manage this burden; many chose to
//!simply accept the performance/power overhead of performing rotation in the
//!presentation engine.This extension allows applications to achieve the performance benefits of
//!pre-rotated rendering by moving much of the above-mentioned burden to the
//!graphics driver.
//!The following is unchanged with this extension:
//! - Applications create a swapchain matching the native orientation of the
//!display.
//!Applications must also set the
//![`SwapchainCreateInfoKHR::pre_transform`] equal to the
//!`currentTransform` as returned by
//![`GetPhysicalDeviceSurfaceCapabilitiesKHR`].The following is changed with this extension:
//! - At [`CmdBeginRenderPass`], the application provides extension struct
//![`RenderPassTransformBeginInfoQCOM`] specifying the render pass
//!transform parameters.
//! - At [`BeginCommandBuffer`] for secondary command buffers, the
//!application provides extension struct
//![`CommandBufferInheritanceRenderPassTransformInfoQCOM`] specifying
//!the render pass transform parameters.
//! - The `renderArea`, viewports, scissors, and `fragmentSize` are
//!all provided in the current (non-rotated) coordinate system.
//!The implementation will transform those into the native (rotated)
//!coordinate system.
//! - The implementation is responsible for transforming shader built-ins
//!(`FragCoord`, `PointCoord`, `SamplePosition`,
//!`PrimitiveShadingRateKHR`, interpolateAt(), dFdx, dFdy, fWidth) into
//!the rotated coordinate system.
//! - The implementation is responsible for transforming `position` to the
//!rotated coordinate system.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - Jeff Leger [jackohound](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_QCOM_render_pass_transform]
//!   @jackohound%0A<<Here describe the issue or question you have about the
//!   VK_QCOM_render_pass_transform extension>>)
//!# New structures
//! - Extending [`CommandBufferInheritanceInfo`]:
//! - [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]
//! - Extending [`RenderPassBeginInfo`]:
//! - [`RenderPassTransformBeginInfoQCOM`]
//!# New constants
//! - [`QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME`]
//! - [`QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION`]
//! - Extending [`RenderPassCreateFlagBits`]:
//! - `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM`
//! - `VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM`
//!# Known issues & F.A.Q
//!1) Some early Adreno drivers (October 2019 through March 2020) advertised
//!support for this extension but expected VK_STRUCTURE_TYPE values different
//!from those in the vukan headers.
//!To cover all Adreno devices on the market, applications need to detect the
//!driver version and use the appropriate VK_STRUCTURE_TYPE values from the
//!table below.The driver version reported in VkPhysicalDeviceProperties.driverVersion is a
//!`uint32_t` type.
//!You can decode the `uint32_t` value into a major.minor.patch version as
//!shown below:
//!```c
//!uint32_t  major = ((driverVersion) >> 22);
//!uint32_t  minor = ((driverVersion) >> 12) & 0x3ff);
//!uint32_t  patch = ((driverVersion) & 0xfff);
//!```
//!If the Adreno major.minor.patch version is greater than or equal to to
//!512.469.0, then simply use the VK_STRUCTURE_TYPE values as defined in
//!vulkan_core.h.
//!If the version is less than or equal to to 512.468.0, then use the alternate
//!values for the two VK_STRUCTURE_TYPEs in the table below.2) Should the extension support only
//! rotations (e.g. 90, 180, 270-degrees),
//!or also mirror transforms (e.g. vertical flips)? Mobile use-cases only
//!require rotation.
//!Other display systems such as projectors might require a flipped transform.**RESOLVED**: In this
//! version of the extension, the functionality is
//!restricted to 90, 180, and 270-degree rotations to address mobile use-cases.3) How does this
//! extension interact with VK_EXT_fragment_density_map?**RESOLVED** Some implementations may not be
//! able to support a render pass
//!that enables both render pass transform and fragment density maps.
//!For simplicity, this extension disallows enabling both features within a
//!single render pass.4) What should this extension be named?We considered names such as
//! “rotated_rendering”, “pre_rotation” and
//!others.
//!Since the functionality is limited to a render pass, it seemed the name
//!should include “render_pass”.
//!While the current extension is limited to rotations, it could be extended to
//!other transforms (like mirror) in the future.**RESOLVED** The name “render_pass_transform” seems
//! like the most accurate
//!description of the introduced functionality.5) How does this extension interact with
//! VK_KHR_fragment_shading_rate?**RESOLVED**: For the same reasons as issue 3, this extension
//! disallows
//!enabling both `pFragmentShadingRateAttachment` and render pass transform
//!within a single render pass.However, pipeline shading rate and primitive shading rate are
//! supported, and
//!their respective `fragmentSize` and `PrimitiveShadingRateKHR` are
//!provided in the current (non-rotated) coordinate system.
//!The implementation is responsible for transforming them to the rotated
//!coordinate system.The [set of supported shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate)**may**
//!be different per transform.
//!Supported rates queried from
//![`GetPhysicalDeviceFragmentShadingRatesKHR`] are in the native (rotated)
//!coordinate system.
//!This means that the application **must** swap the x/y of the reported rates to
//!get the set of rates supported for 90 and 270 degree rotation.
//!# Version History
//! - Revision 1, 2020-02-05 (Jeff Leger)
//! - Revision 2, 2021-03-09 (Matthew Netsch)
//! - Adds interactions with VK_KHR_fragment_shading_rate
//!# Other info
//! * 2021-03-09
//!*
//! - This extension requires `[`VK_KHR_swapchain`]`
//! - This extension interacts with `[`VK_EXT_fragment_density_map`]`
//! - This extension interacts with `[`VK_KHR_fragment_shading_rate`]`
//!*
//! - Jeff Leger, Qualcomm Technologies, Inc.
//! - Brandon Light, Qualcomm Technologies, Inc.
//! - Matthew Netsch, Qualcomm Technologies, Inc.
//!# Related
//! - [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]
//! - [`RenderPassTransformBeginInfoQCOM`]
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
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QCOM_render_pass_transform");
