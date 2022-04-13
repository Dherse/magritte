//![VK_NV_device_generated_commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_generated_commands.html) - device extension
//!# Description
//!This extension allows the device to generate a number of critical graphics
//!commands for command buffers.When rendering a large number of objects, the device can be
//! leveraged to
//!implement a number of critical functions, like updating matrices, or
//!implementing occlusion culling, frustum culling, front to back sorting, etc.
//!Implementing those on the device does not require any special extension,
//!since an application is free to define its own data structures, and just
//!process them using shaders.However, if the application desires to quickly kick off the rendering
//! of the
//!final stream of objects, then unextended Vulkan forces the application to
//!read back the processed stream and issue graphics command from the host.
//!For very large scenes, the synchronization overhead and cost to generate the
//!command buffer can become the bottleneck.
//!This extension allows an application to generate a device side stream of
//!state changes and commands, and convert it efficiently into a command buffer
//!without having to read it back to the host.Furthermore, it allows incremental changes to such
//! command buffers by
//!manipulating only partial sections of a command stream — for example
//!pipeline bindings.
//!Unextended Vulkan requires re-creation of entire command buffers in such a
//!scenario, or updates synchronized on the host.The intended usage for this extension is for the
//! application to:
//! - create [`Buffer`] objects and retrieve physical addresses from them via
//!   [`get_buffer_device_address_ext`]
//! - create a graphics pipeline using [`GraphicsPipelineShaderGroupsCreateInfoNV`] for the ability
//!   to change shaders on the device.
//! - create a [`IndirectCommandsLayoutNV`], which lists the [`IndirectCommandsTokenTypeNV`] it
//!   wants to dynamically execute as an atomic command sequence. This step likely involves some
//!   internal device code compilation, since the intent is for the GPU to generate the command
//!   buffer in the pipeline.
//! - fill the input stream buffers with the data for each of the inputs it needs. Each input is an
//!   array that will be filled with token-dependent data.
//! - set up a preprocess [`Buffer`] that uses memory according to the information retrieved via
//!   [`get_generated_commands_memory_requirements_nv`].
//! - optionally preprocess the generated content using [`cmd_preprocess_generated_commands_nv`],
//!   for example on an asynchronous compute queue, or for the purpose of re-using the data in
//!   multiple executions.
//! - call [`cmd_execute_generated_commands_nv`] to create and execute the actual device commands
//!   for all sequences based on the inputs provided.
//!For each draw in a sequence, the following can be specified:
//! - a different shader group
//! - a number of vertex buffer bindings
//! - a different index buffer, with an optional dynamic offset and index type
//! - a number of different push constants
//! - a flag that encodes the primitive winding
//!While the GPU can be faster than a CPU to generate the commands, it will not
//!happen asynchronously to the device, therefore the primary use-case is
//!generating “less” total work (occlusion culling, classification to use
//!specialized shaders, etc.).
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_KHR_buffer_device_address`]`
//!# Contacts
//! - Christoph Kubisch [pixeljetstream](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_generated_commands]
//!   @pixeljetstream%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_generated_commands extension>>)
//!# New handles
//! - [`IndirectCommandsLayoutNV`]
//!# New functions & commands
//! - [`cmd_bind_pipeline_shader_group_nv`]
//! - [`cmd_execute_generated_commands_nv`]
//! - [`cmd_preprocess_generated_commands_nv`]
//! - [`create_indirect_commands_layout_nv`]
//! - [`destroy_indirect_commands_layout_nv`]
//! - [`get_generated_commands_memory_requirements_nv`]
//!# New structures
//! - [`BindIndexBufferIndirectCommandNV`]
//! - [`BindShaderGroupIndirectCommandNV`]
//! - [`BindVertexBufferIndirectCommandNV`]
//! - [`GeneratedCommandsInfoNV`]
//! - [`GeneratedCommandsMemoryRequirementsInfoNV`]
//! - [`GraphicsShaderGroupCreateInfoNV`]
//! - [`IndirectCommandsLayoutCreateInfoNV`]
//! - [`IndirectCommandsLayoutTokenNV`]
//! - [`IndirectCommandsStreamNV`]
//! - [`SetStateFlagsIndirectCommandNV`]
//! - Extending [`GraphicsPipelineCreateInfo`]:  - [`GraphicsPipelineShaderGroupsCreateInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`]
//!# New enums
//! - [`IndirectCommandsLayoutUsageFlagBitsNV`]
//! - [`IndirectCommandsTokenTypeNV`]
//! - [`IndirectStateFlagBitsNV`]
//!# New bitmasks
//! - [`IndirectCommandsLayoutUsageFlagsNV`]
//! - [`IndirectStateFlagsNV`]
//!# New constants
//! - [`NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME`]
//! - [`NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV`  -
//!   `VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
//!# Known issues & F.A.Q
//!1) How to name this extension ?[`VK_NV_device_generated_commands`]As usual, one of the hardest
//! issues ;)Alternatives: `VK_gpu_commands`, `VK_execute_commands`,
//!`VK_device_commands`, `VK_device_execute_commands`, `VK_device_execute`,
//!`VK_device_created_commands`, `VK_device_recorded_commands`,
//!`VK_device_generated_commands``VK_indirect_generated_commands`2) Should we use a serial stateful
//! token stream or stateless sequence
//!descriptions?Similarly to [`Pipeline`], fixed layouts have the most likelihood to be
//!cross-vendor adoptable.
//!They also benefit from being processable in parallel.
//!This is a different design choice compared to the serial command stream
//!generated through `GL_NV_command_list`.3) How to name a sequence
//! description?`VkIndirectCommandsLayout` as in the NVX extension predecessor.Alternative:
//! `VkGeneratedCommandsLayout`4) Do we want to provide `indirectCommands` inputs with layout or at
//!`indirectCommands` time?Separate layout from data as Vulkan does.
//!Provide full flexibility for `indirectCommands`.5) Should the input be provided as SoA or
//! AoS?Both ways are desireable.
//!AoS can provide portability to other APIs and easier to setup, while SoA
//!allows to update individual inputs in a cache-efficient manner, when others
//!remain static.6) How do we make developers aware of the memory requirements of
//!implementation-dependent data used for the generated commands?Make the API explicit and
//! introduce a `preprocess`[`Buffer`].
//!Developers have to allocate it using
//![`get_generated_commands_memory_requirements_nv`].In the NVX version the requirements were
//! hidden implicitly as part of the
//!command buffer reservation process, however as the memory requirements can
//!be substantial, we want to give developers the ability to budget the memory
//!themselves.
//!By lowering the `maxSequencesCount` the memory consumption can be reduced.
//!Furthermore reuse of the memory is possible, for example for doing explicit
//!preprocessing and execution in a ping-pong fashion.The actual buffer size is
//! implementation-dependent and may be zero, i.e. not
//!always required.When making use of Graphics Shader Groups, the programs should behave
//!similar with regards to vertex inputs, clipping and culling outputs of the
//!geometry stage, as well as sample shading behavior in fragment shaders, to
//!reduce the amount of the worst-case memory approximation.7) Should we allow additional
//! per-sequence dynamic state changes?YesIntroduced a lightweight indirect state flag
//![`IndirectStateFlagBitsNV`].
//!So far only switching front face winding state is exposed.
//!Especially in CAD/DCC mirrored transforms that require such changes are
//!common, and similar flexibility is given in the ray tracing instance
//!description.The flag could be extended further, for example to switch between
//!primitive-lists or -strips, or make other state modifications.Furthermore, as new tokens can be
//! added easily, future extension could add
//!the ability to change any [`DynamicState`].8) How do we allow re-using already “generated”
//! `indirectCommands`?Expose a `preprocessBuffer` to reuse implementation-dependencyFlags data.
//!Set the `isPreprocessed` to true in [`cmd_execute_generated_commands_nv`].9) Under which
//! conditions is [`cmd_execute_generated_commands_nv`] legal?It behaves like a regular draw call
//! command.10) Is [`cmd_preprocess_generated_commands_nv`] copying the input data or
//!referencing it?There are multiple implementations possible:
//! - one could have some emulation code that parses the inputs, and generates an output command
//!   buffer, therefore copying the inputs.
//! - one could just reference the inputs, and have the processing done in pipe at execution time.
//!If the data is mandated to be copied, then it puts a penalty on
//!implementation that could process the inputs directly in pipe.
//!If the data is “referenced”, then it allows both types of implementation.The inputs are
//! “referenced”, and  **must**  not be modified after the call to
//![`cmd_execute_generated_commands_nv`] has completed.11) Which buffer usage flags are required
//! for the buffers referenced by
//![`GeneratedCommandsInfoNV`] ?Reuse existing `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT`
//! - [`GeneratedCommandsInfoNV::preprocess_buffer`]
//! - [`GeneratedCommandsInfoNV::sequences_count_buffer`]
//! - [`GeneratedCommandsInfoNV::sequences_index_buffer`]
//! - [`IndirectCommandsStreamNV::buffer`]
//!12) In which pipeline stage does the device generated command expansion
//!happen?[`cmd_preprocess_generated_commands_nv`] is treated as if it occurs in a
//!separate logical pipeline from either graphics or compute, and that pipeline
//!only includes `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT`, a new stage
//!`VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`, and
//!`VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT`.
//!This new stage has two corresponding new access types,
//!`VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV` and
//!`VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`, used to synchronize reading
//!the buffer inputs and writing the preprocess memory output.The generated output written in the
//! preprocess buffer memory by
//![`cmd_execute_generated_commands_nv`] is considered to be consumed by the
//!`VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT` pipeline stage.Thus, to synchronize from writing the input
//! buffers to preprocessing via
//![`cmd_preprocess_generated_commands_nv`], use:
//! - `dstStageMask` = `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//! - `dstAccessMask` = `VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV`
//!To synchronize from [`cmd_preprocess_generated_commands_nv`] to executing
//!the generated commands by [`cmd_execute_generated_commands_nv`], use:
//! - `srcStageMask` = `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//! - `srcAccessMask` = `VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`
//! - `dstStageMask` = `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`
//! - `dstAccessMask` = `VK_ACCESS_INDIRECT_COMMAND_READ_BIT`
//!When [`cmd_execute_generated_commands_nv`] is used with a
//!`isPreprocessed` of [`FALSE`], the generated commands are implicitly
//!preprocessed, therefore one only needs to synchronize the inputs via:
//! - `dstStageMask` = `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`
//! - `dstAccessMask` = `VK_ACCESS_INDIRECT_COMMAND_READ_BIT`
//!13) What if most token data is “static”, but we frequently want to render
//!a subsection?Added “sequencesIndexBuffer”.
//!This allows to easier sort and filter what should actually be executed.14) What are the changes
//! compared to the previous NVX extension?
//! - Compute dispatch support was removed (was never implemented in drivers). There are different
//!   approaches how dispatching from the device should work, hence we defer this to a future
//!   extension.
//! - The `ObjectTableNVX` was replaced by using physical buffer addresses and introducing Shader
//!   Groups for the graphics pipeline.
//! - Less state changes are possible overall, but the important operations are still there (reduces
//!   complexity of implementation).
//! - The API was redesigned so all inputs must be passed at both preprocessing and execution time
//!   (this was implicit in NVX, now it is explicit)
//! - The reservation of intermediate command space is now mandatory and explicit through a
//!   preprocess buffer.
//! - The [`IndirectStateFlagBitsNV`] were introduced
//!15) When porting from other APIs, their indirect buffers may use different
//!    enums, for example for index buffer types.
//!    How to solve this?Added “pIndexTypeValues” to map custom `uint32_t` values to corresponding
//![`IndexType`].16) Do we need more shader group state overrides?The NVX version allowed all PSO
//! states to be different, however as the goal
//!is not to replace all state setup, but focus on highly-frequent state
//!changes for drawing lots of objects, we reduced the amount of state
//!overrides.
//!Especially VkPipelineLayout as well as VkRenderPass configuration should be
//!left static, the rest is still open for discussion.The current focus is just to allow
//! VertexInput changes as well as shaders,
//!while all shader groups use the same shader stages.Too much flexibility will increase the test
//! coverage requirement as well.
//!However, further extensions could allow more dynamic state as well.17) Do we need more detailed
//! physical device feature queries/enables?An EXT version would need detailed implementor feedback
//! to come up with a
//!good set of features.
//!Please contact us if you are interested, we are happy to make more features
//!optional, or add further restrictions to reduce the minimum feature set of
//!an EXT.18) Is there an interaction with VK_KHR_pipeline_library planned?Yes, a future version of
//! this extension will detail the interaction, once
//!VK_KHR_pipeline_library is no longer provisional.
//!# Version History
//! - Revision 1, 2020-02-20 (Christoph Kubisch)  - Initial version
//! - Revision 2, 2020-03-09 (Christoph Kubisch)  - Remove VK_EXT_debug_report interactions
//! - Revision 3, 2020-03-09 (Christoph Kubisch)  - Fix naming VkPhysicalDeviceGenerated to
//!   VkPhysicalDeviceDeviceGenerated
//!# Other info
//! * 2020-02-20
//! * - This extension requires Vulkan 1.1  - This extension requires
//!   [`VK_EXT_buffer_device_address`] or [`VK_KHR_buffer_device_address`] or Vulkan 1.2 for the
//!   ability to bind vertex and index buffers on the device.  - This extension interacts with
//!   [`VK_NV_mesh_shader`]. If the latter extension is not supported, remove the command token to
//!   initiate mesh tasks drawing in this extension.
//! * - Christoph Kubisch, NVIDIA  - Pierre Boudier, NVIDIA  - Jeff Bolz, NVIDIA  - Eric Werness,
//!   NVIDIA  - Yuriy O’Donnell, Epic Games  - Baldur Karlsson, Valve  - Mathias Schott, NVIDIA  -
//!   Tyson Smith, NVIDIA  - Ingo Esser, NVIDIA
//!# Related
//! - [`BindIndexBufferIndirectCommandNV`]
//! - [`BindShaderGroupIndirectCommandNV`]
//! - [`BindVertexBufferIndirectCommandNV`]
//! - [`GeneratedCommandsInfoNV`]
//! - [`GeneratedCommandsMemoryRequirementsInfoNV`]
//! - [`GraphicsPipelineShaderGroupsCreateInfoNV`]
//! - [`GraphicsShaderGroupCreateInfoNV`]
//! - [`IndirectCommandsLayoutCreateInfoNV`]
//! - [`IndirectCommandsLayoutNV`]
//! - [`IndirectCommandsLayoutTokenNV`]
//! - [`IndirectCommandsLayoutUsageFlagBitsNV`]
//! - [`IndirectCommandsLayoutUsageFlagsNV`]
//! - [`IndirectCommandsStreamNV`]
//! - [`IndirectCommandsTokenTypeNV`]
//! - [`IndirectStateFlagBitsNV`]
//! - [`IndirectStateFlagsNV`]
//! - [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`]
//! - [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`]
//! - [`SetStateFlagsIndirectCommandNV`]
//! - [`cmd_bind_pipeline_shader_group_nv`]
//! - [`cmd_execute_generated_commands_nv`]
//! - [`cmd_preprocess_generated_commands_nv`]
//! - [`create_indirect_commands_layout_nv`]
//! - [`destroy_indirect_commands_layout_nv`]
//! - [`get_generated_commands_memory_requirements_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceAddress,
        DeviceSize, IndexType, Instance, PhysicalDevice, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo, PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo,
        ShaderStageFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_1::MemoryRequirements2,
    AsRaw, Handle, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_generated_commands");
///[vkGetGeneratedCommandsMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) - Retrieve the buffer allocation requirements for generated commands
///# C Specifications
///The generation of commands on the device requires a `preprocess` buffer.
///To retrieve the memory size and alignment requirements of a particular
///execution state call:
///```c
///// Provided by VK_NV_device_generated_commands
///void vkGetGeneratedCommandsMemoryRequirementsNV(
///    VkDevice                                    device,
///    const VkGeneratedCommandsMemoryRequirementsInfoNV* pInfo,
///    VkMemoryRequirements2*                      pMemoryRequirements);
///```
///# Parameters
/// - [`device`] is the logical device that owns the buffer.
/// - [`p_info`] is a pointer to a [`GeneratedCommandsMemoryRequirementsInfoNV`] structure
///   containing parameters required for the memory requirements query.
/// - [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the
///   memory requirements of the buffer object are returned.
///# Description
///## Valid Usage
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_info`] **must**  be a valid pointer to a valid
///   [`GeneratedCommandsMemoryRequirementsInfoNV`] structure
/// - [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`] structure
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Device`]
/// - [`GeneratedCommandsMemoryRequirementsInfoNV`]
/// - [`MemoryRequirements2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
pub type FNGetGeneratedCommandsMemoryRequirementsNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'lt>,
        p_memory_requirements: *mut MemoryRequirements2<'lt>,
    ),
