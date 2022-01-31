# Magritte

Magritte will be a single-backend, asynchronous graphics API for rust. Inspired by [`wgpu-rs`](https://github.com/gfx-rs/wgpu) but designed for native Desktop use instead of compatibility with [*WebGPU*](https://www.w3.org/TR/webgpu/). The final API will be designed to enabled high performance graphics and compute applications with the latest features.

Magritte will exclusively target [*Vulkan*](https://www.vulkan.org/) as its backend. In the future, if *WebGPU* reaches similar features to *Vulkan*, I may work on supporting them as well. 

The goal of this limitation is to make it easier to support modern features such as [ray tracing](https://en.wikipedia.org/wiki/Ray_tracing_(graphics)).

## Feature flags

- `tokio`: Adds [tokio](https://tokio.rs) as a dependency ;
- `futures`: Adds [futures](https://docs.rs/futures/latest/futures/) as a dependency ;
- `async`: Enables asynchronous code, enables the `tokio` and `futures` features ;
- `render-graph`: Enables the render graph, enables the `async` feature ;
- `private-data`: Enables the the generic arguments for data types, `get(&self) -> Option<&T>`, `get_mut(&mut self) -> Option<&mut T>` and `set(&mut self, value: T)` functions that enables arbitrary data.

## State of the art

Currently, the rust ecosystem has a number of **great** crates for dealing with graphics and compute, this section gives a short comparison between Magritte and existing libraries. The crates compared to Magritte are the following:
- [ash]()
- [erupt]()
- [vulkano]()
- [wgpu]()
- [gfx-hal]()
- [glium](https://github.com/glium/glium), [glutin](https://github.com/rust-windowing/glutin) and others

### Ash & erupt

*Ash* and *erupt* are both low-level, unsafe bindings, mostly-generated bindings to the raw Vulkan API. This is similar to Magritte with a few notable exceptions:
- Magritte is a *mostly* safe API ;
- Magritte has more overhead than *ash* and *erupt* ;
- Magritte has higher-level features ;
- Magritte is idiomatic in the rust-sense not the Vulkan sense ;
- Magritte's bindings (`magritte-vk`) are fully documented using the official Vulkan documentation.

### Vulkano

*Vulkano* and Magritte are very similar, they share the design of being high level while trying to keep the overhead as small as possible. In addition, both libraries try to make Vulkan a more productive (i.e easier to use) API.

The main additions are the following:
- Magritte is mostly generated, most of time is spent writing the binding generator and not hand-writing bindings ;
- Magritte has garbage collection and will create threads something I believe vulkano does not do ;
- Magritte has full support for **all** Vulkano extensions, although safe usage is not guaranteed for all of them.

### WGPU-rs

*WGPU-rs* is amazing, it provides safe abstractions running on essentially every platform, while Magritte aims at only supporting Vulkan. This allows *wgpu* to run on more platforms including the web. This is **not** a goal for Magritte. However, Magritte still has a few advantages:
- Magritte is single backend ;
- Magritte provided very high level features such as a render graph ;
- Magritte does not follow a widely-compatible spec which allows for more features ;
- Magritte supports a lot more features than *wgpu* such as ray-tracing ;
- Magritte will (hopefully) be faster than *wgpu* due to having a single backend.

### GFX-hal

*GFX-hal* is a deprecated, unsafe, low-level, generic astraction over common graphics API. It is the foundation that led to the development of *wgpu-hal* that powers *wgpu*. It was quite powerful but differences between the different graphics API led to its development being difficult and eventually halted. Magritte has a few advantages compared to *GFX-hal*:
- Magritte is activelly developped, single backend, safe, high level ;
- Magritte is fully documented thanks to its documentation generator ;
- Magritte only follows a single API making it easier to make ;
- Magritte is mostly generated code.

### Glium, Glutin and others

Those are [OpenGL](https://www.opengl.org//) bindings and not *Vulkan* bindings. This means they are (very broad statements ahead that depend on the version of *OpenGL* and how the individual crates are built) higher-level, single-threaded, full of legacy code (*OpenGL* is at fault here). Magritte gives the following advantages:
- Magritte uses *Vulkan* not *OpenGL* ;
- *OpenGL* can be slow\[[1](https://www.reddit.com/r/Amd/comments/gex7mq/why_are_amds_opengl_driver_so_damn_slow_on/)\] on some platforms ;
- Magritte has support for advanced features found in *Vulkan* while *OpenGL* does not ;
- Magritte provides higher level features.

## Features

### Registry (**WIP**)

All handles are placed in a registry. This registry tracks the lifetime of this objects and allows garbage collection of the object when needed.

This means that handles provided by Magritte are IDs rather than the actual *Vulkan* handles. The specifics of synchronization of handles is handled by the registry itself. The registry also follows the uses of objects to allows rigorous garbage collection of objects.

In practice, you do not need to care about the lifetime of objects as the registry keeps track of ownership and lifetimes for you. The registry even takes into account the binding of handles to queue submissions. Allowing for garbage collection to occur when the handle is no longer used on both the CPU-side (your code) and the GPU-side. This is done by tracking [fences](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFence.html).

The registry therefore ensures that lifetimes defined by the *Vulkan* specifications are respected and that your usage of the API is always valid (from a lifetime perspective).

### Garbage collection (**WIP**)

Using the registry and fences, Magritte handles garbage collection of objects for you. This means that resources are deallocated in a timely manner and cannot be destroyed before it is valid to do so.

### Memory management

Magritte handles memory management by using the [*Vulkan Memory Allocator* (*VMA*)](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) using a set of custom bindings. *VMA* makes it easier to manage memory and handle allocations *properly*.

### Unions (**WIP**)

Magritte makes away with unions by using rust enums. This makes the code safer. There are very few unions in Vulkan and they all have hand-written implementations to make them safer.

### Render graph (**WIP**)

Magritte will come with a capable, multithreaded render graph. This render graph will be setup at runtime, with tasks being created using the [*tokio*](https://tokio.rs/) runtime.

This render graph will automatically handle inter-node synchronization, parallelism, access to resources, allocation of transient resources and resource layout transitions. Note that nodes must do all of this themselves for internal resources.

The render graph also handles framebuffer creation, optionally handles swapchain creation and recreation, swapchain and renderpass caches. It also makes the data from caches easily available to be stored into files.

The render graph is an optional feature enabled using the `render-graph` feature flag.

### Resource synchronization and layout changes

Helpers for handling resource synchronization and layout changes within a single node will be provided, using the [`VK_KHR_synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html) extension. These helpers should make it easier to write correct transitions and barriers. Hopefully eliminating the guess work that can happen when using these features.

### Asynchronous (**WIP**)

Most features blocking features in the API will be made optionally asynchronous. This will make it clearer when the API blocks or calls long functions and will make it easier to multithread the application.

[Queues](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#devsandqueues-queues) are always asynchronous and handled in their own threads. This **guarantees** that queues are always access synchronously. It also makes dealing with synchronization of queues a bit easier.

Asynchronous code can be disabled disabling the `async` feature flag.

### Shader agnostic (SPIRV) (**WIP**)

Magritte is shader agnostic. Meaning that you can use whathever shading language you prefer and pass-in [SPIR-V](https://www.khronos.org/spir/) to Magritte.

### `VK_EXT_private_data` (**WIP**)

Magritte expects the platform to have the [`VK_EXT_private_data`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_private_data.html) extension. This allows the API to have arbitrary data storage attached to *Vulkan* handles without the need for heavier data structures. This data is shared between all copies of the handle. 

This data will always contain an optional label for easier debugging. It will also enable the storage of arbitrary data passed using generic parameters.

The reason why the API uses this extension instead of [`VK_EXT_debug_utils`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_debug_utils.html) is due to its much wider platform support. Almost all recent GPUs with recent drivers support this feature. It has also been promoted to *Vulkan 1.3* which will further aid in its adoption. The only platform that has a poor track record of supporting this extension is *Android*. As Magritte targets desktop use, this is not a major issue.

### Levels (**WIP**)

Similar to the recently announced [Vulkan profiles](https://www.khronos.org/blog/vulkan-1.3-and-roadmap-2022), Magritte will use the abstraction of *levels*. A level is a pre-defined set of features that will be automatically detected and enabled, combined with guarantees for the hardware capabilities (limits). A game can therefore be created with a minimum profile and use different renderers or components based on which profiles the platform supports.

This abstraction will limit the number of variables the developper has to take into account as well as homogenise the features of the different platforms.

Currently, the design calls for 3 levels defined based on the Vulkan API:

1. Vulkan 1.1 support with a set of *basic* extensions
2. Vulkan 1.2 support with a set of additional extensions
3. Vulkan 1.2 support with a set of additional extensions and *with ray tracing*

These levels are expected to be similar to the capabilities of devices that would play modern titles on settings like
1. low, medium ;
2. high ;
3. ultra or ray-traced.

To learn more, have a look at the `level` folder containing [ron](https://github.com/ron-rs/ron) definitions of the levels.

### Builder pattern (**WIP**)

All structs in Magritte are highly generic and use the builder pattern to make them easier to create. These builders change and validate the generic arguments as you are building the structure. This makes the API less error-prone and ensures that required fiels are set.

### Readability and documentation (**WIP**)

The generated bindings are fully documented using the [*Vulkan* docs](https://github.com/KhronosGroup/Vulkan-Docs). They are formatted using [*rustfmt*](https://github.com/rust-lang/rustfmt). This ensures that the code is readable and understandable. The API is therefore transparent and can be understood clearly. The adds an extrea layer of complexity to the binding generator but makes working with the API much easier.

The documentation extends to all parts of the generated bindings, as in:
- structs and their generators ;
- handles ;
- extensions ;
- unions and their enums ;
- enums ;
- bit flags and bit sets ;
- functions ;
- function pointers ;
- base types ;
- opaque types.

### Error handling

The *Vulkan* spec (`vk.xml`) provides detailed information on each functions. These include the specific success and error codes that each function may return. Magritte uses the success codes to return a result for which the success may not be `VK_SUCCESS` but another success code such as `VK_TIMEOUT`. This means that errors returned by Magritte functions are actually hard errors and not additional results.