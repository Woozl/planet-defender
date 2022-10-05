# Planet Defender 

## Made with Rust + WGPU + WASM ([Play here!](https://davidglymph.com/planet-defender))

<br>

![Planet Defender](sc.gif)
 
## Build from Source
Clone the repo:
```
git clone git@github.com:Woozl/planet-defender.git && cd planet-defender
```

Build with cargo (requires [rustup](https://www.rust-lang.org/tools/install))
```
cargo r
```

## To build WASM ES6 module
(requires [wasm-pack](https://rustwasm.github.io/wasm-pack/))
```
wasm-pack build -t web
```