>;
///[vkCreateIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) - Create an indirect command layout object
///# C Specifications
///Indirect command layouts are created by:
///```c
///// Provided by VK_NV_device_generated_commands
///VkResult vkCreateIndirectCommandsLayoutNV(
///    VkDevice                                    device,
///    const VkIndirectCommandsLayoutCreateInfoNV* pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkIndirectCommandsLayoutNV*                 pIndirectCommandsLayout);
///```
///# Parameters
/// - [`device`] is the logical device that creates the indirect command layout.
/// - [`p_create_info`] is a pointer to a [`IndirectCommandsLayoutCreateInfoNV`] structure
///   containing parameters affecting creation of the indirect command layout.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_indirect_commands_layout`] is a pointer to a [`IndirectCommandsLayoutNV`] handle in which
///   the resulting indirect command layout is returned.
///# Description
///## Valid Usage
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid
///   [`IndirectCommandsLayoutCreateInfoNV`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_indirect_commands_layout`] **must**  be a valid pointer to a [`IndirectCommandsLayoutNV`]
///   handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`IndirectCommandsLayoutCreateInfoNV`]
/// - [`IndirectCommandsLayoutNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
pub type FNCreateIndirectCommandsLayoutNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> VulkanResultCodes,
>;
///[vkDestroyIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) - Destroy an indirect commands layout
///# C Specifications
///Indirect command layouts are destroyed by:
///```c
///// Provided by VK_NV_device_generated_commands
///void vkDestroyIndirectCommandsLayoutNV(
///    VkDevice                                    device,
///    VkIndirectCommandsLayoutNV                  indirectCommandsLayout,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the logical device that destroys the layout.
/// - [`indirect_commands_layout`] is the layout to destroy.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
///# Description
///## Valid Usage
/// - All submitted commands that refer to [`indirect_commands_layout`] **must**  have completed
///   execution
/// - If [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created, a
///   compatible set of callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created,
///   [`p_allocator`] **must**  be `NULL`
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`indirect_commands_layout`] is not [`crate::Handle::null`], [`indirect_commands_layout`]
///   **must**  be a valid [`IndirectCommandsLayoutNV`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`indirect_commands_layout`] is a valid handle, it  **must**  have been created, allocated,
///   or retrieved from [`device`]
///
///## Host Synchronization
/// - Host access to [`indirect_commands_layout`] **must**  be externally synchronized
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`IndirectCommandsLayoutNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
pub type FNDestroyIndirectCommandsLayoutNv = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkCmdExecuteGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) - Generate and execute commands on the device
///# C Specifications
///The actual generation of commands as well as their execution on the device
///is handled as single action with:
///```c
///// Provided by VK_NV_device_generated_commands
///void vkCmdExecuteGeneratedCommandsNV(
///    VkCommandBuffer                             commandBuffer,
///    VkBool32                                    isPreprocessed,
///    const VkGeneratedCommandsInfoNV*            pGeneratedCommandsInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`is_preprocessed`] represents whether the input data has already been preprocessed on the
///   device. If it is [`FALSE`] this command will implicitly trigger the preprocessing step,
///   otherwise not.
/// - [`p_generated_commands_info`] is a pointer to a [`GeneratedCommandsInfoNV`] structure
///   containing parameters affecting the generation of commands.
///# Description
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
///   so after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The subpass index of the current render pass  **must**  be equal to the `subpass` member of
///   the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via a
///   descriptor set
/// - Memory backing image subresources used as attachments in the current render pass  **must**
///   not be written in any way other than as an attachment by this command
/// - If any recorded command in the current subpass will write to an image subresource as an
///   attachment, this command  **must**  not read from the memory backing that image subresource in
///   any other way than as an attachment
/// - If any recorded command in the current subpass will read from an image subresource used as an
///   attachment in any way other than as an attachment, this command  **must**  not write to that
///   image subresource as an attachment
/// - If the draw is recorded in a render pass instance with multiview enabled, the maximum instance
///   index  **must**  be less than or equal to
///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
/// - If the bound graphics pipeline was created with
///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and the
///   current subpass has a depth/stencil attachment, then that attachment  **must**  have been
///   created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
/// - If the bound graphics pipeline state was created with both the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`]
///   **must**  have been called in the current command buffer prior to this drawing command, and
///   the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the
///   `scissorCount` parameter of [`cmd_set_scissor_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
///   pipeline  **must**  have been created with
///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic
///   states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount`
///   parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
///   graphics pipeline  **must**  have been created with
///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
///   dynamic states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
///   dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the
///   current command buffer prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current
///   command buffer prior to this drawing command and the `logicOp` **must**  be a valid
///   [`LogicOp`] value
/// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   state enabled, and any of the shader stages of the bound graphics pipeline write to the
///   `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been
///   called in the current command buffer prior to this drawing command, and the `viewportCount`
///   parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
/// - If rasterization is not disabled in the bound graphics pipeline, then for each color
///   attachment in the subpass, if the corresponding image view’s [format features]() do not
///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the
///   corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
///   [`FALSE`]
/// - If rasterization is not disabled in the bound graphics pipeline, and neither the
///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]` extensions
///   are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be
///   the same as the current subpass color and/or depth/stencil attachments
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this
///   command  **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this
///   command  **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not
///   write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**
///   not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
///   [`RenderingInfo::color_attachment_count`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
///   create the currently bound graphics pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `attachmentCount` parameter of
///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
///   pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently
///   bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
/// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the corresponding element of the `pColorAttachmentSamples` member of
///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
///   currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used
///   to create the currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`]
///   equal to [`crate::Handle::null`]
///
/// - All vertex input bindings accessed via vertex input variables declared in the vertex shader
///   entry point’s interface  **must**  have either valid or [`crate::Handle::null`] buffers bound
/// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
///   vertex input variables declared in the vertex shader entry point’s interface  **must**  not be
///   [`crate::Handle::null`]
/// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained
///   within the corresponding vertex buffer binding, as described in [[fxvertex-input]]()
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `primitiveTopology` parameter of
///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the pipeline
///   [`PipelineInputAssemblyStateCreateInfo::topology`] state
/// - If the bound graphics pipeline was created with both the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic states enabled, then
///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer prior to
///   this draw command
/// - If the bound graphics pipeline was created with the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
///   prior to this draw command, and the `pStrides` parameter of [`cmd_bind_vertex_buffers2_ext`]
///   **must**  not be `NULL`
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   dynamic state enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in the
///   current command buffer prior to this draw command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
///   buffer prior to this drawing command
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
///   `VK_SHADER_STAGE_MESH_BIT_NV`
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - If [`is_preprocessed`] is [`TRUE`] then [`cmd_preprocess_generated_commands_nv`] **must**
///   have already been executed on the device, using the same [`p_generated_commands_info`] content
///   as well as the content of the input buffers it references (all except
///   [`GeneratedCommandsInfoNV::preprocess_buffer`]). Furthermore [`p_generated_commands_info`]`s
///   `indirectCommandsLayout` **must**  have been created with the
///   `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV` bit set
/// - [`GeneratedCommandsInfoNV::pipeline`] **must**  match the current bound pipeline at
///   [`GeneratedCommandsInfoNV::pipeline_bind_point`]
/// - Transform feedback  **must**  not be active
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_generated_commands_info`] **must**  be a valid pointer to a valid
///   [`GeneratedCommandsInfoNV`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - This command  **must**  only be called inside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Bool32`]
/// - [`CommandBuffer`]
/// - [`GeneratedCommandsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
pub type FNCmdExecuteGeneratedCommandsNv = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'lt>,
    ),
>;
///[vkCmdPreprocessGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) - Performs preprocessing for generated commands
///# C Specifications
///Commands  **can**  be preprocessed prior execution using the following command:
///```c
///// Provided by VK_NV_device_generated_commands
///void vkCmdPreprocessGeneratedCommandsNV(
///    VkCommandBuffer                             commandBuffer,
///    const VkGeneratedCommandsInfoNV*            pGeneratedCommandsInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer which does the preprocessing.
/// - [`p_generated_commands_info`] is a pointer to a [`GeneratedCommandsInfoNV`] structure
///   containing parameters affecting the preprocessing step.
///# Description
///## Valid Usage
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - [`p_generated_commands_info`]`s `indirectCommandsLayout` **must**  have been created with the
///   `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV` bit set
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_generated_commands_info`] **must**  be a valid pointer to a valid
///   [`GeneratedCommandsInfoNV`] structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - This command  **must**  only be called outside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`CommandBuffer`]
/// - [`GeneratedCommandsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
pub type FNCmdPreprocessGeneratedCommandsNv = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'lt>,
    ),
>;
///[vkCmdBindPipelineShaderGroupNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) - Bind a pipeline object
///# C Specifications
///For pipelines that were created with the support of multiple shader groups
///(see [Graphics Pipeline Shader Groups](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#graphics-shadergroups)), the regular
///[`cmd_bind_pipeline`] command will bind Shader Group `0`.
///To explicitly bind a shader group use:
///```c
///// Provided by VK_NV_device_generated_commands
///void vkCmdBindPipelineShaderGroupNV(
///    VkCommandBuffer                             commandBuffer,
///    VkPipelineBindPoint                         pipelineBindPoint,
///    VkPipeline                                  pipeline,
///    uint32_t                                    groupIndex);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer that the pipeline will be bound to.
/// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the bind point to which
///   the pipeline will be bound.
/// - [`pipeline`] is the pipeline to be bound.
/// - [`group_index`] is the shader group to be bound.
///# Description
///## Valid Usage
/// - [`group_index`] **must**  be `0` or less than the effective
///   [`GraphicsPipelineShaderGroupsCreateInfoNV::group_count`] including the referenced pipelines
/// - The [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The same restrictions as [`cmd_bind_pipeline`] apply as if the bound pipeline was created only
///   with the Shader Group from the [`group_index`] information
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - Both of [`command_buffer`], and [`pipeline`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`CommandBuffer`]
/// - [`Pipeline`]
/// - [`PipelineBindPoint`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
pub type FNCmdBindPipelineShaderGroupNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ),
>;
///[VkIndirectCommandsTokenTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) - Enum specifying token commands
///# C Specifications
///Possible values of those elements of the
///[`IndirectCommandsLayoutCreateInfoNV::tokens`] array specifying
///command tokens (other elements of the array specify command parameters) are:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectCommandsTokenTypeNV {
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV = 0,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV = 1,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV = 2,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV = 3,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV = 4,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV = 5,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV = 6,
///    VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV = 7,
///} VkIndirectCommandsTokenTypeNV;
///```
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutTokenNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsTokenTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IndirectCommandsTokenTypeNV(i32);
impl const Default for IndirectCommandsTokenTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl IndirectCommandsTokenTypeNV {
    ///No documentation found
    pub const SHADER_GROUP: Self = Self(0);
    ///No documentation found
    pub const STATE_FLAGS: Self = Self(1);
    ///No documentation found
    pub const INDEX_BUFFER: Self = Self(2);
    ///No documentation found
    pub const VERTEX_BUFFER: Self = Self(3);
    ///No documentation found
    pub const PUSH_CONSTANT: Self = Self(4);
    ///No documentation found
    pub const DRAW_INDEXED: Self = Self(5);
    ///No documentation found
    pub const DRAW: Self = Self(6);
    ///No documentation found
    pub const DRAW_TASKS: Self = Self(7);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(IndirectCommandsTokenTypeNV))
            .field(match *self {
                Self::SHADER_GROUP => &"SHADER_GROUP",
                Self::STATE_FLAGS => &"STATE_FLAGS",
                Self::INDEX_BUFFER => &"INDEX_BUFFER",
                Self::VERTEX_BUFFER => &"VERTEX_BUFFER",
                Self::PUSH_CONSTANT => &"PUSH_CONSTANT",
                Self::DRAW_INDEXED => &"DRAW_INDEXED",
                Self::DRAW => &"DRAW",
                Self::DRAW_TASKS => &"DRAW_TASKS",
                other => unreachable!(
                    concat!("invalid value for", stringify!(IndirectCommandsTokenTypeNV), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::SHADER_GROUP => &"SHADER_GROUP",
            Self::STATE_FLAGS => &"STATE_FLAGS",
            Self::INDEX_BUFFER => &"INDEX_BUFFER",
            Self::VERTEX_BUFFER => &"VERTEX_BUFFER",
            Self::PUSH_CONSTANT => &"PUSH_CONSTANT",
            Self::DRAW_INDEXED => &"DRAW_INDEXED",
            Self::DRAW => &"DRAW",
            Self::DRAW_TASKS => &"DRAW_TASKS",
            other => unreachable!(
                concat!("invalid value for", stringify!(IndirectCommandsTokenTypeNV), ": {:?}"),
                other
            ),
        })
    }
}
///[VkIndirectCommandsLayoutUsageFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) - Bitmask specifying allowed usage of an indirect commands layout
///# C Specifications
///Bits which  **can**  be set in
///[`IndirectCommandsLayoutCreateInfoNV::flags`], specifying usage
///hints of an indirect command layout, are:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectCommandsLayoutUsageFlagBitsNV {
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 0x00000001,
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 0x00000002,
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 0x00000004,
///} VkIndirectCommandsLayoutUsageFlagBitsNV;
///```
///# Description
/// - [`EXPLICIT_PREPROCESS`] specifies that the layout is always used with the manual preprocessing
///   step through calling [`cmd_preprocess_generated_commands_nv`] and executed by
///   [`cmd_execute_generated_commands_nv`] with `isPreprocessed` set to [`TRUE`].
/// - [`INDEXED_SEQUENCES`] specifies that the input data for the sequences is not implicitly
///   indexed from 0..sequencesUsed but a user provided [`Buffer`] encoding the index is provided.
/// - [`UNORDERED_SEQUENCES`] specifies that the processing of sequences  **can**  happen at an
///   implementation-dependent order, which is not: guaranteed to be coherent using the same input
///   data.
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutUsageFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagBitsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
impl const Default for IndirectCommandsLayoutUsageFlagBitsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl IndirectCommandsLayoutUsageFlagBitsNV {
    ///[`EXPLICIT_PREPROCESS`]
    ///specifies that the layout is always used with the manual preprocessing
    ///step through calling [`cmd_preprocess_generated_commands_nv`] and
    ///executed by [`cmd_execute_generated_commands_nv`] with `isPreprocessed`
    ///set to [`TRUE`].
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    ///[`INDEXED_SEQUENCES`]
    ///specifies that the input data for the sequences is not implicitly
    ///indexed from 0..sequencesUsed but a user provided [`Buffer`]
    ///encoding the index is provided.
    pub const INDEXED_SEQUENCES: Self = Self(2);
    ///[`UNORDERED_SEQUENCES`]
    ///specifies that the processing of sequences  **can**  happen at an
    ///implementation-dependent order, which is not: guaranteed to be coherent
    ///using the same input data.
    pub const UNORDERED_SEQUENCES: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutUsageFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(IndirectCommandsLayoutUsageFlagBitsNV))
            .field(match *self {
                Self::EXPLICIT_PREPROCESS => &"EXPLICIT_PREPROCESS",
                Self::INDEXED_SEQUENCES => &"INDEXED_SEQUENCES",
                Self::UNORDERED_SEQUENCES => &"UNORDERED_SEQUENCES",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(IndirectCommandsLayoutUsageFlagBitsNV),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for IndirectCommandsLayoutUsageFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::EXPLICIT_PREPROCESS => &"EXPLICIT_PREPROCESS",
            Self::INDEXED_SEQUENCES => &"INDEXED_SEQUENCES",
            Self::UNORDERED_SEQUENCES => &"UNORDERED_SEQUENCES",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(IndirectCommandsLayoutUsageFlagBitsNV),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkIndirectStateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) - Bitmask specifiying state that can be altered on the device
///# C Specifications
///A subset of the graphics pipeline state  **can**  be altered using indirect state
///flags:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectStateFlagBitsNV {
///    VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 0x00000001,
///} VkIndirectStateFlagBitsNV;
///```
///# Description
/// - [`FLAG_FRONTFACE`] allows to toggle the [`FrontFace`] rasterization state for subsequent draw
///   operations.
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectStateFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectStateFlagBitsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IndirectStateFlagBitsNV(u32);
impl const Default for IndirectStateFlagBitsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl IndirectStateFlagBitsNV {
    ///[`FLAG_FRONTFACE`] allows to toggle the
    ///[`FrontFace`] rasterization state for subsequent draw operations.
    pub const FLAG_FRONTFACE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for IndirectStateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(IndirectStateFlagBitsNV))
            .field(match *self {
                Self::FLAG_FRONTFACE => &"FLAG_FRONTFACE",
                other => unreachable!(
                    concat!("invalid value for", stringify!(IndirectStateFlagBitsNV), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for IndirectStateFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::FLAG_FRONTFACE => &"FLAG_FRONTFACE",
            other => unreachable!(
                concat!("invalid value for", stringify!(IndirectStateFlagBitsNV), ": {:?}"),
                other
            ),
        })
    }
}
///[VkIndirectCommandsLayoutUsageFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html) - Bitmask specifying allowed usage of an indirect commands layout
///# C Specifications
///Bits which  **can**  be set in
///[`IndirectCommandsLayoutCreateInfoNV::flags`], specifying usage
///hints of an indirect command layout, are:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectCommandsLayoutUsageFlagBitsNV {
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 0x00000001,
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 0x00000002,
///    VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 0x00000004,
///} VkIndirectCommandsLayoutUsageFlagBitsNV;
///```
///# Description
/// - [`EXPLICIT_PREPROCESS`] specifies that the layout is always used with the manual preprocessing
///   step through calling [`cmd_preprocess_generated_commands_nv`] and executed by
///   [`cmd_execute_generated_commands_nv`] with `isPreprocessed` set to [`TRUE`].
/// - [`INDEXED_SEQUENCES`] specifies that the input data for the sequences is not implicitly
///   indexed from 0..sequencesUsed but a user provided [`Buffer`] encoding the index is provided.
/// - [`UNORDERED_SEQUENCES`] specifies that the processing of sequences  **can**  happen at an
///   implementation-dependent order, which is not: guaranteed to be coherent using the same input
///   data.
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutUsageFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsLayoutUsageFlagsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IndirectCommandsLayoutUsageFlagsNV(u32);
impl const Default for IndirectCommandsLayoutUsageFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl From<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from(from: IndirectCommandsLayoutUsageFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl IndirectCommandsLayoutUsageFlagsNV {
    ///[`EXPLICIT_PREPROCESS`]
    ///specifies that the layout is always used with the manual preprocessing
    ///step through calling [`cmd_preprocess_generated_commands_nv`] and
    ///executed by [`cmd_execute_generated_commands_nv`] with `isPreprocessed`
    ///set to [`TRUE`].
    pub const EXPLICIT_PREPROCESS: Self = Self(1);
    ///[`INDEXED_SEQUENCES`]
    ///specifies that the input data for the sequences is not implicitly
    ///indexed from 0..sequencesUsed but a user provided [`Buffer`]
    ///encoding the index is provided.
    pub const INDEXED_SEQUENCES: Self = Self(2);
    ///[`UNORDERED_SEQUENCES`]
    ///specifies that the processing of sequences  **can**  happen at an
    ///implementation-dependent order, which is not: guaranteed to be coherent
    ///using the same input data.
    pub const UNORDERED_SEQUENCES: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::EXPLICIT_PREPROCESS;
        }
        {
            all |= Self::INDEXED_SEQUENCES;
        }
        {
            all |= Self::UNORDERED_SEQUENCES;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for IndirectCommandsLayoutUsageFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<IndirectCommandsLayoutUsageFlagBitsNV>>::from(i));
        }
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectCommandsLayoutUsageFlagBitsNV>>(
        iterator: T,
    ) -> IndirectCommandsLayoutUsageFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectCommandsLayoutUsageFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectCommandsLayoutUsageFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectCommandsLayoutUsageFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectCommandsLayoutUsageFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::EXPLICIT_PREPROCESS) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXPLICIT_PREPROCESS))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::INDEXED_SEQUENCES) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(INDEXED_SEQUENCES))?;
                    }
                    if self.0.contains(IndirectCommandsLayoutUsageFlagsNV::UNORDERED_SEQUENCES) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(UNORDERED_SEQUENCES))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectCommandsLayoutUsageFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkIndirectStateFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectStateFlagBitsNV.html) - Bitmask specifiying state that can be altered on the device
