//![VK_KHR_swapchain](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html) - device extension
//!# Description
//!The [`VK_KHR_swapchain`] extension is the device-level companion to the
//!`[`khr_surface`]` extension.
//!It introduces [`SwapchainKHR`] objects, which provide the ability to
//!present rendering results to a surface.
//!# Revision
//!70
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_surface`]`
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
//! - [`acquire_next_image_khr`]
//! - [`create_swapchain_khr`]
//! - [`destroy_swapchain_khr`]
//! - [`get_swapchain_images_khr`]
//! - [`queue_present_khr`]
//!If [Version 1.1]() is supported:
//! - [`acquire_next_image2_khr`]
//! - [`get_device_group_present_capabilities_khr`]
//! - [`get_device_group_surface_present_modes_khr`]
//! - [`get_physical_device_present_rectangles_khr`]
//!# New structures
//! - [`PresentInfoKHR`]
//! - [`SwapchainCreateInfoKHR`]
//!If [Version 1.1]() is supported:
//! - [`AcquireNextImageInfoKHR`]
//! - [`DeviceGroupPresentCapabilitiesKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImageMemorySwapchainInfoKHR`]
//! - Extending [`ImageCreateInfo`]:  - [`ImageSwapchainCreateInfoKHR`]
//! - Extending [`PresentInfoKHR`]:  - [`DeviceGroupPresentInfoKHR`]
//! - Extending [`SwapchainCreateInfoKHR`]:  - [`DeviceGroupSwapchainCreateInfoKHR`]
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
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_SWAPCHAIN_KHR`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_SUBOPTIMAL_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
//!If [Version 1.1]() is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
//! - Extending [`SwapchainCreateFlagBitsKHR`]:  - `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`  -
//!   `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//!# Known issues & F.A.Q
//!1) Does this extension allow the application to specify the memory backing
//!of the presentable images? **RESOLVED** : No.
//!Unlike standard images, the implementation will allocate the memory backing
//!of the presentable image.2) What operations are allowed on presentable images? **RESOLVED** :
//! This is determined by the image usage flags specified when
//!creating the presentable image’s swapchain.3) Does this extension support MSAA presentable
//! images? **RESOLVED** : No.
//!Presentable images are always single-sampled.
//!Multi-sampled rendering must use regular images.
//!To present the rendering results the application must manually resolve the
//!multi- sampled image to a single-sampled presentable image prior to
//!presentation.4) Does this extension support stereo/multi-view presentable images? **RESOLVED** :
//! Yes.
//!The number of views associated with a presentable image is determined by the
//!`imageArrayLayers` specified when creating a swapchain.
//!All presentable images in a given swapchain use the same array size.5) Are the layers of stereo
//! presentable images half-sized? **RESOLVED** : No.
//!The image extents always match those requested by the application.6) Do the “present” and
//! “acquire next image” commands operate on a
//!queue? If not, do they need to include explicit semaphore objects to
//!interlock them with queue operations? **RESOLVED** : The present command operates on a queue.
//!The image ownership operation it represents happens in order with other
//!operations on the queue, so no explicit semaphore object is required to
//!synchronize its actions.Applications may want to acquire the next image in separate threads from
//!those in which they manage their queue, or in multiple threads.
//!To make such usage easier, the acquire next image command takes a semaphore
//!to signal as a method of explicit synchronization.
//!The application must later queue a wait for this semaphore before queuing
//!execution of any commands using the image.7) Does [`acquire_next_image_khr`] block if no images
//! are available? **RESOLVED** : The command takes a timeout parameter.
//!Special values for the timeout are 0, which makes the call a non-blocking
//!operation, and `UINT64_MAX`, which blocks indefinitely.
//!Values in between will block for up to the specified time.
//!The call will return when an image becomes available or an error occurs.
//!It may, but is not required to, return before the specified timeout expires
//!if the swapchain becomes out of date.8) Can multiple presents be queued using one
//! [`queue_present_khr`] call? **RESOLVED** : Yes.
//![`PresentInfoKHR`] contains a list of swapchains and corresponding image
//!indices that will be presented.
//!When supported, all presentations queued with a single
//![`queue_present_khr`] call will be applied atomically as one operation.
//!The same swapchain must not appear in the list more than once.
//!Later extensions may provide applications stronger guarantees of atomicity
//!for such present operations, and/or allow them to query whether atomic
//!presentation of a particular group of swapchains is possible.9) How do the presentation and
//! acquire next image functions notify the
//!application the targeted surface has changed? **RESOLVED** : Two new result codes are introduced
//! for this purpose:
//! - `VK_SUBOPTIMAL_KHR` - Presentation will still succeed, subject to the window resize behavior,
//!   but the swapchain is no longer configured optimally for the surface it targets. Applications
//!   should query updated surface information and recreate their swapchain at the next convenient
//!   opportunity.
//! - `VK_ERROR_OUT_OF_DATE_KHR` - Failure. The swapchain is no longer compatible with the surface
//!   it targets. The application must query updated surface information and recreate the swapchain
//!   before presentation will succeed.
//!These can be returned by both [`acquire_next_image_khr`] and
//![`queue_present_khr`].10) Does the [`acquire_next_image_khr`] command return a semaphore to the
//!application via an output parameter, or accept a semaphore to signal from
//!the application as an object handle parameter? **RESOLVED** : Accept a semaphore to signal as an
//! object handle.
//!This avoids the need to specify whether the application must destroy the
//!semaphore or whether it is owned by the swapchain, and if the latter, what
//!its lifetime is and whether it can be reused for other operations once it is
//!received from [`acquire_next_image_khr`].11) What types of swapchain queuing behavior should be
//! exposed? Options
//!include swap interval specification, mailbox/most recent vs. FIFO queue
//!management, targeting specific vertical blank intervals or absolute times
//!for a given present operation, and probably others.
//!For some of these, whether they are specified at swapchain creation time or
//!as per-present parameters needs to be decided as well. **RESOLVED** : The base swapchain
//! extension will expose 3 possible behaviors
//!(of which, FIFO will always be supported):
//! - Immediate present: Does not wait for vertical blanking period to update the current image,
//!   likely resulting in visible tearing. No internal queue is used. Present requests are applied
//!   immediately.
//! - Mailbox queue: Waits for the next vertical blanking period to update the current image. No
//!   tearing should be observed. An internal single-entry queue is used to hold pending
//!   presentation requests. If the queue is full when a new presentation request is received, the
//!   new request replaces the existing entry, and any images associated with the prior entry become
//!   available for reuse by the application.
//! - FIFO queue: Waits for the next vertical blanking period to update the current image. No
//!   tearing should be observed. An internal queue containing `numSwapchainImages` - 1 entries is
//!   used to hold pending presentation requests. New requests are appended to the end of the queue,
//!   and one request is removed from the beginning of the queue and processed during each vertical
//!   blanking period in which the queue is non-empty
//!Not all surfaces will support all of these modes, so the modes supported
//!will be returned using a surface information query.
//!All surfaces must support the FIFO queue mode.
//!Applications must choose one of these modes up front when creating a
//!swapchain.
//!Switching modes can be accomplished by recreating the swapchain.12) Can
//! `VK_PRESENT_MODE_MAILBOX_KHR` provide non-blocking guarantees
//!for [`acquire_next_image_khr`]? If so, what is the proper criteria? **RESOLVED** : Yes.
//!The difficulty is not immediately obvious here.
//!Naively, if at least 3 images are requested, mailbox mode should always have
//!an image available for the application if the application does not own any
//!images when the call to [`acquire_next_image_khr`] was made.
//!However, some presentation engines may have more than one “current” image,
//!and would still need to block in some cases.
//!The right requirement appears to be that if the application allocates the
//!surface’s minimum number of images + 1 then it is guaranteed non-blocking
//!behavior when it does not currently own any images.13) Is there a way to create and initialize a
//! new swapchain for a surface
//!that has generated a `VK_SUBOPTIMAL_KHR` return code while still using
//!the old swapchain? **RESOLVED** : Not as part of this specification.
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
//!practical limits on the number of images in a swapchain? **RESOLVED** : Yes.
//!There will often be cases where there is no practical limit to the number of
//!images in a swapchain other than the amount of available resources (i.e.,
//!memory) in the system.
//!Trying to derive a hard limit from things like memory size is prone to
//!failure.
//!It is better in such cases to leave it to applications to figure such soft
//!limits out via trial/failure iterations.15) Should there be a special value for
//![`SurfaceCapabilitiesKHR::current_extent`] to indicate the size of
//!the platform surface is undefined? **RESOLVED** : Yes.
//!On some platforms (Wayland, for example), the surface size is defined by the
//!images presented to it rather than the other way around.16) Should there be a special value for
//![`SurfaceCapabilitiesKHR::max_image_extent`] to indicate there is no
//!practical limit on the surface size? **RESOLVED** : No.
//!It seems unlikely such a system would exist.
//!0 could be used to indicate the platform places no limits on the extents
//!beyond those imposed by Vulkan for normal images, but this query could just
//!as easily return those same limits, so a special “unlimited” value does
//!not seem useful for this field.17) How should surface rotation and mirroring be exposed to
//! applications?
//!How do they specify rotation and mirroring transforms applied prior to
//!presentation? **RESOLVED** : Applications can query both the supported and current transforms
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
//!exactly one? **RESOLVED** : Since some platforms may not support presenting with a transform
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
//!transforms.19) Should support for sRGB be required? **RESOLVED** : In the advent of UHD and HDR
//! display devices, proper color space
//!information is vital to the display pipeline represented by the swapchain.
//!The app can discover the supported format/color-space pairs and select a
//!pair most suited to its rendering needs.
//!Currently only the sRGB color space is supported, future extensions may
//!provide support for more color spaces.
//!See issues 23 and 24.20) Is there a mechanism to modify or replace an existing swapchain with
//! one
//!targeting the same surface? **RESOLVED** : Yes.
//!This is described above in the text.21) Should there be a way to set prerotation and mirroring
//! using native APIs
//!when presenting using a Vulkan swapchain? **RESOLVED** : Yes.
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
//!their target surface? **RESOLVED** : Applications can choose which behavior they prefer.
//!Allowing the content to be clipped could enable more efficient presentation
//!methods on some platforms, but some applications might rely on the content
//!of presentable images to perform techniques such as partial updates or
//!motion blurs.23) What is the purpose of specifying a [`ColorSpaceKHR`] along with
//![`Format`] when creating a swapchain? **RESOLVED** : While Vulkan itself is color space agnostic
//! (e.g. even the
//!meaning of R, G, B and A can be freely defined by the rendering
//!application), the swapchain eventually will have to present the images on a
//!display device with specific color reproduction characteristics.
//!If any color space transformations are necessary before an image can be
//!displayed, the color space of the presented image must be known to the
//!swapchain.
//!A swapchain will only support a restricted set of color format and -space
//!pairs.
//!This set can be discovered via [`get_physical_device_surface_formats_khr`].
//!As it can be expected that most display devices support the sRGB color
//!space, at least one format/color-space pair has to be exposed, where the
//!color space is `VK_COLOR_SPACE_SRGB_NONLINEAR_KHR`.24) How are sRGB formats and the sRGB color
//! space related? **RESOLVED** : While Vulkan exposes a number of SRGB texture formats, using
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
//! targeting them related? **RESOLVED** : A surface must outlive any swapchains targeting it.
//!A [`SurfaceKHR`] owns the binding of the native window to the Vulkan
//!driver.26) How can the client control the way the alpha component of swapchain
//!images is treated by the presentation engine during compositing? **RESOLVED** : We should add
//! new enum values to allow the client to negotiate
//!with the presentation engine on how to treat image alpha values during the
//!compositing process.
//!Since not all platforms can practically control this through the Vulkan
//!driver, a value of `VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR` is provided like
//!for surface transforms.27) Is [`create_swapchain_khr`] the right function to return
//!`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`, or should the various
//!platform-specific [`SurfaceKHR`] factory functions catch this error
//!earlier? **RESOLVED** : For most platforms, the [`SurfaceKHR`] structure is a simple
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
//! - Revision 1, 2015-05-20 (James Jones)  - Initial draft, based on LunarG KHR spec, other KHR
//!   specs, patches attached to bugs.
//! - Revision 2, 2015-05-22 (Ian Elliott)  - Made many agreed-upon changes from 2015-05-21 KHR TSG
//!   meeting. This includes using only a queue for presentation, and having an explicit function to
//!   acquire the next image.  - Fixed typos and other minor mistakes.
//! - Revision 3, 2015-05-26 (Ian Elliott)  - Improved the Description section.  - Added or resolved
//!   issues that were found in improving the Description. For example, pSurfaceDescription is used
//!   consistently, instead of sometimes using pSurface.
//! - Revision 4, 2015-05-27 (James Jones)  - Fixed some grammatical errors and typos  - Filled in
//!   the description of imageUseFlags when creating a swapchain.  - Added a description of
//!   swapInterval.  - Replaced the paragraph describing the order of operations on a queue for
//!   image ownership and presentation.
//! - Revision 5, 2015-05-27 (James Jones)  - Imported relevant issues from the (abandoned)
//!   vk_wsi_persistent_swapchain_images extension.  - Added issues 6 and 7, regarding behavior of
//!   the acquire next image and present commands with respect to queues.  - Updated spec language
//!   and examples to align with proposed resolutions to issues 6 and 7.
//! - Revision 6, 2015-05-27 (James Jones)  - Added issue 8, regarding atomic presentation of
//!   multiple swapchains  - Updated spec language and examples to align with proposed resolution to
//!   issue 8.
//! - Revision 7, 2015-05-27 (James Jones)  - Fixed compilation errors in example code, and made
//!   related spec fixes.
//! - Revision 8, 2015-05-27 (James Jones)  - Added issue 9, and the related VK_SUBOPTIMAL_KHR
//!   result code.  - Renamed VK_OUT_OF_DATE_KHR to VK_ERROR_OUT_OF_DATE_KHR.
//! - Revision 9, 2015-05-27 (James Jones)  - Added inline proposed resolutions (marked with [JRJ])
//!   to some XXX questions/issues. These should be moved to the issues section in a subsequent
//!   update if the proposals are adopted.
//! - Revision 10, 2015-05-28 (James Jones)  - Converted vkAcquireNextImageKHR back to a non-queue
//!   operation that uses a VkSemaphore object for explicit synchronization.  - Added issue 10 to
//!   determine whether vkAcquireNextImageKHR generates or returns semaphores, or whether it
//!   operates on a semaphore provided by the application.
//! - Revision 11, 2015-05-28 (James Jones)  - Marked issues 6, 7, and 8 resolved.  - Renamed
//!   VkSurfaceCapabilityPropertiesKHR to VkSurfacePropertiesKHR to better convey the mutable nature
//!   of the information it contains.
//! - Revision 12, 2015-05-28 (James Jones)  - Added issue 11 with a proposed resolution, and the
//!   related issue 12.  - Updated various sections of the spec to match the proposed resolution to
//!   issue 11.
//! - Revision 13, 2015-06-01 (James Jones)  - Moved some structures to VK_EXT_KHR_swap_chain to
//!   resolve the specification’s issues 1 and 2.
//! - Revision 14, 2015-06-01 (James Jones)  - Added code for example 4 demonstrating how an
//!   application might make use of the two different present and acquire next image KHR result
//!   codes.  - Added issue 13.
//! - Revision 15, 2015-06-01 (James Jones)  - Added issues 14 - 16 and related spec language.  -
//!   Fixed some spelling errors.  - Added language describing the meaningful return values for
//!   vkAcquireNextImageKHR and vkQueuePresentKHR.
//! - Revision 16, 2015-06-02 (James Jones)  - Added issues 17 and 18, as well as related spec
//!   language.  - Removed some erroneous text added by mistake in the last update.
//! - Revision 17, 2015-06-15 (Ian Elliott)  - Changed special value from "-1" to "0" so that the
//!   data types can be unsigned.
//! - Revision 18, 2015-06-15 (Ian Elliott)  - Clarified the values of
//!   VkSurfacePropertiesKHR::minImageCount and the timeout parameter of the vkAcquireNextImageKHR
//!   function.
//! - Revision 19, 2015-06-17 (James Jones)  - Misc. cleanup. Removed resolved inline issues and
//!   fixed typos.  - Fixed clarification of VkSurfacePropertiesKHR::minImageCount made in version
//!   18.  - Added a brief "Image Ownership" definition to the list of terms used in the spec.
//! - Revision 20, 2015-06-17 (James Jones)  - Updated enum-extending values using new convention.
//! - Revision 21, 2015-06-17 (James Jones)  - Added language describing how to use
//!   VK_IMAGE_LAYOUT_PRESENT_SOURCE_KHR.  - Cleaned up an XXX comment regarding the description of
//!   which queues vkQueuePresentKHR can be used on.
//! - Revision 22, 2015-06-17 (James Jones)  - Rebased on Vulkan API version 126.
//! - Revision 23, 2015-06-18 (James Jones)  - Updated language for issue 12 to read as a proposed
//!   resolution.  - Marked issues 11, 12, 13, 16, and 17 resolved.  - Temporarily added links to
//!   the relevant bugs under the remaining unresolved issues.  - Added issues 19 and 20 as well as
//!   proposed resolutions.
//! - Revision 24, 2015-06-19 (Ian Elliott)  - Changed special value for
//!   VkSurfacePropertiesKHR::currentExtent back to “-1” from “0”. This value will never need to be
//!   unsigned, and “0” is actually a legal value.
//! - Revision 25, 2015-06-23 (Ian Elliott)  - Examples now show use of function pointers for
//!   extension functions.  - Eliminated extraneous whitespace.
//! - Revision 26, 2015-06-25 (Ian Elliott)  - Resolved Issues 9 & 10 per KHR TSG meeting.
//! - Revision 27, 2015-06-25 (James Jones)  - Added oldSwapchain member to
//!   VkSwapchainCreateInfoKHR.
//! - Revision 28, 2015-06-25 (James Jones)  - Added the “inherit” bits to the rotation and
//!   mirroring flags and the associated issue 21.
//! - Revision 29, 2015-06-25 (James Jones)  - Added the “clipped” flag to VkSwapchainCreateInfoKHR,
//!   and the associated issue 22.  - Specified that presenting an image does not modify it.
//! - Revision 30, 2015-06-25 (James Jones)  - Added language to the spec that clarifies the
//!   behavior of vkCreateSwapchainKHR() when the oldSwapchain field of VkSwapchainCreateInfoKHR is
//!   not NULL.
//! - Revision 31, 2015-06-26 (Ian Elliott)  - Example of new VkSwapchainCreateInfoKHR members,
//!   “oldSwapchain” and “clipped”.  - Example of using VkSurfacePropertiesKHR::{min|max}ImageCount
//!   to set VkSwapchainCreateInfoKHR::minImageCount.  - Rename vkGetSurfaceInfoKHR()'s 4th
//!   parameter to “pDataSize”, for consistency with other functions.  - Add macro with C-string
//!   name of extension (just to header file).
//! - Revision 32, 2015-06-26 (James Jones)  - Minor adjustments to the language describing the
//!   behavior of “oldSwapchain”  - Fixed the version date on my previous two updates.
//! - Revision 33, 2015-06-26 (Jesse Hall)  - Add usage flags to VkSwapchainCreateInfoKHR
//! - Revision 34, 2015-06-26 (Ian Elliott)  - Rename vkQueuePresentKHR()'s 2nd parameter to
//!   “pPresentInfo”, for consistency with other functions.
//! - Revision 35, 2015-06-26 (Jason Ekstrand)  - Merged the VkRotationFlagBitsKHR and
//!   VkMirrorFlagBitsKHR enums into a single VkSurfaceTransformFlagBitsKHR enum.
//! - Revision 36, 2015-06-26 (Jason Ekstrand)  - Added a VkSurfaceTransformKHR enum that is not a
//!   bitmask. Each value in VkSurfaceTransformKHR corresponds directly to one of the bits in
//!   VkSurfaceTransformFlagBitsKHR so transforming from one to the other is easy. Having a separate
//!   enum means that currentTransform and preTransform are now unambiguous by definition.
//! - Revision 37, 2015-06-29 (Ian Elliott)  - Corrected one of the signatures of
//!   vkAcquireNextImageKHR, which had the last two parameters switched from what it is elsewhere in
//!   the specification and header files.
//! - Revision 38, 2015-06-30 (Ian Elliott)  - Corrected a typo in description of the
//!   vkGetSwapchainInfoKHR() function.  - Corrected a typo in header file comment for
//!   VkPresentInfoKHR::sType.
//! - Revision 39, 2015-07-07 (Daniel Rakos)  - Added error section describing when each error is
//!   expected to be reported.  - Replaced bool32_t with VkBool32.
//! - Revision 40, 2015-07-10 (Ian Elliott)  - Updated to work with version 138 of the `vulkan.h`
//!   header. This includes declaring the VkSwapchainKHR type using the new VK_DEFINE_NONDISP_HANDLE
//!   macro, and no longer extending VkObjectType (which was eliminated).
//! - Revision 41 2015-07-09 (Mathias Heyer)  - Added color space language.
//! - Revision 42, 2015-07-10 (Daniel Rakos)  - Updated query mechanism to reflect the convention
//!   changes done in the core spec.  - Removed “queue” from the name of
//!   VK_STRUCTURE_TYPE_QUEUE_PRESENT_INFO_KHR to be consistent with the established naming
//!   convention.  - Removed reference to the no longer existing VkObjectType enum.
//! - Revision 43, 2015-07-17 (Daniel Rakos)  - Added support for concurrent sharing of swapchain
//!   images across queue families.  - Updated sample code based on recent changes
//! - Revision 44, 2015-07-27 (Ian Elliott)  - Noted that support for VK_PRESENT_MODE_FIFO_KHR is
//!   required. That is ICDs may optionally support IMMEDIATE and MAILBOX, but must support FIFO.
//! - Revision 45, 2015-08-07 (Ian Elliott)  - Corrected a typo in spec file (type and variable name
//!   had wrong case for the imageColorSpace member of the VkSwapchainCreateInfoKHR struct).  -
//!   Corrected a typo in header file (last parameter in PFN_vkGetSurfacePropertiesKHR was missing
//!   “KHR” at the end of type: VkSurfacePropertiesKHR).
//! - Revision 46, 2015-08-20 (Ian Elliott)  - Renamed this extension and all of its enumerations,
//!   types, functions, etc. This makes it compliant with the proposed standard for Vulkan
//!   extensions.  - Switched from “revision” to “version”, including use of the VK_MAKE_VERSION
//!   macro in the header file.  - Made improvements to several descriptions.  - Changed the status
//!   of several issues from PROPOSED to RESOLVED, leaving no unresolved issues.  - Resolved several
//!   TODOs, did miscellaneous cleanup, etc.
//! - Revision 47, 2015-08-20 (Ian Elliott—​porting a 2015-07-29 change from James Jones)  - Moved
//!   the surface transform enums to VK_WSI_swapchain so they could be reused by VK_WSI_display.
//! - Revision 48, 2015-09-01 (James Jones)  - Various minor cleanups.
//! - Revision 49, 2015-09-01 (James Jones)  - Restore single-field revision number.
//! - Revision 50, 2015-09-01 (James Jones)  - Update Example #4 to include code that illustrates
//!   how to use the oldSwapchain field.
//! - Revision 51, 2015-09-01 (James Jones)  - Fix example code compilation errors.
//! - Revision 52, 2015-09-08 (Matthaeus G. Chajdas)  - Corrected a typo.
//! - Revision 53, 2015-09-10 (Alon Or-bach)  - Removed underscore from SWAP_CHAIN left in
//!   VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR.
//! - Revision 54, 2015-09-11 (Jesse Hall)  - Described the execution and memory coherence
//!   requirements for image transitions to and from VK_IMAGE_LAYOUT_PRESENT_SOURCE_KHR.
//! - Revision 55, 2015-09-11 (Ray Smith)  - Added errors for destroying and binding memory to
//!   presentable images
//! - Revision 56, 2015-09-18 (James Jones)  - Added fence argument to vkAcquireNextImageKHR  -
//!   Added example of how to meter a host thread based on presentation rate.
//! - Revision 57, 2015-09-26 (Jesse Hall)  - Replace VkSurfaceDescriptionKHR with VkSurfaceKHR.  -
//!   Added issue 25 with agreed resolution.
//! - Revision 58, 2015-09-28 (Jesse Hall)  - Renamed from VK_EXT_KHR_device_swapchain to
//!   VK_EXT_KHR_swapchain.
//! - Revision 59, 2015-09-29 (Ian Elliott)  - Changed vkDestroySwapchainKHR() to return void.
//! - Revision 60, 2015-10-01 (Jeff Vigil)  - Added error result VK_ERROR_SURFACE_LOST_KHR.
//! - Revision 61, 2015-10-05 (Jason Ekstrand)  - Added the VkCompositeAlpha enum and corresponding
//!   structure fields.
//! - Revision 62, 2015-10-12 (Daniel Rakos)  - Added VK_PRESENT_MODE_FIFO_RELAXED_KHR.
//! - Revision 63, 2015-10-15 (Daniel Rakos)  - Moved surface capability queries to
//!   VK_EXT_KHR_surface.
//! - Revision 64, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_swapchain to
//!   VK_KHR_swapchain.
//! - Revision 65, 2015-10-28 (Ian Elliott)  - Added optional pResult member to VkPresentInfoKHR, so
//!   that per-swapchain results can be obtained from vkQueuePresentKHR().
//! - Revision 66, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to create and destroy
//!   functions.  - Updated resource transition language.  - Updated sample code.
//! - Revision 67, 2015-11-10 (Jesse Hall)  - Add reserved flags bitmask to
//!   VkSwapchainCreateInfoKHR.  - Modify naming and member ordering to match API style conventions,
//!   and so the VkSwapchainCreateInfoKHR image property members mirror corresponding
//!   VkImageCreateInfo members but with an 'image' prefix.  - Make VkPresentInfoKHR::pResults
//!   non-const; it is an output array parameter.  - Make pPresentInfo parameter to
//!   vkQueuePresentKHR const.
//! - Revision 68, 2016-04-05 (Ian Elliott)  - Moved the “validity” include for vkAcquireNextImage
//!   to be in its proper place, after the prototype and list of parameters.  - Clarified language
//!   about presentable images, including how they are acquired, when applications can and cannot
//!   use them, etc. As part of this, removed language about “ownership” of presentable images, and
//!   replaced it with more-consistent language about presentable images being “acquired” by the
//!   application.
//! - 2016-08-23 (Ian Elliott)  - Update the example code, to use the final API command names, to
//!   not have so many characters per line, and to split out a new example to show how to obtain
//!   function pointers. This code is more similar to the LunarG “cube” demo program.
//! - 2016-08-25 (Ian Elliott)  - A note was added at the beginning of the example code, stating
//!   that it will be removed from future versions of the appendix.
//! - Revision 69, 2017-09-07 (Tobias Hector)  - Added interactions with Vulkan 1.1
//! - Revision 70, 2017-10-06 (Ian Elliott)  - Corrected interactions with Vulkan 1.1
//!# Other info
//! * 2017-10-06
//! * No known IP claims.
//! * - Interacts with Vulkan 1.1
//! * - Patrick Doane, Blizzard  - Ian Elliott, LunarG  - Jesse Hall, Google  - Mathias Heyer,
//!   NVIDIA  - James Jones, NVIDIA  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach,
//!   Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Jeff Vigil, Qualcomm  - Chia-I Wu,
//!   LunarG  - Jason Ekstrand, Intel  - Matthaeus G. Chajdas, AMD  - Ray Smith, ARM
//!# Related
//! - [`PresentInfoKHR`]
//! - [`SwapchainCreateFlagBitsKHR`]
//! - [`SwapchainCreateFlagsKHR`]
//! - [`SwapchainCreateInfoKHR`]
//! - [`SwapchainKHR`]
//! - [`acquire_next_image_khr`]
//! - [`create_swapchain_khr`]
//! - [`destroy_swapchain_khr`]
//! - [`get_swapchain_images_khr`]
//! - [`queue_present_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "VK_AMD_display_native_hdr")]
pub use crate::extensions::amd_display_native_hdr::SwapchainDisplayNativeHdrCreateInfoAMD;
#[cfg(feature = "VK_EXT_display_control")]
pub use crate::extensions::ext_display_control::SwapchainCounterCreateInfoEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveInfoEXT;
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
pub use crate::extensions::ext_full_screen_exclusive::SurfaceFullScreenExclusiveWin32InfoEXT;
#[cfg(feature = "VK_GGP_frame_token")]
pub use crate::extensions::ggp_frame_token::PresentFrameTokenGGP;
#[cfg(feature = "VK_GOOGLE_display_timing")]
pub use crate::extensions::google_display_timing::PresentTimesInfoGOOGLE;
#[cfg(feature = "VK_KHR_device_group")]
pub use crate::extensions::khr_device_group::DeviceGroupPresentInfoKHR;
#[cfg(feature = "VK_KHR_device_group")]
pub use crate::extensions::khr_device_group::DeviceGroupSwapchainCreateInfoKHR;
#[cfg(feature = "VK_KHR_display_swapchain")]
pub use crate::extensions::khr_display_swapchain::DisplayPresentInfoKHR;
#[cfg(feature = "VK_KHR_incremental_present")]
pub use crate::extensions::khr_incremental_present::PresentRegionsKHR;
#[cfg(feature = "VK_KHR_present_id")]
pub use crate::extensions::khr_present_id::PresentIdKHR;
use crate::{
    entry::Entry,
    extensions::khr_surface::{
        ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR,
    },
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, BaseOutStructure, Bool32, Device, Extent2D, Fence, Format,
        ImageUsageFlags, ImageViewCreateInfo, Instance, PhysicalDevice, Queue, Semaphore, SharingMode, StructureType,
        VulkanResultCodes,
    },
    vulkan1_2::ImageFormatListCreateInfo,
    AsRaw, Handle, SmallVec, Unique, VulkanResult,
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
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_swapchain");
///[vkCreateSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html) - Create a swapchain
///# C Specifications
///To create a swapchain, call:
///```c
///// Provided by VK_KHR_swapchain
///VkResult vkCreateSwapchainKHR(
///    VkDevice                                    device,
///    const VkSwapchainCreateInfoKHR*             pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkSwapchainKHR*                             pSwapchain);
///```
///# Parameters
/// - [`device`] is the device to create the swapchain for.
/// - [`p_create_info`] is a pointer to a [`SwapchainCreateInfoKHR`] structure specifying the
///   parameters of the created swapchain.
/// - [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
/// - [`p_swapchain`] is a pointer to a [`SwapchainKHR`] handle in which the created swapchain
///   object will be returned.
///# Description
///As mentioned above, if [`create_swapchain_khr`] succeeds, it will return a
///handle to a swapchain containing an array of at least
///`pCreateInfo->minImageCount` presentable images.While acquired by the application, presentable
/// images  **can**  be used in any
///way that equivalent non-presentable images  **can**  be used.
///A presentable image is equivalent to a non-presentable image created with
///the following [`ImageCreateInfo`] parameters:The `pCreateInfo->surface` **must**  not be
/// destroyed until after the
///swapchain is destroyed.If `pCreateInfo->oldSwapchain` is [`crate::Handle::null`], and the native
///window referred to by `pCreateInfo->surface` is already associated with
///a Vulkan swapchain, `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be
///returned.If the native window referred to by `pCreateInfo->surface` is already
///associated with a non-Vulkan graphics API surface,
///`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be returned.The native window referred to by
/// `pCreateInfo->surface` **must**  not become
///associated with a non-Vulkan graphics API surface before all associated
///Vulkan swapchains have been destroyed.[`create_swapchain_khr`] will return
/// `VK_ERROR_DEVICE_LOST` if the
///logical device was lost.
///The [`SwapchainKHR`] is a child of the [`device`], and  **must**  not be
///destroyed before the [`device`].
///However, [`SurfaceKHR`] is not a child of any [`Device`] and is not
///affected by the lost device.
///After successfully recreating a [`Device`], the same [`SurfaceKHR`] **can**  be used to create a
/// new [`SwapchainKHR`], provided the previous one
///was destroyed.If the `oldSwapchain` parameter of [`p_create_info`] is a valid
///swapchain, which has exclusive full-screen access, that access is released
///from `pCreateInfo->oldSwapchain`.
///If the command succeeds in this case, the newly created swapchain will
///automatically acquire exclusive full-screen access from
///`pCreateInfo->oldSwapchain`.In some cases, swapchain creation  **may**  fail if exclusive
/// full-screen mode is
///requested for application control, but for some implementation-specific
///reason exclusive full-screen access is unavailable for the particular
///combination of parameters provided.
///If this occurs, `VK_ERROR_INITIALIZATION_FAILED` will be returned.When the [`SurfaceKHR`] in
/// [`SwapchainCreateInfoKHR`] is a display
///surface, then the [`DisplayModeKHR`] in display surface’s
///[`DisplaySurfaceCreateInfoKHR`] is associated with a particular
///[`DisplayKHR`].
///Swapchain creation  **may**  fail if that [`DisplayKHR`] is not acquired by
///the application.
///In this scenario `VK_ERROR_INITIALIZATION_FAILED` is returned.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`SwapchainCreateInfoKHR`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_swapchain`] **must**  be a valid pointer to a [`SwapchainKHR`] handle
///
///## Host Synchronization
/// - Host access to `pCreateInfo->surface` **must**  be externally synchronized
/// - Host access to `pCreateInfo->oldSwapchain` **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_SURFACE_LOST_KHR`  - `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`  -
///   `VK_ERROR_INITIALIZATION_FAILED`
///# Related
/// - [`khr_swapchain`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateSwapchainKHR")]
pub type FNCreateSwapchainKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_swapchain: *mut SwapchainKHR,
    ) -> VulkanResultCodes,
