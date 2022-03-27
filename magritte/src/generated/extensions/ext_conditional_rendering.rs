use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Buffer, DeviceSize, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_conditional_rendering");
///[VkConditionalRenderingBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) - Structure specifying conditional rendering begin information
///# C Specifications
///The [`ConditionalRenderingBeginInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkConditionalRenderingBeginInfoEXT {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkBuffer                          buffer;
///    VkDeviceSize                      offset;
///    VkConditionalRenderingFlagsEXT    flags;
///} VkConditionalRenderingBeginInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is a buffer containing the predicate for conditional rendering.
/// - [`offset`] is the byte offset into [`buffer`] where the predicate is located.
/// - [`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`] specifying the behavior of
///   conditional rendering.
///# Description
///If the 32-bit value at [`offset`] in [`buffer`] memory is zero, then the
///rendering commands are discarded, otherwise they are executed as normal.
///If the value of the predicate in buffer memory changes while conditional
///rendering is active, the rendering commands **may** be discarded in an
///implementation-dependent way.
///Some implementations may latch the value of the predicate upon beginning
///conditional rendering while others may read it before every rendering
///command.Valid Usage
/// - If [`buffer`] is non-sparse then it **must** be bound completely and contiguously to a single
///   [`DeviceMemory`] object
/// - [`buffer`]**must** have been created with the `VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT`
///   bit set
/// - [`offset`]**must** be less than the size of [`buffer`] by at least 32 bits
/// - [`offset`]**must** be a multiple of 4
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`buffer`]**must** be a valid [`Buffer`] handle
/// - [`flags`]**must** be a valid combination of [`ConditionalRenderingFlagBitsEXT`] values
///# Related
/// - [`VK_EXT_conditional_rendering`]
/// - [`Buffer`]
/// - [`ConditionalRenderingFlagsEXT`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`CmdBeginConditionalRenderingEXT`]
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
pub struct ConditionalRenderingBeginInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is a buffer containing the predicate for conditional
    ///rendering.
    buffer: Buffer,
    ///[`offset`] is the byte offset into [`buffer`] where the predicate is
    ///located.
    offset: DeviceSize,
    ///[`flags`] is a bitmask of [`ConditionalRenderingFlagsEXT`]
    ///specifying the behavior of conditional rendering.
    flags: ConditionalRenderingFlagsEXT,
}
impl<'lt> Default for ConditionalRenderingBeginInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            buffer: Default::default(),
            offset: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> ConditionalRenderingBeginInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ConditionalRenderingFlagsEXT {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ConditionalRenderingFlagsEXT {
        &mut self.flags
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
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
    ///Sets the raw value of [`Self::offset`]
    pub fn set_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.offset = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) - Structure specifying command buffer inheritance information
///# C Specifications
///If the [`p_next`] chain of [`CommandBufferInheritanceInfo`] includes a
///[`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure, then
///that structure controls whether a command buffer **can** be executed while
///conditional rendering is [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering) in the
///primary command buffer.The [`CommandBufferInheritanceConditionalRenderingInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           conditionalRenderingEnable;
///} VkCommandBufferInheritanceConditionalRenderingInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering_enable`] specifies whether the command buffer **can** be executed
///   while conditional rendering is active in the primary command buffer. If this is [`TRUE`], then
///   this command buffer **can** be executed whether the primary command buffer has active
///   conditional rendering or not. If this is [`FALSE`], then the primary command buffer **must**
///   not have conditional rendering active.
///# Description
///If this structure is not present, the behavior is as if
///[`conditional_rendering_enable`] is [`FALSE`].Valid Usage
/// - If the [inherited conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-inheritedConditionalRendering)
///   feature is not enabled, [`conditional_rendering_enable`]**must** be [`FALSE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT`
///# Related
/// - [`VK_EXT_conditional_rendering`]
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
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`conditional_rendering_enable`] specifies whether the command buffer
    ///**can** be executed while conditional rendering is active in the primary
    ///command buffer.
    ///If this is [`TRUE`], then this command buffer **can** be executed
    ///whether the primary command buffer has active conditional rendering or
    ///not.
    ///If this is [`FALSE`], then the primary command buffer **must** not
    ///have conditional rendering active.
    conditional_rendering_enable: Bool32,
}
impl<'lt> Default for CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            conditional_rendering_enable: 0,
        }
    }
}
impl<'lt> CommandBufferInheritanceConditionalRenderingInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable_raw(&self) -> Bool32 {
        self.conditional_rendering_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::conditional_rendering_enable`]
    pub fn set_conditional_rendering_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.conditional_rendering_enable = value;
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
    ///Gets the value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.conditional_rendering_enable as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::conditional_rendering_enable`]
    pub fn conditional_rendering_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conditional_rendering_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conditional_rendering_enable as *mut Bool32)
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
    ///Sets the raw value of [`Self::conditional_rendering_enable`]
    pub fn set_conditional_rendering_enable(&mut self, value: bool) -> &mut Self {
        self.conditional_rendering_enable = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) - Structure describing if a secondary command buffer can be executed if conditional rendering is active in the primary command buffer
