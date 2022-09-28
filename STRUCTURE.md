# Structure

The project is structured as follows:

- `magritte`: the library itself
- `magritte-gc`: a simple garbage collector for Magritte
- `magritte-vma`: Vulkan allocator based on VMA bindings for Magritte
- `magritte-derive`: derive proc macros for magritte (TODO)
- `magritte-shader`: build-script helpers for building shaders and pipeline layouts at compile time (TODO)
- `samples`: a set of samples showcasing the library
- `magritte-vkgen`: the binding generator for the *Vulkan* API