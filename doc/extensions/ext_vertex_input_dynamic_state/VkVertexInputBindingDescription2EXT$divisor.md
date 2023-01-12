[`divisor`] is the number of successive instances that will use the
same value of the vertex attribute when instanced rendering is enabled.
This member  **can**  be set to a value other than `1` if the
[vertexAttributeInstanceRateDivisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateDivisor) feature is enabled.
For example, if the divisor is N, the same vertex attribute will be
applied to N successive instances before moving on to the next vertex
attribute.
The maximum value of [`divisor`] is implementation-dependent and can
be queried using
[`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]::`maxVertexAttribDivisor`.
A value of `0` **can**  be used for the divisor if the
[`vertexAttributeInstanceRateZeroDivisor`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-vertexAttributeInstanceRateZeroDivisor)
feature is enabled.
In this case, the same vertex attribute will be applied to all
instances.