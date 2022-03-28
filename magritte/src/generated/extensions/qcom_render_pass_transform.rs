use crate::{
    extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    vulkan1_0::{BaseOutStructure, Rect2D, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_QCOM_render_pass_transform");
///[VkRenderPassTransformBeginInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html) - Structure describing transform parameters of a render pass instance
///# C Specifications
///To begin a render pass instance with [render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform) enabled, add the
///[`RenderPassTransformBeginInfoQCOM`] to the [`p_next`] chain of
///[`RenderPassBeginInfo`] structure passed to the
///[`CmdBeginRenderPass`] command specifying the render pass transform.The
/// [`RenderPassTransformBeginInfoQCOM`] structure is defined as:
///```c
///// Provided by VK_QCOM_render_pass_transform
///typedef struct VkRenderPassTransformBeginInfoQCOM {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkSurfaceTransformFlagBitsKHR    transform;
///} VkRenderPassTransformBeginInfoQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform to be
///   applied to rasterization.
///# Description
///## Valid Usage
/// - [`transform`] **must**  be `VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`,
///   `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR`, `VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR`, or
///   `VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR`
/// - The `renderpass` **must**  have been created with [`RenderPassCreateInfo::flags`] containing
///   `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM`
///# Related
/// - [`VK_QCOM_render_pass_transform`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
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
pub struct RenderPassTransformBeginInfoQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform to be applied to rasterization.
    transform: SurfaceTransformFlagBitsKHR,
}
impl<'lt> Default for RenderPassTransformBeginInfoQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
        }
    }
}
impl<'lt> RenderPassTransformBeginInfoQCOM<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
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
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
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
    ///Sets the raw value of [`Self::transform`]
    pub fn set_transform(&mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.transform = value;
        self
    }
}
///[VkCommandBufferInheritanceRenderPassTransformInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html) - Structure describing transformed render pass parameters command buffer
///# C Specifications
///To begin recording a secondary command buffer compatible with execution
///inside a render pass using [render
///pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform), add the
///[`CommandBufferInheritanceRenderPassTransformInfoQCOM`] to the
///[`p_next`] chain of [`CommandBufferInheritanceInfo`] structure passed
///to the [`BeginCommandBuffer`] command specifying the parameters for
///transformed rasterization.The [`CommandBufferInheritanceRenderPassTransformInfoQCOM`] structure
/// is
///defined as:
///```c
///// Provided by VK_QCOM_render_pass_transform
///typedef struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkSurfaceTransformFlagBitsKHR    transform;
///    VkRect2D                         renderArea;
///} VkCommandBufferInheritanceRenderPassTransformInfoQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform to be
///   applied to the render pass.
/// - [`render_area`] is the render area that is affected by the command buffer.
///# Description
///When the secondary is recorded to execute within a render pass instance
///using [`CmdExecuteCommands`], the render pass transform parameters of
///the secondary command buffer  **must**  be consistent with the render pass
///transform parameters specified for the render pass instance.
///In particular, the [`transform`] and [`render_area`] for command buffer
/// **must**  be identical to the [`transform`] and [`render_area`] of the render
///pass instance.
///## Valid Usage
/// - [`transform`] **must**  be `VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`,
///   `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR`, `VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR`, or
///   `VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM`
///# Related
/// - [`VK_QCOM_render_pass_transform`]
/// - [`Rect2D`]
/// - [`StructureType`]
/// - [`SurfaceTransformFlagBitsKHR`]
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
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform to be applied to the render pass.
    transform: SurfaceTransformFlagBitsKHR,
    ///[`render_area`] is the render area that is affected by the command
    ///buffer.
    render_area: Rect2D,
}
impl<'lt> Default for CommandBufferInheritanceRenderPassTransformInfoQCOM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
            render_area: Default::default(),
        }
    }
}
impl<'lt> CommandBufferInheritanceRenderPassTransformInfoQCOM<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.transform
    }
    ///Gets the value of [`Self::render_area`]
    pub fn render_area(&self) -> Rect2D {
        self.render_area
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
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.transform
    }
    ///Gets a mutable reference to the value of [`Self::render_area`]
    pub fn render_area_mut(&mut self) -> &mut Rect2D {
        &mut self.render_area
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
    ///Sets the raw value of [`Self::transform`]
    pub fn set_transform(&mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> &mut Self {
        self.transform = value;
        self
    }
    ///Sets the raw value of [`Self::render_area`]
    pub fn set_render_area(&mut self, value: crate::vulkan1_0::Rect2D) -> &mut Self {
        self.render_area = value;
        self
    }
}
