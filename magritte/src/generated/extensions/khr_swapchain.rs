//![VK_KHR_swapchain](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html) - device extension
//!# Description
//!The [`VK_KHR_swapchain`] extension is the device-level companion to the
//!`[`VK_KHR_surface`]` extension.
//!It introduces [`SwapchainKHR`] objects, which provide the ability to
//!present rendering results to a surface.
//!# Revision
//!70
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_surface`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_swapchain]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_swapchain
//!   extension>>)
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_swapchain]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_swapchain
//!   extension>>)
//!# New handles
//! - [`SwapchainKHR`]
//!# New functions & commands
//! - [`AcquireNextImageKHR`]
//! - [`CreateSwapchainKHR`]
//! - [`DestroySwapchainKHR`]
//! - [`GetSwapchainImagesKHR`]
//! - [`QueuePresentKHR`]
//!If [Version 1.1]() is supported:
//! - [`AcquireNextImage2KHR`]
//! - [`GetDeviceGroupPresentCapabilitiesKHR`]
//! - [`GetDeviceGroupSurfacePresentModesKHR`]
//! - [`GetPhysicalDevicePresentRectanglesKHR`]
//!# New structures
//! - [`PresentInfoKHR`]
//! - [`SwapchainCreateInfoKHR`]
//!If [Version 1.1]() is supported:
//! - [`AcquireNextImageInfoKHR`]
//! - [`DeviceGroupPresentCapabilitiesKHR`]
//! - Extending [`BindImageMemoryInfo`]:
//! - [`BindImageMemorySwapchainInfoKHR`]
//!
//! - Extending [`ImageCreateInfo`]:
//! - [`ImageSwapchainCreateInfoKHR`]
//!
//! - Extending [`PresentInfoKHR`]:
//! - [`DeviceGroupPresentInfoKHR`]
//!
//! - Extending [`SwapchainCreateInfoKHR`]:
//! - [`DeviceGroupSwapchainCreateInfoKHR`]
//!# New enums
//! - [`SwapchainCreateFlagBitsKHR`]
//!If [Version 1.1]() is supported:
//! - [`DeviceGroupPresentModeFlagBitsKHR`]
//!# New bitmasks
//! - [`SwapchainCreateFlagsKHR`]
//!If [Version 1.1]() is supported:
//! - [`DeviceGroupPresentModeFlagsKHR`]
//!# New constants
//! - [`KHR_SWAPCHAIN_EXTENSION_NAME`]
//! - [`KHR_SWAPCHAIN_SPEC_VERSION`]
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
//!
//! - Extending [`ObjectType`]:
//! - `VK_OBJECT_TYPE_SWAPCHAIN_KHR`
//!
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_OUT_OF_DATE_KHR`
//! - `VK_SUBOPTIMAL_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
//!
//!If [Version 1.1]() is supported:
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
//!
//! - Extending [`SwapchainCreateFlagBitsKHR`]:
//! - `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`
//! - `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//!# Known issues & F.A.Q
//!1) Does this extension allow the application to specify the memory backing
//!of the presentable images?**RESOLVED**: No.
//!Unlike standard images, the implementation will allocate the memory backing
//!of the presentable image.2) What operations are allowed on presentable images?**RESOLVED**: This
//! is determined by the image usage flags specified when
//!creating the presentable image’s swapchain.3) Does this extension support MSAA presentable
//! images?**RESOLVED**: No.
//!Presentable images are always single-sampled.
//!Multi-sampled rendering must use regular images.
//!To present the rendering results the application must manually resolve the
//!multi- sampled image to a single-sampled presentable image prior to
//!presentation.4) Does this extension support stereo/multi-view presentable images?**RESOLVED**:
//! Yes.
//!The number of views associated with a presentable image is determined by the
//!`imageArrayLayers` specified when creating a swapchain.
//!All presentable images in a given swapchain use the same array size.5) Are the layers of stereo
//! presentable images half-sized?**RESOLVED**: No.
//!The image extents always match those requested by the application.6) Do the “present” and
//! “acquire next image” commands operate on a
//!queue? If not, do they need to include explicit semaphore objects to
//!interlock them with queue operations?**RESOLVED**: The present command operates on a queue.
//!The image ownership operation it represents happens in order with other
//!operations on the queue, so no explicit semaphore object is required to
//!synchronize its actions.Applications may want to acquire the next image in separate threads from
//!those in which they manage their queue, or in multiple threads.
//!To make such usage easier, the acquire next image command takes a semaphore
//!to signal as a method of explicit synchronization.
//!The application must later queue a wait for this semaphore before queuing
//!execution of any commands using the image.7) Does [`AcquireNextImageKHR`] block if no images are
//! available?**RESOLVED**: The command takes a timeout parameter.
//!Special values for the timeout are 0, which makes the call a non-blocking
//!operation, and `UINT64_MAX`, which blocks indefinitely.
//!Values in between will block for up to the specified time.
//!The call will return when an image becomes available or an error occurs.
//!It may, but is not required to, return before the specified timeout expires
//!if the swapchain becomes out of date.8) Can multiple presents be queued using one
//! [`QueuePresentKHR`] call?**RESOLVED**: Yes.
//![`PresentInfoKHR`] contains a list of swapchains and corresponding image
//!indices that will be presented.
//!When supported, all presentations queued with a single
//![`QueuePresentKHR`] call will be applied atomically as one operation.
//!The same swapchain must not appear in the list more than once.
//!Later extensions may provide applications stronger guarantees of atomicity
//!for such present operations, and/or allow them to query whether atomic
//!presentation of a particular group of swapchains is possible.9) How do the presentation and
//! acquire next image functions notify the
//!application the targeted surface has changed?**RESOLVED**: Two new result codes are introduced
//! for this purpose:
//! - `VK_SUBOPTIMAL_KHR` - Presentation will still succeed, subject to
//!the window resize behavior, but the swapchain is no longer configured
//!optimally for the surface it targets.
//!Applications should query updated surface information and recreate their
//!swapchain at the next convenient opportunity.
//! - `VK_ERROR_OUT_OF_DATE_KHR` - Failure.
//!The swapchain is no longer compatible with the surface it targets.
//!The application must query updated surface information and recreate the
//!swapchain before presentation will succeed.
//!These can be returned by both [`AcquireNextImageKHR`] and
//![`QueuePresentKHR`].10) Does the [`AcquireNextImageKHR`] command return a semaphore to the
//!application via an output parameter, or accept a semaphore to signal from
//!the application as an object handle parameter?**RESOLVED**: Accept a semaphore to signal as an
//! object handle.
//!This avoids the need to specify whether the application must destroy the
//!semaphore or whether it is owned by the swapchain, and if the latter, what
//!its lifetime is and whether it can be reused for other operations once it is
//!received from [`AcquireNextImageKHR`].11) What types of swapchain queuing behavior should be
//! exposed? Options
//!include swap interval specification, mailbox/most recent vs. FIFO queue
//!management, targeting specific vertical blank intervals or absolute times
//!for a given present operation, and probably others.
//!For some of these, whether they are specified at swapchain creation time or
//!as per-present parameters needs to be decided as well.**RESOLVED**: The base swapchain extension
//! will expose 3 possible behaviors
//!(of which, FIFO will always be supported):
//! - Immediate present: Does not wait for vertical blanking period to update
//!the current image, likely resulting in visible tearing.
//!No internal queue is used.
//!Present requests are applied immediately.
//! - Mailbox queue: Waits for the next vertical blanking period to update the
//!current image.
//!No tearing should be observed.
//!An internal single-entry queue is used to hold pending presentation
//!requests.
//!If the queue is full when a new presentation request is received, the
//!new request replaces the existing entry, and any images associated with
//!the prior entry become available for reuse by the application.
//! - FIFO queue: Waits for the next vertical blanking period to update the
//!current image.
//!No tearing should be observed.
//!An internal queue containing `numSwapchainImages` - 1 entries
//!is used to hold pending presentation requests.
//!New requests are appended to the end of the queue, and one request is
//!removed from the beginning of the queue and processed during each
//!vertical blanking period in which the queue is non-empty
//!Not all surfaces will support all of these modes, so the modes supported
//!will be returned using a surface information query.
//!All surfaces must support the FIFO queue mode.
//!Applications must choose one of these modes up front when creating a
//!swapchain.
//!Switching modes can be accomplished by recreating the swapchain.12) Can
//! `VK_PRESENT_MODE_MAILBOX_KHR` provide non-blocking guarantees
//!for [`AcquireNextImageKHR`]? If so, what is the proper criteria?**RESOLVED**: Yes.
//!The difficulty is not immediately obvious here.
//!Naively, if at least 3 images are requested, mailbox mode should always have
//!an image available for the application if the application does not own any
//!images when the call to [`AcquireNextImageKHR`] was made.
//!However, some presentation engines may have more than one “current” image,
//!and would still need to block in some cases.
//!The right requirement appears to be that if the application allocates the
//!surface’s minimum number of images + 1 then it is guaranteed non-blocking
//!behavior when it does not currently own any images.13) Is there a way to create and initialize a
//! new swapchain for a surface
//!that has generated a `VK_SUBOPTIMAL_KHR` return code while still using
//!the old swapchain?**RESOLVED**: Not as part of this specification.
//!This could be useful to allow the application to create an “optimal”
//!replacement swapchain and rebuild all its command buffers using it in a
//!background thread at a low priority while continuing to use the
//!“suboptimal” swapchain in the main thread.
//!It could probably use the same “atomic replace” semantics proposed for
//!recreating direct-to-device swapchains without incurring a mode switch.
//!However, after discussion, it was determined some platforms probably could
//!not support concurrent swapchains for the same surface though, so this will
//!be left out of the base KHR extensions.
//!A future extension could add this for platforms where it is supported.14) Should there be a
//! special value for
//![`SurfaceCapabilitiesKHR::max_image_count`] to indicate there are no
//!practical limits on the number of images in a swapchain?**RESOLVED**: Yes.
//!There will often be cases where there is no practical limit to the number of
//!images in a swapchain other than the amount of available resources (i.e.,
//!memory) in the system.
//!Trying to derive a hard limit from things like memory size is prone to
//!failure.
//!It is better in such cases to leave it to applications to figure such soft
//!limits out via trial/failure iterations.15) Should there be a special value for
//![`SurfaceCapabilitiesKHR::current_extent`] to indicate the size of
//!the platform surface is undefined?**RESOLVED**: Yes.
//!On some platforms (Wayland, for example), the surface size is defined by the
//!images presented to it rather than the other way around.16) Should there be a special value for
//![`SurfaceCapabilitiesKHR::max_image_extent`] to indicate there is no
//!practical limit on the surface size?**RESOLVED**: No.
//!It seems unlikely such a system would exist.
//!0 could be used to indicate the platform places no limits on the extents
//!beyond those imposed by Vulkan for normal images, but this query could just
//!as easily return those same limits, so a special “unlimited” value does
//!not seem useful for this field.17) How should surface rotation and mirroring be exposed to
//! applications?
//!How do they specify rotation and mirroring transforms applied prior to
//!presentation?**RESOLVED**: Applications can query both the supported and current transforms
//!of a surface.
//!Both are specified relative to the device’s “natural” display rotation and
//!direction.
//!The supported transforms indicate which orientations the presentation engine
//!accepts images in.
//!For example, a presentation engine that does not support transforming
//!surfaces as part of presentation, and which is presenting to a surface that
//!is displayed with a 90-degree rotation, would return only one supported
//!transform bit: `VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR`.
//!Applications must transform their rendering by the transform they specify
//!when creating the swapchain in `preTransform` field.18) Can surfaces ever not support
//! `VK_MIRROR_NONE`? Can they support
//!vertical and horizontal mirroring simultaneously? Relatedly, should
//!`VK_MIRROR_NONE`[_BIT] be zero, or bit one, and should applications be
//!allowed to specify multiple pre and current mirror transform bits, or
//!exactly one?**RESOLVED**: Since some platforms may not support presenting with a transform
//!other than the native window’s current transform, and prerotation/mirroring
//!are specified relative to the device’s natural rotation and direction,
//!rather than relative to the surface’s current rotation and direction, it is
//!necessary to express lack of support for no mirroring.
//!To allow this, the `MIRROR_NONE` enum must occupy a bit in the flags.
//!Since `MIRROR_NONE` must be a bit in the bitmask rather than a bitmask
//!with no values set, allowing more than one bit to be set in the bitmask
//!would make it possible to describe undefined transforms such as
//!`VK_MIRROR_NONE_BIT` | `VK_MIRROR_HORIZONTAL_BIT`, or a transform
//!that includes both “no mirroring” and “horizontal mirroring”
//!simultaneously.
//!Therefore, it is desirable to allow specifying all supported mirroring
//!transforms using only one bit.
//!The question then becomes, should there be a
//!`VK_MIRROR_HORIZONTAL_AND_VERTICAL_BIT` to represent a simultaneous
//!horizontal and vertical mirror transform? However, such a transform is
//!equivalent to a 180 degree rotation, so presentation engines and
//!applications that wish to support or use such a transform can express it
//!through rotation instead.
//!Therefore, 3 exclusive bits are sufficient to express all needed mirroring
//!transforms.19) Should support for sRGB be required?**RESOLVED**: In the advent of UHD and HDR
//! display devices, proper color space
//!information is vital to the display pipeline represented by the swapchain.
//!The app can discover the supported format/color-space pairs and select a
//!pair most suited to its rendering needs.
//!Currently only the sRGB color space is supported, future extensions may
//!provide support for more color spaces.
//!See issues 23 and 24.20) Is there a mechanism to modify or replace an existing swapchain with
//! one
//!targeting the same surface?**RESOLVED**: Yes.
//!This is described above in the text.21) Should there be a way to set prerotation and mirroring
//! using native APIs
//!when presenting using a Vulkan swapchain?**RESOLVED**: Yes.
//!The transforms that can be expressed in this extension are a subset of those
//!possible on native platforms.
//!If a platform exposes a method to specify the transform of presented images
//!for a given surface using native methods and exposes more transforms or
//!other properties for surfaces than Vulkan supports, it might be impossible,
//!difficult, or inconvenient to set some of those properties using Vulkan KHR
//!extensions and some using the native interfaces.
//!To avoid overwriting properties set using native commands when presenting
//!using a Vulkan swapchain, the application can set the pretransform to
//!“inherit”, in which case the current native properties will be used, or if
//!none are available, a platform-specific default will be used.
//!Platforms that do not specify a reasonable default or do not provide native
//!mechanisms to specify such transforms should not include the inherit bits in
//!the `supportedTransforms` bitmask they return in
//![`SurfaceCapabilitiesKHR`].22) Should the content of presentable images be clipped by objects
//! obscuring
//!their target surface?**RESOLVED**: Applications can choose which behavior they prefer.
//!Allowing the content to be clipped could enable more efficient presentation
//!methods on some platforms, but some applications might rely on the content
//!of presentable images to perform techniques such as partial updates or
//!motion blurs.23) What is the purpose of specifying a [`ColorSpaceKHR`] along with
//![`Format`] when creating a swapchain?**RESOLVED**: While Vulkan itself is color space agnostic
//! (e.g. even the
//!meaning of R, G, B and A can be freely defined by the rendering
//!application), the swapchain eventually will have to present the images on a
//!display device with specific color reproduction characteristics.
//!If any color space transformations are necessary before an image can be
//!displayed, the color space of the presented image must be known to the
//!swapchain.
//!A swapchain will only support a restricted set of color format and -space
//!pairs.
//!This set can be discovered via [`GetPhysicalDeviceSurfaceFormatsKHR`].
//!As it can be expected that most display devices support the sRGB color
//!space, at least one format/color-space pair has to be exposed, where the
//!color space is `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`.24) How are sRGB formats and the sRGB color
//! space related?**RESOLVED**: While Vulkan exposes a number of SRGB texture formats, using
//!such formats does not guarantee working in a specific color space.
//!It merely means that the hardware can directly support applying the
//!non-linear transfer functions defined by the sRGB standard color space when
//!reading from or writing to images of those formats.
//!Still, it is unlikely that a swapchain will expose a `*_SRGB` format
//!along with any color space other than
//!`VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`.On the other hand, non-`*_SRGB` formats will be very likely
//! exposed in
//!pair with a SRGB color space.
//!This means, the hardware will not apply any transfer function when reading
//!from or writing to such images, yet they will still be presented on a device
//!with sRGB display characteristics.
//!In this case the application is responsible for applying the transfer
//!function, for instance by using shader math.25) How are the lifetimes of surfaces and swapchains
//! targeting them related?**RESOLVED**: A surface must outlive any swapchains targeting it.
//!A [`SurfaceKHR`] owns the binding of the native window to the Vulkan
//!driver.26) How can the client control the way the alpha component of swapchain
//!images is treated by the presentation engine during compositing?**RESOLVED**: We should add new
//! enum values to allow the client to negotiate
//!with the presentation engine on how to treat image alpha values during the
//!compositing process.
//!Since not all platforms can practically control this through the Vulkan
//!driver, a value of `VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR` is provided like
//!for surface transforms.27) Is [`CreateSwapchainKHR`] the right function to return
//!`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`, or should the various
//!platform-specific [`SurfaceKHR`] factory functions catch this error
//!earlier?**RESOLVED**: For most platforms, the [`SurfaceKHR`] structure is a simple
//!container holding the data that identifies a native window or other object
//!representing a surface on a particular platform.
//!For the surface factory functions to return this error, they would likely
//!need to register a reference on the native objects with the native display
//!server somehow, and ensure no other such references exist.
//!Surfaces were not intended to be that heavyweight.Swapchains are intended to be the objects that
//! directly manipulate native
//!windows and communicate with the native presentation mechanisms.
//!Swapchains will already need to communicate with the native display server
//!to negotiate allocation and/or presentation of presentable images for a
//!native surface.
//!Therefore, it makes more sense for swapchain creation to be the point at
//!which native object exclusivity is enforced.
//!Platforms may choose to enforce further restrictions on the number of
//![`SurfaceKHR`] objects that may be created for the same native window if
//!such a requirement makes sense on a particular platform, but a global
//!requirement is only sensible at the swapchain level.
//!# Version History
//! - Revision 1, 2015-05-20 (James Jones)
//! - Initial draft, based on LunarG KHR spec, other KHR specs, patches
//!attached to bugs.
//!
//! - Revision 2, 2015-05-22 (Ian Elliott)
//! - Made many agreed-upon changes from 2015-05-21 KHR TSG meeting.
//!This includes using only a queue for presentation, and having an
//!explicit function to acquire the next image.
//! - Fixed typos and other minor mistakes.
//!
//! - Revision 3, 2015-05-26 (Ian Elliott)
//! - Improved the Description section.
//! - Added or resolved issues that were found in improving the Description.
//!For example, pSurfaceDescription is used consistently, instead of
//!sometimes using pSurface.
//!
//! - Revision 4, 2015-05-27 (James Jones)
//! - Fixed some grammatical errors and typos
//! - Filled in the description of imageUseFlags when creating a swapchain.
//! - Added a description of swapInterval.
//! - Replaced the paragraph describing the order of operations on a queue
//!for image ownership and presentation.
//!
//! - Revision 5, 2015-05-27 (James Jones)
//! - Imported relevant issues from the (abandoned)
//!vk_wsi_persistent_swapchain_images extension.
//! - Added issues 6 and 7, regarding behavior of the acquire next image and
//!present commands with respect to queues.
//! - Updated spec language and examples to align with proposed resolutions
//!to issues 6 and 7.
//!
//! - Revision 6, 2015-05-27 (James Jones)
//! - Added issue 8, regarding atomic presentation of multiple swapchains
//! - Updated spec language and examples to align with proposed resolution
//!to issue 8.
//!
//! - Revision 7, 2015-05-27 (James Jones)
//! - Fixed compilation errors in example code, and made related spec fixes.
//!
//! - Revision 8, 2015-05-27 (James Jones)
//! - Added issue 9, and the related VK_SUBOPTIMAL_KHR result code.
//! - Renamed VK_OUT_OF_DATE_KHR to VK_ERROR_OUT_OF_DATE_KHR.
//!
//! - Revision 9, 2015-05-27 (James Jones)
//! - Added inline proposed resolutions (marked with [JRJ]) to some XXX
//!questions/issues.
//!These should be moved to the issues section in a subsequent update if
//!the proposals are adopted.
//!
//! - Revision 10, 2015-05-28 (James Jones)
//! - Converted vkAcquireNextImageKHR back to a non-queue operation that
//!uses a VkSemaphore object for explicit synchronization.
//! - Added issue 10 to determine whether vkAcquireNextImageKHR generates or
//!returns semaphores, or whether it operates on a semaphore provided by
//!the application.
//!
//! - Revision 11, 2015-05-28 (James Jones)
//! - Marked issues 6, 7, and 8 resolved.
//! - Renamed VkSurfaceCapabilityPropertiesKHR to VkSurfacePropertiesKHR to
//!better convey the mutable nature of the information it contains.
//!
//! - Revision 12, 2015-05-28 (James Jones)
//! - Added issue 11 with a proposed resolution, and the related issue 12.
//! - Updated various sections of the spec to match the proposed resolution
//!to issue 11.
//!
//! - Revision 13, 2015-06-01 (James Jones)
//! - Moved some structures to VK_EXT_KHR_swap_chain to resolve the
//!specification’s issues 1 and 2.
//!
//! - Revision 14, 2015-06-01 (James Jones)
//! - Added code for example 4 demonstrating how an application might make
//!use of the two different present and acquire next image KHR result
//!codes.
//! - Added issue 13.
//!
//! - Revision 15, 2015-06-01 (James Jones)
//! - Added issues 14 - 16 and related spec language.
//! - Fixed some spelling errors.
//! - Added language describing the meaningful return values for
//!vkAcquireNextImageKHR and vkQueuePresentKHR.
//!
//! - Revision 16, 2015-06-02 (James Jones)
//! - Added issues 17 and 18, as well as related spec language.
//! - Removed some erroneous text added by mistake in the last update.
//!
//! - Revision 17, 2015-06-15 (Ian Elliott)
//! - Changed special value from "-1" to "0" so that the data types can be
//!unsigned.
//!
//! - Revision 18, 2015-06-15 (Ian Elliott)
//! - Clarified the values of VkSurfacePropertiesKHR::minImageCount and the
//!timeout parameter of the vkAcquireNextImageKHR function.
//!
//! - Revision 19, 2015-06-17 (James Jones)
//! - Misc.
//!cleanup.
//!Removed resolved inline issues and fixed typos.
//! - Fixed clarification of VkSurfacePropertiesKHR::minImageCount made in
//!version 18.
//! - Added a brief "Image Ownership" definition to the list of terms used
//!in the spec.
//!
//! - Revision 20, 2015-06-17 (James Jones)
//! - Updated enum-extending values using new convention.
//!
//! - Revision 21, 2015-06-17 (James Jones)
//! - Added language describing how to use
//!VK_IMAGE_LAYOUT_PRESENT_SOURCE_KHR.
//! - Cleaned up an XXX comment regarding the description of which queues
//!vkQueuePresentKHR can be used on.
//!
//! - Revision 22, 2015-06-17 (James Jones)
//! - Rebased on Vulkan API version 126.
//!
//! - Revision 23, 2015-06-18 (James Jones)
//! - Updated language for issue 12 to read as a proposed resolution.
//! - Marked issues 11, 12, 13, 16, and 17 resolved.
//! - Temporarily added links to the relevant bugs under the remaining
//!unresolved issues.
//! - Added issues 19 and 20 as well as proposed resolutions.
//!
//! - Revision 24, 2015-06-19 (Ian Elliott)
//! - Changed special value for VkSurfacePropertiesKHR::currentExtent back
//!to “-1” from “0”.
//!This value will never need to be unsigned, and “0” is actually a
//!legal value.
//!
//! - Revision 25, 2015-06-23 (Ian Elliott)
//! - Examples now show use of function pointers for extension functions.
//! - Eliminated extraneous whitespace.
//!
//! - Revision 26, 2015-06-25 (Ian Elliott)
//! - Resolved Issues 9 & 10 per KHR TSG meeting.
//!
//! - Revision 27, 2015-06-25 (James Jones)
//! - Added oldSwapchain member to VkSwapchainCreateInfoKHR.
//!
//! - Revision 28, 2015-06-25 (James Jones)
//! - Added the “inherit” bits to the rotation and mirroring flags and the
//!associated issue 21.
//!
//! - Revision 29, 2015-06-25 (James Jones)
//! - Added the “clipped” flag to VkSwapchainCreateInfoKHR, and the
//!associated issue 22.
//! - Specified that presenting an image does not modify it.
//!
//! - Revision 30, 2015-06-25 (James Jones)
//! - Added language to the spec that clarifies the behavior of
//!vkCreateSwapchainKHR() when the oldSwapchain field of
//!VkSwapchainCreateInfoKHR is not NULL.
//!
//! - Revision 31, 2015-06-26 (Ian Elliott)
//! - Example of new VkSwapchainCreateInfoKHR members, “oldSwapchain” and
//!“clipped”.
//! - Example of using VkSurfacePropertiesKHR::{min|max}ImageCount to set
//!VkSwapchainCreateInfoKHR::minImageCount.
//! - Rename vkGetSurfaceInfoKHR()'s 4th parameter to “pDataSize”, for
//!consistency with other functions.
//! - Add macro with C-string name of extension (just to header file).
//!
//! - Revision 32, 2015-06-26 (James Jones)
//! - Minor adjustments to the language describing the behavior of
//!“oldSwapchain”
//! - Fixed the version date on my previous two updates.
//!
//! - Revision 33, 2015-06-26 (Jesse Hall)
//! - Add usage flags to VkSwapchainCreateInfoKHR
//!
//! - Revision 34, 2015-06-26 (Ian Elliott)
//! - Rename vkQueuePresentKHR()'s 2nd parameter to “pPresentInfo”, for
//!consistency with other functions.
//!
//! - Revision 35, 2015-06-26 (Jason Ekstrand)
//! - Merged the VkRotationFlagBitsKHR and VkMirrorFlagBitsKHR enums into a
//!single VkSurfaceTransformFlagBitsKHR enum.
//!
//! - Revision 36, 2015-06-26 (Jason Ekstrand)
//! - Added a VkSurfaceTransformKHR enum that is not a bitmask.
//!Each value in VkSurfaceTransformKHR corresponds directly to one of the
//!bits in VkSurfaceTransformFlagBitsKHR so transforming from one to the
//!other is easy.
//!Having a separate enum means that currentTransform and preTransform
//!are now unambiguous by definition.
//!
//! - Revision 37, 2015-06-29 (Ian Elliott)
//! - Corrected one of the signatures of vkAcquireNextImageKHR, which had
//!the last two parameters switched from what it is elsewhere in the
//!specification and header files.
//!
//! - Revision 38, 2015-06-30 (Ian Elliott)
//! - Corrected a typo in description of the vkGetSwapchainInfoKHR()
//!function.
//! - Corrected a typo in header file comment for VkPresentInfoKHR::sType.
//!
//! - Revision 39, 2015-07-07 (Daniel Rakos)
//! - Added error section describing when each error is expected to be
//!reported.
//! - Replaced bool32_t with VkBool32.
//!
//! - Revision 40, 2015-07-10 (Ian Elliott)
//! - Updated to work with version 138 of the `vulkan.h` header.
//!This includes declaring the VkSwapchainKHR type using the new
//!VK_DEFINE_NONDISP_HANDLE macro, and no longer extending VkObjectType
//!(which was eliminated).
//!
//! - Revision 41 2015-07-09 (Mathias Heyer)
//! - Added color space language.
//!
//! - Revision 42, 2015-07-10 (Daniel Rakos)
//! - Updated query mechanism to reflect the convention changes done in the
//!core spec.
//! - Removed “queue” from the name of
//!VK_STRUCTURE_TYPE_QUEUE_PRESENT_INFO_KHR to be consistent with the
//!established naming convention.
//! - Removed reference to the no longer existing VkObjectType enum.
//!
//! - Revision 43, 2015-07-17 (Daniel Rakos)
//! - Added support for concurrent sharing of swapchain images across queue
//!families.
//! - Updated sample code based on recent changes
//!
//! - Revision 44, 2015-07-27 (Ian Elliott)
//! - Noted that support for VK_PRESENT_MODE_FIFO_KHR is required.
//!That is ICDs may optionally support IMMEDIATE and MAILBOX, but must
//!support FIFO.
//!
//! - Revision 45, 2015-08-07 (Ian Elliott)
//! - Corrected a typo in spec file (type and variable name had wrong case
//!for the imageColorSpace member of the VkSwapchainCreateInfoKHR
//!struct).
//! - Corrected a typo in header file (last parameter in
//!PFN_vkGetSurfacePropertiesKHR was missing “KHR” at the end of type:
//!VkSurfacePropertiesKHR).
//!
//! - Revision 46, 2015-08-20 (Ian Elliott)
//! - Renamed this extension and all of its enumerations, types, functions,
//!etc.
//!This makes it compliant with the proposed standard for Vulkan
//!extensions.
//! - Switched from “revision” to “version”, including use of the
//!VK_MAKE_VERSION macro in the header file.
//! - Made improvements to several descriptions.
//! - Changed the status of several issues from PROPOSED to RESOLVED,
//!leaving no unresolved issues.
//! - Resolved several TODOs, did miscellaneous cleanup, etc.
//!
//! - Revision 47, 2015-08-20 (Ian Elliott—​porting a 2015-07-29 change from
//!James Jones)
//! - Moved the surface transform enums to VK_WSI_swapchain so they could be
//!reused by VK_WSI_display.
//!
//! - Revision 48, 2015-09-01 (James Jones)
//! - Various minor cleanups.
//!
//! - Revision 49, 2015-09-01 (James Jones)
//! - Restore single-field revision number.
//!
//! - Revision 50, 2015-09-01 (James Jones)
//! - Update Example #4 to include code that illustrates how to use the
//!oldSwapchain field.
//!
//! - Revision 51, 2015-09-01 (James Jones)
//! - Fix example code compilation errors.
//!
//! - Revision 52, 2015-09-08 (Matthaeus G. Chajdas)
//! - Corrected a typo.
//!
//! - Revision 53, 2015-09-10 (Alon Or-bach)
//! - Removed underscore from SWAP_CHAIN left in
//!VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR.
//!
//! - Revision 54, 2015-09-11 (Jesse Hall)
//! - Described the execution and memory coherence requirements for image
//!transitions to and from VK_IMAGE_LAYOUT_PRESENT_SOURCE_KHR.
//!
//! - Revision 55, 2015-09-11 (Ray Smith)
//! - Added errors for destroying and binding memory to presentable images
//!
//! - Revision 56, 2015-09-18 (James Jones)
//! - Added fence argument to vkAcquireNextImageKHR
//! - Added example of how to meter a host thread based on presentation
//!rate.
//!
//! - Revision 57, 2015-09-26 (Jesse Hall)
//! - Replace VkSurfaceDescriptionKHR with VkSurfaceKHR.
//! - Added issue 25 with agreed resolution.
//!
//! - Revision 58, 2015-09-28 (Jesse Hall)
//! - Renamed from VK_EXT_KHR_device_swapchain to VK_EXT_KHR_swapchain.
//!
//! - Revision 59, 2015-09-29 (Ian Elliott)
//! - Changed vkDestroySwapchainKHR() to return void.
//!
//! - Revision 60, 2015-10-01 (Jeff Vigil)
//! - Added error result VK_ERROR_SURFACE_LOST_KHR.
//!
//! - Revision 61, 2015-10-05 (Jason Ekstrand)
//! - Added the VkCompositeAlpha enum and corresponding structure fields.
//!
//! - Revision 62, 2015-10-12 (Daniel Rakos)
//! - Added VK_PRESENT_MODE_FIFO_RELAXED_KHR.
//!
//! - Revision 63, 2015-10-15 (Daniel Rakos)
//! - Moved surface capability queries to VK_EXT_KHR_surface.
//!
//! - Revision 64, 2015-10-26 (Ian Elliott)
//! - Renamed from VK_EXT_KHR_swapchain to VK_KHR_swapchain.
//!
//! - Revision 65, 2015-10-28 (Ian Elliott)
//! - Added optional pResult member to VkPresentInfoKHR, so that
//!per-swapchain results can be obtained from vkQueuePresentKHR().
//!
//! - Revision 66, 2015-11-03 (Daniel Rakos)
//! - Added allocation callbacks to create and destroy functions.
//! - Updated resource transition language.
//! - Updated sample code.
//!
//! - Revision 67, 2015-11-10 (Jesse Hall)
//! - Add reserved flags bitmask to VkSwapchainCreateInfoKHR.
//! - Modify naming and member ordering to match API style conventions, and
//!so the VkSwapchainCreateInfoKHR image property members mirror
//!corresponding VkImageCreateInfo members but with an 'image' prefix.
//! - Make VkPresentInfoKHR::pResults non-const; it is an output array
//!parameter.
//! - Make pPresentInfo parameter to vkQueuePresentKHR const.
//!
//! - Revision 68, 2016-04-05 (Ian Elliott)
//! - Moved the “validity” include for vkAcquireNextImage to be in its
//!proper place, after the prototype and list of parameters.
//! - Clarified language about presentable images, including how they are
//!acquired, when applications can and cannot use them, etc.
//!As part of this, removed language about “ownership” of presentable
//!images, and replaced it with more-consistent language about
//!presentable images being “acquired” by the application.
//!
//! - 2016-08-23 (Ian Elliott)
//! - Update the example code, to use the final API command names, to not
//!have so many characters per line, and to split out a new example to
//!show how to obtain function pointers.
//!This code is more similar to the LunarG “cube” demo program.
//!
//! - 2016-08-25 (Ian Elliott)
//! - A note was added at the beginning of the example code, stating that it
//!will be removed from future versions of the appendix.
//!
//! - Revision 69, 2017-09-07 (Tobias Hector)
//! - Added interactions with Vulkan 1.1
//!
//! - Revision 70, 2017-10-06 (Ian Elliott)
//! - Corrected interactions with Vulkan 1.1
//!# Other info
//! * 2017-10-06
//! * No known IP claims.
//!*
//! - Interacts with Vulkan 1.1
//!
//!*
//! - Patrick Doane, Blizzard
//! - Ian Elliott, LunarG
//! - Jesse Hall, Google
//! - Mathias Heyer, NVIDIA
//! - James Jones, NVIDIA
//! - David Mao, AMD
//! - Norbert Nopper, Freescale
//! - Alon Or-bach, Samsung
//! - Daniel Rakos, AMD
//! - Graham Sellers, AMD
//! - Jeff Vigil, Qualcomm
//! - Chia-I Wu, LunarG
//! - Jason Ekstrand, Intel
//! - Matthaeus G. Chajdas, AMD
//! - Ray Smith, ARM
//!# Related
//! - [`PresentInfoKHR`]
//! - [`SwapchainCreateFlagBitsKHR`]
//! - [`SwapchainCreateFlagsKHR`]
//! - [`SwapchainCreateInfoKHR`]
//! - [`SwapchainKHR`]
//! - [`AcquireNextImageKHR`]
//! - [`CreateSwapchainKHR`]
//! - [`DestroySwapchainKHR`]
//! - [`GetSwapchainImagesKHR`]
//! - [`QueuePresentKHR`]
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
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_swapchain");