>;
///[vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html) - Destroy a swapchain object
///# C Specifications
///To destroy a swapchain object call:
///```c
///// Provided by VK_KHR_swapchain
///void vkDestroySwapchainKHR(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    const VkAllocationCallbacks*                pAllocator);
///```
///# Parameters
/// - [`device`] is the [`Device`] associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to destroy.
/// - [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
///# Description
///The application  **must**  not destroy a swapchain until after completion of all
///outstanding operations on images that were acquired from the swapchain.
///[`swapchain`] and all associated [`Image`] handles are destroyed, and
/// **must**  not be acquired or used any more by the application.
///The memory of each [`Image`] will only be freed after that image is no
///longer used by the presentation engine.
///For example, if one image of the swapchain is being displayed in a window,
///the memory for that image  **may**  not be freed until the window is destroyed,
///or another swapchain is created for the window.
///Destroying the swapchain does not invalidate the parent [`SurfaceKHR`],
///and a new swapchain  **can**  be created with it.When a swapchain associated with a display
/// surface is destroyed, if the
///image most recently presented to the display surface is from the swapchain
///being destroyed, then either any display resources modified by presenting
///images from any swapchain associated with the display surface  **must**  be
///reverted by the implementation to their state prior to the first present
///performed on one of these swapchains, or such resources  **must**  be left in
///their current state.If [`swapchain`] has exclusive full-screen access, it is released before
///the swapchain is destroyed.
///## Valid Usage
/// - All uses of presentable images acquired from [`swapchain`] **must**  have completed execution
/// - If [`AllocationCallbacks`] were provided when [`swapchain`] was created, a compatible set of
///   callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`swapchain`] was created, [`p_allocator`]
///   **must**  be `NULL`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`swapchain`] is not [`crate::Handle::null`], [`swapchain`] **must**  be a valid
///   [`SwapchainKHR`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///# Related
/// - [`khr_swapchain`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroySwapchainKHR")]
pub type FNDestroySwapchainKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html) - Obtain the array of presentable images associated with a swapchain
///# C Specifications
///To obtain the array of presentable images associated with a swapchain, call:
///```c
///// Provided by VK_KHR_swapchain
///VkResult vkGetSwapchainImagesKHR(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    uint32_t*                                   pSwapchainImageCount,
///    VkImage*                                    pSwapchainImages);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the swapchain to query.
/// - [`p_swapchain_image_count`] is a pointer to an integer related to the number of presentable
///   images available or queried, as described below.
/// - [`p_swapchain_images`] is either `NULL` or a pointer to an array of [`Image`] handles.
///# Description
///If [`p_swapchain_images`] is `NULL`, then the number of presentable images
///for [`swapchain`] is returned in [`p_swapchain_image_count`].
///Otherwise, [`p_swapchain_image_count`] **must**  point to a variable set by the
///user to the number of elements in the [`p_swapchain_images`] array, and on
///return the variable is overwritten with the number of structures actually
///written to [`p_swapchain_images`].
///If the value of [`p_swapchain_image_count`] is less than the number of
///presentable images for [`swapchain`], at most [`p_swapchain_image_count`]
///structures will be written, and `VK_INCOMPLETE` will be returned instead
///of `VK_SUCCESS`, to indicate that not all the available presentable
///images were returned.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - [`p_swapchain_image_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_swapchain_image_count`] is not `0`, and [`p_swapchain_images`]
///   is not `NULL`, [`p_swapchain_images`] **must**  be a valid pointer to an array of
///   [`p_swapchain_image_count`][`Image`] handles
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`khr_swapchain`]
/// - [`Device`]
/// - [`Image`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetSwapchainImagesKHR")]
pub type FNGetSwapchainImagesKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut SwapchainImage,
    ) -> VulkanResultCodes,
