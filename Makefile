all: ron doc

ron: magritte.ron
	cargo run --bin magritte-ron-build --release -- ./vendors/Vulkan-Headers/registry/vk.xml ./magritte.ron

doc: ron
	cargo run --bin magritte-doc-build --release -- ./magritte.ron ./vendors/Vulkan-Docs/gen/out/man/html ./doc