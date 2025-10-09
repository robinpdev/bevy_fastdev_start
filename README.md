# Bevy start template for fast iteration / developer experience

## Features
- Dynamically linking the engine (faster compile times, smaller executable)
- Hot patching using dioxus (You can make and run changes to the code without fully recompiling / restarting the executable)
- Hot reloading of assets (shaders, images, ...) using [bevy's builtin hot reloading system](https://bevy-cheatbook.github.io/assets/hot-reload.html) 
- Optimized for compile speed (Mold linker, Cranelift codegen, Nightly rust compiler)
- ImGUI debug UI
- Release build optimized for size (about 82MB)

## Prerequisites
- Linux development environment (Can compile to all other supported bevy platforms, but some development tooling is Linux-specific)
- Install the Mold linker, clang and Cranelift (instructions on [the bevy site](https://bevy.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional) )
- install dioxus: ```cargo install dioxus-cli@0.7.0-alpha.1```
- install just: ```cargo install just```

## running
- To start running your debug build, run ```just runhot```
- If you want to hot patch a function, annotate it with 
- To restart the entire scene (e.g. when you changed the setup function), press R in the bevy window



# Bumperr

Cross platform procedural + interactive motion graphics toolkit
### Define graphical operators with code and combine them using a node graph and compositor

### export options:
- Executable (Linux, Mac, Windows)
- Web using webassembly

## why?
There are some distinguishing features that are not available in some motion graphics programs:
- Web export
- Interactivity possible
- Native executables (better performance)
- 2D/3D compositor
- Intuitive usage

## Roadmap
- Dynamically add module instances
- Dynamic parameter updating (using gui)
- Operator chaining
- Write standard operators
    - noise
    - gradient
    - displacement map
    - shapes
    - glow
- compositor features (layering, visibility, ...)