>;
///[vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html) - Retrieve the index of the next available presentable image
///# C Specifications
///To acquire an available presentable image to use, and retrieve the index of
///that image, call:
///```c
///// Provided by VK_KHR_swapchain
///VkResult vkAcquireNextImageKHR(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    uint64_t                                    timeout,
///    VkSemaphore                                 semaphore,
///    VkFence                                     fence,
///    uint32_t*                                   pImageIndex);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the non-retired swapchain from which an image is being acquired.
/// - [`timeout`] specifies how long the function waits, in nanoseconds, if no image is available.
/// - [`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
/// - [`fence`] is [`crate::Handle::null`] or a fence to signal.
/// - [`p_image_index`] is a pointer to a `uint32_t` in which the index of the next image to use
///   (i.e. an index into the array of images returned by [`get_swapchain_images_khr`]) is returned.
///# Description
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  be unsignaled
/// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  not have any uncompleted signal
///   or wait operations pending
/// - If [`fence`] is not [`crate::Handle::null`] it  **must**  be unsignaled and  **must**  not be
///   associated with any other queue command that has not yet completed execution on that queue
/// - [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::Handle::null`]
/// - If the number of currently acquired images is greater than the difference between the number
///   of images in [`swapchain`] and the value of [`SurfaceCapabilitiesKHR::min_image_count`] as
///   returned by a call to [`get_physical_device_surface_capabilities2_khr`] with the `surface`
///   used to create [`swapchain`], [`timeout`] **must**  not be `UINT64_MAX`
/// - [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - If [`semaphore`] is not [`crate::Handle::null`], [`semaphore`] **must**  be a valid
///   [`Semaphore`] handle
/// - If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`] handle
/// - [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value
/// - If [`semaphore`] is a valid handle, it  **must**  have been created, allocated, or retrieved
///   from [`device`]
/// - If [`fence`] is a valid handle, it  **must**  have been created, allocated, or retrieved from
///   [`device`]
/// - Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
/// - Host access to [`semaphore`] **must**  be externally synchronized
/// - Host access to [`fence`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
///# Related
/// - [`khr_swapchain`]
/// - [`Device`]
/// - [`Fence`]
/// - [`Semaphore`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkAcquireNextImageKHR")]
pub type FNAcquireNextImageKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> VulkanResultCodes,
>;
///[vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html) - Queue an image for presentation
///# C Specifications
///After queueing all rendering commands and transitioning the image to the
///correct layout, to queue an image for presentation, call:
///```c
///// Provided by VK_KHR_swapchain
///VkResult vkQueuePresentKHR(
///    VkQueue                                     queue,
///    const VkPresentInfoKHR*                     pPresentInfo);
///```
///# Parameters
/// - [`queue`] is a queue that is capable of presentation to the target surface’s platform on the
///   same device as the image’s swapchain.
/// - [`p_present_info`] is a pointer to a [`PresentInfoKHR`] structure specifying parameters of the
///   presentation.
///# Description
///## Valid Usage
/// - Each element of `pSwapchains` member of [`p_present_info`] **must**  be a swapchain that is
///   created for a surface for which presentation is supported from [`queue`] as determined using a
///   call to [`get_physical_device_surface_support_khr`]
/// - If more than one member of `pSwapchains` was created from a display surface, all display
///   surfaces referenced that refer to the same display  **must**  use the same display mode
/// - When a semaphore wait operation referring to a binary semaphore defined by the elements of the
///   `pWaitSemaphores` member of [`p_present_info`] executes on [`queue`], there  **must**  be no
///   other queues waiting on the same semaphore
/// -    All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be semaphores that are signaled, or have [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) previously submitted for execution
/// - All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be created with a
///   [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
/// - All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  reference a
///   semaphore signal operation that has been submitted for execution and any semaphore signal
///   operations on which it depends (if any)  **must**  have also been submitted for execution
///Any writes to memory backing the images referenced by the
///`pImageIndices` and `pSwapchains` members of [`p_present_info`],
///that are available before [`queue_present_khr`] is executed, are
///automatically made visible to the read access performed by the presentation
///engine.
///This automatic visibility operation for an image happens-after the semaphore
///signal operation, and happens-before the presentation engine accesses the
///image.Queueing an image for presentation defines a set of *queue operations*,
///including waiting on the semaphores and submitting a presentation request to
///the presentation engine.
///However, the scope of this set of queue operations does not include the
///actual processing of the image by the presentation engine.If [`queue_present_khr`] fails to
/// enqueue the corresponding set of queue
///operations, it  **may**  return `VK_ERROR_OUT_OF_HOST_MEMORY` or
///`VK_ERROR_OUT_OF_DEVICE_MEMORY`.
///If it does, the implementation  **must**  ensure that the state and contents of
///any resources or synchronization primitives referenced is unaffected by the
///call or its failure.If [`queue_present_khr`] fails in such a way that the implementation is
///unable to make that guarantee, the implementation  **must**  return
///`VK_ERROR_DEVICE_LOST`.However, if the presentation request is rejected by the presentation
/// engine
///with an error `VK_ERROR_OUT_OF_DATE_KHR`,
///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`,
///or `VK_ERROR_SURFACE_LOST_KHR`, the set of queue operations are still
///considered to be enqueued and thus any semaphore wait operation specified in
///[`PresentInfoKHR`] will execute when the corresponding queue operation
///is complete.Calls to [`queue_present_khr`] **may**  block, but  **must**  return in finite
///time.If any `swapchain` member of [`p_present_info`] was created with
///`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`,
///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` will be returned if that
///swapchain does not have exclusive full-screen access, possibly for
///implementation-specific reasons outside of the application’s control.
///## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`p_present_info`] **must**  be a valid pointer to a valid [`PresentInfoKHR`] structure
///
///## Host Synchronization
/// - Host access to [`queue`] **must**  be externally synchronized
/// - Host access to `pPresentInfo->pWaitSemaphores`[]  **must**  be externally synchronized
/// - Host access to `pPresentInfo->pSwapchains`[]  **must**  be externally synchronized
///
///## Command Properties
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///   - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
///# Related
/// - [`khr_swapchain`]
/// - [`PresentInfoKHR`]
/// - [`Queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkQueuePresentKHR")]
pub type FNQueuePresentKhr = Option<
    for<'lt> unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR<'lt>) -> VulkanResultCodes,
