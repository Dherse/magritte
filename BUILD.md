# Building `magritte-vk`

To build Magritte's Vulkan bindings, you will need to build the Vulkan documentation.
Then you will need to run `magritte-vkgen` to generate the actual bindings.

## Building the Vulkan doc

To build the Vulkan doc, you will need the following tools:

1. [Docker](https://www.docker.com/)

You will need to close this repository recursively in order to get the `vendors` folder filled with the `Vulkan-Docs` repository.

You will then need to run a docker bash:

```sh
docker run --user `id -u`:`id -g` -it --rm -v {path-to-vendors}/Vulkan-Docs:/vulkan khronosgroup/docker-images:asciidoctor-spec /bin bash
```

You will finally need to build the spec (in the opened docker bash)

```sh
cd /vulkan
./makeSpec -clean -spec all html
```

## Building the bindings

In the main directory run

```sh
cargo run --bin magritte-vkgen
```

Once this is done you have successfully built the bindings from source.