[VK_KHR_spirv_1_4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_spirv_1_4.html) - device extension

# Description
This extension allows the use of SPIR-V 1.4 shader modules.
SPIR-V 1.4’s new features primarily make it an easier target for compilers
from high-level languages, rather than exposing new hardware functionality.SPIR-V 1.4 incorporates features that are also available separately as
extensions.
SPIR-V 1.4 shader modules do not need to enable those extensions with the
`OpExtension` opcode, since they are integral parts of SPIR-V 1.4.SPIR-V 1.4 introduces new floating point execution mode capabilities, also
available via `SPV_KHR_float_controls`.
Implementations are not required to support all of these new capabilities;
support can be queried using
[`PhysicalDeviceFloatControlsPropertiesKHR`] from the
`[`khr_shader_float_controls`]` extension.

# Registered extension number
237

# Revision
1

# Dependencies
- Requires Vulkan 1.1
- Requires `[`khr_shader_float_controls`]`

# Deprecation state
- *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_spirv_1_4] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_spirv_1_4 extension>>)

# New constants
- `VK_KHR_SPIRV_1_4_EXTENSION_NAME`
- `VK_KHR_SPIRV_1_4_SPEC_VERSION`

# Known issues & F.A.Q.
1.
Should we have an extension specific to this SPIR-V version, or add a
version-generic query for SPIR-V version? SPIR-V 1.4 does not need any other
API changes. **RESOLVED** : Just expose SPIR-V 1.4.Most new SPIR-V versions introduce optionally-required capabilities or have
implementation-defined limits, and would need more API and specification
changes specific to that version to make them available in Vulkan.
For example, to support the subgroup capabilities added in SPIR-V 1.3
required introducing [`PhysicalDeviceSubgroupProperties`] to allow
querying the supported group operation categories, maximum supported
subgroup size, etc.
While we could expose the parts of a new SPIR-V version that do not need
accompanying changes generically, we will still end up writing extensions
specific to each version for the remaining parts.
Thus the generic mechanism will not reduce future spec-writing effort.
In addition, making it clear which parts of a future version are supported
by the generic mechanism and which cannot be used without specific support
would be difficult to get right ahead of time.2.
Can different stages of the same pipeline use shaders with different SPIR-V
versions? **RESOLVED** : Yes.Mixing SPIR-V versions 1.0-1.3 in the same pipeline has not been disallowed,
so it would be inconsistent to disallow mixing 1.4 with previous versions..
SPIR-V 1.4 does not introduce anything that should cause new difficulties
here.3.
Must Vulkan extensions corresponding to SPIR-V extensions that were promoted
to core in 1.4 be enabled in order to use that functionality in a SPIR-V 1.4
module? **RESOLVED** : No, with caveats.The SPIR-V 1.4 module does not need to declare the SPIR-V extensions, since
the functionality is now part of core, so there is no need to enable the
Vulkan extension that allows SPIR-V modules to declare the SPIR-V extension.
However, when the functionality that is now core in SPIR-V 1.4 is optionally
supported, the query for support is provided by a Vulkan extension, and that
query can only be used if the extension is enabled.This applies to any SPIR-V version; specifically for SPIR-V 1.4 this only
applies to the functionality from `SPV_KHR_float_controls`, which was made
available in Vulkan by `[`khr_shader_float_controls`]`.
Even though the extension was promoted in SPIR-V 1.4, the capabilities are
still optional in implementations that support [`khr_spirv_1_4`].A SPIR-V 1.4 module does not need to enable `SPV_KHR_float_controls` in
order to use the capabilities, so if the application has *a priori*
knowledge that the implementation supports the capabilities, it does not
need to enable `[`khr_shader_float_controls`]`.
However, if it does not have this knowledge and has to query for support at
runtime, it must enable `[`khr_shader_float_controls`]` in order to
use [`PhysicalDeviceFloatControlsPropertiesKHR`].

# Version history
- Revision 1, 2019-04-01 (Jesse Hall)  - Internal draft versions

# Other information
* 2019-04-01
* No known IP claims.
*   - Requires SPIR-V 1.4.  - Promoted to Vulkan 1.2 Core 
*   - Alexander Galazin, Arm  - David Neto, Google  - Jesse Hall, Google  - John Kessenich, Google  - Neil Henning, AMD  - Tom Olson, Arm
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        