# libdav2d bindings [![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

It is a simple FFI binding and safe abstraction over [dav2d][1].


## Building

To build the code, always have a look at [CI](https://github.com/rust-av/dav2d-rs/blob/master/.github/workflows/dav2d.yml) to install the necessary dependencies on all
supported operating systems.

### Overriding the dav2d library

The bindings use [system-deps](https://docs.rs/system-deps) to find dav2d. You may override the `PKG_CONFIG_PATH` or
direcly set the env vars `SYSTEM_DEPS_DAV2D_SEARCH_NATIVE` and/or `SYSTEM_DEPS_DAV2D_LIB`.

## Building with vcpkg for Windows x64

To build with [vcpkg](https://vcpkg.io/en/index.html), you need to follow these
steps:

1. Install `pkg-config` through [chocolatey](https://chocolatey.org/)

       choco install pkgconfiglite

2. Install `dav2d`

       vcpkg install dav2d:x64-windows

3. Add to the `PKG_CONFIG_PATH` environment variable the path `$VCPKG_INSTALLATION_ROOT\installed\x64-windows\lib\pkgconfig`

4. Build code

       cargo build --workspace

To speed up the computation, you can build your packages only in `Release` mode
adding the `set(VCPKG_BUILD_TYPE release)` line to the
`$VCPKG_INSTALLATION_ROOT\triplets\x64-windows.cmake` file.

Building for Windows x86 is the same, just replace `x64` with `x86` in the
steps above.

## Supported versions

The bindings require dav2d>=0.1.0

## TODO
- [x] Simple bindings
- [x] Safe abstraction
- [ ] Examples

[1]: https://code.videolan.org/videolan/dav2d
