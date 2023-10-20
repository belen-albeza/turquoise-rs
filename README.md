# turquoise-rs

Port to Rust + Wasm of [Turquoise](https://wiki.xxiivv.com/site/turquoise.html), a plotting VM originally made for Uxn ([source code here](https://git.sr.ht/~rabbits/turquoise)).

You can see a live build on https://belen-albeza.github.io/turquoise-rs

## Status

Implemented opcodes:

- [ ] PushPop (currently noop)
- [x] Move
- [x] Flip
- [x] Color
- [x] Draw
- [x] Mirror
- [ ] Scale (currently noop)

## Build

You need [cargo](https://doc.rust-lang.org/cargo/) and the [wasm-bindgen CLI](https://github.com/rustwasm/wasm-bindgen). The included `build.sh` works in a unix-like system.

```zsh
./build.sh
```

The output will be copied in a `dist` directory.
