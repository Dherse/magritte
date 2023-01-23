all: ron doc

ron: magritte.ron
	cargo run --bin magritte-ron-build --release -- ./vendors/Vulkan-Headers/registry/vk.xml ./magritte.ron

doc: ron
	cargo run --bin magritte-doc-build --release -- ./magritte.ron ./vendors/Vulkan-Docs/gen/out/man/html ./doc

core:
	cargo run --bin magritte-build --release -- ./magritte.ron ./doc ./magritte/generated

clean:
	cargo clean
	rm -R ./doc
	rm -R ./magritte.ron
	rm -R ./backend/magritte-native-backend/generated
	rm -R ./magritte/generated