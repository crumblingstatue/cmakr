# CMakr

CMakr is a tool to conveniently build/run your cmake projects without having to manually
switch to the appropriate directories.

## `cmakr.toml`

To use CMakr, first you should define a `cmakr.toml` in the root directory of your project
(where the top-level `CMakeLists.txt` resides).

Here is an example `cmakr.toml` for an imaginary video game called shootr:

```TOML
# The default binary to run. This is optional.
default-bin = "shootr"
# The default target to build. Optional.
default-target = "debug"

# A target definition
[target.debug]
# Arguments to be passed to CMake
args = ["-DCMAKE_BUILD_TYPE=Debug"]
# Build command to use. Could be either `make`, `ninja`, etc.
build = "make"

# Another target, just for the heck of it.
[target.release]
args = ["-DCMAKE_BUILD_TYPE=Release"]
build = "make"
```

## Command line usage

```sh
# Build the `release` target
cmakr release
# If you have `default-target` defined in `cmakr.toml`, you can just do
cmakr
# Build and run the `shootr` executable of the `release` target
cmakr release -r shootr
# Now if you have both `default-target` and `default-bin` defined in `cmakr.toml`, you can simply
cmakr -r
```

That's pretty much it in a nutshell.
