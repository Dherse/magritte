//![VK_NV_viewport_swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_viewport_swizzle.html) - device extension
//!# Description
//!This extension provides a new per-viewport swizzle that can modify the
//!position of primitives sent to each viewport.
//!New viewport swizzle state is added for each viewport, and a new position
//!vector is computed for each vertex by selecting from and optionally negating
//!any of the four components of the original position vector.This new viewport swizzle is useful
//! for a number of algorithms, including
//!single-pass cube map rendering (broadcasting a primitive to multiple faces
//!and reorienting the vertex position for each face) and voxel rasterization.
//!The per-viewport component remapping and negation provided by the swizzle
//!allows application code to re-orient three-dimensional geometry with a view
//!along any of the  **X** ,  **Y** , or  **Z**  axes.
//!If a perspective projection and depth buffering is required, 1/W
//!buffering should be used, as described in the single-pass cube map rendering
//!example in the “Issues” section below.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_viewport_swizzle]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_NV_viewport_swizzle
//!   extension>>)
//!# New structures
//! - [`ViewportSwizzleNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:  - [`PipelineViewportSwizzleStateCreateInfoNV`]
//!# New enums
//! - [`ViewportCoordinateSwizzleNV`]
//!# New bitmasks
//! - [`PipelineViewportSwizzleStateCreateFlagsNV`]
//!# New constants
//! - [`NV_VIEWPORT_SWIZZLE_EXTENSION_NAME`]
//! - [`NV_VIEWPORT_SWIZZLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) Where does viewport swizzling occur in the pipeline? **RESOLVED** : Despite being associated
//! with the viewport, viewport swizzling
//!must happen prior to the viewport transform.
//!In particular, it needs to be performed before clipping and perspective
//!division.The viewport mask expansion (`[`VK_NV_viewport_array2`]`) and the
//!viewport swizzle could potentially be performed before or after transform
//!feedback, but feeding back several viewports worth of primitives with
//!different swizzles does not seem particularly useful.
//!This specification applies the viewport mask and swizzle after transform
//!feedback, and makes primitive queries only count each primitive once.2) Any interesting examples
//! of how this extension,
//!`[`VK_NV_viewport_array2`]`, and
//!`[`VK_NV_geometry_shader_passthrough`]` can be used together in practice? **RESOLVED** : One
//! interesting use case for this extension is for single-pass
//!rendering to a cube map.
//!In this example, the application would attach a cube map texture to a
//!layered FBO where the six cube faces are treated as layers.
//!Vertices are sent through the vertex shader without applying a projection
//!matrix, where the `gl_Position` output is (x,y,z,1) and the center
//!of the cube map is at (0,0,0).
//!With unextended Vulkan, one could have a conventional instanced geometry
//!shader that looks something like the following:
//!```c
//!layout(invocations = 6) in;     // separate invocation per face
//!layout(triangles) in;
//!layout(triangle_strip) out;
//!layout(max_vertices = 3) out;
//!
//!in Inputs {
//!vec2 texcoord;
//!vec3 normal;
//!vec4 baseColor;
//!} v[];
//!
//!    out Outputs {
//!    vec2 texcoord;
//!    vec3 normal;
//!    vec4 baseColor;
//!    };
//!
//!    void main()
//!    {
//!    int face = gl_InvocationID;  // which face am I?
//!
//!    // Project gl_Position for each vertex onto the cube map face.
//!    vec4 positions[3];
//!    for (int i = 0; i < 3; i++) {
//!        positions[i] = rotate(gl_in[i].gl_Position, face);
//!    }
//!
//!    // If the primitive does not project onto this face, we are done.
//!    if (shouldCull(positions)) {
//!        return;
//!    }
//!
//!    // Otherwise, emit a copy of the input primitive to the
//!    // appropriate face (using gl_Layer).
//!    for (int i = 0; i < 3; i++) {
//!        gl_Layer = face;
//!        gl_Position = positions[i];
//!        texcoord = v[i].texcoord;
//!        normal = v[i].normal;
//!        baseColor = v[i].baseColor;
//!        EmitVertex();
//!    }
//!}
//!```
//!With passthrough geometry shaders, this can be done using a much simpler
//!shader:
//!```c
//!layout(triangles) in;
//!layout(passthrough) in Inputs {
//!    vec2 texcoord;
//!    vec3 normal;
//!    vec4 baseColor;
//!}
//!layout(passthrough) in gl_PerVertex {
//!    vec4 gl_Position;
//!} gl_in[];
//!layout(viewport_relative) out int gl_Layer;
//!
//!void main()
//!{
//!    // Figure out which faces the primitive projects onto and
//!    // generate a corresponding viewport mask.
//!    uint mask = 0;
//!    for (int i = 0; i < 6; i++) {
//!        if (!shouldCull(face)) {
//!        mask |= 1U << i;
//!        }
//!    }
//!    gl_ViewportMask = mask;
//!    gl_Layer = 0;
//!}
//!```
//!The application code is set up so that each of the six cube faces has a
//!separate viewport (numbered 0 to 5).
//!Each face also has a separate swizzle, programmed via the
//![`PipelineViewportSwizzleStateCreateInfoNV`] pipeline state.
//!The viewport swizzle feature performs the coordinate transformation handled
//!by the `rotate`() function in the original shader.
//!The `viewport_relative` layout qualifier says that the viewport number (0
//!to 5) is added to the base `gl_Layer` value of 0 to determine which layer
//!(cube face) the primitive should be sent to.Note that the use of the passed through input
//! `normal` in this example
//!suggests that the fragment shader in this example would perform an operation
//!like per-fragment lighting.
//!The viewport swizzle would transform the position to be face-relative, but
//!`normal` would remain in the original coordinate system.
//!It seems likely that the fragment shader in either version of the example
//!would want to perform lighting in the original coordinate system.
//!It would likely do this by reconstructing the position of the fragment in
//!the original coordinate system using `gl_FragCoord`, a constant or
//!uniform holding the size of the cube face, and the input
//!`gl_ViewportIndex` (or `gl_Layer`), which identifies the cube face.
//!Since the value of `normal` is in the original coordinate system, it
//!would not need to be modified as part of this coordinate transformation.Note that while the
//! `rotate`() operation in the regular geometry shader
//!above could include an arbitrary post-rotation projection matrix, the
//!viewport swizzle does not support arbitrary math.
//!To get proper projection, 1/W buffering should be used.
//!To do this:
//!0. Program the viewport swizzles to move the pre-projection W eye coordinate (typically 1.0)
//! into the Z coordinate of the swizzle output and the eye coordinate component used for depth into
//! the W coordinate. For example, the viewport corresponding to the +Z face might use a swizzle of
//! (+X, -Y, +W, +Z). The Z normalized device coordinate computed after swizzling would then be
//! z'/w' = 1/Z<sub>eye</sub>.
//!1. On NVIDIA implementations supporting floating-point depth buffers with values outside [0,1],
//! prevent unwanted near plane clipping by enabling `depthClampEnable`. Ensure that the depth clamp
//! does not mess up depth testing by programming the depth range to very large values, such as
//! `minDepthBounds`=-z, `maxDepthBounds`=+z, where z = 2<sup>127</sup>. It should be possible to
//! use IEEE infinity encodings also (`0xFF800000` for `-INF`, `0x7F800000` for `+INF`). Even when
//! near/far clipping is disabled, primitives extending behind the eye will still be clipped because
//! one or more vertices will have a negative W coordinate and fail X/Y clipping tests.On other
//! implementations, scale X, Y, and Z eye coordinates so that vertices on the near plane have a
//! post-swizzle W coordinate of 1.0. For example, if the near plane is at Z<sub>eye</sub> = 1/256,
//! scale X, Y, and Z by 256.
//!2. Adjust depth testing to reflect the fact that 1/W values are large near the eye and small
//! away from the eye. Clear the depth buffer to zero (infinitely far away) and use a depth test of
//! `VK_COMPARE_OP_GREATER` instead of `VK_COMPARE_OP_LESS`.
//!# Version History
//! - Revision 1, 2016-12-22 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2016-12-22
//! * - This extension requires `multiViewport` and `geometryShader` features to be useful.
//! * - Daniel Koch, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PipelineViewportSwizzleStateCreateFlagsNV`]
//! - [`PipelineViewportSwizzleStateCreateInfoNV`]
//! - [`ViewportCoordinateSwizzleNV`]
//! - [`ViewportSwizzleNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_viewport_swizzle");
///[VkViewportCoordinateSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportCoordinateSwizzleNV.html) - Specify how a viewport coordinate is swizzled
///# C Specifications
///Possible values of the [`ViewportSwizzleNV::x`], `y`, `z`,
///and `w` members, specifying swizzling of the corresponding components of
///primitives, are:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef enum VkViewportCoordinateSwizzleNV {
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6,
///    VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7,
///} VkViewportCoordinateSwizzleNV;
///```
///# Description
///These values are described in detail in [Viewport Swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-viewport-swizzle).
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`ViewportSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(i32);
impl const Default for ViewportCoordinateSwizzleNV {
    fn default() -> Self {
        Self(0)
    }
}
impl ViewportCoordinateSwizzleNV {
    ///No documentation found
    pub const POSITIVE_X: Self = Self(0);
    ///No documentation found
    pub const NEGATIVE_X: Self = Self(1);
    ///No documentation found
    pub const POSITIVE_Y: Self = Self(2);
    ///No documentation found
    pub const NEGATIVE_Y: Self = Self(3);
    ///No documentation found
    pub const POSITIVE_Z: Self = Self(4);
    ///No documentation found
    pub const NEGATIVE_Z: Self = Self(5);
    ///No documentation found
    pub const POSITIVE_W: Self = Self(6);
    ///No documentation found
    pub const NEGATIVE_W: Self = Self(7);
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
///[VkPipelineViewportSwizzleStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef VkFlags VkPipelineViewportSwizzleStateCreateFlagsNV;
///```
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`PipelineViewportSwizzleStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(u32);
impl const Default for PipelineViewportSwizzleStateCreateFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineViewportSwizzleStateCreateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineViewportSwizzleStateCreateFlagsNV))
            .field(&self.0)
            .finish()
    }
}
///[VkViewportSwizzleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportSwizzleNV.html) - Structure specifying a viewport swizzle
///# C Specifications
///The [`ViewportSwizzleNV`] structure is defined as:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef struct VkViewportSwizzleNV {
///    VkViewportCoordinateSwizzleNV    x;
///    VkViewportCoordinateSwizzleNV    y;
///    VkViewportCoordinateSwizzleNV    z;
///    VkViewportCoordinateSwizzleNV    w;
///} VkViewportSwizzleNV;
///```
///# Members
/// - [`x`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the x component of the primitive
/// - [`y`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the y component of the primitive
/// - [`z`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the z component of the primitive
/// - [`w`] is a [`ViewportCoordinateSwizzleNV`] value specifying the swizzle operation to apply to
///   the w component of the primitive
///# Description
///## Valid Usage (Implicit)
/// - [`x`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`y`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`z`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
/// - [`w`] **must**  be a valid [`ViewportCoordinateSwizzleNV`] value
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`PipelineViewportSwizzleStateCreateInfoNV`]
/// - [`ViewportCoordinateSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViewportSwizzleNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ViewportSwizzleNV {
    ///[`x`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the x component of the primitive
    pub x: ViewportCoordinateSwizzleNV,
    ///[`y`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the y component of the primitive
    pub y: ViewportCoordinateSwizzleNV,
    ///[`z`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the z component of the primitive
    pub z: ViewportCoordinateSwizzleNV,
    ///[`w`] is a [`ViewportCoordinateSwizzleNV`] value specifying the
    ///swizzle operation to apply to the w component of the primitive
    pub w: ViewportCoordinateSwizzleNV,
}
impl Default for ViewportSwizzleNV {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            z: Default::default(),
            w: Default::default(),
        }
    }
}
impl ViewportSwizzleNV {
    ///Gets the value of [`Self::x`]
    pub fn x(&self) -> ViewportCoordinateSwizzleNV {
        self.x
    }
    ///Gets the value of [`Self::y`]
    pub fn y(&self) -> ViewportCoordinateSwizzleNV {
        self.y
    }
    ///Gets the value of [`Self::z`]
    pub fn z(&self) -> ViewportCoordinateSwizzleNV {
        self.z
    }
    ///Gets the value of [`Self::w`]
    pub fn w(&self) -> ViewportCoordinateSwizzleNV {
        self.w
    }
    ///Gets a mutable reference to the value of [`Self::x`]
    pub fn x_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.x
    }
    ///Gets a mutable reference to the value of [`Self::y`]
    pub fn y_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.y
    }
    ///Gets a mutable reference to the value of [`Self::z`]
    pub fn z_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.z
    }
    ///Gets a mutable reference to the value of [`Self::w`]
    pub fn w_mut(&mut self) -> &mut ViewportCoordinateSwizzleNV {
        &mut self.w
    }
    ///Sets the value of [`Self::x`]
    pub fn set_x(mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.x = value;
        self
    }
    ///Sets the value of [`Self::y`]
    pub fn set_y(mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.y = value;
        self
    }
    ///Sets the value of [`Self::z`]
    pub fn set_z(mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.z = value;
        self
    }
    ///Sets the value of [`Self::w`]
    pub fn set_w(mut self, value: crate::extensions::nv_viewport_swizzle::ViewportCoordinateSwizzleNV) -> Self {
        self.w = value;
        self
    }
}
///[VkPipelineViewportSwizzleStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportSwizzleStateCreateInfoNV.html) - Structure specifying swizzle applied to primitive clip coordinates
///# C Specifications
///Each primitive sent to a given viewport has a swizzle and  **optional**  negation
///applied to its clip coordinates.
///The swizzle that is applied depends on the viewport index, and is controlled
///by the [`PipelineViewportSwizzleStateCreateInfoNV`] pipeline state:
///```c
///// Provided by VK_NV_viewport_swizzle
///typedef struct VkPipelineViewportSwizzleStateCreateInfoNV {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkPipelineViewportSwizzleStateCreateFlagsNV    flags;
///    uint32_t                                       viewportCount;
///    const VkViewportSwizzleNV*                     pViewportSwizzles;
///} VkPipelineViewportSwizzleStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`viewport_count`] is the number of viewport swizzles used by the pipeline.
/// - [`viewport_swizzles`] is a pointer to an array of [`ViewportSwizzleNV`] structures, defining
///   the viewport swizzles.
///# Description
///## Valid Usage
/// - [`viewport_count`] **must**  be greater than or equal to the [`viewport_count`] set in
///   [`PipelineViewportStateCreateInfo`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV`
/// - [`flags`] **must**  be `0`
/// - [`viewport_swizzles`] **must**  be a valid pointer to an array of [`viewport_count`] valid
///   [`ViewportSwizzleNV`] structures
/// - [`viewport_count`] **must**  be greater than `0`
///# Related
/// - [`VK_NV_viewport_swizzle`]
/// - [`PipelineViewportSwizzleStateCreateFlagsNV`]
/// - [`StructureType`]
/// - [`ViewportSwizzleNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineViewportSwizzleStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    ///[`viewport_count`] is the number of viewport swizzles used by the
    ///pipeline.
    pub viewport_count: u32,
    ///[`viewport_swizzles`] is a pointer to an array of
    ///[`ViewportSwizzleNV`] structures, defining the viewport swizzles.
    pub viewport_swizzles: *const ViewportSwizzleNV,
}
impl<'lt> Default for PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            viewport_count: 0,
            viewport_swizzles: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportSwizzleStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::viewport_swizzles`]
    pub fn viewport_swizzles_raw(&self) -> *const ViewportSwizzleNV {
        self.viewport_swizzles
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_swizzles`]
    pub fn set_viewport_swizzles_raw(mut self, value: *const ViewportSwizzleNV) -> Self {
        self.viewport_swizzles = value;
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
    pub fn flags(&self) -> PipelineViewportSwizzleStateCreateFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::viewport_count`]
    pub fn viewport_count(&self) -> u32 {
        self.viewport_count
    }
    ///Gets the value of [`Self::viewport_swizzles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn viewport_swizzles(&self) -> &[ViewportSwizzleNV] {
        std::slice::from_raw_parts(self.viewport_swizzles, self.viewport_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineViewportSwizzleStateCreateFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::viewport_count`]
    pub fn viewport_count_mut(&mut self) -> &mut u32 {
        &mut self.viewport_count
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
        value: crate::extensions::nv_viewport_swizzle::PipelineViewportSwizzleStateCreateFlagsNV,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::viewport_count`]
    pub fn set_viewport_count(mut self, value: u32) -> Self {
        self.viewport_count = value;
        self
    }
    ///Sets the value of [`Self::viewport_swizzles`]
    pub fn set_viewport_swizzles(
        mut self,
        value: &'lt [crate::extensions::nv_viewport_swizzle::ViewportSwizzleNV],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.viewport_swizzles = value.as_ptr();
        self.viewport_count = len_;
        self
    }
}
