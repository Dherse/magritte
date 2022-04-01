# Publishing instructions

After building the bindings following the instructions in `BUILD.md`, you need to run a couple more steps before publishing:

## Regenerate the video bindings

Execute the following commands:
```sh
    cd magritte
    cargo build --features video_bindgen
```

## Low hanging fruits

Go in and manually fix the small number of remaining warnings (typically unused parens).