///# C Specifications
///A subset of the graphics pipeline state  **can**  be altered using indirect state
///flags:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef enum VkIndirectStateFlagBitsNV {
///    VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 0x00000001,
///} VkIndirectStateFlagBitsNV;
///```
///# Description
/// - [`FLAG_FRONTFACE`] allows to toggle the [`FrontFace`] rasterization state for subsequent draw
///   operations.
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectStateFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectStateFlagsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct IndirectStateFlagsNV(u32);
impl const Default for IndirectStateFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl From<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from(from: IndirectStateFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl IndirectStateFlagsNV {
    ///[`FLAG_FRONTFACE`] allows to toggle the
    ///[`FrontFace`] rasterization state for subsequent draw operations.
    pub const FLAG_FRONTFACE: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::FLAG_FRONTFACE;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for IndirectStateFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for IndirectStateFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for IndirectStateFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn extend<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<IndirectStateFlagBitsNV>>::from(i));
        }
    }
}
impl FromIterator<IndirectStateFlagsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from_iter<T: IntoIterator<Item = IndirectStateFlagBitsNV>>(iterator: T) -> IndirectStateFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<IndirectStateFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for IndirectStateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(IndirectStateFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == IndirectStateFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(IndirectStateFlagsNV::FLAG_FRONTFACE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(FLAG_FRONTFACE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(IndirectStateFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html) - Structure describing the device-generated commands features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           deviceGeneratedCommands;
///} VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_generated_commands`] indicates whether the implementation supports functionality to generate commands on the device. See [Device-Generated Commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands).
///If the [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`
///# Related
/// - [`VK_NV_device_generated_commands`]
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
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`device_generated_commands`]
    ///indicates whether the implementation supports functionality to generate
    ///commands on the device.
    ///See [Device-Generated Commands](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-generated-commands).
    pub device_generated_commands: Bool32,
}
impl<'lt> Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            device_generated_commands: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_generated_commands`]
    pub fn device_generated_commands_raw(&self) -> Bool32 {
        self.device_generated_commands
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_generated_commands`]
    pub fn set_device_generated_commands_raw(mut self, value: Bool32) -> Self {
        self.device_generated_commands = value;
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
    ///Gets the value of [`Self::device_generated_commands`]
    pub fn device_generated_commands(&self) -> bool {
        unsafe { std::mem::transmute(self.device_generated_commands as u8) }
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
    ///Gets a mutable reference to the value of [`Self::device_generated_commands`]
    pub fn device_generated_commands_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.device_generated_commands as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.device_generated_commands as *mut Bool32)
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
    ///Sets the value of [`Self::device_generated_commands`]
    pub fn set_device_generated_commands(mut self, value: bool) -> Self {
        self.device_generated_commands = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html) - Structure describing push descriptor limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxGraphicsShaderGroupCount;
///    uint32_t           maxIndirectSequenceCount;
///    uint32_t           maxIndirectCommandsTokenCount;
///    uint32_t           maxIndirectCommandsStreamCount;
///    uint32_t           maxIndirectCommandsTokenOffset;
///    uint32_t           maxIndirectCommandsStreamStride;
///    uint32_t           minSequencesCountBufferOffsetAlignment;
///    uint32_t           minSequencesIndexBufferOffsetAlignment;
///    uint32_t           minIndirectCommandsBufferOffsetAlignment;
///} VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_graphics_shader_group_count`] is the maximum number of shader groups in
///   [`GraphicsPipelineShaderGroupsCreateInfoNV`].
/// - [`max_indirect_sequence_count`] is the maximum number of sequences in
///   [`GeneratedCommandsInfoNV`] and in [`GeneratedCommandsMemoryRequirementsInfoNV`].
/// - [`max_indirect_commands_token_count`] is the maximum number of tokens in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`max_indirect_commands_stream_count`] is the maximum number of streams in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`max_indirect_commands_token_offset`] is the maximum offset in
///   [`IndirectCommandsLayoutTokenNV`].
/// - [`max_indirect_commands_stream_stride`] is the maximum stream stride in
///   [`IndirectCommandsLayoutCreateInfoNV`].
/// - [`min_sequences_count_buffer_offset_alignment`] is the minimum alignment for memory addresses
///   which  **can**  be used in [`GeneratedCommandsInfoNV`].
/// - [`min_sequences_index_buffer_offset_alignment`] is the minimum alignment for memory addresses
///   which  **can**  be used in [`GeneratedCommandsInfoNV`].
/// - [`min_indirect_commands_buffer_offset_alignment`] is the minimum alignment for memory
///   addresses used in [`IndirectCommandsStreamNV`], and as preprocess buffer in
///   [`GeneratedCommandsInfoNV`].
///# Description
///If the [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_graphics_shader_group_count`] is the maximum number of shader groups
    ///in [`GraphicsPipelineShaderGroupsCreateInfoNV`].
    pub max_graphics_shader_group_count: u32,
    ///[`max_indirect_sequence_count`] is the maximum number of sequences in
    ///[`GeneratedCommandsInfoNV`] and in
    ///[`GeneratedCommandsMemoryRequirementsInfoNV`].
    pub max_indirect_sequence_count: u32,
    ///[`max_indirect_commands_token_count`] is the maximum number of tokens in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    pub max_indirect_commands_token_count: u32,
    ///[`max_indirect_commands_stream_count`] is the maximum number of streams in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    pub max_indirect_commands_stream_count: u32,
    ///[`max_indirect_commands_token_offset`] is the maximum offset in
    ///[`IndirectCommandsLayoutTokenNV`].
    pub max_indirect_commands_token_offset: u32,
    ///[`max_indirect_commands_stream_stride`] is the maximum stream stride in
    ///[`IndirectCommandsLayoutCreateInfoNV`].
    pub max_indirect_commands_stream_stride: u32,
    ///[`min_sequences_count_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses which  **can**  be used in
    ///[`GeneratedCommandsInfoNV`].
    pub min_sequences_count_buffer_offset_alignment: u32,
    ///[`min_sequences_index_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses which  **can**  be used in
    ///[`GeneratedCommandsInfoNV`].
    pub min_sequences_index_buffer_offset_alignment: u32,
    ///[`min_indirect_commands_buffer_offset_alignment`] is the minimum alignment
    ///for memory addresses used in [`IndirectCommandsStreamNV`], and as
    ///preprocess buffer in [`GeneratedCommandsInfoNV`].
    pub min_indirect_commands_buffer_offset_alignment: u32,
}
impl<'lt> Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_graphics_shader_group_count: 0,
            max_indirect_sequence_count: 0,
            max_indirect_commands_token_count: 0,
            max_indirect_commands_stream_count: 0,
            max_indirect_commands_token_offset: 0,
            max_indirect_commands_stream_stride: 0,
            min_sequences_count_buffer_offset_alignment: 0,
            min_sequences_index_buffer_offset_alignment: 0,
            min_indirect_commands_buffer_offset_alignment: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::max_graphics_shader_group_count`]
    pub fn max_graphics_shader_group_count(&self) -> u32 {
        self.max_graphics_shader_group_count
    }
    ///Gets the value of [`Self::max_indirect_sequence_count`]
    pub fn max_indirect_sequence_count(&self) -> u32 {
        self.max_indirect_sequence_count
    }
    ///Gets the value of [`Self::max_indirect_commands_token_count`]
    pub fn max_indirect_commands_token_count(&self) -> u32 {
        self.max_indirect_commands_token_count
    }
    ///Gets the value of [`Self::max_indirect_commands_stream_count`]
    pub fn max_indirect_commands_stream_count(&self) -> u32 {
        self.max_indirect_commands_stream_count
    }
    ///Gets the value of [`Self::max_indirect_commands_token_offset`]
    pub fn max_indirect_commands_token_offset(&self) -> u32 {
        self.max_indirect_commands_token_offset
    }
    ///Gets the value of [`Self::max_indirect_commands_stream_stride`]
    pub fn max_indirect_commands_stream_stride(&self) -> u32 {
        self.max_indirect_commands_stream_stride
    }
    ///Gets the value of [`Self::min_sequences_count_buffer_offset_alignment`]
    pub fn min_sequences_count_buffer_offset_alignment(&self) -> u32 {
        self.min_sequences_count_buffer_offset_alignment
    }
    ///Gets the value of [`Self::min_sequences_index_buffer_offset_alignment`]
    pub fn min_sequences_index_buffer_offset_alignment(&self) -> u32 {
        self.min_sequences_index_buffer_offset_alignment
    }
    ///Gets the value of [`Self::min_indirect_commands_buffer_offset_alignment`]
    pub fn min_indirect_commands_buffer_offset_alignment(&self) -> u32 {
        self.min_indirect_commands_buffer_offset_alignment
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
    ///Gets a mutable reference to the value of [`Self::max_graphics_shader_group_count`]
    pub fn max_graphics_shader_group_count_mut(&mut self) -> &mut u32 {
        &mut self.max_graphics_shader_group_count
    }
    ///Gets a mutable reference to the value of [`Self::max_indirect_sequence_count`]
    pub fn max_indirect_sequence_count_mut(&mut self) -> &mut u32 {
        &mut self.max_indirect_sequence_count
    }
    ///Gets a mutable reference to the value of [`Self::max_indirect_commands_token_count`]
    pub fn max_indirect_commands_token_count_mut(&mut self) -> &mut u32 {
        &mut self.max_indirect_commands_token_count
    }
    ///Gets a mutable reference to the value of [`Self::max_indirect_commands_stream_count`]
    pub fn max_indirect_commands_stream_count_mut(&mut self) -> &mut u32 {
        &mut self.max_indirect_commands_stream_count
    }
    ///Gets a mutable reference to the value of [`Self::max_indirect_commands_token_offset`]
    pub fn max_indirect_commands_token_offset_mut(&mut self) -> &mut u32 {
        &mut self.max_indirect_commands_token_offset
    }
    ///Gets a mutable reference to the value of [`Self::max_indirect_commands_stream_stride`]
    pub fn max_indirect_commands_stream_stride_mut(&mut self) -> &mut u32 {
        &mut self.max_indirect_commands_stream_stride
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_sequences_count_buffer_offset_alignment`]
    pub fn min_sequences_count_buffer_offset_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_sequences_count_buffer_offset_alignment
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_sequences_index_buffer_offset_alignment`]
    pub fn min_sequences_index_buffer_offset_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_sequences_index_buffer_offset_alignment
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_indirect_commands_buffer_offset_alignment`]
    pub fn min_indirect_commands_buffer_offset_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_indirect_commands_buffer_offset_alignment
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
    ///Sets the value of [`Self::max_graphics_shader_group_count`]
    pub fn set_max_graphics_shader_group_count(mut self, value: u32) -> Self {
        self.max_graphics_shader_group_count = value;
        self
    }
    ///Sets the value of [`Self::max_indirect_sequence_count`]
    pub fn set_max_indirect_sequence_count(mut self, value: u32) -> Self {
        self.max_indirect_sequence_count = value;
        self
    }
    ///Sets the value of [`Self::max_indirect_commands_token_count`]
    pub fn set_max_indirect_commands_token_count(mut self, value: u32) -> Self {
        self.max_indirect_commands_token_count = value;
        self
    }
    ///Sets the value of [`Self::max_indirect_commands_stream_count`]
    pub fn set_max_indirect_commands_stream_count(mut self, value: u32) -> Self {
        self.max_indirect_commands_stream_count = value;
        self
    }
    ///Sets the value of [`Self::max_indirect_commands_token_offset`]
    pub fn set_max_indirect_commands_token_offset(mut self, value: u32) -> Self {
        self.max_indirect_commands_token_offset = value;
        self
    }
    ///Sets the value of [`Self::max_indirect_commands_stream_stride`]
    pub fn set_max_indirect_commands_stream_stride(mut self, value: u32) -> Self {
        self.max_indirect_commands_stream_stride = value;
        self
    }
    ///Sets the value of [`Self::min_sequences_count_buffer_offset_alignment`]
    pub fn set_min_sequences_count_buffer_offset_alignment(mut self, value: u32) -> Self {
        self.min_sequences_count_buffer_offset_alignment = value;
        self
    }
    ///Sets the value of [`Self::min_sequences_index_buffer_offset_alignment`]
    pub fn set_min_sequences_index_buffer_offset_alignment(mut self, value: u32) -> Self {
        self.min_sequences_index_buffer_offset_alignment = value;
        self
    }
    ///Sets the value of [`Self::min_indirect_commands_buffer_offset_alignment`]
    pub fn set_min_indirect_commands_buffer_offset_alignment(mut self, value: u32) -> Self {
        self.min_indirect_commands_buffer_offset_alignment = value;
        self
    }
}
///[VkGraphicsShaderGroupCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsShaderGroupCreateInfoNV.html) - Structure specifying override parameters for each shader group
///# C Specifications
///The [`GraphicsShaderGroupCreateInfoNV`] structure provides the state
///overrides for each shader group.
///Each shader group behaves like a pipeline that was created from its state as
///well as the remaining parent’s state.
///It is defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGraphicsShaderGroupCreateInfoNV {
///    VkStructureType                                 sType;
///    const void*                                     pNext;
///    uint32_t                                        stageCount;
///    const VkPipelineShaderStageCreateInfo*          pStages;
///    const VkPipelineVertexInputStateCreateInfo*     pVertexInputState;
///    const VkPipelineTessellationStateCreateInfo*    pTessellationState;
///} VkGraphicsShaderGroupCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stage_count`] is the number of entries in the [`stages`] array.
/// - [`stages`] is a pointer to an array [`PipelineShaderStageCreateInfo`] structures specifying
///   the set of the shader stages to be included in this shader group.
/// - [`vertex_input_state`] is a pointer to a [`PipelineVertexInputStateCreateInfo`] structure.
/// - [`tessellation_state`] is a pointer to a [`PipelineTessellationStateCreateInfo`] structure,
///   and is ignored if the shader group does not include a tessellation control shader stage and
///   tessellation evaluation shader stage.
///# Description
///## Valid Usage
/// - For [`stage_count`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`stage_count`] apply
/// - For [`stages`], the same restrictions as in [`GraphicsPipelineCreateInfo`]::[`stages`] apply
/// - For [`vertex_input_state`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`vertex_input_state`] apply
/// - For [`tessellation_state`], the same restrictions as in
///   [`GraphicsPipelineCreateInfo`]::[`tessellation_state`] apply
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`stages`] **must**  be a valid pointer to an array of [`stage_count`] valid
///   [`PipelineShaderStageCreateInfo`] structures
/// - [`stage_count`] **must**  be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`GraphicsPipelineShaderGroupsCreateInfoNV`]
/// - [`PipelineShaderStageCreateInfo`]
/// - [`PipelineTessellationStateCreateInfo`]
/// - [`PipelineVertexInputStateCreateInfo`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGraphicsShaderGroupCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GraphicsShaderGroupCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`stage_count`] is the number of entries in the [`stages`] array.
    pub stage_count: u32,
    ///[`stages`] is a pointer to an array
    ///[`PipelineShaderStageCreateInfo`] structures specifying the set of
    ///the shader stages to be included in this shader group.
    pub stages: *const PipelineShaderStageCreateInfo<'lt>,
    ///[`vertex_input_state`] is a pointer to a
    ///[`PipelineVertexInputStateCreateInfo`] structure.
    pub vertex_input_state: *const PipelineVertexInputStateCreateInfo<'lt>,
    ///[`tessellation_state`] is a pointer to a
    ///[`PipelineTessellationStateCreateInfo`] structure, and is ignored if
    ///the shader group does not include a tessellation control shader stage
    ///and tessellation evaluation shader stage.
    pub tessellation_state: *const PipelineTessellationStateCreateInfo<'lt>,
}
impl<'lt> Default for GraphicsShaderGroupCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            stage_count: 0,
            stages: std::ptr::null(),
            vertex_input_state: std::ptr::null(),
            tessellation_state: std::ptr::null(),
        }
    }
}
impl<'lt> GraphicsShaderGroupCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::stages`]
    pub fn stages_raw(&self) -> *const PipelineShaderStageCreateInfo<'lt> {
        self.stages
    }
    ///Gets the raw value of [`Self::vertex_input_state`]
    pub fn vertex_input_state_raw(&self) -> *const PipelineVertexInputStateCreateInfo<'lt> {
        self.vertex_input_state
    }
    ///Gets the raw value of [`Self::tessellation_state`]
    pub fn tessellation_state_raw(&self) -> *const PipelineTessellationStateCreateInfo<'lt> {
        self.tessellation_state
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::stages`]
    pub fn set_stages_raw(mut self, value: *const PipelineShaderStageCreateInfo<'lt>) -> Self {
        self.stages = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_input_state`]
    pub fn set_vertex_input_state_raw(mut self, value: *const PipelineVertexInputStateCreateInfo<'lt>) -> Self {
        self.vertex_input_state = value;
        self
    }
    ///Sets the raw value of [`Self::tessellation_state`]
    pub fn set_tessellation_state_raw(mut self, value: *const PipelineTessellationStateCreateInfo<'lt>) -> Self {
        self.tessellation_state = value;
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
    ///Gets the value of [`Self::stage_count`]
    pub fn stage_count(&self) -> u32 {
        self.stage_count
    }
    ///Gets the value of [`Self::stages`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn stages(&self) -> &[PipelineShaderStageCreateInfo<'lt>] {
        std::slice::from_raw_parts(self.stages, self.stage_count as usize)
    }
    ///Gets the value of [`Self::vertex_input_state`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn vertex_input_state(&self) -> &PipelineVertexInputStateCreateInfo<'lt> {
        &*self.vertex_input_state
    }
    ///Gets the value of [`Self::tessellation_state`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn tessellation_state(&self) -> &PipelineTessellationStateCreateInfo<'lt> {
        &*self.tessellation_state
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::stage_count`]
    pub fn stage_count_mut(&mut self) -> &mut u32 {
        &mut self.stage_count
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
    ///Sets the value of [`Self::stage_count`]
    pub fn set_stage_count(mut self, value: u32) -> Self {
        self.stage_count = value;
        self
    }
    ///Sets the value of [`Self::stages`]
    pub fn set_stages(mut self, value: &'lt [crate::vulkan1_0::PipelineShaderStageCreateInfo<'lt>]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.stages = value.as_ptr();
        self.stage_count = len_;
        self
    }
    ///Sets the value of [`Self::vertex_input_state`]
    pub fn set_vertex_input_state(
        mut self,
        value: &'lt crate::vulkan1_0::PipelineVertexInputStateCreateInfo<'lt>,
    ) -> Self {
        self.vertex_input_state = value as *const _;
        self
    }
    ///Sets the value of [`Self::tessellation_state`]
    pub fn set_tessellation_state(
        mut self,
        value: &'lt crate::vulkan1_0::PipelineTessellationStateCreateInfo<'lt>,
    ) -> Self {
        self.tessellation_state = value as *const _;
        self
    }
}
///[VkGraphicsPipelineShaderGroupsCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html) - Structure specifying parameters of a newly created multi shader group pipeline
///# C Specifications
///The [`GraphicsPipelineShaderGroupsCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
///    VkStructureType                             sType;
///    const void*                                 pNext;
///    uint32_t                                    groupCount;
///    const VkGraphicsShaderGroupCreateInfoNV*    pGroups;
///    uint32_t                                    pipelineCount;
///    const VkPipeline*                           pPipelines;
///} VkGraphicsPipelineShaderGroupsCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`group_count`] is the number of elements in the [`groups`] array.
/// - [`groups`] is a pointer to an array of [`GraphicsShaderGroupCreateInfoNV`] structures
///   specifying which state of the original [`GraphicsPipelineCreateInfo`] each shader group
///   overrides.
/// - [`pipeline_count`] is the number of elements in the [`pipelines`] array.
/// - [`pipelines`] is a pointer to an array of graphics [`Pipeline`] structures which are
///   referenced within the created pipeline, including all their shader groups.
///# Description
///When referencing shader groups by index, groups defined in the referenced
///pipelines are treated as if they were defined as additional entries in
///[`groups`].
///They are appended in the order they appear in the [`pipelines`] array and
///in the [`groups`] array when those pipelines were defined.The application  **must**  maintain
/// the lifetime of all such referenced pipelines
///based on the pipelines that make use of them.
///## Valid Usage
/// - [`group_count`] **must**  be at least `1` and as maximum
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
/// - The sum of [`group_count`] including those groups added from referenced [`pipelines`] **must**
///   also be as maximum
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_graphics_shader_group_count`]
/// - The state of the first element of [`groups`] **must**  match its equivalent within the
///   parent’s [`GraphicsPipelineCreateInfo`]
/// - Each element of [`groups`] **must**  in combination with the rest of the pipeline state yield
///   a valid state configuration
/// - All elements of [`groups`] **must**  use the same shader stage combinations unless any mesh
///   shader stage is used, then either combination of task and mesh or just mesh shader is valid
/// - Mesh and regular primitive shading stages cannot be mixed across [`groups`]
/// - Each element of [`pipelines`] **must**  have been created with identical state to the pipeline
///   currently created except the state that can be overridden by
///   [`GraphicsShaderGroupCreateInfoNV`]
/// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
/// - [`groups`] **must**  be a valid pointer to an array of [`group_count`] valid
///   [`GraphicsShaderGroupCreateInfoNV`] structures
/// - If [`pipeline_count`] is not `0`, [`pipelines`] **must**  be a valid pointer to an array of
///   [`pipeline_count`] valid [`Pipeline`] handles
/// - [`group_count`] **must**  be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`GraphicsShaderGroupCreateInfoNV`]
/// - [`Pipeline`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGraphicsPipelineShaderGroupsCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`group_count`] is the number of elements in the [`groups`] array.
    pub group_count: u32,
    ///[`groups`] is a pointer to an array of
    ///[`GraphicsShaderGroupCreateInfoNV`] structures specifying which
    ///state of the original [`GraphicsPipelineCreateInfo`] each shader
    ///group overrides.
    pub groups: *const GraphicsShaderGroupCreateInfoNV<'lt>,
    ///[`pipeline_count`] is the number of elements in the [`pipelines`]
    ///array.
    pub pipeline_count: u32,
    ///[`pipelines`] is a pointer to an array of graphics [`Pipeline`]
    ///structures which are referenced within the created pipeline, including
    ///all their shader groups.
    pub pipelines: *const Pipeline,
}
impl<'lt> Default for GraphicsPipelineShaderGroupsCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            group_count: 0,
            groups: std::ptr::null(),
            pipeline_count: 0,
            pipelines: std::ptr::null(),
        }
    }
}
impl<'lt> GraphicsPipelineShaderGroupsCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::groups`]
    pub fn groups_raw(&self) -> *const GraphicsShaderGroupCreateInfoNV<'lt> {
        self.groups
    }
    ///Gets the raw value of [`Self::pipelines`]
    pub fn pipelines_raw(&self) -> *const Pipeline {
        self.pipelines
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::groups`]
    pub fn set_groups_raw(mut self, value: *const GraphicsShaderGroupCreateInfoNV<'lt>) -> Self {
        self.groups = value;
        self
    }
    ///Sets the raw value of [`Self::pipelines`]
    pub fn set_pipelines_raw(mut self, value: *const Pipeline) -> Self {
        self.pipelines = value;
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
    ///Gets the value of [`Self::group_count`]
    pub fn group_count(&self) -> u32 {
        self.group_count
    }
    ///Gets the value of [`Self::groups`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn groups(&self) -> &[GraphicsShaderGroupCreateInfoNV<'lt>] {
        std::slice::from_raw_parts(self.groups, self.group_count as usize)
    }
    ///Gets the value of [`Self::pipeline_count`]
    pub fn pipeline_count(&self) -> u32 {
        self.pipeline_count
    }
    ///Gets the value of [`Self::pipelines`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pipelines(&self) -> &[Pipeline] {
        std::slice::from_raw_parts(self.pipelines, self.pipeline_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::group_count`]
    pub fn group_count_mut(&mut self) -> &mut u32 {
        &mut self.group_count
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_count`]
    pub fn pipeline_count_mut(&mut self) -> &mut u32 {
        &mut self.pipeline_count
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
    ///Sets the value of [`Self::group_count`]
    pub fn set_group_count(mut self, value: u32) -> Self {
        self.group_count = value;
        self
    }
    ///Sets the value of [`Self::groups`]
    pub fn set_groups(
        mut self,
        value: &'lt [crate::extensions::nv_device_generated_commands::GraphicsShaderGroupCreateInfoNV<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.groups = value.as_ptr();
        self.group_count = len_;
        self
    }
    ///Sets the value of [`Self::pipeline_count`]
    pub fn set_pipeline_count(mut self, value: u32) -> Self {
        self.pipeline_count = value;
        self
    }
    ///Sets the value of [`Self::pipelines`]
    pub fn set_pipelines(mut self, value: &'lt [crate::vulkan1_0::Pipeline]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pipelines = value.as_ptr();
        self.pipeline_count = len_;
        self
    }
}
///[VkBindShaderGroupIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindShaderGroupIndirectCommandNV.html) - Structure specifying input data for a single shader group command token
///# C Specifications
///The [`BindShaderGroupIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindShaderGroupIndirectCommandNV {
///    uint32_t    groupIndex;
///} VkBindShaderGroupIndirectCommandNV;
///```
///# Members
/// - `index` specifies which shader group of the current bound graphics pipeline is used.
///# Description
///## Valid Usage
/// - The current bound graphics pipeline, as well as the pipelines it may reference,  **must**
///   have been created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
/// - The `index` **must**  be within range of the accessible shader groups of the current bound
///   graphics pipeline. See [`cmd_bind_pipeline_shader_group_nv`] for further details
///# Related
/// - [`VK_NV_device_generated_commands`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindShaderGroupIndirectCommandNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindShaderGroupIndirectCommandNV {
    ///No documentation found
    pub group_index: u32,
}
impl Default for BindShaderGroupIndirectCommandNV {
    fn default() -> Self {
        Self { group_index: 0 }
    }
}
impl BindShaderGroupIndirectCommandNV {
    ///Gets the value of [`Self::group_index`]
    pub fn group_index(&self) -> u32 {
        self.group_index
    }
    ///Gets a mutable reference to the value of [`Self::group_index`]
    pub fn group_index_mut(&mut self) -> &mut u32 {
        &mut self.group_index
    }
    ///Sets the value of [`Self::group_index`]
    pub fn set_group_index(mut self, value: u32) -> Self {
        self.group_index = value;
        self
    }
}
///[VkBindIndexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindIndexBufferIndirectCommandNV.html) - Structure specifying input data for a single index buffer command token
///# C Specifications
///The [`BindIndexBufferIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindIndexBufferIndirectCommandNV {
///    VkDeviceAddress    bufferAddress;
///    uint32_t           size;
///    VkIndexType        indexType;
///} VkBindIndexBufferIndirectCommandNV;
///```
///# Members
/// - [`buffer_address`] specifies a physical address of the [`Buffer`] used as index buffer.
/// - [`size`] is the byte size range which is available for this operation from the provided
///   address.
/// - [`index_type`] is a [`IndexType`] value specifying how indices are treated. Instead of the
///   Vulkan enum values, a custom `uint32_t` value  **can**  be mapped to an [`IndexType`] by
///   specifying the [`IndirectCommandsLayoutTokenNV::index_types`] and
///   [`IndirectCommandsLayoutTokenNV::index_type_values`] arrays.
///# Description
///## Valid Usage
/// - The buffer’s usage flag from which the address was acquired  **must**  have the
///   `VK_BUFFER_USAGE_INDEX_BUFFER_BIT` bit set
/// - The [`buffer_address`] **must**  be aligned to the [`index_type`] used
/// - Each element of the buffer from which the address was acquired and that is non-sparse
///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
///
///## Valid Usage (Implicit)
/// - [`index_type`] **must**  be a valid [`IndexType`] value
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`DeviceAddress`]
/// - [`IndexType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindIndexBufferIndirectCommandNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindIndexBufferIndirectCommandNV {
    ///[`buffer_address`] specifies a physical address of the [`Buffer`]
    ///used as index buffer.
    pub buffer_address: DeviceAddress,
    ///[`size`] is the byte size range which is available for this operation
    ///from the provided address.
    pub size: u32,
    ///[`index_type`] is a [`IndexType`] value specifying how indices are
    ///treated.
    ///Instead of the Vulkan enum values, a custom `uint32_t` value  **can**  be
    ///mapped to an [`IndexType`] by specifying the
    ///[`IndirectCommandsLayoutTokenNV`]::`pIndexTypes` and
    ///[`IndirectCommandsLayoutTokenNV`]::`pIndexTypeValues` arrays.
    pub index_type: IndexType,
}
impl Default for BindIndexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: 0,
            index_type: Default::default(),
        }
    }
}
impl BindIndexBufferIndirectCommandNV {
    ///Gets the value of [`Self::buffer_address`]
    pub fn buffer_address(&self) -> DeviceAddress {
        self.buffer_address
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> u32 {
        self.size
    }
    ///Gets the value of [`Self::index_type`]
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer_address`]
    pub fn buffer_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.buffer_address
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut u32 {
        &mut self.size
    }
    ///Gets a mutable reference to the value of [`Self::index_type`]
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Sets the value of [`Self::buffer_address`]
    pub fn set_buffer_address(mut self, value: crate::vulkan1_0::DeviceAddress) -> Self {
        self.buffer_address = value;
        self
    }
    ///Sets the value of [`Self::size`]
    pub fn set_size(mut self, value: u32) -> Self {
        self.size = value;
        self
    }
    ///Sets the value of [`Self::index_type`]
    pub fn set_index_type(mut self, value: crate::vulkan1_0::IndexType) -> Self {
        self.index_type = value;
        self
    }
}
///[VkBindVertexBufferIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindVertexBufferIndirectCommandNV.html) - Structure specifying input data for a single vertex buffer command token
///# C Specifications
///The [`BindVertexBufferIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV` token.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkBindVertexBufferIndirectCommandNV {
///    VkDeviceAddress    bufferAddress;
///    uint32_t           size;
///    uint32_t           stride;
///} VkBindVertexBufferIndirectCommandNV;
///```
///# Members
/// - [`buffer_address`] specifies a physical address of the [`Buffer`] used as vertex input
///   binding.
/// - [`size`] is the byte size range which is available for this operation from the provided
///   address.
/// - [`stride`] is the byte size stride for this vertex input binding as in
///   [`VertexInputBindingDescription`]::[`stride`]. It is only used if
///   [`IndirectCommandsLayoutTokenNV::vertex_dynamic_stride`] was set, otherwise the stride is
///   inherited from the current bound graphics pipeline.
///# Description
///## Valid Usage
/// - The buffer’s usage flag from which the address was acquired  **must**  have the
///   `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` bit set
/// - Each element of the buffer from which the address was acquired and that is non-sparse
///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`DeviceAddress`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindVertexBufferIndirectCommandNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindVertexBufferIndirectCommandNV {
    ///[`buffer_address`] specifies a physical address of the [`Buffer`]
    ///used as vertex input binding.
    pub buffer_address: DeviceAddress,
    ///[`size`] is the byte size range which is available for this operation
    ///from the provided address.
    pub size: u32,
    ///[`stride`] is the byte size stride for this vertex input binding as in
    ///[`VertexInputBindingDescription`]::[`stride`].
    ///It is only used if
    ///[`IndirectCommandsLayoutTokenNV`]::`vertexDynamicStride` was
    ///set, otherwise the stride is inherited from the current bound graphics
    ///pipeline.
    pub stride: u32,
}
impl Default for BindVertexBufferIndirectCommandNV {
    fn default() -> Self {
        Self {
            buffer_address: Default::default(),
            size: 0,
            stride: 0,
        }
    }
}
impl BindVertexBufferIndirectCommandNV {
    ///Gets the value of [`Self::buffer_address`]
    pub fn buffer_address(&self) -> DeviceAddress {
        self.buffer_address
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> u32 {
        self.size
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> u32 {
        self.stride
    }
    ///Gets a mutable reference to the value of [`Self::buffer_address`]
    pub fn buffer_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.buffer_address
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut u32 {
        &mut self.size
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut u32 {
        &mut self.stride
    }
    ///Sets the value of [`Self::buffer_address`]
    pub fn set_buffer_address(mut self, value: crate::vulkan1_0::DeviceAddress) -> Self {
        self.buffer_address = value;
        self
    }
    ///Sets the value of [`Self::size`]
    pub fn set_size(mut self, value: u32) -> Self {
        self.size = value;
        self
    }
    ///Sets the value of [`Self::stride`]
    pub fn set_stride(mut self, value: u32) -> Self {
        self.stride = value;
        self
    }
}
///[VkSetStateFlagsIndirectCommandNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSetStateFlagsIndirectCommandNV.html) - Structure specifying input data for a single state flag command token
///# C Specifications
///The [`SetStateFlagsIndirectCommandNV`] structure specifies the input
///data for the `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` token.
///Which state is changed depends on the [`IndirectStateFlagBitsNV`]
///specified at [`IndirectCommandsLayoutNV`] creation time.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkSetStateFlagsIndirectCommandNV {
///    uint32_t    data;
///} VkSetStateFlagsIndirectCommandNV;
///```
///# Members
/// - [`data`] encodes packed state that this command alters.  - Bit `0`: If set represents
///   `VK_FRONT_FACE_CLOCKWISE`, otherwise `VK_FRONT_FACE_COUNTER_CLOCKWISE`
///# Related
/// - [`VK_NV_device_generated_commands`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSetStateFlagsIndirectCommandNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SetStateFlagsIndirectCommandNV {
    ///[`data`] encodes packed state that this command alters.
    /// - Bit `0`: If set represents `VK_FRONT_FACE_CLOCKWISE`, otherwise
    ///   `VK_FRONT_FACE_COUNTER_CLOCKWISE`
    pub data: u32,
}
impl Default for SetStateFlagsIndirectCommandNV {
    fn default() -> Self {
        Self { data: 0 }
    }
}
impl SetStateFlagsIndirectCommandNV {
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> u32 {
        self.data
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
    ///Sets the value of [`Self::data`]
    pub fn set_data(mut self, value: u32) -> Self {
        self.data = value;
        self
    }
}
///[VkIndirectCommandsStreamNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsStreamNV.html) - Structure specifying input streams for generated command tokens
///# C Specifications
///The [`IndirectCommandsStreamNV`] structure specifies the input data for
///one or more tokens at processing time.
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsStreamNV {
///    VkBuffer        buffer;
///    VkDeviceSize    offset;
///} VkIndirectCommandsStreamNV;
///```
///# Members
/// - [`buffer`] specifies the [`Buffer`] storing the functional arguments for each sequence. These
///   arguments  **can**  be written by the device.
/// - [`offset`] specified an offset into [`buffer`] where the arguments start.
///# Description
///## Valid Usage
/// - The [`buffer`]’s usage flag  **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - The [`offset`] **must**  be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_indirect_commands_buffer_offset_alignment`]
/// - If [`buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
///
///## Valid Usage (Implicit)
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`GeneratedCommandsInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsStreamNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct IndirectCommandsStreamNV {
    ///[`buffer`] specifies the [`Buffer`] storing the functional
    ///arguments for each sequence.
    ///These arguments  **can**  be written by the device.
    pub buffer: Buffer,
    ///[`offset`] specified an offset into [`buffer`] where the arguments
    ///start.
    pub offset: DeviceSize,
}
impl Default for IndirectCommandsStreamNV {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            offset: Default::default(),
        }
    }
}
impl IndirectCommandsStreamNV {
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Sets the value of [`Self::buffer`]
    pub fn set_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.buffer = value;
        self
    }
    ///Sets the value of [`Self::offset`]
    pub fn set_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.offset = value;
        self
    }
}
///[VkIndirectCommandsLayoutTokenNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutTokenNV.html) - Struct specifying the details of an indirect command layout token
///# C Specifications
///The [`IndirectCommandsLayoutTokenNV`] structure specifies details to the
///function arguments that need to be known at layout creation time:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsLayoutTokenNV {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkIndirectCommandsTokenTypeNV    tokenType;
///    uint32_t                         stream;
///    uint32_t                         offset;
///    uint32_t                         vertexBindingUnit;
///    VkBool32                         vertexDynamicStride;
///    VkPipelineLayout                 pushconstantPipelineLayout;
///    VkShaderStageFlags               pushconstantShaderStageFlags;
///    uint32_t                         pushconstantOffset;
///    uint32_t                         pushconstantSize;
///    VkIndirectStateFlagsNV           indirectStateFlags;
///    uint32_t                         indexTypeCount;
///    const VkIndexType*               pIndexTypes;
///    const uint32_t*                  pIndexTypeValues;
///} VkIndirectCommandsLayoutTokenNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`token_type`] specifies the token command type.
/// - [`stream`] is the index of the input stream containing the token argument data.
/// - [`offset`] is a relative starting offset within the input stream memory for the token argument
///   data.
/// - [`vertex_binding_unit`] is used for the vertex buffer binding command.
/// - [`vertex_dynamic_stride`] sets if the vertex buffer stride is provided by the binding command
///   rather than the current bound graphics pipeline state.
/// - [`pushconstant_pipeline_layout`] is the [`PipelineLayout`] used for the push constant command.
/// - [`pushconstant_shader_stage_flags`] are the shader stage flags used for the push constant
///   command.
/// - [`pushconstant_offset`] is the offset used for the push constant command.
/// - [`pushconstant_size`] is the size used for the push constant command.
/// - [`indirect_state_flags`] are the active states for the state flag command.
/// - [`index_type_count`] is the optional size of the [`index_types`] and [`index_type_values`]
///   array pairings. If not zero, it allows to register a custom `uint32_t` value to be treated as
///   specific [`IndexType`].
/// - [`index_types`] is the used [`IndexType`] for the corresponding `uint32_t` value entry in
///   [`index_type_values`].
///# Description
///## Valid Usage
/// - [`stream`] **must**  be smaller than [`IndirectCommandsLayoutCreateInfoNV::stream_count`]
/// - [`offset`] **must**  be less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_offset`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV`,
///   [`vertex_binding_unit`] **must**  stay within device supported limits for the appropriate
///   commands
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_pipeline_layout`] **must**  be valid
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_offset`] **must**  be a multiple of `4`
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_size`]
///   **must**  be a multiple of `4`
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`,
///   [`pushconstant_offset`] **must**  be less than
///   [`PhysicalDeviceLimits::max_push_constants_size`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, [`pushconstant_size`]
///   **must**  be less than or equal to [`PhysicalDeviceLimits::max_push_constants_size`] minus
///   [`pushconstant_offset`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the
///   range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each shader stage
///   in [`pushconstant_shader_stage_flags`], there  **must**  be a push constant range in
///   [`pushconstant_pipeline_layout`] that includes that byte and that stage
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, for each byte in the
///   range specified by [`pushconstant_offset`] and [`pushconstant_size`] and for each push
///   constant range that overlaps that byte, [`pushconstant_shader_stage_flags`] **must**  include
///   all stages in that push constant range’s [`PushConstantRange::stage_flags`]
/// - If [`token_type`] is `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV`,
///   [`indirect_state_flags`] **must**  not be `0`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`token_type`] **must**  be a valid [`IndirectCommandsTokenTypeNV`] value
/// - If [`pushconstant_pipeline_layout`] is not [`crate::Handle::null`],
///   [`pushconstant_pipeline_layout`] **must**  be a valid [`PipelineLayout`] handle
/// - [`pushconstant_shader_stage_flags`] **must**  be a valid combination of
///   [`ShaderStageFlagBits`] values
/// - [`indirect_state_flags`] **must**  be a valid combination of [`IndirectStateFlagBitsNV`]
///   values
/// - If [`index_type_count`] is not `0`, [`index_types`] **must**  be a valid pointer to an array
///   of [`index_type_count`] valid [`IndexType`] values
/// - If [`index_type_count`] is not `0`, [`index_type_values`] **must**  be a valid pointer to an
///   array of [`index_type_count`]`uint32_t` values
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Bool32`]
/// - [`IndexType`]
/// - [`IndirectCommandsLayoutCreateInfoNV`]
/// - [`IndirectCommandsTokenTypeNV`]
/// - [`IndirectStateFlagsNV`]
/// - [`PipelineLayout`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsLayoutTokenNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct IndirectCommandsLayoutTokenNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`token_type`] specifies the token command type.
    pub token_type: IndirectCommandsTokenTypeNV,
    ///[`stream`] is the index of the input stream containing the token
    ///argument data.
    pub stream: u32,
    ///[`offset`] is a relative starting offset within the input stream
    ///memory for the token argument data.
    pub offset: u32,
    ///[`vertex_binding_unit`] is used for the vertex buffer binding command.
    pub vertex_binding_unit: u32,
    ///[`vertex_dynamic_stride`] sets if the vertex buffer stride is provided
    ///by the binding command rather than the current bound graphics pipeline
    ///state.
    pub vertex_dynamic_stride: Bool32,
    ///[`pushconstant_pipeline_layout`] is the [`PipelineLayout`] used for
    ///the push constant command.
    pub pushconstant_pipeline_layout: PipelineLayout,
    ///[`pushconstant_shader_stage_flags`] are the shader stage flags used for
    ///the push constant command.
    pub pushconstant_shader_stage_flags: ShaderStageFlags,
    ///[`pushconstant_offset`] is the offset used for the push constant
    ///command.
    pub pushconstant_offset: u32,
    ///[`pushconstant_size`] is the size used for the push constant command.
    pub pushconstant_size: u32,
    ///[`indirect_state_flags`] are the active states for the state flag
    ///command.
    pub indirect_state_flags: IndirectStateFlagsNV,
    ///[`index_type_count`] is the optional size of the [`index_types`] and
    ///[`index_type_values`] array pairings.
    ///If not zero, it allows to register a custom `uint32_t` value to be
    ///treated as specific [`IndexType`].
    pub index_type_count: u32,
    ///[`index_types`] is the used [`IndexType`] for the corresponding
    ///`uint32_t` value entry in [`index_type_values`].
    pub index_types: *const IndexType,
    ///No documentation found
    pub index_type_values: *const u32,
}
impl<'lt> Default for IndirectCommandsLayoutTokenNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
            p_next: std::ptr::null(),
            token_type: Default::default(),
            stream: 0,
            offset: 0,
            vertex_binding_unit: 0,
            vertex_dynamic_stride: 0,
            pushconstant_pipeline_layout: Default::default(),
            pushconstant_shader_stage_flags: Default::default(),
            pushconstant_offset: 0,
            pushconstant_size: 0,
            indirect_state_flags: Default::default(),
            index_type_count: 0,
            index_types: std::ptr::null(),
            index_type_values: std::ptr::null(),
        }
    }
}
impl<'lt> IndirectCommandsLayoutTokenNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::vertex_dynamic_stride`]
    pub fn vertex_dynamic_stride_raw(&self) -> Bool32 {
        self.vertex_dynamic_stride
    }
    ///Gets the raw value of [`Self::index_types`]
    pub fn index_types_raw(&self) -> *const IndexType {
        self.index_types
    }
    ///Gets the raw value of [`Self::index_type_values`]
    pub fn index_type_values_raw(&self) -> *const u32 {
        self.index_type_values
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_dynamic_stride`]
    pub fn set_vertex_dynamic_stride_raw(mut self, value: Bool32) -> Self {
        self.vertex_dynamic_stride = value;
        self
    }
    ///Sets the raw value of [`Self::index_types`]
    pub fn set_index_types_raw(mut self, value: *const IndexType) -> Self {
        self.index_types = value;
        self
    }
    ///Sets the raw value of [`Self::index_type_values`]
    pub fn set_index_type_values_raw(mut self, value: *const u32) -> Self {
        self.index_type_values = value;
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
    ///Gets the value of [`Self::token_type`]
    pub fn token_type(&self) -> IndirectCommandsTokenTypeNV {
        self.token_type
    }
    ///Gets the value of [`Self::stream`]
    pub fn stream(&self) -> u32 {
        self.stream
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> u32 {
        self.offset
    }
    ///Gets the value of [`Self::vertex_binding_unit`]
    pub fn vertex_binding_unit(&self) -> u32 {
        self.vertex_binding_unit
    }
    ///Gets the value of [`Self::vertex_dynamic_stride`]
    pub fn vertex_dynamic_stride(&self) -> bool {
        unsafe { std::mem::transmute(self.vertex_dynamic_stride as u8) }
    }
    ///Gets the value of [`Self::pushconstant_pipeline_layout`]
    pub fn pushconstant_pipeline_layout(&self) -> PipelineLayout {
        self.pushconstant_pipeline_layout
    }
    ///Gets the value of [`Self::pushconstant_shader_stage_flags`]
    pub fn pushconstant_shader_stage_flags(&self) -> ShaderStageFlags {
        self.pushconstant_shader_stage_flags
    }
    ///Gets the value of [`Self::pushconstant_offset`]
    pub fn pushconstant_offset(&self) -> u32 {
        self.pushconstant_offset
    }
    ///Gets the value of [`Self::pushconstant_size`]
    pub fn pushconstant_size(&self) -> u32 {
        self.pushconstant_size
    }
    ///Gets the value of [`Self::indirect_state_flags`]
    pub fn indirect_state_flags(&self) -> IndirectStateFlagsNV {
        self.indirect_state_flags
    }
    ///Gets the value of [`Self::index_type_count`]
    pub fn index_type_count(&self) -> u32 {
        self.index_type_count
    }
    ///Gets the value of [`Self::index_types`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn index_types(&self) -> &[IndexType] {
        std::slice::from_raw_parts(self.index_types, self.index_type_count as usize)
    }
    ///Gets the value of [`Self::index_type_values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn index_type_values(&self) -> &[u32] {
        std::slice::from_raw_parts(self.index_type_values, self.index_type_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::token_type`]
    pub fn token_type_mut(&mut self) -> &mut IndirectCommandsTokenTypeNV {
        &mut self.token_type
    }
    ///Gets a mutable reference to the value of [`Self::stream`]
    pub fn stream_mut(&mut self) -> &mut u32 {
        &mut self.stream
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut u32 {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::vertex_binding_unit`]
    pub fn vertex_binding_unit_mut(&mut self) -> &mut u32 {
        &mut self.vertex_binding_unit
    }
    ///Gets a mutable reference to the value of [`Self::vertex_dynamic_stride`]
    pub fn vertex_dynamic_stride_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vertex_dynamic_stride as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vertex_dynamic_stride as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::pushconstant_pipeline_layout`]
    pub fn pushconstant_pipeline_layout_mut(&mut self) -> &mut PipelineLayout {
        &mut self.pushconstant_pipeline_layout
    }
    ///Gets a mutable reference to the value of [`Self::pushconstant_shader_stage_flags`]
    pub fn pushconstant_shader_stage_flags_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.pushconstant_shader_stage_flags
    }
    ///Gets a mutable reference to the value of [`Self::pushconstant_offset`]
    pub fn pushconstant_offset_mut(&mut self) -> &mut u32 {
        &mut self.pushconstant_offset
    }
    ///Gets a mutable reference to the value of [`Self::pushconstant_size`]
    pub fn pushconstant_size_mut(&mut self) -> &mut u32 {
        &mut self.pushconstant_size
    }
    ///Gets a mutable reference to the value of [`Self::indirect_state_flags`]
    pub fn indirect_state_flags_mut(&mut self) -> &mut IndirectStateFlagsNV {
        &mut self.indirect_state_flags
    }
    ///Gets a mutable reference to the value of [`Self::index_type_count`]
    pub fn index_type_count_mut(&mut self) -> &mut u32 {
        &mut self.index_type_count
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
    ///Sets the value of [`Self::token_type`]
    pub fn set_token_type(
        mut self,
        value: crate::extensions::nv_device_generated_commands::IndirectCommandsTokenTypeNV,
    ) -> Self {
        self.token_type = value;
        self
    }
    ///Sets the value of [`Self::stream`]
    pub fn set_stream(mut self, value: u32) -> Self {
        self.stream = value;
        self
    }
    ///Sets the value of [`Self::offset`]
    pub fn set_offset(mut self, value: u32) -> Self {
        self.offset = value;
        self
    }
    ///Sets the value of [`Self::vertex_binding_unit`]
    pub fn set_vertex_binding_unit(mut self, value: u32) -> Self {
        self.vertex_binding_unit = value;
        self
    }
    ///Sets the value of [`Self::vertex_dynamic_stride`]
    pub fn set_vertex_dynamic_stride(mut self, value: bool) -> Self {
        self.vertex_dynamic_stride = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::pushconstant_pipeline_layout`]
    pub fn set_pushconstant_pipeline_layout(mut self, value: crate::vulkan1_0::PipelineLayout) -> Self {
        self.pushconstant_pipeline_layout = value;
        self
    }
    ///Sets the value of [`Self::pushconstant_shader_stage_flags`]
    pub fn set_pushconstant_shader_stage_flags(mut self, value: crate::vulkan1_0::ShaderStageFlags) -> Self {
        self.pushconstant_shader_stage_flags = value;
        self
    }
    ///Sets the value of [`Self::pushconstant_offset`]
    pub fn set_pushconstant_offset(mut self, value: u32) -> Self {
        self.pushconstant_offset = value;
        self
    }
    ///Sets the value of [`Self::pushconstant_size`]
    pub fn set_pushconstant_size(mut self, value: u32) -> Self {
        self.pushconstant_size = value;
        self
    }
    ///Sets the value of [`Self::indirect_state_flags`]
    pub fn set_indirect_state_flags(
        mut self,
        value: crate::extensions::nv_device_generated_commands::IndirectStateFlagsNV,
    ) -> Self {
        self.indirect_state_flags = value;
        self
    }
    ///Sets the value of [`Self::index_type_count`]
    pub fn set_index_type_count(mut self, value: u32) -> Self {
        self.index_type_count = value;
        self
    }
    ///Sets the value of [`Self::index_types`]
    pub fn set_index_types(mut self, value: &'lt [crate::vulkan1_0::IndexType]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.index_types = value.as_ptr();
        self.index_type_count = len_;
        self
    }
    ///Sets the value of [`Self::index_type_values`]
    pub fn set_index_type_values(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.index_type_values = value.as_ptr();
        self.index_type_count = len_;
        self
    }
}
///[VkIndirectCommandsLayoutCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutCreateInfoNV.html) - Structure specifying the parameters of a newly created indirect commands layout object
///# C Specifications
///The [`IndirectCommandsLayoutCreateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkIndirectCommandsLayoutCreateInfoNV {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    VkIndirectCommandsLayoutUsageFlagsNV      flags;
///    VkPipelineBindPoint                       pipelineBindPoint;
///    uint32_t                                  tokenCount;
///    const VkIndirectCommandsLayoutTokenNV*    pTokens;
///    uint32_t                                  streamCount;
///    const uint32_t*                           pStreamStrides;
///} VkIndirectCommandsLayoutCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] that this layout targets.
/// - [`flags`] is a bitmask of [`IndirectCommandsLayoutUsageFlagBitsNV`] specifying usage hints of
///   this layout.
/// - [`token_count`] is the length of the individual command sequence.
/// - [`tokens`] is an array describing each command token in detail. See
///   [`IndirectCommandsTokenTypeNV`] and [`IndirectCommandsLayoutTokenNV`] below for details.
/// - [`stream_count`] is the number of streams used to provide the token inputs.
/// - [`stream_strides`] is an array defining the byte stride for each input stream.
///# Description
///The following code illustrates some of the flags:
///```c
///void cmdProcessAllSequences(cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens,
/// sequencesCount, indexbuffer, indexbufferOffset)
///{
///  for (s = 0; s < sequencesCount; s++)
///  {
///    sUsed = s;
///
///    if (indirectCommandsLayout.flags &
/// VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV) {
///      sUsed = indexbuffer.load_uint32( sUsed * sizeof(uint32_t) + indexbufferOffset);
///    }
///
///    if (indirectCommandsLayout.flags &
/// VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV) {
///      sUsed = incoherent_implementation_dependent_permutation[ sUsed ];
///    }
///
///    cmdProcessSequence( cmd, pipeline, indirectCommandsLayout, pIndirectCommandsTokens, sUsed );
///  }
///}
///```
///
///## Valid Usage
/// - The [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - [`token_count`] **must**  be greater than `0` and less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_token_count`]
/// - If [`tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV` it
///   **must**  be the first element of the array and there  **must**  be only a single element of
///   such token type
/// - If [`tokens`] contains an entry of `VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV` there
///   **must**  be only a single element of such token type
/// - All state tokens in [`tokens`] **must**  occur prior work provoking tokens
///   (`VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV`, `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV`,
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV`)
/// - The content of [`tokens`] **must**  include one single work provoking token that is compatible
///   with the [`pipeline_bind_point`]
/// - [`stream_count`] **must**  be greater than `0` and less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_count`]
/// - each element of [`stream_strides`] **must**  be greater than `0`and less than or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_commands_stream_stride`].
///   Furthermore the alignment of each token input  **must**  be ensured
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`IndirectCommandsLayoutUsageFlagBitsNV`] values
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - [`tokens`] **must**  be a valid pointer to an array of [`token_count`] valid
///   [`IndirectCommandsLayoutTokenNV`] structures
/// - [`stream_strides`] **must**  be a valid pointer to an array of [`stream_count`]`uint32_t`
///   values
/// - [`token_count`] **must**  be greater than `0`
/// - [`stream_count`] **must**  be greater than `0`
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutTokenNV`]
/// - [`IndirectCommandsLayoutUsageFlagsNV`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`create_indirect_commands_layout_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsLayoutCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct IndirectCommandsLayoutCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of
    ///[`IndirectCommandsLayoutUsageFlagBitsNV`] specifying usage hints of
    ///this layout.
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] that this
    ///layout targets.
    pub pipeline_bind_point: PipelineBindPoint,
    ///[`token_count`] is the length of the individual command sequence.
    pub token_count: u32,
    ///[`tokens`] is an array describing each command token in detail.
    ///See [`IndirectCommandsTokenTypeNV`] and
    ///[`IndirectCommandsLayoutTokenNV`] below for details.
    pub tokens: *const IndirectCommandsLayoutTokenNV<'lt>,
    ///[`stream_count`] is the number of streams used to provide the token
    ///inputs.
    pub stream_count: u32,
    ///[`stream_strides`] is an array defining the byte stride for each input
    ///stream.
    pub stream_strides: *const u32,
}
impl<'lt> Default for IndirectCommandsLayoutCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            token_count: 0,
            tokens: std::ptr::null(),
            stream_count: 0,
            stream_strides: std::ptr::null(),
        }
    }
}
impl<'lt> IndirectCommandsLayoutCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::tokens`]
    pub fn tokens_raw(&self) -> *const IndirectCommandsLayoutTokenNV<'lt> {
        self.tokens
    }
    ///Gets the raw value of [`Self::stream_strides`]
    pub fn stream_strides_raw(&self) -> *const u32 {
        self.stream_strides
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::tokens`]
    pub fn set_tokens_raw(mut self, value: *const IndirectCommandsLayoutTokenNV<'lt>) -> Self {
        self.tokens = value;
        self
    }
    ///Sets the raw value of [`Self::stream_strides`]
    pub fn set_stream_strides_raw(mut self, value: *const u32) -> Self {
        self.stream_strides = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> IndirectCommandsLayoutUsageFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Gets the value of [`Self::token_count`]
    pub fn token_count(&self) -> u32 {
        self.token_count
    }
    ///Gets the value of [`Self::tokens`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn tokens(&self) -> &[IndirectCommandsLayoutTokenNV<'lt>] {
        std::slice::from_raw_parts(self.tokens, self.token_count as usize)
    }
    ///Gets the value of [`Self::stream_count`]
    pub fn stream_count(&self) -> u32 {
        self.stream_count
    }
    ///Gets the value of [`Self::stream_strides`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn stream_strides(&self) -> &[u32] {
        std::slice::from_raw_parts(self.stream_strides, self.stream_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut IndirectCommandsLayoutUsageFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Gets a mutable reference to the value of [`Self::token_count`]
    pub fn token_count_mut(&mut self) -> &mut u32 {
        &mut self.token_count
    }
    ///Gets a mutable reference to the value of [`Self::stream_count`]
    pub fn stream_count_mut(&mut self) -> &mut u32 {
        &mut self.stream_count
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
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutUsageFlagsNV,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::pipeline_bind_point`]
    pub fn set_pipeline_bind_point(mut self, value: crate::vulkan1_0::PipelineBindPoint) -> Self {
        self.pipeline_bind_point = value;
        self
    }
    ///Sets the value of [`Self::token_count`]
    pub fn set_token_count(mut self, value: u32) -> Self {
        self.token_count = value;
        self
    }
    ///Sets the value of [`Self::tokens`]
    pub fn set_tokens(
        mut self,
        value: &'lt [crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutTokenNV<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.tokens = value.as_ptr();
        self.token_count = len_;
        self
    }
    ///Sets the value of [`Self::stream_count`]
    pub fn set_stream_count(mut self, value: u32) -> Self {
        self.stream_count = value;
        self
    }
    ///Sets the value of [`Self::stream_strides`]
    pub fn set_stream_strides(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.stream_strides = value.as_ptr();
        self.stream_count = len_;
        self
    }
}
///[VkGeneratedCommandsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsInfoNV.html) - Structure specifying parameters for the generation of commands
///# C Specifications
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGeneratedCommandsInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkPipelineBindPoint                  pipelineBindPoint;
///    VkPipeline                           pipeline;
///    VkIndirectCommandsLayoutNV           indirectCommandsLayout;
///    uint32_t                             streamCount;
///    const VkIndirectCommandsStreamNV*    pStreams;
///    uint32_t                             sequencesCount;
///    VkBuffer                             preprocessBuffer;
///    VkDeviceSize                         preprocessOffset;
///    VkDeviceSize                         preprocessSize;
///    VkBuffer                             sequencesCountBuffer;
///    VkDeviceSize                         sequencesCountOffset;
///    VkBuffer                             sequencesIndexBuffer;
///    VkDeviceSize                         sequencesIndexOffset;
///} VkGeneratedCommandsInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] used for the [`pipeline`].
/// - [`pipeline`] is the [`Pipeline`] used in the generation and execution process.
/// - [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that provides the command
///   sequence to generate.
/// - [`stream_count`] defines the number of input streams
/// - [`streams`] is a pointer to an array of [`stream_count`][`IndirectCommandsStreamNV`]
///   structures providing the input data for the tokens used in [`indirect_commands_layout`].
/// - [`sequences_count`] is the maximum number of sequences to reserve. If
///   [`sequences_count_buffer`] is [`crate::Handle::null`], this is also the actual number of
///   sequences generated.
/// - [`preprocess_buffer`] is the [`Buffer`] that is used for preprocessing the input data for
///   execution. If this structure is used with [`cmd_execute_generated_commands_nv`] with its
///   `isPreprocessed` set to [`TRUE`], then the preprocessing step is skipped and data is only read
///   from this buffer.
/// - [`preprocess_offset`] is the byte offset into [`preprocess_buffer`] where the preprocessed
///   data is stored.
/// - [`preprocess_size`] is the maximum byte size within the [`preprocess_buffer`] after the
///   [`preprocess_offset`] that is available for preprocessing.
/// - [`sequences_count_buffer`] is a [`Buffer`] in which the actual number of sequences is provided
///   as single `uint32_t` value.
/// - [`sequences_count_offset`] is the byte offset into [`sequences_count_buffer`] where the count
///   value is stored.
/// - [`sequences_index_buffer`] is a [`Buffer`] that encodes the used sequence indices as
///   `uint32_t` array.
/// - [`sequences_index_offset`] is the byte offset into [`sequences_index_buffer`] where the index
///   values start.
///# Description
///## Valid Usage
/// - The provided [`pipeline`] **must**  match the pipeline bound at execution time
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`] **must**  have been
///   created with multiple shader groups
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV`, then the [`pipeline`] **must**  have been
///   created with `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV` set in
///   [`GraphicsPipelineCreateInfo::flags`]
/// - If the [`indirect_commands_layout`] uses a token of
///   `VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV`, then the [`pipeline`]`s [`PipelineLayout`]
///   **must**  match the [`IndirectCommandsLayoutTokenNV::pushconstant_pipeline_layout`]
/// - [`stream_count`] **must**  match the [`indirect_commands_layout`]’s [`stream_count`]
/// - [`sequences_count`] **must**  be less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`] and
///   [`GeneratedCommandsMemoryRequirementsInfoNV::max_sequences_count`] that was used to determine
///   the [`preprocess_size`]
/// - [`preprocess_buffer`] **must**  have the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set in its
///   usage flag
/// - [`preprocess_offset`] **must**  be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_indirect_commands_buffer_offset_alignment`]
/// - If [`preprocess_buffer`] is non-sparse then it  **must**  be bound completely and contiguously
///   to a single [`DeviceMemory`] object
/// - [`preprocess_size`] **must**  be at least equal to the memory requirement`s size returned by
///   [`get_generated_commands_memory_requirements_nv`] using the matching inputs
///   ([`indirect_commands_layout`], …​) as within this structure
/// - [`sequences_count_buffer`] **can**  be set if the actual used count of sequences is sourced
///   from the provided buffer. In that case the [`sequences_count`] serves as upper bound
/// - If [`sequences_count_buffer`] is not [`crate::Handle::null`], its usage flag  **must**  have
///   the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - If [`sequences_count_buffer`] is not [`crate::Handle::null`], [`sequences_count_offset`]
///   **must**  be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_sequences_count_buffer_offset_alignment`]
/// - If [`sequences_count_buffer`] is not [`crate::Handle::null`] and is non-sparse then it
///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
/// - If [`indirect_commands_layout`]’s `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV`
///   is set, [`sequences_index_buffer`] **must**  be set otherwise it  **must**  be
///   [`crate::Handle::null`]
/// - If [`sequences_index_buffer`] is not [`crate::Handle::null`], its usage flag  **must**  have
///   the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
/// - If [`sequences_index_buffer`] is not [`crate::Handle::null`], [`sequences_index_offset`]
///   **must**  be aligned to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::
///   min_sequences_index_buffer_offset_alignment`]
/// - If [`sequences_index_buffer`] is not [`crate::Handle::null`] and is non-sparse then it
///   **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
/// - [`streams`] **must**  be a valid pointer to an array of [`stream_count`] valid
///   [`IndirectCommandsStreamNV`] structures
/// - [`preprocess_buffer`] **must**  be a valid [`Buffer`] handle
/// - If [`sequences_count_buffer`] is not [`crate::Handle::null`], [`sequences_count_buffer`]
///   **must**  be a valid [`Buffer`] handle
/// - If [`sequences_index_buffer`] is not [`crate::Handle::null`], [`sequences_index_buffer`]
///   **must**  be a valid [`Buffer`] handle
/// - [`stream_count`] **must**  be greater than `0`
/// - Each of [`indirect_commands_layout`], [`pipeline`], [`preprocess_buffer`],
///   [`sequences_count_buffer`], and [`sequences_index_buffer`] that are valid handles of
///   non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`Buffer`]
/// - [`DeviceSize`]
/// - [`IndirectCommandsLayoutNV`]
/// - [`IndirectCommandsStreamNV`]
/// - [`Pipeline`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`cmd_execute_generated_commands_nv`]
/// - [`cmd_preprocess_generated_commands_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeneratedCommandsInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeneratedCommandsInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] used for the
    ///[`pipeline`].
    pub pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline`] is the [`Pipeline`] used in the generation and
    ///execution process.
    pub pipeline: Pipeline,
    ///[`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`]
    ///that provides the command sequence to generate.
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    ///[`stream_count`] defines the number of input streams
    pub stream_count: u32,
    ///[`streams`] is a pointer to an array of [`stream_count`][`IndirectCommandsStreamNV`]
    /// structures providing the input data for the tokens used in [`indirect_commands_layout`].
    pub streams: *const IndirectCommandsStreamNV,
    ///[`sequences_count`] is the maximum number of sequences to reserve.
    ///If [`sequences_count_buffer`] is [`crate::Handle::null`], this is also the
    ///actual number of sequences generated.
    pub sequences_count: u32,
    ///[`preprocess_buffer`] is the [`Buffer`] that is used for
    ///preprocessing the input data for execution.
    ///If this structure is used with [`cmd_execute_generated_commands_nv`]
    ///with its `isPreprocessed` set to [`TRUE`], then the preprocessing
    ///step is skipped and data is only read from this buffer.
    pub preprocess_buffer: Buffer,
    ///[`preprocess_offset`] is the byte offset into [`preprocess_buffer`]
    ///where the preprocessed data is stored.
    pub preprocess_offset: DeviceSize,
    ///[`preprocess_size`] is the maximum byte size within the
    ///[`preprocess_buffer`] after the [`preprocess_offset`] that is
    ///available for preprocessing.
    pub preprocess_size: DeviceSize,
    ///[`sequences_count_buffer`] is a [`Buffer`] in which the actual
    ///number of sequences is provided as single `uint32_t` value.
    pub sequences_count_buffer: Buffer,
    ///[`sequences_count_offset`] is the byte offset into
    ///[`sequences_count_buffer`] where the count value is stored.
    pub sequences_count_offset: DeviceSize,
    ///[`sequences_index_buffer`] is a [`Buffer`] that encodes the used
    ///sequence indices as `uint32_t` array.
    pub sequences_index_buffer: Buffer,
    ///[`sequences_index_offset`] is the byte offset into
    ///[`sequences_index_buffer`] where the index values start.
    pub sequences_index_offset: DeviceSize,
}
impl<'lt> Default for GeneratedCommandsInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GENERATED_COMMANDS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            stream_count: 0,
            streams: std::ptr::null(),
            sequences_count: 0,
            preprocess_buffer: Default::default(),
            preprocess_offset: Default::default(),
            preprocess_size: Default::default(),
            sequences_count_buffer: Default::default(),
            sequences_count_offset: Default::default(),
            sequences_index_buffer: Default::default(),
            sequences_index_offset: Default::default(),
        }
    }
}
impl<'lt> GeneratedCommandsInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::streams`]
    pub fn streams_raw(&self) -> *const IndirectCommandsStreamNV {
        self.streams
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::streams`]
    pub fn set_streams_raw(mut self, value: *const IndirectCommandsStreamNV) -> Self {
        self.streams = value;
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
    ///Gets the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets the value of [`Self::indirect_commands_layout`]
    pub fn indirect_commands_layout(&self) -> IndirectCommandsLayoutNV {
        self.indirect_commands_layout
    }
    ///Gets the value of [`Self::stream_count`]
    pub fn stream_count(&self) -> u32 {
        self.stream_count
    }
    ///Gets the value of [`Self::streams`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn streams(&self) -> &[IndirectCommandsStreamNV] {
        std::slice::from_raw_parts(self.streams, self.stream_count as usize)
    }
    ///Gets the value of [`Self::sequences_count`]
    pub fn sequences_count(&self) -> u32 {
        self.sequences_count
    }
    ///Gets the value of [`Self::preprocess_buffer`]
    pub fn preprocess_buffer(&self) -> Buffer {
        self.preprocess_buffer
    }
    ///Gets the value of [`Self::preprocess_offset`]
    pub fn preprocess_offset(&self) -> DeviceSize {
        self.preprocess_offset
    }
    ///Gets the value of [`Self::preprocess_size`]
    pub fn preprocess_size(&self) -> DeviceSize {
        self.preprocess_size
    }
    ///Gets the value of [`Self::sequences_count_buffer`]
    pub fn sequences_count_buffer(&self) -> Buffer {
        self.sequences_count_buffer
    }
    ///Gets the value of [`Self::sequences_count_offset`]
    pub fn sequences_count_offset(&self) -> DeviceSize {
        self.sequences_count_offset
    }
    ///Gets the value of [`Self::sequences_index_buffer`]
    pub fn sequences_index_buffer(&self) -> Buffer {
        self.sequences_index_buffer
    }
    ///Gets the value of [`Self::sequences_index_offset`]
    pub fn sequences_index_offset(&self) -> DeviceSize {
        self.sequences_index_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::indirect_commands_layout`]
    pub fn indirect_commands_layout_mut(&mut self) -> &mut IndirectCommandsLayoutNV {
        &mut self.indirect_commands_layout
    }
    ///Gets a mutable reference to the value of [`Self::stream_count`]
    pub fn stream_count_mut(&mut self) -> &mut u32 {
        &mut self.stream_count
    }
    ///Gets a mutable reference to the value of [`Self::sequences_count`]
    pub fn sequences_count_mut(&mut self) -> &mut u32 {
        &mut self.sequences_count
    }
    ///Gets a mutable reference to the value of [`Self::preprocess_buffer`]
    pub fn preprocess_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.preprocess_buffer
    }
    ///Gets a mutable reference to the value of [`Self::preprocess_offset`]
    pub fn preprocess_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.preprocess_offset
    }
    ///Gets a mutable reference to the value of [`Self::preprocess_size`]
    pub fn preprocess_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.preprocess_size
    }
    ///Gets a mutable reference to the value of [`Self::sequences_count_buffer`]
    pub fn sequences_count_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.sequences_count_buffer
    }
    ///Gets a mutable reference to the value of [`Self::sequences_count_offset`]
    pub fn sequences_count_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.sequences_count_offset
    }
    ///Gets a mutable reference to the value of [`Self::sequences_index_buffer`]
    pub fn sequences_index_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.sequences_index_buffer
    }
    ///Gets a mutable reference to the value of [`Self::sequences_index_offset`]
    pub fn sequences_index_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.sequences_index_offset
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
    ///Sets the value of [`Self::pipeline_bind_point`]
    pub fn set_pipeline_bind_point(mut self, value: crate::vulkan1_0::PipelineBindPoint) -> Self {
        self.pipeline_bind_point = value;
        self
    }
    ///Sets the value of [`Self::pipeline`]
    pub fn set_pipeline(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.pipeline = value;
        self
    }
    ///Sets the value of [`Self::indirect_commands_layout`]
    pub fn set_indirect_commands_layout(
        mut self,
        value: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    ) -> Self {
        self.indirect_commands_layout = value;
        self
    }
    ///Sets the value of [`Self::stream_count`]
    pub fn set_stream_count(mut self, value: u32) -> Self {
        self.stream_count = value;
        self
    }
    ///Sets the value of [`Self::streams`]
    pub fn set_streams(
        mut self,
        value: &'lt [crate::extensions::nv_device_generated_commands::IndirectCommandsStreamNV],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.streams = value.as_ptr();
        self.stream_count = len_;
        self
    }
    ///Sets the value of [`Self::sequences_count`]
    pub fn set_sequences_count(mut self, value: u32) -> Self {
        self.sequences_count = value;
        self
    }
    ///Sets the value of [`Self::preprocess_buffer`]
    pub fn set_preprocess_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.preprocess_buffer = value;
        self
    }
    ///Sets the value of [`Self::preprocess_offset`]
    pub fn set_preprocess_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.preprocess_offset = value;
        self
    }
    ///Sets the value of [`Self::preprocess_size`]
    pub fn set_preprocess_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.preprocess_size = value;
        self
    }
    ///Sets the value of [`Self::sequences_count_buffer`]
    pub fn set_sequences_count_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.sequences_count_buffer = value;
        self
    }
    ///Sets the value of [`Self::sequences_count_offset`]
    pub fn set_sequences_count_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.sequences_count_offset = value;
        self
    }
    ///Sets the value of [`Self::sequences_index_buffer`]
    pub fn set_sequences_index_buffer(mut self, value: crate::vulkan1_0::Buffer) -> Self {
        self.sequences_index_buffer = value;
        self
    }
    ///Sets the value of [`Self::sequences_index_offset`]
    pub fn set_sequences_index_offset(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.sequences_index_offset = value;
        self
    }
}
///[VkGeneratedCommandsMemoryRequirementsInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html) - Structure specifying parameters for the reservation of preprocess buffer space
///# C Specifications
///```c
///// Provided by VK_NV_device_generated_commands
///typedef struct VkGeneratedCommandsMemoryRequirementsInfoNV {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkPipelineBindPoint           pipelineBindPoint;
///    VkPipeline                    pipeline;
///    VkIndirectCommandsLayoutNV    indirectCommandsLayout;
///    uint32_t                      maxSequencesCount;
///} VkGeneratedCommandsMemoryRequirementsInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_bind_point`] is the [`PipelineBindPoint`] of the [`pipeline`] that this buffer
///   memory is intended to be used with during the execution.
/// - [`pipeline`] is the [`Pipeline`] that this buffer memory is intended to be used with during
///   the execution.
/// - [`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`] that this buffer memory is
///   intended to be used with.
/// - [`max_sequences_count`] is the maximum number of sequences that this buffer memory in
///   combination with the other state provided  **can**  be used with.
///# Description
///## Valid Usage
/// - [`max_sequences_count`] **must**  be less or equal to
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV::max_indirect_sequence_count`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
/// - [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
/// - Both of [`indirect_commands_layout`], and [`pipeline`] **must**  have been created, allocated,
///   or retrieved from the same [`Device`]
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`IndirectCommandsLayoutNV`]
/// - [`Pipeline`]
/// - [`PipelineBindPoint`]
/// - [`StructureType`]
/// - [`get_generated_commands_memory_requirements_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeneratedCommandsMemoryRequirementsInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline_bind_point`] is the [`PipelineBindPoint`] of the
    ///[`pipeline`] that this buffer memory is intended to be used with
    ///during the execution.
    pub pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline`] is the [`Pipeline`] that this buffer memory is
    ///intended to be used with during the execution.
    pub pipeline: Pipeline,
    ///[`indirect_commands_layout`] is the [`IndirectCommandsLayoutNV`]
    ///that this buffer memory is intended to be used with.
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    ///[`max_sequences_count`] is the maximum number of sequences that this
    ///buffer memory in combination with the other state provided  **can**  be used
    ///with.
    pub max_sequences_count: u32,
}
impl<'lt> Default for GeneratedCommandsMemoryRequirementsInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: std::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            max_sequences_count: 0,
        }
    }
}
impl<'lt> GeneratedCommandsMemoryRequirementsInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets the value of [`Self::indirect_commands_layout`]
    pub fn indirect_commands_layout(&self) -> IndirectCommandsLayoutNV {
        self.indirect_commands_layout
    }
    ///Gets the value of [`Self::max_sequences_count`]
    pub fn max_sequences_count(&self) -> u32 {
        self.max_sequences_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::indirect_commands_layout`]
    pub fn indirect_commands_layout_mut(&mut self) -> &mut IndirectCommandsLayoutNV {
        &mut self.indirect_commands_layout
    }
    ///Gets a mutable reference to the value of [`Self::max_sequences_count`]
    pub fn max_sequences_count_mut(&mut self) -> &mut u32 {
        &mut self.max_sequences_count
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
    ///Sets the value of [`Self::pipeline_bind_point`]
    pub fn set_pipeline_bind_point(mut self, value: crate::vulkan1_0::PipelineBindPoint) -> Self {
        self.pipeline_bind_point = value;
        self
    }
    ///Sets the value of [`Self::pipeline`]
    pub fn set_pipeline(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.pipeline = value;
        self
    }
    ///Sets the value of [`Self::indirect_commands_layout`]
    pub fn set_indirect_commands_layout(
        mut self,
        value: crate::extensions::nv_device_generated_commands::IndirectCommandsLayoutNV,
    ) -> Self {
        self.indirect_commands_layout = value;
        self
    }
    ///Sets the value of [`Self::max_sequences_count`]
    pub fn set_max_sequences_count(mut self, value: u32) -> Self {
        self.max_sequences_count = value;
        self
    }
}
impl Device {
    ///[vkGetGeneratedCommandsMemoryRequirementsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html) - Retrieve the buffer allocation requirements for generated commands
    ///# C Specifications
    ///The generation of commands on the device requires a `preprocess` buffer.
    ///To retrieve the memory size and alignment requirements of a particular
    ///execution state call:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///void vkGetGeneratedCommandsMemoryRequirementsNV(
    ///    VkDevice                                    device,
    ///    const VkGeneratedCommandsMemoryRequirementsInfoNV* pInfo,
    ///    VkMemoryRequirements2*                      pMemoryRequirements);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the buffer.
    /// - [`p_info`] is a pointer to a [`GeneratedCommandsMemoryRequirementsInfoNV`] structure
    ///   containing parameters required for the memory requirements query.
    /// - [`p_memory_requirements`] is a pointer to a [`MemoryRequirements2`] structure in which the
    ///   memory requirements of the buffer object are returned.
    ///# Description
    ///## Valid Usage
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_info`] **must**  be a valid pointer to a valid
    ///   [`GeneratedCommandsMemoryRequirementsInfoNV`] structure
    /// - [`p_memory_requirements`] **must**  be a valid pointer to a [`MemoryRequirements2`]
    ///   structure
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`Device`]
    /// - [`GeneratedCommandsMemoryRequirementsInfoNV`]
    /// - [`MemoryRequirements2`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetGeneratedCommandsMemoryRequirementsNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_generated_commands_memory_requirements_nv<'lt>(
        self: &Unique<Device>,
        p_info: &GeneratedCommandsMemoryRequirementsInfoNV<'lt>,
        p_memory_requirements: Option<MemoryRequirements2<'lt>>,
    ) -> MemoryRequirements2<'lt> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.get_generated_commands_memory_requirements_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.get_generated_commands_memory_requirements_nv())
            .unwrap_unchecked();
        let mut p_memory_requirements = p_memory_requirements.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            p_info as *const GeneratedCommandsMemoryRequirementsInfoNV<'lt>,
            &mut p_memory_requirements,
        );
        {
            p_memory_requirements.p_next = std::ptr::null_mut();
            p_memory_requirements
        }
    }
}
impl Device {
    ///[vkCreateIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIndirectCommandsLayoutNV.html) - Create an indirect command layout object
    ///# C Specifications
    ///Indirect command layouts are created by:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///VkResult vkCreateIndirectCommandsLayoutNV(
    ///    VkDevice                                    device,
    ///    const VkIndirectCommandsLayoutCreateInfoNV* pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkIndirectCommandsLayoutNV*                 pIndirectCommandsLayout);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the indirect command layout.
    /// - [`p_create_info`] is a pointer to a [`IndirectCommandsLayoutCreateInfoNV`] structure
    ///   containing parameters affecting creation of the indirect command layout.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_indirect_commands_layout`] is a pointer to a [`IndirectCommandsLayoutNV`] handle in
    ///   which the resulting indirect command layout is returned.
    ///# Description
    ///## Valid Usage
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid
    ///   [`IndirectCommandsLayoutCreateInfoNV`] structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_indirect_commands_layout`] **must**  be a valid pointer to a
    ///   [`IndirectCommandsLayoutNV`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`IndirectCommandsLayoutCreateInfoNV`]
    /// - [`IndirectCommandsLayoutNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateIndirectCommandsLayoutNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_indirect_commands_layout_nv<'lt>(
        self: &Unique<Device>,
        p_create_info: &IndirectCommandsLayoutCreateInfoNV<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<IndirectCommandsLayoutNV>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.create_indirect_commands_layout_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.create_indirect_commands_layout_nv())
            .unwrap_unchecked();
        let mut p_indirect_commands_layout = MaybeUninit::<IndirectCommandsLayoutNV>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const IndirectCommandsLayoutCreateInfoNV<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_indirect_commands_layout.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_indirect_commands_layout.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyIndirectCommandsLayoutNV.html) - Destroy an indirect commands layout
    ///# C Specifications
    ///Indirect command layouts are destroyed by:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///void vkDestroyIndirectCommandsLayoutNV(
    ///    VkDevice                                    device,
    ///    VkIndirectCommandsLayoutNV                  indirectCommandsLayout,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that destroys the layout.
    /// - [`indirect_commands_layout`] is the layout to destroy.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    ///# Description
    ///## Valid Usage
    /// - All submitted commands that refer to [`indirect_commands_layout`] **must**  have completed
    ///   execution
    /// - If [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created, a
    ///   compatible set of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`indirect_commands_layout`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`indirect_commands_layout`] is not [`crate::Handle::null`],
    ///   [`indirect_commands_layout`] **must**  be a valid [`IndirectCommandsLayoutNV`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`indirect_commands_layout`] is a valid handle, it  **must**  have been created,
    ///   allocated, or retrieved from [`device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`indirect_commands_layout`] **must**  be externally synchronized
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`IndirectCommandsLayoutNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyIndirectCommandsLayoutNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_indirect_commands_layout_nv<'lt>(
        self: &Unique<Device>,
        indirect_commands_layout: Option<IndirectCommandsLayoutNV>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.destroy_indirect_commands_layout_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.destroy_indirect_commands_layout_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            indirect_commands_layout.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdExecuteGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdExecuteGeneratedCommandsNV.html) - Generate and execute commands on the device
    ///# C Specifications
    ///The actual generation of commands as well as their execution on the device
    ///is handled as single action with:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///void vkCmdExecuteGeneratedCommandsNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkBool32                                    isPreprocessed,
    ///    const VkGeneratedCommandsInfoNV*            pGeneratedCommandsInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`is_preprocessed`] represents whether the input data has already been preprocessed on the
    ///   device. If it is [`FALSE`] this command will implicitly trigger the preprocessing step,
    ///   otherwise not.
    /// - [`p_generated_commands_info`] is a pointer to a [`GeneratedCommandsInfoNV`] structure
    ///   containing parameters affecting the generation of commands.
    ///# Description
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`VK_NV_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and
    ///   done so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
    ///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
    ///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The subpass index of the current render pass  **must**  be equal to the `subpass` member
    ///   of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`]
    ///   bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via
    ///   a descriptor set
    /// - Memory backing image subresources used as attachments in the current render pass  **must**
    ///   not be written in any way other than as an attachment by this command
    /// - If any recorded command in the current subpass will write to an image subresource as an
    ///   attachment, this command  **must**  not read from the memory backing that image
    ///   subresource in any other way than as an attachment
    /// - If any recorded command in the current subpass will read from an image subresource used as
    ///   an attachment in any way other than as an attachment, this command  **must**  not write to
    ///   that image subresource as an attachment
    /// - If the draw is recorded in a render pass instance with multiview enabled, the maximum
    ///   instance index  **must**  be less than or equal to
    ///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
    /// - If the bound graphics pipeline was created with
    ///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and
    ///   the current subpass has a depth/stencil attachment, then that attachment  **must**  have
    ///   been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
    ///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with both the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
    ///   states enabled then both [`cmd_set_viewport_with_count`] and
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
    ///   pipeline  **must**  have been created with
    ///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
    ///   dynamic states enabled then the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
    ///   graphics pipeline  **must**  have been created with
    ///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to
    ///   the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`]
    ///   **must**  be greater than or equal to the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal
    ///   to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
    ///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
    ///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then
    ///   [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    ///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the
    ///   current command buffer prior to this drawing command and the `logicOp` **must**  be a
    ///   valid [`LogicOp`] value
    /// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
    ///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    ///   dynamic state enabled, and any of the shader stages of the bound graphics pipeline write
    ///   to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**
    ///   have been called in the current command buffer prior to this drawing command, and the
    ///   `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
    /// - If rasterization is not disabled in the bound graphics pipeline, then for each color
    ///   attachment in the subpass, if the corresponding image view’s [format features]() do not
    ///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of
    ///   the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
    ///   [`FALSE`]
    /// - If rasterization is not disabled in the bound graphics pipeline, and neither the
    ///   `[`VK_AMD_mixed_attachment_samples`]` nor the `[`VK_NV_framebuffer_mixed_samples`]`
    ///   extensions are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`]
    ///   **must**  be the same as the current subpass color and/or depth/stencil attachments
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this
    ///   command  **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any
    ///   values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not
    ///   write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not
    ///   write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command
    ///   **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
    ///   this command  **must**  not write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
    ///   [`RenderingInfo::color_attachment_count`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
    ///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
    ///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
    ///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
    ///   create the currently bound graphics pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `attachmentCount` parameter of
    ///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
    ///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
    ///   pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
    /// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the corresponding element of the `pColorAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the value of
    ///   [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently
    ///   bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline  **must**  have been created with a
    ///   [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]
    ///
    /// - All vertex input bindings accessed via vertex input variables declared in the vertex
    ///   shader entry point’s interface  **must**  have either valid or [`crate::Handle::null`]
    ///   buffers bound
    /// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
    ///   vertex input variables declared in the vertex shader entry point’s interface  **must**
    ///   not be [`crate::Handle::null`]
    /// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely
    ///   contained within the corresponding vertex buffer binding, as described in
    ///   [[fxvertex-input]]()
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `primitiveTopology` parameter of
    ///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the
    ///   pipeline [`PipelineInputAssemblyStateCreateInfo::topology`] state
    /// - If the bound graphics pipeline was created with both the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`
    ///   dynamic states enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in
    ///   the current command buffer prior to this draw command
    /// - If the bound graphics pipeline was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command, and the `pStrides` parameter of
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  not be `NULL`
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
    ///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
    ///   `VK_SHADER_STAGE_MESH_BIT_NV`
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - If [`is_preprocessed`] is [`TRUE`] then [`cmd_preprocess_generated_commands_nv`] **must**
    ///   have already been executed on the device, using the same [`p_generated_commands_info`]
    ///   content as well as the content of the input buffers it references (all except
    ///   [`GeneratedCommandsInfoNV::preprocess_buffer`]). Furthermore
    ///   [`p_generated_commands_info`]`s `indirectCommandsLayout` **must**  have been created with
    ///   the `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV` bit set
    /// - [`GeneratedCommandsInfoNV::pipeline`] **must**  match the current bound pipeline at
    ///   [`GeneratedCommandsInfoNV::pipeline_bind_point`]
    /// - Transform feedback  **must**  not be active
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_generated_commands_info`] **must**  be a valid pointer to a valid
    ///   [`GeneratedCommandsInfoNV`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - This command  **must**  only be called inside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`Bool32`]
    /// - [`CommandBuffer`]
    /// - [`GeneratedCommandsInfoNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdExecuteGeneratedCommandsNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_execute_generated_commands_nv<'lt>(
        self: &Unique<CommandBuffer>,
        is_preprocessed: bool,
        p_generated_commands_info: &GeneratedCommandsInfoNV<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_execute_generated_commands_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_execute_generated_commands_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            is_preprocessed as u8 as u32,
            p_generated_commands_info as *const GeneratedCommandsInfoNV<'lt>,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdPreprocessGeneratedCommandsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdPreprocessGeneratedCommandsNV.html) - Performs preprocessing for generated commands
    ///# C Specifications
    ///Commands  **can**  be preprocessed prior execution using the following command:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///void vkCmdPreprocessGeneratedCommandsNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkGeneratedCommandsInfoNV*            pGeneratedCommandsInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer which does the preprocessing.
    /// - [`p_generated_commands_info`] is a pointer to a [`GeneratedCommandsInfoNV`] structure
    ///   containing parameters affecting the preprocessing step.
    ///# Description
    ///## Valid Usage
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - [`p_generated_commands_info`]`s `indirectCommandsLayout` **must**  have been created with
    ///   the `VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV` bit set
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_generated_commands_info`] **must**  be a valid pointer to a valid
    ///   [`GeneratedCommandsInfoNV`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - This command  **must**  only be called outside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`CommandBuffer`]
    /// - [`GeneratedCommandsInfoNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdPreprocessGeneratedCommandsNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_preprocess_generated_commands_nv<'lt>(
        self: &Unique<CommandBuffer>,
        p_generated_commands_info: &GeneratedCommandsInfoNV<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_preprocess_generated_commands_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_preprocess_generated_commands_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_generated_commands_info as *const GeneratedCommandsInfoNV<'lt>,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdBindPipelineShaderGroupNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindPipelineShaderGroupNV.html) - Bind a pipeline object
    ///# C Specifications
    ///For pipelines that were created with the support of multiple shader groups
    ///(see [Graphics Pipeline Shader Groups](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#graphics-shadergroups)), the regular
    ///[`cmd_bind_pipeline`] command will bind Shader Group `0`.
    ///To explicitly bind a shader group use:
    ///```c
    ///// Provided by VK_NV_device_generated_commands
    ///void vkCmdBindPipelineShaderGroupNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkPipelineBindPoint                         pipelineBindPoint,
    ///    VkPipeline                                  pipeline,
    ///    uint32_t                                    groupIndex);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer that the pipeline will be bound to.
    /// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the bind point to
    ///   which the pipeline will be bound.
    /// - [`pipeline`] is the pipeline to be bound.
    /// - [`group_index`] is the shader group to be bound.
    ///# Description
    ///## Valid Usage
    /// - [`group_index`] **must**  be `0` or less than the effective
    ///   [`GraphicsPipelineShaderGroupsCreateInfoNV::group_count`] including the referenced
    ///   pipelines
    /// - The [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The same restrictions as [`cmd_bind_pipeline`] apply as if the bound pipeline was created
    ///   only with the Shader Group from the [`group_index`] information
    /// - The [[`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV::device_generated_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-deviceGeneratedCommands)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
    /// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - Both of [`command_buffer`], and [`pipeline`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_device_generated_commands`]
    /// - [`CommandBuffer`]
    /// - [`Pipeline`]
    /// - [`PipelineBindPoint`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBindPipelineShaderGroupNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        self: &Unique<CommandBuffer>,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_bind_pipeline_shader_group_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_device_generated_commands()
            .and_then(|vtable| vtable.cmd_bind_pipeline_shader_group_nv())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            pipeline_bind_point,
            pipeline,
            group_index.unwrap_or_default() as _,
        );
        ()
    }
}
///[VkIndirectCommandsLayoutNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsLayoutNV.html) - Opaque handle to an indirect commands layout object
///# C Specifications
///The device-side command generation happens through an iterative processing
///of an atomic sequence comprised of command tokens, which are represented by:
///```c
///// Provided by VK_NV_device_generated_commands
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkIndirectCommandsLayoutNV)
///```
///# Related
/// - [`VK_NV_device_generated_commands`]
/// - [`GeneratedCommandsInfoNV`]
/// - [`GeneratedCommandsMemoryRequirementsInfoNV`]
/// - [`create_indirect_commands_layout_nv`]
/// - [`destroy_indirect_commands_layout_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct IndirectCommandsLayoutNV(pub u64);
impl IndirectCommandsLayoutNV {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for IndirectCommandsLayoutNV {}
impl Default for IndirectCommandsLayoutNV {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for IndirectCommandsLayoutNV {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Storage = u64;
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
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_indirect_commands_layout_nv(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<IndirectCommandsLayoutNV> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Device`] for functions from `VK_NV_device_generated_commands`
pub struct DeviceNvDeviceGeneratedCommandsVTable {
    ///See [`FNGetGeneratedCommandsMemoryRequirementsNv`] for more information.
    pub get_generated_commands_memory_requirements_nv: FNGetGeneratedCommandsMemoryRequirementsNv,
    ///See [`FNCreateIndirectCommandsLayoutNv`] for more information.
    pub create_indirect_commands_layout_nv: FNCreateIndirectCommandsLayoutNv,
    ///See [`FNDestroyIndirectCommandsLayoutNv`] for more information.
    pub destroy_indirect_commands_layout_nv: FNDestroyIndirectCommandsLayoutNv,
    ///See [`FNCmdExecuteGeneratedCommandsNv`] for more information.
    pub cmd_execute_generated_commands_nv: FNCmdExecuteGeneratedCommandsNv,
    ///See [`FNCmdPreprocessGeneratedCommandsNv`] for more information.
    pub cmd_preprocess_generated_commands_nv: FNCmdPreprocessGeneratedCommandsNv,
    ///See [`FNCmdBindPipelineShaderGroupNv`] for more information.
    pub cmd_bind_pipeline_shader_group_nv: FNCmdBindPipelineShaderGroupNv,
}
impl DeviceNvDeviceGeneratedCommandsVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_generated_commands_memory_requirements_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetGeneratedCommandsMemoryRequirementsNV").as_ptr(),
                ))
            },
            create_indirect_commands_layout_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCreateIndirectCommandsLayoutNV").as_ptr(),
                ))
            },
            destroy_indirect_commands_layout_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyIndirectCommandsLayoutNV").as_ptr(),
                ))
            },
            cmd_execute_generated_commands_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdExecuteGeneratedCommandsNV").as_ptr(),
                ))
            },
            cmd_preprocess_generated_commands_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdPreprocessGeneratedCommandsNV").as_ptr(),
                ))
            },
            cmd_bind_pipeline_shader_group_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBindPipelineShaderGroupNV").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_generated_commands_memory_requirements_nv`]. See
    /// [`FNGetGeneratedCommandsMemoryRequirementsNv`] for more information.
    pub fn get_generated_commands_memory_requirements_nv(&self) -> FNGetGeneratedCommandsMemoryRequirementsNv {
        self.get_generated_commands_memory_requirements_nv
    }
    ///Gets [`Self::create_indirect_commands_layout_nv`]. See [`FNCreateIndirectCommandsLayoutNv`]
    /// for more information.
    pub fn create_indirect_commands_layout_nv(&self) -> FNCreateIndirectCommandsLayoutNv {
        self.create_indirect_commands_layout_nv
    }
    ///Gets [`Self::destroy_indirect_commands_layout_nv`]. See
    /// [`FNDestroyIndirectCommandsLayoutNv`] for more information.
    pub fn destroy_indirect_commands_layout_nv(&self) -> FNDestroyIndirectCommandsLayoutNv {
        self.destroy_indirect_commands_layout_nv
    }
    ///Gets [`Self::cmd_execute_generated_commands_nv`]. See [`FNCmdExecuteGeneratedCommandsNv`]
    /// for more information.
    pub fn cmd_execute_generated_commands_nv(&self) -> FNCmdExecuteGeneratedCommandsNv {
        self.cmd_execute_generated_commands_nv
    }
    ///Gets [`Self::cmd_preprocess_generated_commands_nv`]. See
    /// [`FNCmdPreprocessGeneratedCommandsNv`] for more information.
    pub fn cmd_preprocess_generated_commands_nv(&self) -> FNCmdPreprocessGeneratedCommandsNv {
        self.cmd_preprocess_generated_commands_nv
    }
    ///Gets [`Self::cmd_bind_pipeline_shader_group_nv`]. See [`FNCmdBindPipelineShaderGroupNv`] for
    /// more information.
    pub fn cmd_bind_pipeline_shader_group_nv(&self) -> FNCmdBindPipelineShaderGroupNv {
        self.cmd_bind_pipeline_shader_group_nv
    }
}
