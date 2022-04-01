# Publishing instructions

After building the bindings following the instructions in `BUILD.md`, you need to run a couple more steps before publishing:

## Regenerate the video bindings

Execute the following commands:
```sh
    cd magritte
    cargo build --features video_bindgen
```

## Low hanging fruits

To fix the easily auto-fixed warnings in the generated code, using the following command:

```sh
    cargo fix --allow-dirty
```