>;
///[vkCreateImageView](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html) - Create an image view from an existing image
///# C Specifications
///To create an image view, call:
///```c
///// Provided by VK_VERSION_1_0
///VkResult vkCreateImageView(
///    VkDevice                                    device,
///    const VkImageViewCreateInfo*                pCreateInfo,
///    const VkAllocationCallbacks*                pAllocator,
///    VkImageView*                                pView);
///```
///# Parameters
/// - [`device`] is the logical device that creates the image view.
/// - [`p_create_info`] is a pointer to a [`ImageViewCreateInfo`] structure containing parameters to
///   be used to create the image view.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_view`] is a pointer to a [`ImageView`] handle in which the resulting image view object is
///   returned.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_create_info`] **must**  be a valid pointer to a valid [`ImageViewCreateInfo`] structure
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_view`] **must**  be a valid pointer to a [`ImageView`] handle
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`crate::vulkan1_0`]
/// - [`AllocationCallbacks`]
/// - [`Device`]
/// - [`ImageView`]
/// - [`ImageViewCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateImageView")]
pub type FNCreateSwapchainImageView = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ImageViewCreateInfo<'lt>,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_view: *mut SwapchainImageView,
    ) -> VulkanResultCodes,
>;
///[VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) - Bitmask controlling swapchain creation
///# C Specifications
///Bits which  **can**  be set in [`SwapchainCreateInfoKHR::flags`],
///specifying parameters of swapchain creation, are:
///```c
///// Provided by VK_KHR_swapchain
///typedef enum VkSwapchainCreateFlagBitsKHR {
///  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///    VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
///  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain
///    VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR = 0x00000002,
///  // Provided by VK_KHR_swapchain_mutable_format
///    VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR = 0x00000004,
///} VkSwapchainCreateFlagBitsKHR;
///```
///# Description
/// - [`SPLIT_INSTANCE_BIND_REGIONS`] specifies that images created from the swapchain (i.e. with
///   the `swapchain` member of [`ImageSwapchainCreateInfoKHR`] set to this swapchain’s handle)
///   **must**  use `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`.
/// - [`PROTECTED`] specifies that images created from the swapchain are protected images.
/// - [`MUTABLE_FORMAT`] specifies that the images of the swapchain  **can**  be used to create a
///   [`ImageView`] with a different format than what the swapchain was created with. The list of
///   allowed image view formats is specified by adding a [`ImageFormatListCreateInfo`] structure to
///   the `pNext` chain of [`SwapchainCreateInfoKHR`]. In addition, this flag also specifies that
///   the swapchain  **can**  be created with usage flags that are not supported for the format the
///   swapchain is created with but are supported for at least one of the allowed image view
///   formats.
///# Related
/// - [`khr_swapchain`]
/// - [`SwapchainCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SwapchainCreateFlagBitsKHR(u32);
impl const Default for SwapchainCreateFlagBitsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl SwapchainCreateFlagBitsKHR {
    ///[`SPLIT_INSTANCE_BIND_REGIONS`] specifies
    ///that images created from the swapchain (i.e. with the `swapchain`
    ///member of [`ImageSwapchainCreateInfoKHR`] set to this swapchain’s
    ///handle)  **must**  use `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`.
    ///
    ///Provided by [`crate::extensions::khr_device_group`]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    ///[`PROTECTED`] specifies that images
    ///created from the swapchain are protected images.
    pub const PROTECTED: Self = Self(2);
    ///[`MUTABLE_FORMAT`] specifies that the
    ///images of the swapchain  **can**  be used to create a [`ImageView`] with
    ///a different format than what the swapchain was created with.
    ///The list of allowed image view formats is specified by adding a
    ///[`ImageFormatListCreateInfo`] structure to the `pNext` chain of
    ///[`SwapchainCreateInfoKHR`].
    ///In addition, this flag also specifies that the swapchain  **can**  be created
    ///with usage flags that are not supported for the format the swapchain is
    ///created with but are supported for at least one of the allowed image
    ///view formats.
    ///
    ///Provided by [`crate::extensions::khr_swapchain_mutable_format`]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
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
impl std::fmt::Debug for SwapchainCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(SwapchainCreateFlagBitsKHR))
            .field(match *self {
                #[cfg(feature = "VK_KHR_device_group")]
                Self::SPLIT_INSTANCE_BIND_REGIONS => &"SPLIT_INSTANCE_BIND_REGIONS",
                Self::PROTECTED => &"PROTECTED",
                #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
                Self::MUTABLE_FORMAT => &"MUTABLE_FORMAT",
                other => unreachable!(
                    concat!("invalid value for", stringify!(SwapchainCreateFlagBitsKHR), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for SwapchainCreateFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            #[cfg(feature = "VK_KHR_device_group")]
            Self::SPLIT_INSTANCE_BIND_REGIONS => &"SPLIT_INSTANCE_BIND_REGIONS",
            Self::PROTECTED => &"PROTECTED",
            #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
            Self::MUTABLE_FORMAT => &"MUTABLE_FORMAT",
            other => unreachable!(
                concat!("invalid value for", stringify!(SwapchainCreateFlagBitsKHR), ": {:?}"),
                other
            ),
        })
    }
}
///[VkSwapchainCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateFlagBitsKHR.html) - Bitmask controlling swapchain creation
///# C Specifications
///Bits which  **can**  be set in [`SwapchainCreateInfoKHR::flags`],
///specifying parameters of swapchain creation, are:
///```c
///// Provided by VK_KHR_swapchain
///typedef enum VkSwapchainCreateFlagBitsKHR {
///  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///    VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = 0x00000001,
///  // Provided by VK_VERSION_1_1 with VK_KHR_swapchain
///    VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR = 0x00000002,
///  // Provided by VK_KHR_swapchain_mutable_format
///    VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR = 0x00000004,
///} VkSwapchainCreateFlagBitsKHR;
///```
///# Description
/// - [`SPLIT_INSTANCE_BIND_REGIONS`] specifies that images created from the swapchain (i.e. with
///   the `swapchain` member of [`ImageSwapchainCreateInfoKHR`] set to this swapchain’s handle)
///   **must**  use `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`.
/// - [`PROTECTED`] specifies that images created from the swapchain are protected images.
/// - [`MUTABLE_FORMAT`] specifies that the images of the swapchain  **can**  be used to create a
///   [`ImageView`] with a different format than what the swapchain was created with. The list of
///   allowed image view formats is specified by adding a [`ImageFormatListCreateInfo`] structure to
///   the `pNext` chain of [`SwapchainCreateInfoKHR`]. In addition, this flag also specifies that
///   the swapchain  **can**  be created with usage flags that are not supported for the format the
///   swapchain is created with but are supported for at least one of the allowed image view
///   formats.
///# Related
/// - [`khr_swapchain`]
/// - [`SwapchainCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSwapchainCreateFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SwapchainCreateFlagsKHR(u32);
impl const Default for SwapchainCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from(from: SwapchainCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl SwapchainCreateFlagsKHR {
    ///[`SPLIT_INSTANCE_BIND_REGIONS`] specifies
    ///that images created from the swapchain (i.e. with the `swapchain`
    ///member of [`ImageSwapchainCreateInfoKHR`] set to this swapchain’s
    ///handle)  **must**  use `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`.
    ///
    ///Provided by [`crate::extensions::khr_device_group`]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    ///[`PROTECTED`] specifies that images
    ///created from the swapchain are protected images.
    pub const PROTECTED: Self = Self(2);
    ///[`MUTABLE_FORMAT`] specifies that the
    ///images of the swapchain  **can**  be used to create a [`ImageView`] with
    ///a different format than what the swapchain was created with.
    ///The list of allowed image view formats is specified by adding a
    ///[`ImageFormatListCreateInfo`] structure to the `pNext` chain of
    ///[`SwapchainCreateInfoKHR`].
    ///In addition, this flag also specifies that the swapchain  **can**  be created
    ///with usage flags that are not supported for the format the swapchain is
    ///created with but are supported for at least one of the allowed image
    ///view formats.
    ///
    ///Provided by [`crate::extensions::khr_swapchain_mutable_format`]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
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
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::SPLIT_INSTANCE_BIND_REGIONS;
        }
        {
            all |= Self::PROTECTED;
        }
        #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
        {
            all |= Self::MUTABLE_FORMAT;
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
impl const std::ops::BitOr for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SwapchainCreateFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SwapchainCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SwapchainCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SPLIT_INSTANCE_BIND_REGIONS))?;
                    }
                    if self.0.contains(SwapchainCreateFlagsKHR::PROTECTED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PROTECTED))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
                    if self.0.contains(SwapchainCreateFlagsKHR::MUTABLE_FORMAT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(MUTABLE_FORMAT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SwapchainCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html) - Structure specifying parameters of a newly created swapchain object
///# C Specifications
///The [`SwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_swapchain
///typedef struct VkSwapchainCreateInfoKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkSwapchainCreateFlagsKHR        flags;
///    VkSurfaceKHR                     surface;
///    uint32_t                         minImageCount;
///    VkFormat                         imageFormat;
///    VkColorSpaceKHR                  imageColorSpace;
///    VkExtent2D                       imageExtent;
///    uint32_t                         imageArrayLayers;
///    VkImageUsageFlags                imageUsage;
///    VkSharingMode                    imageSharingMode;
///    uint32_t                         queueFamilyIndexCount;
///    const uint32_t*                  pQueueFamilyIndices;
///    VkSurfaceTransformFlagBitsKHR    preTransform;
///    VkCompositeAlphaFlagBitsKHR      compositeAlpha;
///    VkPresentModeKHR                 presentMode;
///    VkBool32                         clipped;
///    VkSwapchainKHR                   oldSwapchain;
///} VkSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`SwapchainCreateFlagBitsKHR`] indicating parameters of the
///   swapchain creation.
/// - [`surface`] is the surface onto which the swapchain will present images. If the creation
///   succeeds, the swapchain becomes associated with [`surface`].
/// - [`min_image_count`] is the minimum number of presentable images that the application needs.
///   The implementation will either create the swapchain with at least that many images, or it will
///   fail to create the swapchain.
/// - [`image_format`] is a [`Format`] value specifying the format the swapchain image(s) will be
///   created with.
/// - [`image_color_space`] is a [`ColorSpaceKHR`] value specifying the way the swapchain interprets
///   image data.
/// - [`image_extent`] is the size (in pixels) of the swapchain image(s). The behavior is
///   platform-dependent if the image extent does not match the surface’s `currentExtent` as
///   returned by [`get_physical_device_surface_capabilities_khr`].
/// - [`image_array_layers`] is the number of views in a multiview/stereo surface. For
///   non-stereoscopic-3D applications, this value is 1.
/// - [`image_usage`] is a bitmask of [`ImageUsageFlagBits`] describing the intended usage of the
///   (acquired) swapchain images.
/// - [`image_sharing_mode`] is the sharing mode used for the image(s) of the swapchain.
/// - [`queue_family_index_count`] is the number of queue families having access to the image(s) of
///   the swapchain when [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
/// - [`queue_family_indices`] is a pointer to an array of queue family indices having access to the
///   images(s) of the swapchain when [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
/// - [`pre_transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform,
///   relative to the presentation engine’s natural orientation, applied to the image content prior
///   to presentation. If it does not match the `currentTransform` value returned by
///   [`get_physical_device_surface_capabilities_khr`], the presentation engine will transform the
///   image content as part of the presentation operation.
/// - [`composite_alpha`] is a [`CompositeAlphaFlagBitsKHR`] value indicating the alpha compositing
///   mode to use when this surface is composited together with other surfaces on certain window
///   systems.
/// - [`present_mode`] is the presentation mode the swapchain will use. A swapchain’s present mode
///   determines how incoming present requests will be processed and queued internally.
/// - [`clipped`] specifies whether the Vulkan implementation is allowed to discard rendering
///   operations that affect regions of the surface that are not visible.  - If set to [`TRUE`], the
///   presentable images associated with the swapchain  **may**  not own all of their pixels. Pixels
///   in the presentable images that correspond to regions of the target surface obscured by another
///   window on the desktop, or subject to some other clipping mechanism will have undefined content
///   when read back. Fragment shaders  **may**  not execute for these pixels, and thus any side
///   effects they would have had will not occur. Setting [`TRUE`] does not guarantee any clipping
///   will occur, but allows more efficient presentation methods to be used on some platforms.  - If
///   set to [`FALSE`], presentable images associated with the swapchain will own all of the pixels
///   they contain.
/// - [`old_swapchain`] is [`crate::Handle::null`], or the existing non-retired swapchain currently
///   associated with [`surface`]. Providing a valid [`old_swapchain`] **may**  aid in the resource
///   reuse, and also allows the application to still present any images that are already acquired
///   from it.
///# Description
///Upon calling [`create_swapchain_khr`] with an [`old_swapchain`] that is
///not [`crate::Handle::null`], [`old_swapchain`] is retired — even if creation
///of the new swapchain fails.
///The new swapchain is created in the non-retired state whether or not
///[`old_swapchain`] is [`crate::Handle::null`].Upon calling [`create_swapchain_khr`] with an
/// [`old_swapchain`] that is
///not [`crate::Handle::null`], any images from [`old_swapchain`] that are not
///acquired by the application  **may**  be freed by the implementation, which  **may**
///occur even if creation of the new swapchain fails.
///The application  **can**  destroy [`old_swapchain`] to free all memory
///associated with [`old_swapchain`].
///## Valid Usage
/// - [`surface`] **must**  be a surface that is supported by the device as determined using
///   [`get_physical_device_surface_support_khr`]
/// - [`min_image_count`] **must**  be less than or equal to the value returned in the
///   `maxImageCount` member of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for the surface if the returned
///   `maxImageCount` is not zero
/// - If [`present_mode`] is not `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` nor
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`, then [`min_image_count`] **must**  be greater
///   than or equal to the value returned in the [`min_image_count`] member of the
///   [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for the surface
/// - [`min_image_count`] **must**  be `1` if [`present_mode`] is either
///   `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`
/// - [`image_format`] and [`image_color_space`] **must**  match the `format` and `colorSpace`
///   members, respectively, of one of the [`SurfaceFormatKHR`] structures returned by
///   [`get_physical_device_surface_formats_khr`] for the surface
/// - [`image_extent`] **must**  be between `minImageExtent` and `maxImageExtent`, inclusive, where
///   `minImageExtent` and `maxImageExtent` are members of the [`SurfaceCapabilitiesKHR`] structure
///   returned by [`get_physical_device_surface_capabilities_khr`] for the surface
/// - [`image_extent`] members `width` and `height` **must**  both be non-zero
/// - [`image_array_layers`] **must**  be greater than `0` and less than or equal to the
///   `maxImageArrayLayers` member of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for the surface
/// - If [`present_mode`] is `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR`, [`image_usage`] **must**  be
///   a subset of the supported usage flags present in the `supportedUsageFlags` member of the
///   [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for [`surface`]
/// - If [`present_mode`] is `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`, [`image_usage`] **must**  be a subset of the
///   supported usage flags present in the `sharedPresentSupportedUsageFlags` member of the
///   [`SharedPresentSurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities2_khr`] for [`surface`]
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`queue_family_indices`] **must**
///   be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`queue_family_index_count`]
///   **must**  be greater than `1`
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of
///   [`queue_family_indices`] **must**  be unique and  **must**  be less than
///   `pQueueFamilyPropertyCount` returned by either [`get_physical_device_queue_family_properties`]
///   or [`get_physical_device_queue_family_properties2`] for the `physicalDevice` that was used to
///   create `device`
/// - [`pre_transform`] **must**  be one of the bits present in the `supportedTransforms` member of
///   the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for the surface
/// - [`composite_alpha`] **must**  be one of the bits present in the `supportedCompositeAlpha`
///   member of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities_khr`] for the surface
/// - [`present_mode`] **must**  be one of the [`PresentModeKHR`] values returned by
///   [`get_physical_device_surface_present_modes_khr`] for the surface
/// - If the logical device was created with [`DeviceGroupDeviceCreateInfo::physical_device_count`]
///   equal to 1, [`flags`] **must**  not contain
///   `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
/// - If [`old_swapchain`] is not [`crate::Handle::null`], [`old_swapchain`] **must**  be a
///   non-retired swapchain associated with native window referred to by [`surface`]
/// - The [implied image creation parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#swapchain-wsi-image-create-info)
///   of the swapchain  **must**  be supported as reported by
///   [`get_physical_device_image_format_properties`]
/// - If [`flags`] contains `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR` then the [`p_next`] chain
///   **must**  include a [`ImageFormatListCreateInfo`] structure with a `viewFormatCount` greater
///   than zero and `pViewFormats` **must**  have an element equal to [`image_format`]
/// -    If a [`ImageFormatListCreateInfo`] structure was included in the [`p_next`] chain and [`ImageFormatListCreateInfo::view_format_count`] is not zero then all of the formats in [`ImageFormatListCreateInfo::view_formats`] **must**  be compatible with the `format` as described in the [compatibility table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility)
/// - If [`flags`] does not contain `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR` and the [`p_next`]
///   chain include a [`ImageFormatListCreateInfo`] structure then
///   [`ImageFormatListCreateInfo::view_format_count`] **must**  be `0` or `1`
/// - If [`flags`] contains `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`, then
///   [`SurfaceProtectedCapabilitiesKHR::supports_protected`] **must**  be [`TRUE`] in the
///   [`SurfaceProtectedCapabilitiesKHR`] structure returned by
///   [`get_physical_device_surface_capabilities2_khr`] for [`surface`]
/// - If the [`p_next`] chain includes a [`SurfaceFullScreenExclusiveInfoEXT`] structure with its
///   `fullScreenExclusive` member set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`, and
///   [`surface`] was created using [`create_win32_surface_khr`], a
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure  **must**  be included in the [`p_next`]
///   chain
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`DeviceGroupSwapchainCreateInfoKHR`],
///   [`ImageFormatListCreateInfo`], [`SurfaceFullScreenExclusiveInfoEXT`],
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`], [`SwapchainCounterCreateInfoEXT`], or
///   [`SwapchainDisplayNativeHdrCreateInfoAMD`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`SwapchainCreateFlagBitsKHR`] values
/// - [`surface`] **must**  be a valid [`SurfaceKHR`] handle
/// - [`image_format`] **must**  be a valid [`Format`] value
/// - [`image_color_space`] **must**  be a valid [`ColorSpaceKHR`] value
/// - [`image_usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`image_usage`] **must**  not be `0`
/// - [`image_sharing_mode`] **must**  be a valid [`SharingMode`] value
/// - [`pre_transform`] **must**  be a valid [`SurfaceTransformFlagBitsKHR`] value
/// - [`composite_alpha`] **must**  be a valid [`CompositeAlphaFlagBitsKHR`] value
/// - [`present_mode`] **must**  be a valid [`PresentModeKHR`] value
/// - If [`old_swapchain`] is not [`crate::Handle::null`], [`old_swapchain`] **must**  be a valid
///   [`SwapchainKHR`] handle
/// - If [`old_swapchain`] is a valid handle, it  **must**  have been created, allocated, or
///   retrieved from [`surface`]
/// - Both of [`old_swapchain`], and [`surface`] that are valid handles of non-ignored parameters
///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
///# Related
/// - [`khr_swapchain`]
/// - [`Bool32`]
/// - [`ColorSpaceKHR`]
/// - [`CompositeAlphaFlagBitsKHR`]
/// - [`Extent2D`]
/// - [`Format`]
/// - [`ImageUsageFlags`]
/// - [`PresentModeKHR`]
/// - [`SharingMode`]
/// - [`StructureType`]
/// - [`SurfaceKHR`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SwapchainCreateFlagsKHR`]
/// - [`SwapchainKHR`]
/// - [`create_shared_swapchains_khr`]
/// - [`create_swapchain_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SwapchainCreateFlagBitsKHR`]
    ///indicating parameters of the swapchain creation.
    pub flags: SwapchainCreateFlagsKHR,
    ///[`surface`] is the surface onto which the swapchain will present
    ///images.
    ///If the creation succeeds, the swapchain becomes associated with
    ///[`surface`].
    pub surface: SurfaceKHR,
    ///[`min_image_count`] is the minimum number of presentable images that the
    ///application needs.
    ///The implementation will either create the swapchain with at least that
    ///many images, or it will fail to create the swapchain.
    pub min_image_count: u32,
    ///[`image_format`] is a [`Format`] value specifying the format the
    ///swapchain image(s) will be created with.
    pub image_format: Format,
    ///[`image_color_space`] is a [`ColorSpaceKHR`] value specifying the
    ///way the swapchain interprets image data.
    pub image_color_space: ColorSpaceKHR,
    ///[`image_extent`] is the size (in pixels) of the swapchain image(s).
    ///The behavior is platform-dependent if the image extent does not match
    ///the surface’s `currentExtent` as returned by
    ///[`get_physical_device_surface_capabilities_khr`].
    pub image_extent: Extent2D,
    ///[`image_array_layers`] is the number of views in a multiview/stereo
    ///surface.
    ///For non-stereoscopic-3D applications, this value is 1.
    pub image_array_layers: u32,
    ///[`image_usage`] is a bitmask of [`ImageUsageFlagBits`] describing
    ///the intended usage of the (acquired) swapchain images.
    pub image_usage: ImageUsageFlags,
    ///[`image_sharing_mode`] is the sharing mode used for the image(s) of the
    ///swapchain.
    pub image_sharing_mode: SharingMode,
    ///[`queue_family_index_count`] is the number of queue families having
    ///access to the image(s) of the swapchain when [`image_sharing_mode`] is
    ///`VK_SHARING_MODE_CONCURRENT`.
    pub queue_family_index_count: u32,
    ///[`queue_family_indices`] is a pointer to an array of queue family
    ///indices having access to the images(s) of the swapchain when
    ///[`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
    pub queue_family_indices: *const u32,
    ///[`pre_transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform, relative to the presentation engine’s natural
    ///orientation, applied to the image content prior to presentation.
    ///If it does not match the `currentTransform` value returned by
    ///[`get_physical_device_surface_capabilities_khr`], the presentation engine
    ///will transform the image content as part of the presentation operation.
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    ///[`composite_alpha`] is a [`CompositeAlphaFlagBitsKHR`] value
    ///indicating the alpha compositing mode to use when this surface is
    ///composited together with other surfaces on certain window systems.
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    ///[`present_mode`] is the presentation mode the swapchain will use.
    ///A swapchain’s present mode determines how incoming present requests will
    ///be processed and queued internally.
    pub present_mode: PresentModeKHR,
    ///[`clipped`] specifies whether the Vulkan implementation is allowed to
    ///discard rendering operations that affect regions of the surface that are
    ///not visible.
    /// - If set to [`TRUE`], the presentable images associated with the swapchain  **may**  not own
    ///   all of their pixels. Pixels in the presentable images that correspond to regions of the
    ///   target surface obscured by another window on the desktop, or subject to some other
    ///   clipping mechanism will have undefined content when read back. Fragment shaders  **may**
    ///   not execute for these pixels, and thus any side effects they would have had will not
    ///   occur. Setting [`TRUE`] does not guarantee any clipping will occur, but allows more
    ///   efficient presentation methods to be used on some platforms.
    /// - If set to [`FALSE`], presentable images associated with the swapchain will own all of the
    ///   pixels they contain.
    pub clipped: Bool32,
    ///[`old_swapchain`] is [`crate::Handle::null`], or the existing non-retired
    ///swapchain currently associated with [`surface`].
    ///Providing a valid [`old_swapchain`] **may**  aid in the resource reuse, and
    ///also allows the application to still present any images that are already
    ///acquired from it.
    pub old_swapchain: SwapchainKHR,
}
impl<'lt> Default for SwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: 0,
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: 0,
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: 0,
            queue_family_indices: std::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: 0,
            old_swapchain: Default::default(),
        }
    }
}
impl<'lt> SwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::queue_family_indices`]
    pub fn queue_family_indices_raw(&self) -> *const u32 {
        self.queue_family_indices
    }
    ///Gets the raw value of [`Self::clipped`]
    pub fn clipped_raw(&self) -> Bool32 {
        self.clipped
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices_raw(mut self, value: *const u32) -> Self {
        self.queue_family_indices = value;
        self
    }
    ///Sets the raw value of [`Self::clipped`]
    pub fn set_clipped_raw(mut self, value: Bool32) -> Self {
        self.clipped = value;
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
    pub fn flags(&self) -> SwapchainCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::surface`]
    pub fn surface(&self) -> SurfaceKHR {
        self.surface
    }
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::image_format`]
    pub fn image_format(&self) -> Format {
        self.image_format
    }
    ///Gets the value of [`Self::image_color_space`]
    pub fn image_color_space(&self) -> ColorSpaceKHR {
        self.image_color_space
    }
    ///Gets the value of [`Self::image_extent`]
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Gets the value of [`Self::image_array_layers`]
    pub fn image_array_layers(&self) -> u32 {
        self.image_array_layers
    }
    ///Gets the value of [`Self::image_usage`]
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Gets the value of [`Self::image_sharing_mode`]
    pub fn image_sharing_mode(&self) -> SharingMode {
        self.image_sharing_mode
    }
    ///Gets the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count(&self) -> u32 {
        self.queue_family_index_count
    }
    ///Gets the value of [`Self::queue_family_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn queue_family_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.queue_family_indices, self.queue_family_index_count as usize)
    }
    ///Gets the value of [`Self::pre_transform`]
    pub fn pre_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.pre_transform
    }
    ///Gets the value of [`Self::composite_alpha`]
    pub fn composite_alpha(&self) -> CompositeAlphaFlagBitsKHR {
        self.composite_alpha
    }
    ///Gets the value of [`Self::present_mode`]
    pub fn present_mode(&self) -> PresentModeKHR {
        self.present_mode
    }
    ///Gets the value of [`Self::clipped`]
    pub fn clipped(&self) -> bool {
        unsafe { std::mem::transmute(self.clipped as u8) }
    }
    ///Gets the value of [`Self::old_swapchain`]
    pub fn old_swapchain(&self) -> SwapchainKHR {
        self.old_swapchain
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SwapchainCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    pub fn surface_mut(&mut self) -> &mut SurfaceKHR {
        &mut self.surface
    }
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut self.min_image_count
    }
    ///Gets a mutable reference to the value of [`Self::image_format`]
    pub fn image_format_mut(&mut self) -> &mut Format {
        &mut self.image_format
    }
    ///Gets a mutable reference to the value of [`Self::image_color_space`]
    pub fn image_color_space_mut(&mut self) -> &mut ColorSpaceKHR {
        &mut self.image_color_space
    }
    ///Gets a mutable reference to the value of [`Self::image_extent`]
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
    }
    ///Gets a mutable reference to the value of [`Self::image_array_layers`]
    pub fn image_array_layers_mut(&mut self) -> &mut u32 {
        &mut self.image_array_layers
    }
    ///Gets a mutable reference to the value of [`Self::image_usage`]
    pub fn image_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.image_usage
    }
    ///Gets a mutable reference to the value of [`Self::image_sharing_mode`]
    pub fn image_sharing_mode_mut(&mut self) -> &mut SharingMode {
        &mut self.image_sharing_mode
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index_count
    }
    ///Gets a mutable reference to the value of [`Self::pre_transform`]
    pub fn pre_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.pre_transform
    }
    ///Gets a mutable reference to the value of [`Self::composite_alpha`]
    pub fn composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagBitsKHR {
        &mut self.composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::present_mode`]
    pub fn present_mode_mut(&mut self) -> &mut PresentModeKHR {
        &mut self.present_mode
    }
    ///Gets a mutable reference to the value of [`Self::clipped`]
    pub fn clipped_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.clipped as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.clipped as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::old_swapchain`]
    pub fn old_swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.old_swapchain
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
    pub fn set_flags(mut self, value: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::surface`]
    pub fn set_surface(mut self, value: crate::extensions::khr_surface::SurfaceKHR) -> Self {
        self.surface = value;
        self
    }
    ///Sets the value of [`Self::min_image_count`]
    pub fn set_min_image_count(mut self, value: u32) -> Self {
        self.min_image_count = value;
        self
    }
    ///Sets the value of [`Self::image_format`]
    pub fn set_image_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.image_format = value;
        self
    }
    ///Sets the value of [`Self::image_color_space`]
    pub fn set_image_color_space(mut self, value: crate::extensions::khr_surface::ColorSpaceKHR) -> Self {
        self.image_color_space = value;
        self
    }
    ///Sets the value of [`Self::image_extent`]
    pub fn set_image_extent(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.image_extent = value;
        self
    }
    ///Sets the value of [`Self::image_array_layers`]
    pub fn set_image_array_layers(mut self, value: u32) -> Self {
        self.image_array_layers = value;
        self
    }
    ///Sets the value of [`Self::image_usage`]
    pub fn set_image_usage(mut self, value: crate::vulkan1_0::ImageUsageFlags) -> Self {
        self.image_usage = value;
        self
    }
    ///Sets the value of [`Self::image_sharing_mode`]
    pub fn set_image_sharing_mode(mut self, value: crate::vulkan1_0::SharingMode) -> Self {
        self.image_sharing_mode = value;
        self
    }
    ///Sets the value of [`Self::queue_family_index_count`]
    pub fn set_queue_family_index_count(mut self, value: u32) -> Self {
        self.queue_family_index_count = value;
        self
    }
    ///Sets the value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_family_indices = value.as_ptr();
        self.queue_family_index_count = len_;
        self
    }
    ///Sets the value of [`Self::pre_transform`]
    pub fn set_pre_transform(mut self, value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.pre_transform = value;
        self
    }
    ///Sets the value of [`Self::composite_alpha`]
    pub fn set_composite_alpha(mut self, value: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR) -> Self {
        self.composite_alpha = value;
        self
    }
    ///Sets the value of [`Self::present_mode`]
    pub fn set_present_mode(mut self, value: crate::extensions::khr_surface::PresentModeKHR) -> Self {
        self.present_mode = value;
        self
    }
    ///Sets the value of [`Self::clipped`]
    pub fn set_clipped(mut self, value: bool) -> Self {
        self.clipped = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::old_swapchain`]
    pub fn set_old_swapchain(mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> Self {
        self.old_swapchain = value;
        self
    }
}
#[cfg(feature = "VK_EXT_display_control")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, SwapchainCounterCreateInfoEXT<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut SwapchainCounterCreateInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut SwapchainCounterCreateInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, DeviceGroupSwapchainCreateInfoKHR<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut DeviceGroupSwapchainCreateInfoKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut DeviceGroupSwapchainCreateInfoKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_AMD_display_native_hdr")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, SwapchainDisplayNativeHdrCreateInfoAMD<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut SwapchainDisplayNativeHdrCreateInfoAMD<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut SwapchainDisplayNativeHdrCreateInfoAMD<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, ImageFormatListCreateInfo<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut ImageFormatListCreateInfo<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut ImageFormatListCreateInfo<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, SurfaceFullScreenExclusiveInfoEXT<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut SurfaceFullScreenExclusiveInfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut SurfaceFullScreenExclusiveInfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_EXT_full_screen_exclusive")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, SurfaceFullScreenExclusiveWin32InfoEXT<'extender>> for SwapchainCreateInfoKHR<'this>
{
    type Out = SwapchainCreateInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut SurfaceFullScreenExclusiveWin32InfoEXT<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut SurfaceFullScreenExclusiveWin32InfoEXT<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
///[VkPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html) - Structure describing parameters of a queue presentation
///# C Specifications
///The [`PresentInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_swapchain
///typedef struct VkPresentInfoKHR {
///    VkStructureType          sType;
///    const void*              pNext;
///    uint32_t                 waitSemaphoreCount;
///    const VkSemaphore*       pWaitSemaphores;
///    uint32_t                 swapchainCount;
///    const VkSwapchainKHR*    pSwapchains;
///    const uint32_t*          pImageIndices;
///    VkResult*                pResults;
///} VkPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`wait_semaphore_count`] is the number of semaphores to wait for before issuing the present
///   request. The number  **may**  be zero.
/// - [`wait_semaphores`] is `NULL` or a pointer to an array of [`Semaphore`] objects with
///   [`wait_semaphore_count`] entries, and specifies the semaphores to wait for before issuing the
///   present request.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`swapchains`] is a pointer to an array of [`SwapchainKHR`] objects with [`swapchain_count`]
///   entries. A given swapchain  **must**  not appear in this list more than once.
/// - [`image_indices`] is a pointer to an array of indices into the array of each swapchain’s
///   presentable images, with [`swapchain_count`] entries. Each entry in this array identifies the
///   image to present on the corresponding entry in the [`swapchains`] array.
/// - [`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements with
///   [`swapchain_count`] entries. Applications that do not need per-swapchain results  **can**  use
///   `NULL` for [`results`]. If non-`NULL`, each entry in [`results`] will be set to the
///   [`VulkanResultCodes`] for presenting the swapchain corresponding to the same index in
///   [`swapchains`].
///# Description
///Before an application  **can**  present an image, the image’s layout  **must**  be
///transitioned to the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///layout, or for a shared presentable image the
///`VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
///layout.
///## Valid Usage
/// - Each element of [`image_indices`] **must**  be the index of a presentable image acquired from
///   the swapchain specified by the corresponding element of the [`swapchains`] array, and the
///   presented image subresource  **must**  be in the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` or
///   `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout at the time the operation is executed on a
///   [`Device`]
/// - If a [`PresentIdKHR`] structure is included in the [`p_next`] chain, and the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId)
///   feature is not enabled, each `presentIds` entry in that structure  **must**  be NULL
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`DeviceGroupPresentInfoKHR`],
///   [`DisplayPresentInfoKHR`], [`PresentFrameTokenGGP`], [`PresentIdKHR`], [`PresentRegionsKHR`],
///   or [`PresentTimesInfoGOOGLE`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - If [`wait_semaphore_count`] is not `0`, [`wait_semaphores`] **must**  be a valid pointer to an
///   array of [`wait_semaphore_count`] valid [`Semaphore`] handles
/// - [`swapchains`] **must**  be a valid pointer to an array of [`swapchain_count`] valid
///   [`SwapchainKHR`] handles
/// - [`image_indices`] **must**  be a valid pointer to an array of [`swapchain_count`]`uint32_t`
///   values
/// - If [`results`] is not `NULL`, [`results`] **must**  be a valid pointer to an array of
///   [`swapchain_count`][`VulkanResultCodes`] values
/// - [`swapchain_count`] **must**  be greater than `0`
/// - Both of the elements of [`swapchains`], and the elements of [`wait_semaphores`] that are valid
///   handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from
///   the same [`Instance`]
///# Related
/// - [`khr_swapchain`]
/// - [`VulkanResultCodes`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
/// - [`queue_present_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PresentInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_count`] is the number of semaphores to wait for before
    ///issuing the present request.
    ///The number  **may**  be zero.
    pub wait_semaphore_count: u32,
    ///[`wait_semaphores`] is `NULL` or a pointer to an array of
    ///[`Semaphore`] objects with [`wait_semaphore_count`] entries, and
    ///specifies the semaphores to wait for before issuing the present request.
    pub wait_semaphores: *const Semaphore,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    pub swapchain_count: u32,
    ///[`swapchains`] is a pointer to an array of [`SwapchainKHR`]
    ///objects with [`swapchain_count`] entries.
    ///A given swapchain  **must**  not appear in this list more than once.
    pub swapchains: *const SwapchainKHR,
    ///[`image_indices`] is a pointer to an array of indices into the array
    ///of each swapchain’s presentable images, with [`swapchain_count`]
    ///entries.
    ///Each entry in this array identifies the image to present on the
    ///corresponding entry in the [`swapchains`] array.
    pub image_indices: *const u32,
    ///[`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements
    ///with [`swapchain_count`] entries.
    ///Applications that do not need per-swapchain results  **can**  use `NULL` for
    ///[`results`].
    ///If non-`NULL`, each entry in [`results`] will be set to the
    ///[`VulkanResultCodes`] for presenting the swapchain corresponding to the same
    ///index in [`swapchains`].
    pub results: *mut VulkanResultCodes,
}
impl<'lt> Default for PresentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PRESENT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_count: 0,
            wait_semaphores: std::ptr::null(),
            swapchain_count: 0,
            swapchains: std::ptr::null(),
            image_indices: std::ptr::null(),
            results: std::ptr::null_mut(),
        }
    }
}
impl<'lt> PresentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphores`]
    pub fn wait_semaphores_raw(&self) -> *const Semaphore {
        self.wait_semaphores
    }
    ///Gets the raw value of [`Self::swapchains`]
    pub fn swapchains_raw(&self) -> *const SwapchainKHR {
        self.swapchains
    }
    ///Gets the raw value of [`Self::image_indices`]
    pub fn image_indices_raw(&self) -> *const u32 {
        self.image_indices
    }
    ///Gets the raw value of [`Self::results`]
    pub fn results_raw(&self) -> *mut VulkanResultCodes {
        self.results
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphores`]
    pub fn set_wait_semaphores_raw(mut self, value: *const Semaphore) -> Self {
        self.wait_semaphores = value;
        self
    }
    ///Sets the raw value of [`Self::swapchains`]
    pub fn set_swapchains_raw(mut self, value: *const SwapchainKHR) -> Self {
        self.swapchains = value;
        self
    }
    ///Sets the raw value of [`Self::image_indices`]
    pub fn set_image_indices_raw(mut self, value: *const u32) -> Self {
        self.image_indices = value;
        self
    }
    ///Sets the raw value of [`Self::results`]
    pub fn set_results_raw(mut self, value: *mut VulkanResultCodes) -> Self {
        self.results = value;
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
    ///Gets the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count(&self) -> u32 {
        self.wait_semaphore_count
    }
    ///Gets the value of [`Self::wait_semaphores`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphores(&self) -> &[Semaphore] {
        std::slice::from_raw_parts(self.wait_semaphores, self.wait_semaphore_count as usize)
    }
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::swapchains`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn swapchains(&self) -> &[SwapchainKHR] {
        std::slice::from_raw_parts(self.swapchains, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::image_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn image_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.image_indices, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::results`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn results(&self) -> &[VulkanResultCodes] {
        std::slice::from_raw_parts(self.results, self.swapchain_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count_mut(&mut self) -> &mut u32 {
        &mut self.wait_semaphore_count
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut self.swapchain_count
    }
    ///Gets a mutable reference to the value of [`Self::results`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn results_mut(&mut self) -> &mut [VulkanResultCodes] {
        std::slice::from_raw_parts_mut(self.results, self.swapchain_count as usize)
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
    ///Sets the value of [`Self::wait_semaphore_count`]
    pub fn set_wait_semaphore_count(mut self, value: u32) -> Self {
        self.wait_semaphore_count = value;
        self
    }
    ///Sets the value of [`Self::wait_semaphores`]
    pub fn set_wait_semaphores(mut self, value: &'lt [crate::vulkan1_0::Semaphore]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphores = value.as_ptr();
        self.wait_semaphore_count = len_;
        self
    }
    ///Sets the value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(mut self, value: u32) -> Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the value of [`Self::swapchains`]
    pub fn set_swapchains(mut self, value: &'lt [crate::extensions::khr_swapchain::SwapchainKHR]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.swapchains = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the value of [`Self::image_indices`]
    pub fn set_image_indices(mut self, value: &'lt [u32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.image_indices = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the value of [`Self::results`]
    pub fn set_results(mut self, value: &'lt mut [crate::vulkan1_0::VulkanResultCodes]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.results = value.as_mut_ptr();
        self.swapchain_count = len_;
        self
    }
}
#[cfg(feature = "VK_KHR_display_swapchain")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other> crate::Chain<'other, DisplayPresentInfoKHR<'extender>>
    for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut DisplayPresentInfoKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut DisplayPresentInfoKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_KHR_incremental_present")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other> crate::Chain<'other, PresentRegionsKHR<'extender>>
    for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut PresentRegionsKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut PresentRegionsKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_KHR_device_group")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, DeviceGroupPresentInfoKHR<'extender>> for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut DeviceGroupPresentInfoKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut DeviceGroupPresentInfoKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_KHR_present_id")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other> crate::Chain<'other, PresentIdKHR<'extender>>
    for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut PresentIdKHR<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut PresentIdKHR<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_GOOGLE_display_timing")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other>
    crate::Chain<'other, PresentTimesInfoGOOGLE<'extender>> for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut PresentTimesInfoGOOGLE<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut PresentTimesInfoGOOGLE<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
#[cfg(feature = "VK_GGP_frame_token")]
unsafe impl<'this: 'extender + 'other, 'extender: 'other, 'other> crate::Chain<'other, PresentFrameTokenGGP<'extender>>
    for PresentInfoKHR<'this>
{
    type Out = PresentInfoKHR<'other>;
    #[must_use]
    #[inline]
    fn chain(mut self, new: &'other mut PresentFrameTokenGGP<'extender>) -> Self::Out {
        unsafe {
            crate::chaining::insert_ptr_in_chain(
                &mut self as *mut Self as *mut BaseOutStructure<'other>,
                new as *mut PresentFrameTokenGGP<'extender> as *mut BaseOutStructure<'other>,
            );
            std::mem::transmute(self)
        }
    }
}
impl Device {
    ///[vkCreateSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html) - Create a swapchain
    ///# C Specifications
    ///To create a swapchain, call:
    ///```c
    ///// Provided by VK_KHR_swapchain
    ///VkResult vkCreateSwapchainKHR(
    ///    VkDevice                                    device,
    ///    const VkSwapchainCreateInfoKHR*             pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkSwapchainKHR*                             pSwapchain);
    ///```
    ///# Parameters
    /// - [`device`] is the device to create the swapchain for.
    /// - [`p_create_info`] is a pointer to a [`SwapchainCreateInfoKHR`] structure specifying the
    ///   parameters of the created swapchain.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    /// - [`p_swapchain`] is a pointer to a [`SwapchainKHR`] handle in which the created swapchain
    ///   object will be returned.
    ///# Description
    ///As mentioned above, if [`create_swapchain_khr`] succeeds, it will return a
    ///handle to a swapchain containing an array of at least
    ///`pCreateInfo->minImageCount` presentable images.While acquired by the application,
    /// presentable images  **can**  be used in any
    ///way that equivalent non-presentable images  **can**  be used.
    ///A presentable image is equivalent to a non-presentable image created with
    ///the following [`ImageCreateInfo`] parameters:The `pCreateInfo->surface` **must**  not be
    /// destroyed until after the
    ///swapchain is destroyed.If `pCreateInfo->oldSwapchain` is [`crate::Handle::null`], and the
    /// native
    ///window referred to by `pCreateInfo->surface` is already associated with
    ///a Vulkan swapchain, `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be
    ///returned.If the native window referred to by `pCreateInfo->surface` is already
    ///associated with a non-Vulkan graphics API surface,
    ///`VK_ERROR_NATIVE_WINDOW_IN_USE_KHR` **must**  be returned.The native window referred to by
    /// `pCreateInfo->surface` **must**  not become
    ///associated with a non-Vulkan graphics API surface before all associated
    ///Vulkan swapchains have been destroyed.[`create_swapchain_khr`] will return
    /// `VK_ERROR_DEVICE_LOST` if the
    ///logical device was lost.
    ///The [`SwapchainKHR`] is a child of the [`device`], and  **must**  not be
    ///destroyed before the [`device`].
    ///However, [`SurfaceKHR`] is not a child of any [`Device`] and is not
    ///affected by the lost device.
    ///After successfully recreating a [`Device`], the same [`SurfaceKHR`] **can**  be used to
    /// create a new [`SwapchainKHR`], provided the previous one
    ///was destroyed.If the `oldSwapchain` parameter of [`p_create_info`] is a valid
    ///swapchain, which has exclusive full-screen access, that access is released
    ///from `pCreateInfo->oldSwapchain`.
    ///If the command succeeds in this case, the newly created swapchain will
    ///automatically acquire exclusive full-screen access from
    ///`pCreateInfo->oldSwapchain`.In some cases, swapchain creation  **may**  fail if exclusive
    /// full-screen mode is
    ///requested for application control, but for some implementation-specific
    ///reason exclusive full-screen access is unavailable for the particular
    ///combination of parameters provided.
    ///If this occurs, `VK_ERROR_INITIALIZATION_FAILED` will be returned.When the [`SurfaceKHR`] in
    /// [`SwapchainCreateInfoKHR`] is a display
    ///surface, then the [`DisplayModeKHR`] in display surface’s
    ///[`DisplaySurfaceCreateInfoKHR`] is associated with a particular
    ///[`DisplayKHR`].
    ///Swapchain creation  **may**  fail if that [`DisplayKHR`] is not acquired by
    ///the application.
    ///In this scenario `VK_ERROR_INITIALIZATION_FAILED` is returned.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`SwapchainCreateInfoKHR`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_swapchain`] **must**  be a valid pointer to a [`SwapchainKHR`] handle
    ///
    ///## Host Synchronization
    /// - Host access to `pCreateInfo->surface` **must**  be externally synchronized
    /// - Host access to `pCreateInfo->oldSwapchain` **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`  -
    ///   `VK_ERROR_NATIVE_WINDOW_IN_USE_KHR`  - `VK_ERROR_INITIALIZATION_FAILED`
    ///# Related
    /// - [`khr_swapchain`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`SwapchainCreateInfoKHR`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateSwapchainKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_swapchain_khr<'lt>(
        self: &Unique<Device>,
        p_create_info: &SwapchainCreateInfoKHR<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SwapchainKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.create_swapchain_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.create_swapchain_khr())
            .unwrap_unchecked();
        let mut p_swapchain = MaybeUninit::<SwapchainKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_create_info as *const SwapchainCreateInfoKHR<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_swapchain.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_swapchain.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroySwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html) - Destroy a swapchain object
    ///# C Specifications
    ///To destroy a swapchain object call:
    ///```c
    ///// Provided by VK_KHR_swapchain
    ///void vkDestroySwapchainKHR(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    ///# Parameters
    /// - [`device`] is the [`Device`] associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to destroy.
    /// - [`p_allocator`] is the allocator used for host memory allocated for the swapchain object when there is no more specific allocator available (see [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)).
    ///# Description
    ///The application  **must**  not destroy a swapchain until after completion of all
    ///outstanding operations on images that were acquired from the swapchain.
    ///[`swapchain`] and all associated [`Image`] handles are destroyed, and
    /// **must**  not be acquired or used any more by the application.
    ///The memory of each [`Image`] will only be freed after that image is no
    ///longer used by the presentation engine.
    ///For example, if one image of the swapchain is being displayed in a window,
    ///the memory for that image  **may**  not be freed until the window is destroyed,
    ///or another swapchain is created for the window.
    ///Destroying the swapchain does not invalidate the parent [`SurfaceKHR`],
    ///and a new swapchain  **can**  be created with it.When a swapchain associated with a display
    /// surface is destroyed, if the
    ///image most recently presented to the display surface is from the swapchain
    ///being destroyed, then either any display resources modified by presenting
    ///images from any swapchain associated with the display surface  **must**  be
    ///reverted by the implementation to their state prior to the first present
    ///performed on one of these swapchains, or such resources  **must**  be left in
    ///their current state.If [`swapchain`] has exclusive full-screen access, it is released before
    ///the swapchain is destroyed.
    ///## Valid Usage
    /// - All uses of presentable images acquired from [`swapchain`] **must**  have completed
    ///   execution
    /// - If [`AllocationCallbacks`] were provided when [`swapchain`] was created, a compatible set
    ///   of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`swapchain`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`swapchain`] is not [`crate::Handle::null`], [`swapchain`] **must**  be a valid
    ///   [`SwapchainKHR`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters
    ///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    ///# Related
    /// - [`khr_swapchain`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroySwapchainKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_swapchain_khr<'lt>(
        self: &Unique<Device>,
        swapchain: Option<SwapchainKHR>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.destroy_swapchain_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.destroy_swapchain_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            swapchain.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl SwapchainKHR {
    ///[vkGetSwapchainImagesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html) - Obtain the array of presentable images associated with a swapchain
    ///# C Specifications
    ///To obtain the array of presentable images associated with a swapchain, call:
    ///```c
    ///// Provided by VK_KHR_swapchain
    ///VkResult vkGetSwapchainImagesKHR(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    uint32_t*                                   pSwapchainImageCount,
    ///    VkImage*                                    pSwapchainImages);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the swapchain to query.
    /// - [`p_swapchain_image_count`] is a pointer to an integer related to the number of
    ///   presentable images available or queried, as described below.
    /// - [`p_swapchain_images`] is either `NULL` or a pointer to an array of [`Image`] handles.
    ///# Description
    ///If [`p_swapchain_images`] is `NULL`, then the number of presentable images
    ///for [`swapchain`] is returned in [`p_swapchain_image_count`].
    ///Otherwise, [`p_swapchain_image_count`] **must**  point to a variable set by the
    ///user to the number of elements in the [`p_swapchain_images`] array, and on
    ///return the variable is overwritten with the number of structures actually
    ///written to [`p_swapchain_images`].
    ///If the value of [`p_swapchain_image_count`] is less than the number of
    ///presentable images for [`swapchain`], at most [`p_swapchain_image_count`]
    ///structures will be written, and `VK_INCOMPLETE` will be returned instead
    ///of `VK_SUCCESS`, to indicate that not all the available presentable
    ///images were returned.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - [`p_swapchain_image_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_swapchain_image_count`] is not `0`, and
    ///   [`p_swapchain_images`] is not `NULL`, [`p_swapchain_images`] **must**  be a valid pointer
    ///   to an array of [`p_swapchain_image_count`][`Image`] handles
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`khr_swapchain`]
    /// - [`Device`]
    /// - [`Image`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetSwapchainImagesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_swapchain_images_khr(
        self: &Unique<SwapchainKHR>,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: Option<usize>,
    ) -> VulkanResult<SmallVec<Unique<SwapchainImage>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.get_swapchain_images_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.get_swapchain_images_khr())
            .unwrap_unchecked();
        let mut p_swapchain_image_count = match p_swapchain_image_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.device().as_raw(), swapchain, &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_swapchain_images =
            SmallVec::<SwapchainImage>::from_elem(Default::default(), p_swapchain_image_count as usize);
        let _return = _function(
            self.device().as_raw(),
            swapchain,
            &mut p_swapchain_image_count,
            p_swapchain_images.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(
                _return,
                p_swapchain_images
                    .into_iter()
                    .map(|i| Unique::new(self, i, AtomicBool::default()))
                    .collect(),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkAcquireNextImageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html) - Retrieve the index of the next available presentable image
    ///# C Specifications
    ///To acquire an available presentable image to use, and retrieve the index of
    ///that image, call:
    ///```c
    ///// Provided by VK_KHR_swapchain
    ///VkResult vkAcquireNextImageKHR(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    uint64_t                                    timeout,
    ///    VkSemaphore                                 semaphore,
    ///    VkFence                                     fence,
    ///    uint32_t*                                   pImageIndex);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the non-retired swapchain from which an image is being acquired.
    /// - [`timeout`] specifies how long the function waits, in nanoseconds, if no image is
    ///   available.
    /// - [`semaphore`] is [`crate::Handle::null`] or a semaphore to signal.
    /// - [`fence`] is [`crate::Handle::null`] or a fence to signal.
    /// - [`p_image_index`] is a pointer to a `uint32_t` in which the index of the next image to use
    ///   (i.e. an index into the array of images returned by [`get_swapchain_images_khr`]) is
    ///   returned.
    ///# Description
    ///## Valid Usage
    /// - [`swapchain`] **must**  not be in the retired state
    /// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  be unsignaled
    /// - If [`semaphore`] is not [`crate::Handle::null`] it  **must**  not have any uncompleted
    ///   signal or wait operations pending
    /// - If [`fence`] is not [`crate::Handle::null`] it  **must**  be unsignaled and  **must**  not
    ///   be associated with any other queue command that has not yet completed execution on that
    ///   queue
    /// - [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::Handle::null`]
    /// - If the number of currently acquired images is greater than the difference between the
    ///   number of images in [`swapchain`] and the value of
    ///   [`SurfaceCapabilitiesKHR::min_image_count`] as returned by a call to
    ///   [`get_physical_device_surface_capabilities2_khr`] with the `surface` used to create
    ///   [`swapchain`], [`timeout`] **must**  not be `UINT64_MAX`
    /// - [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - If [`semaphore`] is not [`crate::Handle::null`], [`semaphore`] **must**  be a valid
    ///   [`Semaphore`] handle
    /// - If [`fence`] is not [`crate::Handle::null`], [`fence`] **must**  be a valid [`Fence`]
    ///   handle
    /// - [`p_image_index`] **must**  be a valid pointer to a `uint32_t` value
    /// - If [`semaphore`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    /// - If [`fence`] is a valid handle, it  **must**  have been created, allocated, or retrieved
    ///   from [`device`]
    /// - Both of [`device`], and [`swapchain`] that are valid handles of non-ignored parameters
    ///   **must**  have been created, allocated, or retrieved from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    /// - Host access to [`semaphore`] **must**  be externally synchronized
    /// - Host access to [`fence`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_TIMEOUT`  - `VK_NOT_READY`  - `VK_SUBOPTIMAL_KHR`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
    ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///# Related
    /// - [`khr_swapchain`]
    /// - [`Device`]
    /// - [`Fence`]
    /// - [`Semaphore`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkAcquireNextImageKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn acquire_next_image_khr(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
        timeout: Option<u64>,
        semaphore: Option<Semaphore>,
        fence: Option<Fence>,
    ) -> VulkanResult<u32> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.acquire_next_image_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.acquire_next_image_khr())
            .unwrap_unchecked();
        let mut p_image_index = Default::default();
        let _return = _function(
            self.as_raw(),
            swapchain,
            timeout.unwrap_or_default() as _,
            semaphore.unwrap_or_default(),
            fence.unwrap_or_default(),
            &mut p_image_index,
        );
        match _return {
            VulkanResultCodes::SUCCESS
            | VulkanResultCodes::TIMEOUT
            | VulkanResultCodes::NOT_READY
            | VulkanResultCodes::SUBOPTIMAL_KHR => VulkanResult::Success(_return, p_image_index),
            e => VulkanResult::Err(e),
        }
    }
}
impl SwapchainImage {
    ///[vkCreateImageView](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateImageView.html) - Create an image view from an existing image
    ///# C Specifications
    ///To create an image view, call:
    ///```c
    ///// Provided by VK_VERSION_1_0
    ///VkResult vkCreateImageView(
    ///    VkDevice                                    device,
    ///    const VkImageViewCreateInfo*                pCreateInfo,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkImageView*                                pView);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that creates the image view.
    /// - [`p_create_info`] is a pointer to a [`ImageViewCreateInfo`] structure containing
    ///   parameters to be used to create the image view.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_view`] is a pointer to a [`ImageView`] handle in which the resulting image view object
    ///   is returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_create_info`] **must**  be a valid pointer to a valid [`ImageViewCreateInfo`]
    ///   structure
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_view`] **must**  be a valid pointer to a [`ImageView`] handle
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`crate::vulkan1_0`]
    /// - [`AllocationCallbacks`]
    /// - [`Device`]
    /// - [`ImageView`]
    /// - [`ImageViewCreateInfo`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateImageView")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_swapchain_image_view<'lt>(
        self: &Unique<SwapchainImage>,
        p_create_info: &ImageViewCreateInfo<'lt>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<SwapchainImageView>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.create_swapchain_image_view())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.create_swapchain_image_view())
            .unwrap_unchecked();
        let mut p_view = MaybeUninit::<SwapchainImageView>::uninit();
        let _return = _function(
            self.device().as_raw(),
            p_create_info as *const ImageViewCreateInfo<'lt>,
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_view.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => {
                VulkanResult::Success(_return, Unique::new(self, p_view.assume_init(), AtomicBool::default()))
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl Queue {
    ///[vkQueuePresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html) - Queue an image for presentation
    ///# C Specifications
    ///After queueing all rendering commands and transitioning the image to the
    ///correct layout, to queue an image for presentation, call:
    ///```c
    ///// Provided by VK_KHR_swapchain
    ///VkResult vkQueuePresentKHR(
    ///    VkQueue                                     queue,
    ///    const VkPresentInfoKHR*                     pPresentInfo);
    ///```
    ///# Parameters
    /// - [`queue`] is a queue that is capable of presentation to the target surface’s platform on
    ///   the same device as the image’s swapchain.
    /// - [`p_present_info`] is a pointer to a [`PresentInfoKHR`] structure specifying parameters of
    ///   the presentation.
    ///# Description
    ///## Valid Usage
    /// - Each element of `pSwapchains` member of [`p_present_info`] **must**  be a swapchain that
    ///   is created for a surface for which presentation is supported from [`queue`] as determined
    ///   using a call to [`get_physical_device_surface_support_khr`]
    /// - If more than one member of `pSwapchains` was created from a display surface, all display
    ///   surfaces referenced that refer to the same display  **must**  use the same display mode
    /// - When a semaphore wait operation referring to a binary semaphore defined by the elements of
    ///   the `pWaitSemaphores` member of [`p_present_info`] executes on [`queue`], there  **must**
    ///   be no other queues waiting on the same semaphore
    /// - All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be semaphores
    ///   that are signaled, or have [semaphore signal operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling)
    ///   previously submitted for execution
    /// - All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  be created
    ///   with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
    /// - All elements of the `pWaitSemaphores` member of [`p_present_info`] **must**  reference a
    ///   semaphore signal operation that has been submitted for execution and any semaphore signal
    ///   operations on which it depends (if any)  **must**  have also been submitted for execution
    ///Any writes to memory backing the images referenced by the
    ///`pImageIndices` and `pSwapchains` members of [`p_present_info`],
    ///that are available before [`queue_present_khr`] is executed, are
    ///automatically made visible to the read access performed by the presentation
    ///engine.
    ///This automatic visibility operation for an image happens-after the semaphore
    ///signal operation, and happens-before the presentation engine accesses the
    ///image.Queueing an image for presentation defines a set of *queue operations*,
    ///including waiting on the semaphores and submitting a presentation request to
    ///the presentation engine.
    ///However, the scope of this set of queue operations does not include the
    ///actual processing of the image by the presentation engine.If [`queue_present_khr`] fails to
    /// enqueue the corresponding set of queue
    ///operations, it  **may**  return `VK_ERROR_OUT_OF_HOST_MEMORY` or
    ///`VK_ERROR_OUT_OF_DEVICE_MEMORY`.
    ///If it does, the implementation  **must**  ensure that the state and contents of
    ///any resources or synchronization primitives referenced is unaffected by the
    ///call or its failure.If [`queue_present_khr`] fails in such a way that the implementation is
    ///unable to make that guarantee, the implementation  **must**  return
    ///`VK_ERROR_DEVICE_LOST`.However, if the presentation request is rejected by the presentation
    /// engine
    ///with an error `VK_ERROR_OUT_OF_DATE_KHR`,
    ///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`,
    ///or `VK_ERROR_SURFACE_LOST_KHR`, the set of queue operations are still
    ///considered to be enqueued and thus any semaphore wait operation specified in
    ///[`PresentInfoKHR`] will execute when the corresponding queue operation
    ///is complete.Calls to [`queue_present_khr`] **may**  block, but  **must**  return in finite
    ///time.If any `swapchain` member of [`p_present_info`] was created with
    ///`VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`,
    ///`VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` will be returned if that
    ///swapchain does not have exclusive full-screen access, possibly for
    ///implementation-specific reasons outside of the application’s control.
    ///## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`p_present_info`] **must**  be a valid pointer to a valid [`PresentInfoKHR`] structure
    ///
    ///## Host Synchronization
    /// - Host access to [`queue`] **must**  be externally synchronized
    /// - Host access to `pPresentInfo->pWaitSemaphores`[]  **must**  be externally synchronized
    /// - Host access to `pPresentInfo->pSwapchains`[]  **must**  be externally synchronized
    ///
    ///## Command Properties
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_SUBOPTIMAL_KHR`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`  -
    ///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`
    ///# Related
    /// - [`khr_swapchain`]
    /// - [`PresentInfoKHR`]
    /// - [`Queue`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkQueuePresentKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn queue_present_khr<'lt>(
        self: &Unique<Queue>,
        p_present_info: &PresentInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.queue_present_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_swapchain()
            .and_then(|vtable| vtable.queue_present_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), p_present_info as *const PresentInfoKHR<'lt>);
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::SUBOPTIMAL_KHR => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///[VkSwapchainKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainKHR.html) - Opaque handle to a swapchain object
///# C Specifications
///A swapchain object (a.k.a.
///swapchain) provides the ability to present rendering results to a surface.
///Swapchain objects are represented by [`SwapchainKHR`] handles:
///```c
///// Provided by VK_KHR_swapchain
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSwapchainKHR)
///```
///# Related
/// - [`khr_swapchain`]
/// - [`AcquireNextImageInfoKHR`]
/// - [`BindImageMemorySwapchainInfoKHR`]
/// - [`ImageSwapchainCreateInfoKHR`]
/// - [`PresentInfoKHR`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`acquire_full_screen_exclusive_mode_ext`]
/// - [`acquire_next_image_khr`]
/// - [`create_shared_swapchains_khr`]
/// - [`create_swapchain_khr`]
/// - [`destroy_swapchain_khr`]
/// - [`get_past_presentation_timing_google`]
/// - [`get_refresh_cycle_duration_google`]
/// - [`get_swapchain_counter_ext`]
/// - [`get_swapchain_images_khr`]
/// - [`get_swapchain_status_khr`]
/// - [`queue_present_khr`]
/// - [`release_full_screen_exclusive_mode_ext`]
/// - [`set_hdr_metadata_ext`]
/// - [`set_local_dimming_amd`]
/// - [`wait_for_present_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSwapchainKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct SwapchainKHR(pub u64);
impl SwapchainKHR {
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
unsafe impl Send for SwapchainKHR {}
impl Default for SwapchainKHR {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for SwapchainKHR {
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
            self.device().destroy_swapchain_khr(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<SwapchainKHR> {
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
///[VkImage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImage.html) - Opaque handle to an image object
///# C Specifications
///Images represent multidimensional - up to 3 - arrays of data which  **can**  be
///used for various purposes (e.g. attachments, textures), by binding them to a
///graphics or compute pipeline via descriptor sets, or by directly specifying
///them as parameters to certain commands.Images are represented by [`Image`] handles:
///```c
///// Provided by VK_VERSION_1_0
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImage)
///```
///# Related
/// - [`crate::vulkan1_0`]
/// - [`BindImageMemoryInfo`]
/// - [`BlitImageInfo2`]
/// - [`CopyBufferToImageInfo2`]
/// - [`CopyImageInfo2`]
/// - [`CopyImageToBufferInfo2`]
/// - [`DedicatedAllocationMemoryAllocateInfoNV`]
/// - [`ImageMemoryBarrier`]
/// - [`ImageMemoryBarrier2`]
/// - [`ImageMemoryRequirementsInfo2`]
/// - [`ImageSparseMemoryRequirementsInfo2`]
/// - [`ImageViewCreateInfo`]
/// - [`MemoryDedicatedAllocateInfo`]
/// - [`ResolveImageInfo2`]
/// - [`SparseImageMemoryBindInfo`]
/// - [`SparseImageOpaqueMemoryBindInfo`]
/// - [`bind_image_memory`]
/// - [`cmd_blit_image`]
/// - [`cmd_clear_color_image`]
/// - [`cmd_clear_depth_stencil_image`]
/// - [`cmd_copy_buffer_to_image`]
/// - [`cmd_copy_image`]
/// - [`cmd_copy_image_to_buffer`]
/// - [`cmd_resolve_image`]
/// - [`create_image`]
/// - [`destroy_image`]
/// - [`get_image_drm_format_modifier_properties_ext`]
/// - [`get_image_memory_requirements`]
/// - [`get_image_sparse_memory_requirements`]
/// - [`get_image_subresource_layout`]
/// - [`get_swapchain_images_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImage")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct SwapchainImage(pub u64);
impl SwapchainImage {
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
unsafe impl Send for SwapchainImage {}
impl Default for SwapchainImage {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for SwapchainImage {
    type Parent = Unique<SwapchainKHR>;
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
    unsafe fn destroy(self: &mut Unique<Self>) {}
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<SwapchainImage> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent().parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///[VkImageView](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageView.html) - Opaque handle to an image view object
///# C Specifications
///Image objects are not directly accessed by pipeline shaders for reading or
///writing image data.
///Instead, *image views* representing contiguous ranges of the image
///subresources and containing additional metadata are used for that purpose.
///Views  **must**  be created on images of compatible types, and  **must**  represent a
///valid subset of image subresources.Image views are represented by [`ImageView`] handles:
///```c
///// Provided by VK_VERSION_1_0
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImageView)
///```
///# Related
/// - [`crate::vulkan1_0`]
/// - [`DescriptorImageInfo`]
/// - [`FramebufferCreateInfo`]
/// - [`ImageViewHandleInfoNVX`]
/// - [`RenderPassAttachmentBeginInfo`]
/// - [`RenderingAttachmentInfo`]
/// - [`RenderingFragmentDensityMapAttachmentInfoEXT`]
/// - [`RenderingFragmentShadingRateAttachmentInfoKHR`]
/// - [`VideoPictureResourceKHR`]
/// - [`cmd_bind_invocation_mask_huawei`]
/// - [`cmd_bind_shading_rate_image_nv`]
/// - [`create_image_view`]
/// - [`destroy_image_view`]
/// - [`get_image_view_address_nvx`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageView")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct SwapchainImageView(pub u64);
impl SwapchainImageView {
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
unsafe impl Send for SwapchainImageView {}
impl Default for SwapchainImageView {
    fn default() -> Self {
        Self::null()
    }
}
impl Handle for SwapchainImageView {
    type Parent = Unique<SwapchainImage>;
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
            self.device().destroy_image_view(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<SwapchainImageView> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent().parent().parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_swapchain`
pub struct DeviceKhrSwapchainVTable {
    ///See [`FNCreateSwapchainKhr`] for more information.
    pub create_swapchain_khr: FNCreateSwapchainKhr,
    ///See [`FNDestroySwapchainKhr`] for more information.
    pub destroy_swapchain_khr: FNDestroySwapchainKhr,
    ///See [`FNGetSwapchainImagesKhr`] for more information.
    pub get_swapchain_images_khr: FNGetSwapchainImagesKhr,
    ///See [`FNAcquireNextImageKhr`] for more information.
    pub acquire_next_image_khr: FNAcquireNextImageKhr,
    ///See [`FNQueuePresentKhr`] for more information.
    pub queue_present_khr: FNQueuePresentKhr,
    ///See [`FNCreateSwapchainImageView`] for more information.
    pub create_swapchain_image_view: FNCreateSwapchainImageView,
}
impl DeviceKhrSwapchainVTable {
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
            create_swapchain_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateSwapchainKHR").as_ptr()))
            },
            destroy_swapchain_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDestroySwapchainKHR").as_ptr()))
            },
            get_swapchain_images_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetSwapchainImagesKHR").as_ptr()))
            },
            acquire_next_image_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkAcquireNextImageKHR").as_ptr()))
            },
            queue_present_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkQueuePresentKHR").as_ptr()))
            },
            create_swapchain_image_view: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateImageView").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_swapchain_khr`]. See [`FNCreateSwapchainKhr`] for more information.
    pub fn create_swapchain_khr(&self) -> FNCreateSwapchainKhr {
        self.create_swapchain_khr
    }
    ///Gets [`Self::destroy_swapchain_khr`]. See [`FNDestroySwapchainKhr`] for more information.
    pub fn destroy_swapchain_khr(&self) -> FNDestroySwapchainKhr {
        self.destroy_swapchain_khr
    }
    ///Gets [`Self::get_swapchain_images_khr`]. See [`FNGetSwapchainImagesKhr`] for more
    /// information.
    pub fn get_swapchain_images_khr(&self) -> FNGetSwapchainImagesKhr {
        self.get_swapchain_images_khr
    }
    ///Gets [`Self::acquire_next_image_khr`]. See [`FNAcquireNextImageKhr`] for more information.
    pub fn acquire_next_image_khr(&self) -> FNAcquireNextImageKhr {
        self.acquire_next_image_khr
    }
    ///Gets [`Self::queue_present_khr`]. See [`FNQueuePresentKhr`] for more information.
    pub fn queue_present_khr(&self) -> FNQueuePresentKhr {
        self.queue_present_khr
    }
    ///Gets [`Self::create_swapchain_image_view`]. See [`FNCreateSwapchainImageView`] for more
    /// information.
    pub fn create_swapchain_image_view(&self) -> FNCreateSwapchainImageView {
        self.create_swapchain_image_view
    }
}
