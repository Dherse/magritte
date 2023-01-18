all: ron doc

ron: magritte.ron
	cargo run --bin magritte-ron-build --release -- ./vendors/Vulkan-Headers/registry/vk.xml ./magritte.ron

doc: ron
	cargo run --bin magritte-doc-build --release -- ./magritte.ron ./vendors/Vulkan-Docs/gen/out/man/html ./doc

native-backend:
	cargo run --bin magritte-native-backend-build --release -- ./magritte.ron ./doc ./backend/magritte-native-backend/generated

clean:
	cargo clean
	rm ./doc
	rm ./magritte.ron
	rm ./backend/magritte-native-backend/generated