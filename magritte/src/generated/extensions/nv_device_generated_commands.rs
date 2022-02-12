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
//! - create [`Buffer`] objects and retrieve physical addresses from them
//!via [`GetBufferDeviceAddressEXT`]
//! - create a graphics pipeline using
//![`GraphicsPipelineShaderGroupsCreateInfoNV`] for the ability to
//!change shaders on the device.
//! - create a [`IndirectCommandsLayoutNV`], which lists the
//![`IndirectCommandsTokenTypeNV`] it wants to dynamically execute as
//!an atomic command sequence.
//!This step likely involves some internal device code compilation, since
//!the intent is for the GPU to generate the command buffer in the
//!pipeline.
//! - fill the input stream buffers with the data for each of the inputs it
//!needs.
//!Each input is an array that will be filled with token-dependent data.
//! - set up a preprocess [`Buffer`] that uses memory according to the
//!information retrieved via
//![`GetGeneratedCommandsMemoryRequirementsNV`].
//! - optionally preprocess the generated content using
//![`CmdPreprocessGeneratedCommandsNV`], for example on an asynchronous
//!compute queue, or for the purpose of re-using the data in multiple
//!executions.
//! - call [`CmdExecuteGeneratedCommandsNV`] to create and execute the
//!actual device commands for all sequences based on the inputs provided.
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
//! - [`CmdBindPipelineShaderGroupNV`]
//! - [`CmdExecuteGeneratedCommandsNV`]
//! - [`CmdPreprocessGeneratedCommandsNV`]
//! - [`CreateIndirectCommandsLayoutNV`]
//! - [`DestroyIndirectCommandsLayoutNV`]
//! - [`GetGeneratedCommandsMemoryRequirementsNV`]
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
//! - Extending [`GraphicsPipelineCreateInfo`]:
//! - [`GraphicsPipelineShaderGroupsCreateInfoNV`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceDeviceGeneratedCommandsFeaturesNV`]
//!
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`]
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
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV`
//! - `VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`
//!
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV`
//!
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV`
//!
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV`
//! - `VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV`
//! - `VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV`
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
//![`GetGeneratedCommandsMemoryRequirementsNV`].In the NVX version the requirements were hidden
//! implicitly as part of the
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
//!Set the `isPreprocessed` to true in [`CmdExecuteGeneratedCommandsNV`].9) Under which conditions
//! is [`CmdExecuteGeneratedCommandsNV`] legal?It behaves like a regular draw call command.10) Is
//! [`CmdPreprocessGeneratedCommandsNV`] copying the input data or
//!referencing it?There are multiple implementations possible:
//! - one could have some emulation code that parses the inputs, and generates
//!an output command buffer, therefore copying the inputs.
//! - one could just reference the inputs, and have the processing done in
//!pipe at execution time.
//!If the data is mandated to be copied, then it puts a penalty on
//!implementation that could process the inputs directly in pipe.
//!If the data is “referenced”, then it allows both types of implementation.The inputs are
//! “referenced”, and **must** not be modified after the call to
//![`CmdExecuteGeneratedCommandsNV`] has completed.11) Which buffer usage flags are required for
//! the buffers referenced by
//![`GeneratedCommandsInfoNV`] ?Reuse existing `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT`
//! - [`GeneratedCommandsInfoNV::preprocess_buffer`]
//! - [`GeneratedCommandsInfoNV::sequences_count_buffer`]
//! - [`GeneratedCommandsInfoNV::sequences_index_buffer`]
//! - [`IndirectCommandsStreamNV::buffer`]
//!12) In which pipeline stage does the device generated command expansion
//!happen?[`CmdPreprocessGeneratedCommandsNV`] is treated as if it occurs in a
//!separate logical pipeline from either graphics or compute, and that pipeline
//!only includes `VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT`, a new stage
//!`VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`, and
//!`VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT`.
//!This new stage has two corresponding new access types,
//!`VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV` and
//!`VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`, used to synchronize reading
//!the buffer inputs and writing the preprocess memory output.The generated output written in the
//! preprocess buffer memory by
//![`CmdExecuteGeneratedCommandsNV`] is considered to be consumed by the
//!`VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT` pipeline stage.Thus, to synchronize from writing the input
//! buffers to preprocessing via
//![`CmdPreprocessGeneratedCommandsNV`], use:
//! - `dstStageMask` = `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//! - `dstAccessMask` = `VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV`
//!To synchronize from [`CmdPreprocessGeneratedCommandsNV`] to executing
//!the generated commands by [`CmdExecuteGeneratedCommandsNV`], use:
//! - `srcStageMask` = `VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV`
//! - `srcAccessMask` = `VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV`
//! - `dstStageMask` = `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`
//! - `dstAccessMask` = `VK_ACCESS_INDIRECT_COMMAND_READ_BIT`
//!When [`CmdExecuteGeneratedCommandsNV`] is used with a
//!`isPreprocessed` of [`FALSE`], the generated commands are implicitly
//!preprocessed, therefore one only needs to synchronize the inputs via:
//! - `dstStageMask` = `VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT`
//! - `dstAccessMask` = `VK_ACCESS_INDIRECT_COMMAND_READ_BIT`
//!13) What if most token data is “static”, but we frequently want to render
//!a subsection?Added “sequencesIndexBuffer”.
//!This allows to easier sort and filter what should actually be executed.14) What are the changes
//! compared to the previous NVX extension?
//! - Compute dispatch support was removed (was never implemented in drivers).
//!There are different approaches how dispatching from the device should
//!work, hence we defer this to a future extension.
//! - The `ObjectTableNVX` was replaced by using physical buffer addresses and
//!introducing Shader Groups for the graphics pipeline.
//! - Less state changes are possible overall, but the important operations
//!are still there (reduces complexity of implementation).
//! - The API was redesigned so all inputs must be passed at both
//!preprocessing and execution time (this was implicit in NVX, now it is
//!explicit)
//! - The reservation of intermediate command space is now mandatory and
//!explicit through a preprocess buffer.
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
//! - Revision 1, 2020-02-20 (Christoph Kubisch)
//! - Initial version
//!
//! - Revision 2, 2020-03-09 (Christoph Kubisch)
//! - Remove VK_EXT_debug_report interactions
//!
//! - Revision 3, 2020-03-09 (Christoph Kubisch)
//! - Fix naming VkPhysicalDeviceGenerated to
//!VkPhysicalDeviceDeviceGenerated
//!# Other info
//! * 2020-02-20
//!*
//! - This extension requires Vulkan 1.1
//! - This extension requires [`VK_EXT_buffer_device_address`] or
//![`VK_KHR_buffer_device_address`] or Vulkan 1.2 for the ability to bind
//!vertex and index buffers on the device.
//! - This extension interacts with [`VK_NV_mesh_shader`].
//!If the latter extension is not supported, remove the command token to
//!initiate mesh tasks drawing in this extension.
//!
//!*
//! - Christoph Kubisch, NVIDIA
//! - Pierre Boudier, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Eric Werness, NVIDIA
//! - Yuriy O’Donnell, Epic Games
//! - Baldur Karlsson, Valve
//! - Mathias Schott, NVIDIA
//! - Tyson Smith, NVIDIA
//! - Ingo Esser, NVIDIA
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
//! - [`CmdBindPipelineShaderGroupNV`]
//! - [`CmdExecuteGeneratedCommandsNV`]
//! - [`CmdPreprocessGeneratedCommandsNV`]
//! - [`CreateIndirectCommandsLayoutNV`]
//! - [`DestroyIndirectCommandsLayoutNV`]
//! - [`GetGeneratedCommandsMemoryRequirementsNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_generated_commands");
///[VkIndirectCommandsTokenTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkIndirectCommandsTokenTypeNV.html) - Enum specifying token commands
///# C Specifications
///Possible values of those elements of the
///[`IndirectCommandsLayoutCreateInfoNV::p_tokens`] array specifying
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
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct IndirectCommandsTokenTypeNV(i32);
impl const Default for IndirectCommandsTokenTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for IndirectCommandsTokenTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("IndirectCommandsTokenTypeNV")
            .field(match *self {
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP => &"INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS => &"INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER => &"INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER => &"INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT => &"INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW",
                Self::INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS => &"INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS",
                other => unreachable!("invalid value for `IndirectCommandsTokenTypeNV`: {:?}", other),
            })
            .finish()
    }
}
impl IndirectCommandsTokenTypeNV {
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP: Self = Self(0);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS: Self = Self(1);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER: Self = Self(2);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER: Self = Self(3);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT: Self = Self(4);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED: Self = Self(5);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW: Self = Self(6);
    ///No documentation found
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS: Self = Self(7);
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
}
