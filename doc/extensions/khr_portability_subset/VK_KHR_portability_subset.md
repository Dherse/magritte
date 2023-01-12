[VK_KHR_portability_subset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_portability_subset.html) - device extension

# Description
The `VK_KHR_portability_subset extension allows a non-conformant Vulkan
implementation to be built on top of another non-Vulkan graphics API, and
identifies differences between that implementation and a fully-conformant
native Vulkan implementation.This extension provides Vulkan implementations with the ability to mark
otherwise-required capabilities as unsupported, or to establish additional
properties and limits that the application should adhere to in order to
guarantee portable behaviour and operation across platforms, including
platforms where Vulkan is not natively supported.The goal of this specification is to document, and make queryable,
capabilities which are required to be supported by a fully-conformant Vulkan
1.0 implementation, but may be optional for an implementation of the Vulkan
1.0 Portability Subset.The intent is that this extension will be advertised only on implementations
of the Vulkan 1.0 Portability Subset, and not on conformant implementations
of Vulkan 1.0.
Fully-conformant Vulkan implementations provide all the required capabilies,
and so will not provide this extension.
Therefore, the existence of this extension can be used to determine that an
implementation is likely not fully conformant with the Vulkan spec.If this extension is supported by the Vulkan implementation, the application
must enable this extension.This extension defines several new structures that can be chained to the
existing structures used by certain standard Vulkan calls, in order to query
for non-conformant portable behavior.

# Registered extension number
164

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`
-  **This is a *provisional* extension and  **must**  be used with caution. See the [description](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#boilerplate-provisional-header) of provisional header files for enablement and stability details.**

# Contacts
- Bill Hollings [billhollings](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_portability_subset] @billhollings%0A<<Here describe the issue or question you have about the VK_KHR_portability_subset extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePortabilitySubsetFeaturesKHR`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePortabilitySubsetPropertiesKHR`]

# New constants
- `VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME`
- `VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR`

# Known issues & F.A.Q.
None.

# Version history
- Revision 1, 2020-07-21 (Bill Hollings)  - Initial draft.

# Other information
* 2020-07-21
* No known IP claims.
*   - Bill Hollings, The Brenwill Workshop Ltd.  - Daniel Koch, NVIDIA  - Dzmitry Malyshau, Mozilla  - Chip Davis, CodeWeavers  - Dan Ginsburg, Valve  - Mike Weiblen, LunarG  - Neil Trevett, NVIDIA  - Alexey Knyazev, Independent

# Related
- [`PhysicalDevicePortabilitySubsetFeaturesKHR`]
- [`PhysicalDevicePortabilitySubsetPropertiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        