///# C Specifications
///The [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_conditional_rendering
///typedef struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           conditionalRendering;
///    VkBool32           inheritedConditionalRendering;
///} VkPhysicalDeviceConditionalRenderingFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conditional_rendering`] specifies whether conditional rendering is supported.
/// - [`inherited_conditional_rendering`] specifies whether a secondary command buffer **can** be
///   executed while conditional rendering is active in the primary command buffer.
///If the [`PhysicalDeviceConditionalRenderingFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceConditionalRenderingFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT`
///# Related
/// - [`VK_EXT_conditional_rendering`]
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
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`conditional_rendering`] specifies
    ///whether conditional rendering is supported.
    conditional_rendering: Bool32,
    ///[`inherited_conditional_rendering`] specifies whether a secondary
    ///command buffer **can** be executed while conditional rendering is active in
    ///the primary command buffer.
    inherited_conditional_rendering: Bool32,
}
impl<'lt> Default for PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            conditional_rendering: 0,
            inherited_conditional_rendering: 0,
        }
    }
}
impl<'lt> PhysicalDeviceConditionalRenderingFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::conditional_rendering`]
    pub fn conditional_rendering_raw(&self) -> Bool32 {
        self.conditional_rendering
    }
    ///Gets the raw value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering_raw(&self) -> Bool32 {
        self.inherited_conditional_rendering
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::conditional_rendering`]
    pub fn set_conditional_rendering_raw(&mut self, value: Bool32) -> &mut Self {
        self.conditional_rendering = value;
        self
    }
    ///Sets the raw value of [`Self::inherited_conditional_rendering`]
    pub fn set_inherited_conditional_rendering_raw(&mut self, value: Bool32) -> &mut Self {
        self.inherited_conditional_rendering = value;
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
    ///Gets the value of [`Self::conditional_rendering`]
    pub fn conditional_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.conditional_rendering as u8) }
    }
    ///Gets the value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering(&self) -> bool {
        unsafe { std::mem::transmute(self.inherited_conditional_rendering as u8) }
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
    ///Gets a mutable reference to the value of [`Self::conditional_rendering`]
    pub fn conditional_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::inherited_conditional_rendering`]
    pub fn inherited_conditional_rendering_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.inherited_conditional_rendering as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.inherited_conditional_rendering as *mut Bool32)
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
    ///Sets the raw value of [`Self::conditional_rendering`]
    pub fn set_conditional_rendering(&mut self, value: bool) -> &mut Self {
        self.conditional_rendering = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::inherited_conditional_rendering`]
    pub fn set_inherited_conditional_rendering(&mut self, value: bool) -> &mut Self {
        self.inherited_conditional_rendering = value as u8 as u32;
        self
    }
}
