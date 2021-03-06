# ⚗️ Magritte

Magritte will be a single-backend, asynchronous rendering API for rust. Inspired by [`wgpu-rs`](https://github.com/gfx-rs/wgpu) but designed for native Desktop use instead of compatibility with [*WebGPU*](https://www.w3.org/TR/webgpu/). The final API will be designed to enabled high performance graphics and compute applications with the latest features.

Magritte will exclusively target [*Vulkan*](https://www.vulkan.org/) as its backend. In the future, if *WebGPU* reaches similar features to *Vulkan*, I may work on supporting them as well. 

The goal of this limitation is to make it easier to support modern features such as [ray tracing](https://en.wikipedia.org/wiki/Ray_tracing_(graphics)).

## Build instructions

Detailing binding generation instructions are present in `BUILD.md` and an overview of the project's structure is available in `STRUCTURE.md`.
Note that to generate the bindings, you will need docker and python. However, for Magritte itself, you don't need anything.

## Overview of features

- Low overhead
- Higher level (than raw *Vulkan*)
- Easier to write (than raw *Vulkan*)
- Memory management (using [*Vulkan Memory Allocator*](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator))
- Asynchronous (support for [*tokio*](https://tokio.rs))
- Fully documented using the official *Vulkan* documentation
- Almost fully generated
- Full support for **all** *Vulkan* extensions
- Full support for **desktop** and **portable** platforms: Windows *(to be tested)*, Linux *(to be tested)*, macOS *(to be tested)*
- Nicer error handling

## What magritte is and is not

Magritte is not really a set of bindings to *Vulkan*. It is instead a code generator that **generates** bindings to *Vulkan*. While this distinction is not really important for the user, it means that the bulk of the work is placed on the code generator (`magritte-vkgen`) rather than the bindings themselves as those are not manually written. The handwritten code is and **must** be a very small part of the library. The end goal of this library is to be easily updated along with the *Vulkan* specification which cannot be done by writing manual bindings.

It is clear when looking at the generated output that it would be almost impossible for a single person to write a full set of high level bindings complete with thorough documentation. It represents hundreds of thousands of lines of code that one person could not maintain. However, it is possible to do this for a subset of *Vulkan*, this is what [*vulkano*](https://github.com/vulkano-rs/vulkano) does. However, I wanted (*almost*) full support for the *Vulkan* specification with little to no compromise, hence I created these bindings.

## Feature flags

- `tokio`: Adds [tokio](https://tokio.rs) as a dependency ;
- `futures`: Adds [futures](https://docs.rs/futures/latest/futures/) as a dependency ;
- `async`: Enables asynchronous code, enables the `tokio` and `futures` features ;
- `render-graph`: Enables the render graph, enables the `async` feature ;
- `libloading`: Enables automatic run-time loading of the Vulkan lib ;
- `log`: Enables logging in some areas ;
- `smallvec`: Enables `smallvec` avoiding most allocations inside of Magritte ;
- `window`: Enables surface easier creation ;
- `validation`: Enables helpers for Vulkan validation layers ;
- `all`: Enables **all** Vulkan features ;
- `VK_**`: Enables a given Vulkan feature.

## State of the art

Currently, the rust ecosystem has a number of **great** crates for dealing with graphics and compute, this section gives a short comparison between Magritte and existing libraries. The crates compared to Magritte are the following:
- [*ash*](https://github.com/MaikKlein/ash)
- [*erupt*](https://crates.io/crates/erupt)
- [*vulkano*](https://github.com/vulkano-rs/vulkano)
- [*wgpu*](https://github.com/gfx-rs/wgpu)
- [*gfx-hal*](https://github.com/gfx-rs/gfx)
- [*glium*](https://github.com/glium/glium), [*glutin*](https://github.com/rust-windowing/glutin) and others

### Ash & erupt

*Ash* and *erupt* are both low-level, unsafe, mostly-generated bindings to the raw *Vulkan* API. This is similar to Magritte with a few notable exceptions:
- Magritte has higher-level features ;
- Magritte is a rendering library, not just raw bindings ;
- Magritte's bindings (`magritte-vk`) are fully documented using the official *Vulkan* documentation.

### Vulkano

*Vulkano* and Magritte are very similar, they share the design of being higher level while trying to keep the overhead as small as possible. In addition, both libraries try to make *Vulkan* a more productive (i.e easier to use) API.

The main additions are the following:
- Magritte is mostly generated, most of time is spent writing the binding generator and not hand-writing bindings ;
- Magritte can automate synchronizations, allocations, shader compilations and more using `magritte` with `magritte-vk` ;
- Magritte has full support for **all** *Vulkan* extensions, although safe usage is not guaranteed for all of them.

### WGPU-rs

*WGPU-rs* is amazing, it provides safe abstractions running on essentially every platform, while Magritte aims at only supporting *Vulkan*. This allows *wgpu* to run on more platforms including the web. This is **not** a goal for Magritte. However, Magritte still has a few advantages:
- Magritte is single backend and therefore should be more uniform across targets ;
- Magritte provides very high level features such as a render graph ;
- Magritte does not follow a widely-compatible spec which allows for more features ;
- Magritte supports a lot more features than *wgpu* currently does such as ray-tracing ;
- Magritte will (hopefully) be faster than *wgpu* due to having a single backend.

### GFX-hal

*GFX-hal* is a deprecated, unsafe, low-level, generic astraction over common graphics API. It is the foundation that led to the development of *wgpu-hal* that powers *wgpu*. It was quite powerful but differences between the different graphics API led to its development being difficult and eventually halted. Magritte has a few advantages compared to *GFX-hal*:
- Magritte is activelly developped, single backend, high level ;
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

### Memory management

Magritte handles memory management by using the [*Vulkan Memory Allocator* (*VMA*)](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) using a set of custom bindings. *VMA* makes it easier to manage memory and handle allocations *properly*.

```rust
    // Assuming you have created an allocator, it is as simple as:
    let allocator = Allocator::new(&device, None, None)?;

    // Then, we need to gather the information for creating the buffer,
    // in our case:
    //  - the size in bytes
    //  - the usage
    //  - the fact that this buffer will only get accessed from a single queue
    let buffer_info = BufferCreateInfo::default()
        .with_size(1024)
        .with_usage(BufferUsageFlags::INDEX_BUFFER)
        .with_sharing_mode(SharingMode::EXCLUSIVE);

    // Then, we create the buffer with the following information:
    //  - the buffer create info
    //  - the creation flags (empty in most cases)
    //  - the usage
    //  - the memory type or `None` to let VMA decide
    //  - the priority or `None` to use the default value
    //  - the user data as a void pointer or `None` for no user data
    let buffer = vulkan.allocator().create_buffer(
        &buffer_info,
        magritte_vma::AllocationCreateFlags::empty(),
        magritte_vma::BufferUsage::Flags {
            required: MemoryPropertyFlags::HOST_VISIBLE,
            preferred: MemoryPropertyFlags::DEVICE_LOCAL | MemoryPropertyFlags::HOST_COHERENT,
        },
        None,
        None,
        None,
    )?;
```

### Render graph (**WIP**)

Magritte will comes with a capable, multithreaded render graph. This render graph will be setup at runtime, with tasks being created using the [*tokio*](https://tokio.rs/) runtime.

This render graph will automatically handle inter-node synchronization, parallelism, access to resources, allocation of transient resources and resource layout transitions. Note that nodes must do all of this themselves for internal resources.

The render graph also handles framebuffer creation, optionally handles swapchain creation and recreation, swapchain and renderpass caches. It also makes the data from caches easily available to be stored into files.

### Resource synchronization and layout changes

Helpers for handling resource synchronization and layout changes within a single node will be provided, using the [`VK_KHR_synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html) extension. These helpers should make it easier to write correct transitions and barriers. Hopefully eliminating the guess work that can happen when using these features.

### Asynchronous (**WIP**)

Most features blocking features in the API will be made optionally asynchronous. This will make it clearer when the API blocks or calls long functions and will make it easier to multithread the application.

Asynchronous code can be disabled by disabling the `async` feature flag.

### Builder pattern (**WIP**)

All structs in Magritte are highly generic and use the builder pattern to make them easier to create. These builders change and validate the generic arguments as you are building the structure. This makes the API less error-prone and ensures that required fiels are set.

### Readability and documentation (**WIP**)

The generated bindings are fully documented using the [*Vulkan* docs](https://github.com/KhronosGroup/Vulkan-Docs). They are formatted using [*rustfmt*](https://github.com/rust-lang/rustfmt). This ensures that the code is readable and understandable. The API is therefore transparent and can be understood clearly. The adds an extra layer of complexity to the binding generator but makes working with the API much easier.

The documentation extends to all parts of the generated bindings, as in:
- vulkan versions ;
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

### Native format support (future feature)

One of the end goals for magritte is to support all common Vulkan formats using generated struct and code along with some crates for decoding and encoding compressed formats. All of this will be available using feature flag so that it may remain optional.

## Status

### Code generation

Here are the following families that need to be implemented in the code generator (crossed items refer to deliberately unsupported features):

- [ ] Documentation for Vulkan version
- [x] Documentation for extensions
- [x] Handles
- [x] Higher level handles (`Unique`)
- [x] Loaders
- [x] Extensions (doc only)
- [x] Opaque types (aliases of `c_void`)
- [x] ~Type aliases~
- [x] Structs
- [x] Structure pointer chains
- [x] ~Struct builders~
- [x] Unions
- [x] Function pointers
- [x] Base types
- [x] Constants
- [x] Constant aliases - no code generated since we ignore all aliases
- [x] Enums
- [x] Bit masks
- [x] Bit mask flags
- [x] ~Function/command aliases~ 
- [x] Functions
- [ ] Commands & command buffers
- [x] Conditional compilation
- [x] Fixing naming issues
- [x] Dedicated `VkImage` for `VkSwapchainKHR`
- [x] ~Remove `khr` in function name~
- [x] Function alias flattening (**huge milestone**)
- [x] Implement Debug for enums and bitflags.
- [ ] Refactor the code generator to group all edge-cases into one easily accessible module, currently edge cases are spread throughout the project.
- [ ] Refactor function generation to make it cleaner and integrated within `Function`, `FunctionArgument`.
- [x] Fix Debug of masks which is broken
- [x] Improve flag/bit/enum naming when numbers are involved, it's a huge mess right now